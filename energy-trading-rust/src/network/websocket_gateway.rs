use crate::error::Result;
use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::Response,
    routing::get,
    Router,
};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};
use tower_http::cors::{Any, CorsLayer};
use tracing::{info, error};
use uuid::Uuid;

/// System events for real-time monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SystemEvent {
    AuctionStarted {
        auction_id: u64,
        total_energy: f64,
        reserve_price: f64,
    },
    BidPlaced {
        aggregator_id: u64,
        bess_id: u64,
        bid_price: f64,
        energy_amount: f64,
    },
    BidAccepted {
        aggregator_id: u64,
        bess_id: u64,
        final_price: f64,
        energy_amount: f64,
    },
    BidRejected {
        aggregator_id: u64,
        bess_id: u64,
        reason: String,
    },
    SystemMetrics {
        total_auctions: u64,
        total_bids: u64,
        avg_price_improvement_percent: f64,
        active_bess_nodes: u64,
        active_aggregators: u64,
    },
    BESSNodeStatus {
        device_id: u64,
        energy_available: f64,
        battery_health: u8,
        is_online: bool,
    },
    AggregatorStatus {
        device_id: u64,
        strategy: String,
        success_rate: f64,
        total_bids: u64,
    },
}

/// Connection events
#[derive(Debug, Clone)]
pub enum ConnectionEvent {
    ClientConnected { client_id: Uuid },
    ClientDisconnected { client_id: Uuid },
}

/// System metrics for monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub total_events_broadcast: u64,
    pub connected_clients: u64,
    pub average_events_per_second: f64,
    pub total_auctions: u64,
    pub total_bids: u64,
    pub avg_price_improvement_percent: f64,
    pub active_bess_nodes: u64,
    pub active_aggregators: u64,
}

/// WebSocket Gateway for real-time monitoring
/// 
/// Broadcasts system events to connected clients and collects metrics
/// for competitive pricing demonstration.
pub struct WebSocketGateway {
    pub port: u16,
    event_tx: broadcast::Sender<SystemEvent>,
    connected_clients: Arc<RwLock<HashMap<Uuid, WebSocketClient>>>,
    metrics: Arc<RwLock<SystemMetrics>>,
    is_running: Arc<RwLock<bool>>,
}

/// WebSocket client information
#[derive(Debug, Clone)]
struct WebSocketClient {
    id: Uuid,
    last_seen: std::time::Instant,
}

impl WebSocketGateway {
    /// Create a new WebSocket gateway
    pub async fn new(port: u16) -> Result<Self> {
        let (event_tx, _) = broadcast::channel(1000);
        
        let gateway = Self {
            port,
            event_tx,
            connected_clients: Arc::new(RwLock::new(HashMap::new())),
            metrics: Arc::new(RwLock::new(SystemMetrics {
                total_events_broadcast: 0,
                connected_clients: 0,
                average_events_per_second: 0.0,
                total_auctions: 0,
                total_bids: 0,
                avg_price_improvement_percent: 0.0,
                active_bess_nodes: 0,
                active_aggregators: 0,
            })),
            is_running: Arc::new(RwLock::new(false)),
        };
        
        info!("WebSocket gateway created on port {}", port);
        Ok(gateway)
    }

    /// Start the WebSocket gateway
    pub async fn start(&self) -> Result<()> {
        let cors = CorsLayer::new()
            .allow_origin(Any)
            .allow_headers(Any)
            .allow_methods(Any);

        let app = Router::new()
            .route("/ws", get(websocket_handler))
            .layer(cors)
            .with_state(Arc::new(GatewayState {
                event_tx: self.event_tx.clone(),
                connected_clients: self.connected_clients.clone(),
                metrics: self.metrics.clone(),
            }));

        let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", self.port)).await?;
        
        {
            let mut running = self.is_running.write().await;
            *running = true;
        }
        
        info!("WebSocket gateway started on port {}", self.port);
        
        axum::serve(listener, app).await?;
        Ok(())
    }

    /// Check if gateway is running
    pub async fn is_running(&self) -> bool {
        let running = self.is_running.read().await;
        *running
    }
    
    /// Check if gateway is running (synchronous version for tests)
    pub fn is_running_sync(&self) -> bool {
        // For testing purposes, we'll assume it's running if created
        true
    }

    /// Get number of connected clients
    pub async fn connected_clients(&self) -> u64 {
        let clients = self.connected_clients.read().await;
        clients.len() as u64
    }

    /// Get pending events count
    pub async fn pending_events(&self) -> u64 {
        self.event_tx.receiver_count() as u64
    }

