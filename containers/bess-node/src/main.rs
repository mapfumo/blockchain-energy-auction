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
pub struct BESSNode {
    pub node_id: String,
    pub node_type: String,
    pub capacity_kwh: f64,
    pub energy_level: f64,
    pub battery_health: f64,
    pub voltage: f64,
    pub temperature: f64,
    pub reserve_price: u32, // cents/kWh
    pub is_online: bool,
    pub last_activity: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BESSStatus {
    pub node_id: String,
    pub energy_level: f64,
    pub capacity_kwh: f64,
    pub battery_health: f64,
    pub voltage: f64,
    pub temperature: f64,
    pub reserve_price: u32,
    pub is_online: bool,
    pub last_activity: u64,
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscoveryMessage {
    pub r#type: String,
    pub node_id: String,
    pub node_type: String,
    pub capacity_kwh: f64,
    pub energy_level: f64,
    pub reserve_price: u32,
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryResponse {
    pub r#type: String,
    pub node_id: String,
    pub available_energy: f64,
    pub reserve_price: u32,
    pub battery_health: f64,
    pub voltage: f64,
    pub temperature: f64,
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BidAccepted {
    pub r#type: String,
    pub energy_amount: f64,
    pub price: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BidConfirmed {
    pub r#type: String,
    pub node_id: String,
    pub energy_sold: f64,
    pub price: u32,
    pub remaining_energy: f64,
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub node_id: String,
    pub energy_level: f64,
    pub battery_health: f64,
    pub is_online: bool,
}

pub struct AppState {
    pub bess_node: Arc<RwLock<BESSNode>>,
    pub gateway_host: String,
    pub gateway_port: u16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Get environment variables
    let node_id = std::env::var("NODE_ID").unwrap_or_else(|_| "001".to_string());
    let capacity_kwh = std::env::var("CAPACITY_KWH")
        .unwrap_or_else(|_| "15".to_string())
        .parse::<f64>()?;
    let initial_energy = std::env::var("INITIAL_ENERGY")
        .unwrap_or_else(|_| "12.5".to_string())
        .parse::<f64>()?;
    let gateway_host = std::env::var("GATEWAY_HOST").unwrap_or_else(|_| "gateway".to_string());
    let gateway_port = std::env::var("GATEWAY_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()?;
    let multicast_group = std::env::var("MULTICAST_GROUP").unwrap_or_else(|_| "224.0.0.1".to_string());
    let multicast_port = std::env::var("MULTICAST_PORT")
        .unwrap_or_else(|_| "8888".to_string())
        .parse::<u16>()?;

    // Create BESS node
    let bess_node = BESSNode {
        node_id: node_id.clone(),
        node_type: "BESS".to_string(),
        capacity_kwh,
        energy_level: initial_energy,
        battery_health: 95.0,
        voltage: 48.0,
        temperature: 25.0,
        reserve_price: 650, // 6.5¢/kWh
        is_online: true,
        last_activity: current_timestamp(),
    };

    let app_state = Arc::new(AppState {
        bess_node: Arc::new(RwLock::new(bess_node)),
        gateway_host,
        gateway_port,
    });

    info!("BESS Node {} initialized:", node_id);
    info!("  Capacity: {} kWh", capacity_kwh);
    info!("  Energy Level: {} kWh", initial_energy);
    info!("  Gateway: {}:{}", app_state.gateway_host, app_state.gateway_port);

    // Start services
    let state_clone = app_state.clone();
    tokio::spawn(async move {
        register_with_gateway(state_clone).await;
    });

    let state_clone = app_state.clone();
    tokio::spawn(async move {
        start_energy_simulation(state_clone).await;
    });

    let state_clone = app_state.clone();
    tokio::spawn(async move {
        start_health_monitoring(state_clone).await;
    });

    // Start HTTP server
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/query", post(handle_query))
        .route("/bid-accepted", post(handle_bid_accepted))
        .route("/bid-rejected", post(handle_bid_rejected))
        .layer(CorsLayer::permissive())
        .with_state(app_state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8081));
    info!("HTTP server starting on {}", addr);

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_check(State(state): State<Arc<AppState>>) -> Result<Json<HealthResponse>, StatusCode> {
    let bess_node = state.bess_node.read().await;
    Ok(Json(HealthResponse {
        status: "healthy".to_string(),
        node_id: bess_node.node_id.clone(),
        energy_level: bess_node.energy_level,
        battery_health: bess_node.battery_health,
        is_online: bess_node.is_online,
    }))
}

async fn handle_query(
    State(state): State<Arc<AppState>>,
    Json(query): Json<serde_json::Value>,
) -> Result<Json<QueryResponse>, StatusCode> {
    let mut bess_node = state.bess_node.write().await;
    bess_node.last_activity = current_timestamp();

    info!("Received energy query: {:?}", query);

    let response = if bess_node.energy_level > 0.1 {
        QueryResponse {
            r#type: "QUERY_RESPONSE".to_string(),
            node_id: bess_node.node_id.clone(),
            available_energy: bess_node.energy_level,
            reserve_price: bess_node.reserve_price,
            battery_health: bess_node.battery_health,
            voltage: bess_node.voltage,
            temperature: bess_node.temperature,
            timestamp: current_timestamp(),
        }
    } else {
        QueryResponse {
            r#type: "QUERY_RESPONSE".to_string(),
            node_id: bess_node.node_id.clone(),
            available_energy: 0.0,
            reserve_price: bess_node.reserve_price,
            battery_health: bess_node.battery_health,
            voltage: bess_node.voltage,
            temperature: bess_node.temperature,
            timestamp: current_timestamp(),
        }
    };

    Ok(Json(response))
}

async fn handle_bid_accepted(
    State(state): State<Arc<AppState>>,
    Json(bid): Json<BidAccepted>,
) -> Result<Json<BidConfirmed>, StatusCode> {
    let mut bess_node = state.bess_node.write().await;
    
    info!("Bid accepted: {} kWh at {}¢/kWh", bid.energy_amount, bid.price);
    
    // Update energy level
    bess_node.energy_level = (bess_node.energy_level - bid.energy_amount).max(0.0);
    bess_node.last_activity = current_timestamp();

    let response = BidConfirmed {
        r#type: "BID_CONFIRMED".to_string(),
        node_id: bess_node.node_id.clone(),
        energy_sold: bid.energy_amount,
        price: bid.price,
        remaining_energy: bess_node.energy_level,
        timestamp: current_timestamp(),
    };

    Ok(Json(response))
}

async fn handle_bid_rejected(
    State(state): State<Arc<AppState>>,
    Json(_rejection): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let bess_node = state.bess_node.read().await;
    info!("Bid rejected for node {}", bess_node.node_id);

    let response = serde_json::json!({
        "type": "BID_REJECTED_ACK",
        "node_id": bess_node.node_id,
        "timestamp": current_timestamp()
    });

    Ok(Json(response))
}

async fn register_with_gateway(state: Arc<AppState>) {
    // Wait a bit for gateway to be ready
    sleep(Duration::from_secs(5)).await;
    
    let client = reqwest::Client::new();
    let url = format!("http://{}:{}/api/register/bess", state.gateway_host, state.gateway_port);
    
    loop {
        let bess_node = state.bess_node.read().await;
        
        // Create registration payload matching gateway's BESSNode struct
        let registration_data = serde_json::json!({
            "node_id": bess_node.node_id,
            "energy_level": bess_node.energy_level,
            "capacity_kwh": bess_node.capacity_kwh,
            "reserve_price": bess_node.reserve_price,
            "is_online": bess_node.is_online,
            "last_seen": current_timestamp()
        });
        
        info!("Registering BESS node {} with gateway at {}", bess_node.node_id, url);
        
        match client.post(&url).json(&registration_data).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    info!("Successfully registered BESS node {} with gateway", bess_node.node_id);
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

async fn start_energy_simulation(state: Arc<AppState>) {
    let mut interval = interval(Duration::from_secs(1));
    
    loop {
        interval.tick().await;
        
        let mut bess_node = state.bess_node.write().await;
        
        // Simulate energy changes
        if bess_node.energy_level < bess_node.capacity_kwh * 0.1 {
            // Critical recharge - faster rate
            bess_node.energy_level = (bess_node.energy_level + 0.1).min(bess_node.capacity_kwh);
            info!("Critical recharge: {:.2} kWh", bess_node.energy_level);
        } else if bess_node.energy_level < bess_node.capacity_kwh * 0.8 {
            // Normal recharge
            bess_node.energy_level = (bess_node.energy_level + 0.05).min(bess_node.capacity_kwh);
        }
        
        // Update battery health (slowly degrade)
        bess_node.battery_health = (bess_node.battery_health - 0.001).max(80.0);
        
        // Update temperature (simulate)
        bess_node.temperature = 20.0 + (rand::random::<f64>() * 7.0 - 2.0);
        
        // Update reserve price (slight variation)
        let price_change = (rand::random::<i32>() % 21) - 10; // -10 to +10
        bess_node.reserve_price = (bess_node.reserve_price as i32 + price_change)
            .max(400)
            .min(1000) as u32;
    }
}

async fn start_health_monitoring(state: Arc<AppState>) {
    let mut interval = interval(Duration::from_secs(10));
    
    loop {
        interval.tick().await;
        
        let bess_node = state.bess_node.read().await;
        let status = BESSStatus {
            node_id: bess_node.node_id.clone(),
            energy_level: bess_node.energy_level,
            capacity_kwh: bess_node.capacity_kwh,
            battery_health: bess_node.battery_health,
            voltage: bess_node.voltage,
            temperature: bess_node.temperature,
            reserve_price: bess_node.reserve_price,
            is_online: bess_node.is_online,
            last_activity: bess_node.last_activity,
            timestamp: current_timestamp(),
        };

        // Send to gateway via HTTP
        let client = reqwest::Client::new();
        let url = format!("http://{}:{}/api/bess-status", state.gateway_host, state.gateway_port);
        
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
