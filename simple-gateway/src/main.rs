use axum::{
    extract::{ws::WebSocketUpgrade, State},
    response::Response,
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
    time::interval,
};
use tower_http::cors::CorsLayer;
use tracing::{info, warn, error};
use uuid::Uuid;
use futures_util::stream::StreamExt;
use futures_util::SinkExt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BESSNode {
    pub node_id: String,
    pub energy_level: f64,
    pub capacity_kwh: f64,
    pub reserve_price: u32,
    pub is_online: bool,
    pub last_seen: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Aggregator {
    pub aggregator_id: String,
    pub strategy: String,
    pub max_bid_price: u32,
    pub reputation_score: u32,
    pub is_online: bool,
    pub last_seen: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Auction {
    pub auction_id: String,
    pub total_energy: f64,
    pub reserve_price: u32,
    pub status: String,
    pub started_at: u64,
    pub bids: Vec<Bid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bid {
    pub aggregator_id: String,
    pub bid_price: u32,
    pub energy_amount: f64,
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub total_bess_nodes: usize,
    pub total_aggregators: usize,
    pub active_auctions: usize,
    pub total_energy_available: f64,
    pub average_bid_price: f64,
}

pub struct AppState {
    pub bess_nodes: Arc<RwLock<HashMap<String, BESSNode>>>,
    pub aggregators: Arc<RwLock<HashMap<String, Aggregator>>>,
    pub auctions: Arc<RwLock<HashMap<String, Auction>>>,
    pub metrics: Arc<RwLock<SystemMetrics>>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    let app_state = Arc::new(AppState {
        bess_nodes: Arc::new(RwLock::new(HashMap::new())),
        aggregators: Arc::new(RwLock::new(HashMap::new())),
        auctions: Arc::new(RwLock::new(HashMap::new())),
        metrics: Arc::new(RwLock::new(SystemMetrics {
            total_bess_nodes: 0,
            total_aggregators: 0,
            active_auctions: 0,
            total_energy_available: 0.0,
            average_bid_price: 0.0,
        })),
    });

    info!("Starting Energy Trading Gateway");

    // Start services
    let state_clone = app_state.clone();
    tokio::spawn(async move {
        start_multicast_discovery(state_clone).await;
    });

    let state_clone = app_state.clone();
    tokio::spawn(async move {
        start_auction_simulation(state_clone).await;
    });

    let state_clone = app_state.clone();
    tokio::spawn(async move {
        start_metrics_update(state_clone).await;
    });

    // Start HTTP server
    let app = Router::new()
        .route("/", get(health_check))
        .route("/health", get(health_check))
        .route("/api/bess-status", post(handle_bess_status))
        .route("/api/aggregator-status", post(handle_aggregator_status))
        .route("/ws", get(websocket_handler))
        .layer(CorsLayer::permissive())
        .with_state(app_state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    info!("Gateway starting on {}", addr);

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_check() -> &'static str {
    "Energy Trading Gateway is running!"
}

async fn handle_bess_status(
    State(state): State<Arc<AppState>>,
    axum::Json(bess_node): axum::Json<BESSNode>,
) -> &'static str {
    let mut nodes = state.bess_nodes.write().await;
    nodes.insert(bess_node.node_id.clone(), bess_node);
    info!("Updated BESS node status");
    "OK"
}

async fn handle_aggregator_status(
    State(state): State<Arc<AppState>>,
    axum::Json(aggregator): axum::Json<Aggregator>,
) -> &'static str {
    let mut aggregators = state.aggregators.write().await;
    aggregators.insert(aggregator.aggregator_id.clone(), aggregator);
    info!("Updated Aggregator status");
    "OK"
}

async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> Response {
    ws.on_upgrade(|socket| websocket_connection(socket, state))
}

async fn websocket_connection(socket: axum::extract::ws::WebSocket, state: Arc<AppState>) {
    info!("New WebSocket connection established");
    
    let (mut sender, mut receiver) = socket.split();
    
    // Send initial data
    let initial_data = serde_json::json!({
        "type": "INITIAL_DATA",
        "bess_nodes": state.bess_nodes.read().await.values().collect::<Vec<_>>(),
        "aggregators": state.aggregators.read().await.values().collect::<Vec<_>>(),
        "auctions": state.auctions.read().await.values().collect::<Vec<_>>(),
        "metrics": *state.metrics.read().await,
    });
    
    if let Ok(msg) = serde_json::to_string(&initial_data) {
        let _ = sender.send(axum::extract::ws::Message::Text(msg)).await;
    }
    
    // Handle incoming messages
    while let Some(msg) = receiver.next().await {
        match msg {
            Ok(axum::extract::ws::Message::Text(text)) => {
                info!("Received WebSocket message: {}", text);
            }
            Ok(axum::extract::ws::Message::Close(_)) => {
                info!("WebSocket connection closed");
                break;
            }
            Err(e) => {
                error!("WebSocket error: {}", e);
                break;
            }
            _ => {}
        }
    }
}

async fn start_multicast_discovery(state: Arc<AppState>) {
    let socket = match UdpSocket::bind("0.0.0.0:8888").await {
        Ok(socket) => socket,
        Err(e) => {
            error!("Failed to create UDP socket: {}", e);
            return;
        }
    };

    info!("Started multicast discovery on 224.0.0.1:8888");

    let mut buffer = [0; 1024];
    loop {
        match socket.recv_from(&mut buffer).await {
            Ok((len, addr)) => {
                if let Ok(message) = serde_json::from_slice::<serde_json::Value>(&buffer[..len]) {
                    if let Some(msg_type) = message.get("type").and_then(|t| t.as_str()) {
                        match msg_type {
                            "BESS_DISCOVERY" => {
                                if let Ok(bess_node) = serde_json::from_value::<BESSNode>(message) {
                                    info!("Discovered BESS node {}: {} kWh available", 
                                          bess_node.node_id, bess_node.energy_level);
                                    state.bess_nodes.write().await
                                        .insert(bess_node.node_id.clone(), bess_node);
                                }
                            }
                            "AGGREGATOR_DISCOVERY" => {
                                if let Ok(aggregator) = serde_json::from_value::<Aggregator>(message) {
                                    info!("Discovered Aggregator {}: strategy {}", 
                                          aggregator.aggregator_id, aggregator.strategy);
                                    state.aggregators.write().await
                                        .insert(aggregator.aggregator_id.clone(), aggregator);
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
            Err(e) => {
                error!("Multicast discovery error: {}", e);
            }
        }
    }
}

async fn start_auction_simulation(state: Arc<AppState>) {
    let mut interval = interval(Duration::from_secs(30));
    let mut auction_counter = 1;

    loop {
        interval.tick().await;

        // Create a new auction
        let auction_id = format!("auction-{}", auction_counter);
        let total_energy = 10.0 + (rand::random::<f64>() * 20.0); // 10-30 kWh
        let reserve_price = 500 + (rand::random::<u32>() % 300); // 5-8¢/kWh

        let auction = Auction {
            auction_id: auction_id.clone(),
            total_energy,
            reserve_price,
            status: "active".to_string(),
            started_at: current_timestamp(),
            bids: Vec::new(),
        };

        info!("Created auction {}: {} kWh at {}¢/kWh", 
              auction_id, total_energy, reserve_price);

        state.auctions.write().await.insert(auction_id, auction);
        auction_counter += 1;
    }
}

async fn start_metrics_update(state: Arc<AppState>) {
    let mut interval = interval(Duration::from_secs(5));

    loop {
        interval.tick().await;

        let bess_nodes = state.bess_nodes.read().await;
        let aggregators = state.aggregators.read().await;
        let auctions = state.auctions.read().await;

        let total_energy_available: f64 = bess_nodes.values()
            .map(|node| node.energy_level)
            .sum();

        let total_bid_price: f64 = auctions.values()
            .flat_map(|auction| &auction.bids)
            .map(|bid| bid.bid_price as f64)
            .sum();

        let total_bids = auctions.values()
            .map(|auction| auction.bids.len())
            .sum::<usize>();

        let average_bid_price = if total_bids > 0 {
            total_bid_price / total_bids as f64
        } else {
            0.0
        };

        let mut metrics = state.metrics.write().await;
        *metrics = SystemMetrics {
            total_bess_nodes: bess_nodes.len(),
            total_aggregators: aggregators.len(),
            active_auctions: auctions.len(),
            total_energy_available,
            average_bid_price,
        };

        info!("Updated metrics: {} BESS nodes, {} aggregators, {} auctions, {:.2} kWh available",
              metrics.total_bess_nodes, metrics.total_aggregators, 
              metrics.active_auctions, metrics.total_energy_available);
    }
}

fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
