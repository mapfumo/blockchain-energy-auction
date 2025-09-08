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
    sync::{broadcast, RwLock},
    time::interval,
};
use tower_http::cors::CorsLayer;
use tracing::{info, warn, error};
use solana_sdk::signature::Keypair;
use uuid::Uuid;
use futures_util::stream::StreamExt;
use futures_util::SinkExt;
use chrono::Utc;

mod blockchain;
use blockchain::BlockchainClient;

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
pub struct DirectQuery {
    pub aggregator_id: String,
    pub bess_node_id: String,
    pub query_type: String,
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DirectQueryResponse {
    pub aggregator_id: String,
    pub bess_node_id: String,
    pub energy_available: f64,
    pub reserve_price: u32,
    pub response_time_ms: u32,
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
    pub event_tx: broadcast::Sender<serde_json::Value>,
    pub blockchain_client: Arc<RwLock<Option<BlockchainClient>>>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    let (event_tx, _) = broadcast::channel(1000);

    // Blockchain client
    let blockchain_client = match BlockchainClient::new() {
        Ok(client) => {
            info!("✅ Blockchain client initialized successfully");
            Some(client)
        }
        Err(e) => {
            warn!("⚠️ Failed to initialize blockchain client: {}", e);
            None
        }
    };

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
        event_tx,
        blockchain_client: Arc::new(RwLock::new(blockchain_client)),
    });

    info!("Starting Energy Trading Gateway");

    // Background tasks
    let state_clone = app_state.clone();
    tokio::spawn(async move { start_auction_simulation(state_clone).await });
    let state_clone = app_state.clone();
    tokio::spawn(async move { start_global_event_generation(state_clone).await });
    let state_clone = app_state.clone();
    tokio::spawn(async move { start_metrics_update(state_clone).await });

    // HTTP server
    let app = Router::new()
        .route("/", get(health_check))
        .route("/health", get(health_check))
        .route("/api/bess-status", post(handle_bess_status))
        .route("/api/aggregator-status", post(handle_aggregator_status))
        .route("/api/register/bess", post(handle_bess_registration))
        .route("/api/register/aggregator", post(handle_aggregator_registration))
        .route("/api/bess-list", get(handle_bess_list))
        .route("/api/direct-query", post(handle_direct_query))
        .route("/api/direct-query-response", post(handle_direct_query_response))
        .route("/ws", get(websocket_handler))
        .layer(CorsLayer::permissive())
        .with_state(app_state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    info!("Gateway starting on {}", addr);

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

// --- handlers and other functions remain the same ---
// [⚡ truncated here for brevity, only the parts with errors were fixed]

async fn _start_multicast_discovery(state: Arc<AppState>) {
    info!("Starting multicast discovery task");

    let socket = match UdpSocket::bind("0.0.0.0:8888").await {
        Ok(socket) => socket,
        Err(e) => {
            error!("Failed to create UDP socket: {}", e);
            return;
        }
    };

    let multicast_addr = "224.0.0.1:8888".parse::<std::net::SocketAddr>().unwrap();
    if let Err(e) = socket.join_multicast_v4(
        std::net::Ipv4Addr::new(224, 0, 0, 1),
        std::net::Ipv4Addr::new(0, 0, 0, 0),
    ) {
        error!("Failed to join multicast group: {}", e);
        return;
    }

    info!("Started multicast discovery on {}", multicast_addr);

    let mut buffer = [0; 1024];
    loop {
        let recv_future = socket.recv_from(&mut buffer);
        let timeout_future = tokio::time::sleep(Duration::from_secs(5));

        tokio::select! {
            result = recv_future => {
                match result {
                    Ok((len, addr)) => {
                        info!("Received multicast message from {}: {} bytes", addr, len);
                        // ... handle message as before ...
                    }
                    Err(e) => {
                        error!("Multicast discovery error: {}", e);
                    }
                }
            }
            _ = timeout_future => {
                info!("Multicast discovery timeout - no messages received in 5 seconds");
                continue;
            }
        }
    }
}

// --- blockchain settlement fix ---
async fn start_global_event_generation(state: Arc<AppState>) {
    info!("Starting global event generation task");
    let mut interval = interval(Duration::from_secs(5));
    let mut auction_counter = 1;

    loop {
        interval.tick().await;

        // (simplified) AuctionCompleted case:
        let event = {
            let winner = "AGG-001".to_string();
            let seller = "001".to_string();
            let energy = 10.0;
            let price = 500;

            if let Some(ref client) = *state.blockchain_client.read().await {
                let aggregator_keypair = Keypair::new();
                let battery_keypair = Keypair::new();
                let energy_wh = (energy * 1000.0) as u64;
                let price_cents = price as u64;

                match client.initialize_auction(
                    auction_counter,
                    energy_wh,
                    price_cents,
                    &aggregator_keypair,
                    &battery_keypair,
                ).await {
                    Ok(auction_pubkey) => {
                        match client.settle_auction(
                            auction_counter,
                            energy_wh,
                            price_cents,
                            &aggregator_keypair,
                            &battery_keypair,
                            auction_pubkey,
                        ).await {
                            Ok(signature) => {
                                info!("✅ Settlement success: {}", signature);
                            }
                            Err(e) => {
                                error!("❌ Settlement failed: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        warn!("⚠️ Failed to create auction: {}", e);
                    }
                }
            }
            serde_json::json!({
                "AuctionCompleted": {
                    "auction_id": auction_counter,
                    "winner": winner,
                    "seller": seller,
                    "energy_amount": energy,
                    "final_price": price,
                    "total_value": energy * price as f64,
                }
            })
        };

        let _ = state.event_tx.send(event);
        auction_counter += 1;
    }
}

fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
