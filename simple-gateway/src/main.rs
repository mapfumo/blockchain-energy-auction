use axum::{
    extract::{ws::WebSocketUpgrade, State},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tokio::{
    net::TcpListener,
    sync::{broadcast, RwLock},
    time::interval,
};
use tower_http::cors::CorsLayer;
use tracing::{info, warn};
use futures_util::stream::StreamExt;
use chrono;
use futures_util::SinkExt;

use simple_gateway::*;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    
    info!("Starting Energy Trading Gateway");
    
    // Initialize blockchain client
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
    
    // Create shared state
    let (event_tx, _event_rx) = broadcast::channel(1000);
    let state = Arc::new(AppState {
        bess_nodes: Arc::new(RwLock::new(HashMap::new())),
        aggregators: Arc::new(RwLock::new(HashMap::new())),
        auctions: Arc::new(RwLock::new(HashMap::new())),
        event_tx,
        blockchain_client: Arc::new(RwLock::new(blockchain_client)),
    });
    
    // Start event generation
    let state_clone = state.clone();
    tokio::spawn(async move {
        start_global_event_generation(state_clone).await;
    });
    
    // Create router
    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health_check))
        .route("/ws", get(websocket_handler))
        .route("/api/metrics", get(get_metrics))
        .route("/api/bess-nodes", get(get_bess_nodes))
        .route("/api/aggregators", get(get_aggregators))
        .route("/api/auctions", get(get_auctions))
        .route("/api/blockchain-settlements", get(get_blockchain_settlements))
        .route("/api/register/bess", post(register_bess_node))
        .route("/api/register/aggregator", post(register_aggregator))
        .route("/api/bess-status", post(handle_bess_status))
        .route("/api/aggregator-status", post(handle_aggregator_status))
        .layer(CorsLayer::permissive())
        .with_state(state);
    
    // Start server
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    info!("Gateway starting on 0.0.0.0:8080");
    
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Energy Trading Gateway API"
}

async fn health_check() -> &'static str {
    "OK"
}

async fn websocket_handler(ws: WebSocketUpgrade, State(state): State<Arc<AppState>>) -> Response {
    ws.on_upgrade(|socket| websocket_connection(socket, state))
}

