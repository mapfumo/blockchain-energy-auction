use energy_trading::network::websocket_gateway::SystemEvent;
use serde_json;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use futures_util::{SinkExt, StreamExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    
    println!("🔌 Testing WebSocket Connection to Gateway...");
    
    // Connect to WebSocket
    let url = "ws://localhost:8080/ws";
    let (ws_stream, _) = connect_async(url).await?;
    println!("✅ Connected to WebSocket gateway at {}", url);
    
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();
    
    // Spawn task to receive messages
    let receive_task = tokio::spawn(async move {
        while let Some(msg) = ws_receiver.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    println!("📨 Received: {}", text);
                    // Try to parse as SystemEvent
                    if let Ok(event) = serde_json::from_str::<SystemEvent>(&text) {
                        println!("📊 Parsed event: {:?}", event);
                    }
                }
                Ok(Message::Close(_)) => {
                    println!("🔌 WebSocket closed by server");
                    break;
                }
                Err(e) => {
                    println!("❌ WebSocket error: {}", e);
                    break;
                }
                _ => {}
            }
        }
    });
    
    // Send a test message
    let test_event = SystemEvent::SystemMetrics {
        total_auctions: 5,
        total_bids: 12,
        avg_price_improvement_percent: 15.5,
        active_bess_nodes: 3,
        active_aggregators: 2,
    };
    
    let test_message = serde_json::to_string(&test_event)?;
    ws_sender.send(Message::Text(test_message)).await?;
    println!("📤 Sent test event: {:?}", test_event);
    
    // Wait for a few seconds to receive any messages
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    
    // Close the connection
    ws_sender.close().await?;
    receive_task.abort();
    
    println!("✅ WebSocket test completed");
    Ok(())
}
