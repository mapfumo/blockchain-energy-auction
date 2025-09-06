use energy_trading::network::websocket_gateway::{SystemEvent, WebSocketGateway};
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    
    println!("🎯 Energy Trading Event Generator");
    println!("=================================");
    
    // Create WebSocket gateway
    let gateway = WebSocketGateway::new(8080).await?;
    println!("✅ WebSocket Gateway created on port 8080");
    
    // Start the gateway in a separate task
    let gateway_handle = tokio::spawn(async move {
        if let Err(e) = gateway.start().await {
            eprintln!("❌ Gateway error: {}", e);
        }
    });
    
    // Wait a moment for the gateway to start
    sleep(Duration::from_secs(2)).await;
    
    // Create a new gateway instance for broadcasting
    let broadcast_gateway = WebSocketGateway::new(8081).await?;
    
    println!("📡 Generating test events...");
    
    // Generate some test events
    let events = vec![
        SystemEvent::AuctionStarted {
            auction_id: 1,
            total_energy: 100.0,
            reserve_price: 15.0,
        },
        SystemEvent::BidPlaced {
            aggregator_id: 1,
            bess_id: 123,
            bid_price: 18.0,
            energy_amount: 10.0,
        },
        SystemEvent::BidPlaced {
            aggregator_id: 2,
            bess_id: 123,
            bid_price: 20.0,
            energy_amount: 15.0,
        },
        SystemEvent::BidAccepted {
            aggregator_id: 2,
            bess_id: 123,
            final_price: 20.0,
            energy_amount: 15.0,
        },
        SystemEvent::SystemMetrics {
            total_auctions: 1,
            total_bids: 2,
            avg_price_improvement_percent: 33.3,
            active_bess_nodes: 1,
            active_aggregators: 2,
        },
    ];
    
    // Broadcast events with delays
    for (i, event) in events.iter().enumerate() {
        println!("📤 Broadcasting event {}: {:?}", i + 1, event);
        broadcast_gateway.broadcast_event(event.clone()).await?;
        sleep(Duration::from_secs(2)).await;
    }
    
    println!("✅ All events broadcasted successfully!");
    println!("🌐 Check the dashboard at http://localhost:3000 to see the events");
    
    // Keep the gateway running
    gateway_handle.await?;
    
    Ok(())
}