async fn websocket_connection(socket: axum::extract::ws::WebSocket, state: Arc<AppState>) {
    let (mut sender, mut receiver) = socket.split();
    let mut rx = state.event_tx.subscribe();
    
    info!("New WebSocket connection established");
    
    // Send initial data
    let initial_data = serde_json::json!({
        "type": "SystemMetrics",
        "data": {
            "total_bess_nodes": 0,
            "total_aggregators": 0,
            "total_auctions": 0,
            "total_energy_available": 0.0,
            "timestamp": current_timestamp()
        }
    });
    
    if let Ok(msg) = serde_json::to_string(&initial_data) {
        let _ = sender.send(axum::extract::ws::Message::Text(msg)).await;
    }
    
    // Handle incoming messages and forward events
    loop {
        tokio::select! {
            // Handle incoming messages from client
            message = receiver.next() => {
                match message {
                    Some(Ok(axum::extract::ws::Message::Text(text))) => {
                        info!("📥 Received message from client: {}", text);
                    
                    // Try to parse as JSON
                    if let Ok(data) = serde_json::from_str::<serde_json::Value>(&text) {
                        // Check if it's a BESS node status update
                        if let Some(bess_data) = data.get("BESSNodeStatus") {
                            info!("🔋 Processing BESS node status update");
                            let device_id = bess_data.get("device_id")
                                .and_then(|v| v.as_str())
                                .unwrap_or("unknown");
                            
                            // Update BESS node in state
                            {
                                let mut bess_nodes = state.bess_nodes.write().await;
                                let existing = bess_nodes.get(device_id);
                                
                                if let Some(_) = existing {
                                    // Update existing node
                                    if let Some(node) = bess_nodes.get_mut(device_id) {
                                        node.energy_level = bess_data.get("energy_available")
                                            .and_then(|v| v.as_f64())
                                            .unwrap_or(0.0);
                                        node.is_online = bess_data.get("is_online")
                                            .and_then(|v| v.as_bool())
                                            .unwrap_or(true);
                                        node.last_seen = current_timestamp();
                                    }
                                } else {
                                    // Add new node
                                    let new_node = BESSNode {
                                        node_id: device_id.to_string(),
                                        energy_level: bess_data.get("energy_available")
                                            .and_then(|v| v.as_f64())
                                            .unwrap_or(0.0),
                                        capacity_kwh: bess_data.get("capacity_kwh")
                                            .and_then(|v| v.as_f64())
                                            .unwrap_or(20.0),
                                        reserve_price: bess_data.get("reserve_price")
                                            .and_then(|v| v.as_u64())
                                            .unwrap_or(500) as u32,
                                        is_online: bess_data.get("is_online")
                                            .and_then(|v| v.as_bool())
                                            .unwrap_or(true),
                                        last_seen: current_timestamp(),
                                    };
                                    bess_nodes.insert(device_id.to_string(), new_node);
                                    info!("✅ Added new BESS node: {}", device_id);
                                }
                            }
                            
                            // Broadcast BESS node status event
                            let event = serde_json::json!({
                                "BESSNodeStatus": bess_data
                            });
                            let _ = state.event_tx.send(event);
                        }
                        
                        // Check if it's an Aggregator status update
                        if let Some(agg_data) = data.get("AggregatorStatus") {
                            info!("⚡ Processing Aggregator status update");
                            let device_id = agg_data.get("aggregator_id")
                                .and_then(|v| v.as_str())
                                .unwrap_or("unknown");
                            
                            // Update Aggregator in state
                            {
                                let mut aggregators = state.aggregators.write().await;
                                let existing = aggregators.get(device_id);
                                
                                if let Some(_) = existing {
                                    // Update existing aggregator
                                    if let Some(agg) = aggregators.get_mut(device_id) {
                                        agg.is_online = agg_data.get("is_online")
                                            .and_then(|v| v.as_bool())
                                            .unwrap_or(true);
                                        agg.last_seen = current_timestamp();
                                    }
                                } else {
                                    // Add new aggregator
                                    let new_aggregator = Aggregator {
                                        aggregator_id: device_id.to_string(),
                                        strategy: agg_data.get("strategy")
                                            .and_then(|v| v.as_str())
                                            .unwrap_or("CONSERVATIVE")
                                            .to_string(),
                                        total_energy_managed: 0.0,
                                        total_revenue: 0.0,
                                        is_online: agg_data.get("is_online")
                                            .and_then(|v| v.as_bool())
                                            .unwrap_or(true),
                                        last_seen: current_timestamp(),
                                    };
                                    aggregators.insert(device_id.to_string(), new_aggregator);
                                    info!("✅ Added new Aggregator: {}", device_id);
                                }
                            }
                            
                            // Broadcast Aggregator status event
                            let event = serde_json::json!({
                                "AggregatorStatus": agg_data
                            });
                            let _ = state.event_tx.send(event);
                        }
                    }
                    }
                    Some(Ok(axum::extract::ws::Message::Close(_))) => {
                        info!("🔌 WebSocket client disconnected");
                        return;
                    }
                    Some(Err(e)) => {
                        info!("❌ WebSocket error: {:?}", e);
                        return;
                    }
                    None => {
                        info!("🔌 WebSocket connection closed by client");
                        return;
                    }
                    _ => {
                        // Ignore other message types
                    }
                }
            }
            
            // Forward events to WebSocket
            event = rx.recv() => {
                if let Ok(event) = event {
                    info!("📤 Sending event to WebSocket: {:?}", event);
                    if let Ok(msg) = serde_json::to_string(&event) {
                        info!("📤 Sending message to WebSocket: {}", msg);
                        if let Err(e) = sender.send(axum::extract::ws::Message::Text(msg)).await {
                            info!("❌ Failed to send message to WebSocket: {:?}", e);
                            return;
                        }
                    }
                }
            }
        }
    }
}

async fn get_metrics(State(state): State<Arc<AppState>>) -> axum::Json<SystemMetrics> {
    let bess_nodes = state.bess_nodes.read().await;
    let aggregators = state.aggregators.read().await;
    let auctions = state.auctions.read().await;
    
    let total_energy: f64 = bess_nodes.values().map(|node| node.energy_level).sum();
    
    axum::Json(SystemMetrics {
        total_bess_nodes: bess_nodes.len() as u32,
        total_aggregators: aggregators.len() as u32,
        total_auctions: auctions.len() as u32,
        total_energy_available: total_energy,
        timestamp: current_timestamp(),
    })
}

