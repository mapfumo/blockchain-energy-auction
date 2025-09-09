use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tokio::{
    net::{TcpListener, UdpSocket},
    sync::RwLock,
    time::{interval, sleep},
};
use tower_http::cors::CorsLayer;
use tracing::{info, warn, error};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Aggregator {
    pub aggregator_id: String,
    pub node_type: String,
    pub strategy: String,
    pub max_bid_price: u32,
    pub reputation_score: u32,
    pub successful_settlements: u32,
    pub total_energy_traded: f64,
    pub total_usdc_paid: u64,
    pub is_online: bool,
    pub last_activity: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AggregatorStatus {
    pub aggregator_id: String,
    pub strategy: String,
    pub reputation_score: u32,
    pub successful_settlements: u32,
    pub total_energy_traded: f64,
    pub total_usdc_paid: u64,
    pub available_bess_nodes: usize,
    pub pending_bids: usize,
    pub is_online: bool,
    pub last_activity: u64,
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscoveryMessage {
    pub r#type: String,
    pub aggregator_id: String,
    pub strategy: String,
    pub max_bid_price: u32,
    pub reputation_score: u32,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BESSDiscovery {
    pub r#type: String,
    pub node_id: String,
    pub node_type: String,
    pub capacity_kwh: f64,
    pub energy_level: f64,
    pub reserve_price: u32,
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuctionStarted {
    pub r#type: String,
    pub auction_id: String,
    pub total_energy: f64,
    pub reserve_price: u32,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BidPlaced {
    pub r#type: String,
    pub aggregator_id: String,
    pub auction_id: String,
    pub bid_price: u32,
    pub bid_amount: u64,
    pub total_energy: f64,
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BidAccepted {
    pub r#type: String,
    pub auction_id: String,
    pub energy_amount: f64,
    pub final_price: u32,
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BidRejected {
    pub r#type: String,
    pub auction_id: String,
    pub reason: String,
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub aggregator_id: String,
    pub strategy: String,
    pub reputation_score: u32,
    pub successful_settlements: u32,
    pub is_online: bool,
}

pub struct AppState {
    pub aggregator: Arc<RwLock<Aggregator>>,
    pub available_bess_nodes: Arc<RwLock<HashMap<String, BESSDiscovery>>>,
    pub pending_bids: Arc<RwLock<HashMap<String, BidPlaced>>>,
    pub gateway_host: String,
    pub gateway_port: u16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Get environment variables
    let aggregator_id = std::env::var("AGGREGATOR_ID").unwrap_or_else(|_| "001".to_string());
    let strategy = std::env::var("STRATEGY").unwrap_or_else(|_| "CONSERVATIVE".to_string());
    let max_bid_price = std::env::var("MAX_BID_PRICE")
        .unwrap_or_else(|_| "800".to_string())
        .parse::<u32>()?;
    let gateway_host = std::env::var("GATEWAY_HOST").unwrap_or_else(|_| "gateway".to_string());
    let gateway_port = std::env::var("GATEWAY_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()?;
    let multicast_group = std::env::var("MULTICAST_GROUP").unwrap_or_else(|_| "224.0.0.1".to_string());
    let multicast_port = std::env::var("MULTICAST_PORT")
        .unwrap_or_else(|_| "8888".to_string())
        .parse::<u16>()?;

    // Create Aggregator
    let aggregator = Aggregator {
        aggregator_id: aggregator_id.clone(),
        node_type: "AGGREGATOR".to_string(),
        strategy: strategy.clone(),
        max_bid_price,
        reputation_score: 50,
        successful_settlements: 0,
        total_energy_traded: 0.0,
        total_usdc_paid: 0,
        is_online: true,
        last_activity: current_timestamp(),
    };

    let app_state = Arc::new(AppState {
        aggregator: Arc::new(RwLock::new(aggregator)),
        available_bess_nodes: Arc::new(RwLock::new(HashMap::new())),
        pending_bids: Arc::new(RwLock::new(HashMap::new())),
        gateway_host,
        gateway_port,
    });

    info!("Aggregator {} initialized:", aggregator_id);
    info!("  Strategy: {}", strategy);
    info!("  Max Bid Price: {}¢/kWh", max_bid_price);
    info!("  Gateway: {}:{}", app_state.gateway_host, app_state.gateway_port);

    // Start services
    let state_clone = app_state.clone();
    tokio::spawn(async move {
        register_with_gateway(state_clone).await;
    });

    let state_clone = app_state.clone();
    tokio::spawn(async move {
        start_bess_querying(state_clone).await;
    });

    let state_clone = app_state.clone();
    tokio::spawn(async move {
        start_health_monitoring(state_clone).await;
    });

    // Start HTTP server
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/auction-started", post(handle_auction_started))
        .route("/bid-accepted", post(handle_bid_accepted))
        .route("/bid-rejected", post(handle_bid_rejected))
        .layer(CorsLayer::permissive())
        .with_state(app_state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8082));
    info!("HTTP server starting on {}", addr);

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_check(State(state): State<Arc<AppState>>) -> Result<Json<HealthResponse>, StatusCode> {
    let aggregator = state.aggregator.read().await;
    Ok(Json(HealthResponse {
        status: "healthy".to_string(),
        aggregator_id: aggregator.aggregator_id.clone(),
        strategy: aggregator.strategy.clone(),
        reputation_score: aggregator.reputation_score,
        successful_settlements: aggregator.successful_settlements,
        is_online: aggregator.is_online,
    }))
}

async fn handle_auction_started(
    State(state): State<Arc<AppState>>,
    Json(auction): Json<AuctionStarted>,
) -> Result<Json<BidPlaced>, StatusCode> {
    let mut aggregator = state.aggregator.write().await;
    aggregator.last_activity = current_timestamp();

    info!("Auction {} started: {} kWh at {}¢/kWh", 
          auction.auction_id, auction.total_energy, auction.reserve_price);

    // Calculate bid price based on strategy
    let bid_price = calculate_bid_price(auction.reserve_price, &aggregator.strategy);
    
    if bid_price > 0 {
        let bid_amount = (auction.total_energy * bid_price as f64) as u64;
        
        let bid = BidPlaced {
            r#type: "BID_PLACED".to_string(),
            aggregator_id: aggregator.aggregator_id.clone(),
            auction_id: auction.auction_id.clone(),
            bid_price,
            bid_amount,
            total_energy: auction.total_energy,
            timestamp: current_timestamp(),
        };

        // Store pending bid
        state.pending_bids.write().await.insert(auction.auction_id.clone(), bid.clone());

        info!("Placing bid on auction {}: {}¢/kWh (${:.2})", 
              auction.auction_id, bid_price, bid_amount as f64 / 100.0);

        Ok(Json(bid))
    } else {
        info!("Not bidding on auction {} (strategy: {})", auction.auction_id, aggregator.strategy);
        Err(StatusCode::NO_CONTENT)
    }
}

async fn handle_bid_accepted(
    State(state): State<Arc<AppState>>,
    Json(bid): Json<BidAccepted>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let mut aggregator = state.aggregator.write().await;
    
    info!("Bid accepted for auction {}: {} kWh at {}¢/kWh", 
          bid.auction_id, bid.energy_amount, bid.final_price);
    
    // Update aggregator stats
    aggregator.successful_settlements += 1;
    aggregator.total_energy_traded += bid.energy_amount;
    aggregator.total_usdc_paid += (bid.energy_amount * bid.final_price as f64) as u64;
    aggregator.reputation_score = (aggregator.reputation_score + 1).min(100);
    aggregator.last_activity = current_timestamp();

    // Remove from pending bids
    state.pending_bids.write().await.remove(&bid.auction_id);

    let response = serde_json::json!({
        "type": "BID_ACCEPTED_ACK",
        "aggregator_id": aggregator.aggregator_id,
        "auction_id": bid.auction_id,
        "energy_amount": bid.energy_amount,
        "final_price": bid.final_price,
        "timestamp": current_timestamp()
    });

    Ok(Json(response))
}

async fn handle_bid_rejected(
    State(state): State<Arc<AppState>>,
    Json(rejection): Json<BidRejected>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let aggregator = state.aggregator.read().await;
    info!("Bid rejected for auction {}: {}", rejection.auction_id, rejection.reason);

    // Remove from pending bids
    state.pending_bids.write().await.remove(&rejection.auction_id);

    let response = serde_json::json!({
        "type": "BID_REJECTED_ACK",
        "aggregator_id": aggregator.aggregator_id,
        "auction_id": rejection.auction_id,
        "reason": rejection.reason,
        "timestamp": current_timestamp()
    });

    Ok(Json(response))
}

fn calculate_bid_price(reserve_price: u32, strategy: &str) -> u32 {
    match strategy {
        "CONSERVATIVE" => {
            // Conservative: bid close to reserve price
            let multiplier = 1.0 + (rand::random::<f64>() * 0.1); // 1.0 to 1.1
            (reserve_price as f64 * multiplier) as u32
        }
        "AGGRESSIVE" => {
            // Aggressive: bid higher to win
            let multiplier = 1.2 + (rand::random::<f64>() * 0.2); // 1.2 to 1.4
            (reserve_price as f64 * multiplier) as u32
        }
        "OPPORTUNISTIC" => {
            // Opportunistic: bid only if price is very low
            if reserve_price < 600 { // Less than 6¢/kWh
                let multiplier = 1.1 + (rand::random::<f64>() * 0.15); // 1.1 to 1.25
                (reserve_price as f64 * multiplier) as u32
            } else {
                0
            }
        }
        _ => 0,
    }
}

async fn register_with_gateway(state: Arc<AppState>) {
    // Wait a bit for gateway to be ready
    sleep(Duration::from_secs(5)).await;
    
    let client = reqwest::Client::new();
    let url = format!("http://{}:{}/api/register/aggregator", state.gateway_host, state.gateway_port);
    
    loop {
        let aggregator = state.aggregator.read().await;
        
        // Create registration payload matching gateway's Aggregator struct
        let registration_data = serde_json::json!({
            "aggregator_id": aggregator.aggregator_id,
            "strategy": aggregator.strategy,
            "max_bid_price": aggregator.max_bid_price,
            "reputation_score": aggregator.reputation_score,
            "is_online": aggregator.is_online,
            "last_seen": current_timestamp()
        });
        
        info!("Registering aggregator {} with gateway at {}", aggregator.aggregator_id, url);
        
        match client.post(&url).json(&registration_data).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    info!("Successfully registered aggregator {} with gateway", aggregator.aggregator_id);
                    break; // Registration successful, exit loop
                } else {
                    warn!("Gateway registration failed with status: {}", response.status());
                }
            }
            Err(e) => {
                warn!("Failed to register with gateway: {}", e);
            }
        }
        
        // Wait before retrying
        sleep(Duration::from_secs(10)).await;
    }
}

async fn start_bess_querying(state: Arc<AppState>) {
    // Wait for registration to complete
    sleep(Duration::from_secs(10)).await;
    
    let client = reqwest::Client::new();
    let gateway_url = format!("http://{}:{}/api/bess-nodes", state.gateway_host, state.gateway_port);
    
    let mut interval = interval(Duration::from_secs(30)); // Query every 30 seconds
    
    loop {
        interval.tick().await;
        
        // Get BESS list from gateway
        match client.get(&gateway_url).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    match response.json::<Vec<serde_json::Value>>().await {
                        Ok(bess_nodes) => {
                            info!("Retrieved {} BESS nodes from gateway", bess_nodes.len());
                            
                            // Query each BESS node directly
                            for bess_node in &bess_nodes {
                                if let Some(node_id) = bess_node.get("node_id").and_then(|id| id.as_str()) {
                                    query_bess_node_directly(state.clone(), node_id, bess_node).await;
                                }
                            }
                        }
                        Err(e) => {
                            warn!("Failed to parse BESS list response: {}", e);
                        }
                    }
                } else {
                    warn!("Failed to get BESS list from gateway: {}", response.status());
                }
            }
            Err(e) => {
                warn!("Failed to request BESS list from gateway: {}", e);
            }
        }
    }
}

async fn query_bess_node_directly(state: Arc<AppState>, node_id: &str, bess_data: &serde_json::Value) {
    let aggregator = state.aggregator.read().await;
    let client = reqwest::Client::new();
    
    // Construct BESS node URL (assuming BESS nodes run on port 8081)
    let bess_url = format!("http://bess-{}:8081/query", node_id);
    
    // Create query payload
    let query_data = serde_json::json!({
        "aggregator_id": aggregator.aggregator_id,
        "query_type": "energy_availability",
        "timestamp": current_timestamp()
    });
    
    info!("AGG-{} querying BESS-{} directly at {}", aggregator.aggregator_id, node_id, bess_url);
    
    // Report the direct query to the gateway
    let gateway_query_url = format!("http://{}:{}/api/direct-query", state.gateway_host, state.gateway_port);
    let direct_query_data = serde_json::json!({
        "aggregator_id": aggregator.aggregator_id,
        "bess_node_id": node_id,
        "query_type": "energy_availability",
        "timestamp": current_timestamp()
    });
    
    let _ = client.post(&gateway_query_url).json(&direct_query_data).send().await;
    
    match client.post(&bess_url).json(&query_data).send().await {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<serde_json::Value>().await {
                    Ok(query_response) => {
                        if let Some(available_energy) = query_response.get("available_energy").and_then(|e| e.as_f64()) {
                            if let Some(reserve_price) = query_response.get("reserve_price").and_then(|p| p.as_u64()) {
                                info!("BESS-{} responded to AGG-{}: {:.1} kWh at {:.2}c/kWh", 
                                      node_id, aggregator.aggregator_id, available_energy, reserve_price as f64 / 100.0);
                                
                                // Report the response to the gateway
                                let gateway_response_url = format!("http://{}:{}/api/direct-query-response", state.gateway_host, state.gateway_port);
                                let response_data = serde_json::json!({
                                    "aggregator_id": aggregator.aggregator_id,
                                    "bess_node_id": node_id,
                                    "energy_available": available_energy,
                                    "reserve_price": reserve_price,
                                    "response_time_ms": 50, // Simulated response time
                                    "timestamp": current_timestamp()
                                });
                                
                                let _ = client.post(&gateway_response_url).json(&response_data).send().await;
                                
                                // Store the response in available_bess_nodes
                                let mut available_nodes = state.available_bess_nodes.write().await;
                                available_nodes.insert(node_id.to_string(), BESSDiscovery {
                                    r#type: "BESS_DISCOVERY".to_string(),
                                    node_id: node_id.to_string(),
                                    node_type: "BESS".to_string(),
                                    capacity_kwh: bess_data.get("capacity_kwh").and_then(|c| c.as_f64()).unwrap_or(0.0),
                                    energy_level: available_energy,
                                    reserve_price: reserve_price as u32,
                                    timestamp: current_timestamp(),
                                });
                            }
                        }
                    }
                    Err(e) => {
                        warn!("Failed to parse BESS response from {}: {}", node_id, e);
                    }
                }
            } else {
                warn!("BESS-{} query failed with status: {}", node_id, response.status());
            }
        }
        Err(e) => {
            warn!("Failed to query BESS-{}: {}", node_id, e);
        }
    }
}