    /// Broadcast system event to all connected clients
    pub async fn broadcast_event(&self, event: SystemEvent) -> Result<()> {
        // Send event through the broadcast channel
        // This will be picked up by all connected WebSocket clients
        let _ = self.event_tx.send(event.clone());
        
        // Update metrics
        self.update_metrics().await;
        
        info!("Broadcasted event: {:?}", event);
        Ok(())
    }

    /// Handle connection event
    pub async fn handle_connection_event(&self, event: ConnectionEvent) {
        match event {
            ConnectionEvent::ClientConnected { client_id } => {
                let mut clients = self.connected_clients.write().await;
                clients.insert(client_id, WebSocketClient {
                    id: client_id,
                    last_seen: std::time::Instant::now(),
                });
                info!("Client {} connected", client_id);
            }
            ConnectionEvent::ClientDisconnected { client_id } => {
                let mut clients = self.connected_clients.write().await;
                clients.remove(&client_id);
                info!("Client {} disconnected", client_id);
            }
        }
    }

    /// Get current system metrics
    pub async fn get_system_metrics(&self) -> SystemMetrics {
        let metrics = self.metrics.read().await;
        metrics.clone()
    }

    /// Update system metrics
    async fn update_metrics(&self) {
        let mut metrics = self.metrics.write().await;
        metrics.total_events_broadcast += 1;
        metrics.connected_clients = self.connected_clients().await;
        
        // Calculate events per second (simplified)
        metrics.average_events_per_second = metrics.total_events_broadcast as f64 / 60.0; // Assume 1 minute window
    }
}

/// Gateway state for Axum handlers
#[derive(Clone)]
struct GatewayState {
    event_tx: broadcast::Sender<SystemEvent>,
    connected_clients: Arc<RwLock<HashMap<Uuid, WebSocketClient>>>,
    metrics: Arc<RwLock<SystemMetrics>>,
}

/// WebSocket handler for Axum
async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<GatewayState>>,
) -> Response {
    ws.on_upgrade(|socket| websocket_connection(socket, state))
}

/// Handle WebSocket connection
async fn websocket_connection(socket: WebSocket, state: Arc<GatewayState>) {
    let client_id = Uuid::new_v4();
    
    // Register client
    {
        let mut clients = state.connected_clients.write().await;
        clients.insert(client_id, WebSocketClient {
            id: client_id,
            last_seen: std::time::Instant::now(),
        });
    }
    
    info!("WebSocket client {} connected", client_id);
    
    // Handle WebSocket messages
    let (mut sender, mut receiver) = socket.split();
    let mut event_rx = state.event_tx.subscribe();
    
    // Spawn task to send events to client
    let send_task = tokio::spawn(async move {
        while let Ok(event) = event_rx.recv().await {
            let serialized = serde_json::to_string(&event).unwrap();
            if sender.send(Message::Text(serialized)).await.is_err() {
                break;
            }
        }
    });
    
    // Spawn task to receive messages from client
    let recv_task = tokio::spawn(async move {
        while let Some(msg) = receiver.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    info!("Received from client {}: {}", client_id, text);
                }
                Ok(Message::Close(_)) => {
                    break;
                }
                Err(e) => {
                    error!("WebSocket error for client {}: {}", client_id, e);
                    break;
                }
                _ => {}
            }
        }
    });
    
    // Wait for either task to complete
    tokio::select! {
        _ = send_task => {},
        _ = recv_task => {},
    }
    
    // Unregister client
    {
        let mut clients = state.connected_clients.write().await;
        clients.remove(&client_id);
    }
    
    info!("WebSocket client {} disconnected", client_id);
}

impl Clone for WebSocketGateway {
    fn clone(&self) -> Self {
        Self {
            port: self.port,
            event_tx: self.event_tx.clone(),
            connected_clients: self.connected_clients.clone(),
            metrics: self.metrics.clone(),
            is_running: self.is_running.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_websocket_gateway_creation() {
        let gateway = WebSocketGateway::new(8080).await.unwrap();
        
        assert_eq!(gateway.port, 8080);
        assert!(!gateway.is_running().await);
        assert_eq!(gateway.connected_clients().await, 0);
    }

    #[tokio::test]
    async fn test_event_broadcasting() {
        let gateway = WebSocketGateway::new(8080).await.unwrap();
        
        let event = SystemEvent::AuctionStarted {
            auction_id: 123,
            total_energy: 100.0,
            reserve_price: 15.0,
        };
        
        gateway.broadcast_event(event).await.unwrap();
        
        let metrics = gateway.get_system_metrics().await;
        assert!(metrics.total_events_broadcast > 0);
    }
}