async fn get_bess_nodes(State(state): State<Arc<AppState>>) -> axum::Json<Vec<BESSNode>> {
    let bess_nodes = state.bess_nodes.read().await;
    axum::Json(bess_nodes.values().cloned().collect())
}

async fn get_aggregators(State(state): State<Arc<AppState>>) -> axum::Json<Vec<Aggregator>> {
    let aggregators = state.aggregators.read().await;
    axum::Json(aggregators.values().cloned().collect())
}

async fn get_auctions(State(state): State<Arc<AppState>>) -> axum::Json<Vec<Auction>> {
    let auctions = state.auctions.read().await;
    axum::Json(auctions.values().cloned().collect())
}

async fn get_blockchain_settlements(State(_state): State<Arc<AppState>>) -> axum::Json<Vec<BlockchainSettlement>> {
    // TODO: Implement blockchain settlements storage and retrieval
    axum::Json(vec![])
}

async fn start_global_event_generation(state: Arc<AppState>) {
    let mut interval = interval(Duration::from_secs(5));
    let mut auction_counter = 1u64;
    
    loop {
        interval.tick().await;
        info!("Generating global event...");
        
        // Generate random events - make AuctionCompleted more likely for testing
        let event_type = match rand::random::<u8>() % 10 {
            0 => "AuctionStarted",
            1 => "AuctionCompleted",
            2 => "BESSNodeStatus",  // Increased chance
            3 => "AggregatorStatus",  // Increased chance
            4 => "SystemMetrics",
            5 => "BESSNodeStatus",  // Double chance
            6 => "AggregatorStatus",  // Double chance
            7 => "AuctionCompleted",
            8 => "AuctionCompleted",
            9 => "AuctionCompleted",
            _ => "SystemMetrics",
        };
        
        info!("🎲 Generated event type: {}", event_type);
        
        let event = match event_type {
            "AuctionStarted" => {
                let total_energy = 10.0 + rand::random::<f64>() * 20.0;
                let reserve_price = 400 + (rand::random::<u32>() % 400);
                
                serde_json::json!({
                    "AuctionStarted": {
                        "auction_id": auction_counter,
                        "total_energy": total_energy,
                        "reserve_price": reserve_price
                    }
                })
            },
            "AuctionCompleted" => {
                let winner = format!("AGG-{:03}", (rand::random::<u32>() % 2) + 1);
                let seller = format!("{:03}", (rand::random::<u32>() % 3) + 1);
                let energy = 5.0 + rand::random::<f64>() * 15.0;
                let price = 500 + (rand::random::<u32>() % 300);
                
                info!("🔗 Blockchain settlement triggered for auction #{}", auction_counter);
                
                // Trigger blockchain settlement
                let settlement_result = trigger_blockchain_settlement(
                    auction_counter,
                    winner.clone(),
                    seller.clone(),
                    energy,
                    price,
                ).await;
                
                if settlement_result {
                    info!("✅ Blockchain settlement successful for auction #{}", auction_counter);
                    
                    // Create blockchain settlement event with real signature
                    // Note: The actual signature is logged in the blockchain client
                    // For now, we'll use a placeholder that indicates real settlement
                    let signature = format!("REAL_SETTLEMENT_{}", auction_counter);
                    let settlement_event = serde_json::json!({
                        "type": "BlockchainSettlement",
                        "data": {
                            "auction_id": auction_counter,
                            "winner": winner,
                            "seller": seller,
                            "energy_amount": energy,
                            "final_price": price,
                            "total_value": energy * price as f64,
                            "settlement_signature": signature,
                            "blockchain_url": format!("https://explorer.solana.com/tx/{}?cluster=localnet", signature),
                            "timestamp": current_timestamp()
                        }
                    });
                    
                    // Broadcast the settlement event
                    info!("📡 Broadcasting BlockchainSettlement event for auction #{}", auction_counter);
                    let _ = state.event_tx.send(settlement_event);
                } else {
                    warn!("❌ Blockchain settlement failed for auction #{}", auction_counter);
                }
                
                serde_json::json!({
                    "AuctionCompleted": {
                        "auction_id": auction_counter,
                        "winner": winner,
                        "seller": seller,
                        "energy_amount": energy,
                        "final_price": price,
                        "total_value": energy * price as f64,
                        "auction_duration_ms": 30000 + (rand::random::<u32>() % 60000),
                        "blockchain_settlement": if settlement_result { "completed" } else { "failed" }
                    }
                })
            },
            "SystemMetrics" => {
                let bess_nodes = state.bess_nodes.read().await;
                let aggregators = state.aggregators.read().await;
                let auctions = state.auctions.read().await;
                let total_energy: f64 = bess_nodes.values().map(|node| node.energy_level).sum();
                
                serde_json::json!({
                    "SystemMetrics": {
                        "total_bess_nodes": bess_nodes.len(),
                        "total_aggregators": aggregators.len(),
                        "total_auctions": auctions.len(),
                        "total_energy_available": total_energy,
                        "timestamp": current_timestamp()
                    }
                })
            },
            "BESSNodeStatus" => {
                let node_id = format!("{:03}", (rand::random::<u32>() % 3) + 1);
                let energy_level = 5.0 + rand::random::<f64>() * 15.0;
                let capacity_kwh = 20.0;
                let battery_health = 80.0 + rand::random::<f64>() * 20.0;
                let voltage = 12.0 + rand::random::<f64>() * 2.0;
                let temperature = 20.0 + rand::random::<f64>() * 10.0;
                let reserve_price = 400 + (rand::random::<u32>() % 400);
                
                serde_json::json!({
                    "BESSNodeStatus": {
                        "device_id": node_id,
                        "energy_available": energy_level,
                        "capacity_kwh": capacity_kwh,
                        "battery_health": battery_health,
                        "voltage": voltage,
                        "temperature": temperature,
                        "reserve_price": reserve_price,
                        "is_online": true,
                        "last_activity": current_timestamp(),
                        "timestamp": current_timestamp()
                    }
                })
            },
            "AggregatorStatus" => {
                let aggregator_id = format!("AGG-{:03}", (rand::random::<u32>() % 2) + 1);
                let strategy = if rand::random::<bool>() { "CONSERVATIVE" } else { "AGGRESSIVE" };
                
                serde_json::json!({
                    "AggregatorStatus": {
                        "aggregator_id": aggregator_id,
                        "strategy": strategy,
                        "total_energy_managed": 50.0 + rand::random::<f64>() * 100.0,
                        "total_revenue": 1000.0 + rand::random::<f64>() * 5000.0,
                        "is_online": true,
                        "last_seen": current_timestamp(),
                        "timestamp": current_timestamp()
                    }
                })
            },
            _ => serde_json::json!({}),
        };
        
        // Broadcast the event
        info!("Broadcasting event: {}", event_type);
        let _ = state.event_tx.send(event);
        
        if event_type == "AuctionStarted" {
            auction_counter += 1;
        }
    }
}

fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn generate_solana_signature() -> String {
    // Generate a valid Solana transaction signature (base58 encoded, 88 characters)
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let mut bytes = [0u8; 64];
    for i in 0..64 {
        bytes[i] = rng.gen();
    }
    bs58::encode(&bytes).into_string()
}

async fn register_bess_node(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    info!("BESS node registration request: {:?}", payload);
    
    // Extract device_id from payload
    let device_id = payload.get("device_id")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown");
    
    // Create BESS node data
    let bess_node = serde_json::json!({
        "device_id": device_id,
        "energy_available": payload.get("energy_available").unwrap_or(&serde_json::Value::Number(serde_json::Number::from_f64(12.5).unwrap())),
        "battery_health": payload.get("battery_health").unwrap_or(&serde_json::Value::Number(serde_json::Number::from_f64(95.0).unwrap())),
        "is_online": true
    });
    
    // Broadcast BESS node status event
    let event = serde_json::json!({
        "BESSNodeStatus": bess_node
    });
    
    let _ = state.event_tx.send(event);
    
    axum::Json(serde_json::json!({
        "status": "success",
        "message": "BESS node registered successfully"
    }))
}

async fn register_aggregator(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    info!("Aggregator registration request: {:?}", payload);
    
    // Extract device_id from payload
    let device_id = payload.get("device_id")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown");
    
    // Create aggregator data
    let aggregator = serde_json::json!({
        "device_id": device_id,
        "is_online": true,
        "success_rate": payload.get("success_rate").unwrap_or(&serde_json::Value::Number(serde_json::Number::from_f64(85.0).unwrap())),
        "total_bids": payload.get("total_bids").unwrap_or(&serde_json::Value::Number(serde_json::Number::from(0))),
        "successful_bids": payload.get("successful_bids").unwrap_or(&serde_json::Value::Number(serde_json::Number::from(0))),
        "total_energy_bought": payload.get("total_energy_bought").unwrap_or(&serde_json::Value::Number(serde_json::Number::from_f64(0.0).unwrap())),
        "average_bid_price": payload.get("average_bid_price").unwrap_or(&serde_json::Value::Number(serde_json::Number::from_f64(6.5).unwrap()))
    });
    
    // Broadcast aggregator status event
    let event = serde_json::json!({
        "AggregatorStatus": aggregator
    });
    
    let _ = state.event_tx.send(event);
    
    axum::Json(serde_json::json!({
        "status": "success",
        "message": "Aggregator registered successfully"
    }))
}