async fn start_health_monitoring(state: Arc<AppState>) {
    let mut interval = interval(Duration::from_secs(15));
    
    loop {
        interval.tick().await;
        
        let aggregator = state.aggregator.read().await;
        let available_bess_nodes = state.available_bess_nodes.read().await;
        let pending_bids = state.pending_bids.read().await;
        
        let status = AggregatorStatus {
            aggregator_id: aggregator.aggregator_id.clone(),
            strategy: aggregator.strategy.clone(),
            reputation_score: aggregator.reputation_score,
            successful_settlements: aggregator.successful_settlements,
            total_energy_traded: aggregator.total_energy_traded,
            total_usdc_paid: aggregator.total_usdc_paid,
            available_bess_nodes: available_bess_nodes.len(),
            pending_bids: pending_bids.len(),
            is_online: aggregator.is_online,
            last_activity: aggregator.last_activity,
            timestamp: current_timestamp(),
        };

        // Send to gateway via HTTP
        let client = reqwest::Client::new();
        let url = format!("http://{}:{}/api/aggregator-status", state.gateway_host, state.gateway_port);
        
        if let Err(e) = client.post(&url).json(&status).send().await {
            warn!("Failed to send status to gateway: {}", e);
        } else {
            info!("Status sent to gateway");
        }
    }
}

fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