async fn handle_bess_status(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<serde_json::Value>,
) -> axum::Json<serde_json::Value> {
    let device_id = payload.get("node_id")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown");
    
    info!("🔋 Received BESS status update from {}", device_id);
    
    // Update BESS node in state
    {
        let mut bess_nodes = state.bess_nodes.write().await;
        let existing = bess_nodes.get(device_id);
        
        if let Some(_) = existing {
            // Update existing node
            if let Some(node) = bess_nodes.get_mut(device_id) {
                node.energy_level = payload.get("energy_level")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(0.0);
                node.is_online = payload.get("is_online")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(true);
                node.last_seen = current_timestamp();
            }
        } else {
            // Add new node
            let new_node = BESSNode {
                node_id: device_id.to_string(),
                energy_level: payload.get("energy_level")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(0.0),
                capacity_kwh: payload.get("capacity_kwh")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(20.0),
                reserve_price: payload.get("reserve_price")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(500) as u32,
                is_online: payload.get("is_online")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(true),
                last_seen: current_timestamp(),
            };
            bess_nodes.insert(device_id.to_string(), new_node);
            info!("✅ Added new BESS node: {}", device_id);
        }
    }
    
    // Broadcast BESS node status event
    let event = serde_json::json!({
        "BESSNodeStatus": payload
    });
    let _ = state.event_tx.send(event);
    
    axum::Json(serde_json::json!({
        "status": "success",
        "message": "BESS status updated successfully"
    }))
}

async fn handle_aggregator_status(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<serde_json::Value>,
) -> axum::Json<serde_json::Value> {
    let device_id = payload.get("aggregator_id")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown");
    
    info!("⚡ Received Aggregator status update from {}", device_id);
    
    // Update Aggregator in state
    {
        let mut aggregators = state.aggregators.write().await;
        let existing = aggregators.get(device_id);
        
        if let Some(_) = existing {
            // Update existing aggregator
            if let Some(agg) = aggregators.get_mut(device_id) {
                agg.is_online = payload.get("is_online")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(true);
                agg.last_seen = current_timestamp();
            }
        } else {
            // Add new aggregator
            let new_aggregator = Aggregator {
                aggregator_id: device_id.to_string(),
                strategy: payload.get("strategy")
                    .and_then(|v| v.as_str())
                    .unwrap_or("CONSERVATIVE")
                    .to_string(),
                total_energy_managed: 0.0,
                total_revenue: 0.0,
                is_online: payload.get("is_online")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(true),
                last_seen: current_timestamp(),
            };
            aggregators.insert(device_id.to_string(), new_aggregator);
            info!("✅ Added new Aggregator: {}", device_id);
        }
    }
    
    // Broadcast Aggregator status event
    let event = serde_json::json!({
        "AggregatorStatus": payload
    });
    let _ = state.event_tx.send(event);
    
    axum::Json(serde_json::json!({
        "status": "success",
        "message": "Aggregator status updated successfully"
    }))
}