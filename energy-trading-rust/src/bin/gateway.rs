use energy_trading::network::websocket_gateway::{WebSocketGateway, SystemEvent};
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    println!("Starting Energy Trading WebSocket Gateway...");

    // Create and start the WebSocket gateway
    let gateway = WebSocketGateway::new(8080).await?;
    
    println!("WebSocket Gateway starting on port 8080...");
    println!("Connect to: ws://localhost:8080/ws");
    
    // Clone the gateway for event generation
    let event_gateway = gateway.clone();
    
    // Spawn event generator task
    tokio::spawn(async move {
        sleep(Duration::from_secs(5)).await; // Wait for clients to connect
        
        println!("🎯 Starting event generator...");
        
        let mut auction_id = 1;
        let mut bid_id = 1;
        
        loop {
            // Generate auction started event
            let auction_event = SystemEvent::AuctionStarted {
                auction_id,
                total_energy: 100.0 + (auction_id as f64 * 10.0),
                reserve_price: 15.0 + (auction_id as f64 * 0.5),
            };
            event_gateway.broadcast_event(auction_event).await.unwrap();
            
            sleep(Duration::from_secs(3)).await;
            
            // Generate bid events
            for i in 1..=3 {
                let bid_event = SystemEvent::BidPlaced {
                    aggregator_id: i,
                    bess_id: 123,
                    bid_price: 18.0 + (i as f64 * 2.0),
                    energy_amount: 10.0 + (i as f64 * 5.0),
                };
                event_gateway.broadcast_event(bid_event).await.unwrap();
                sleep(Duration::from_secs(2)).await;
            }
            
            // Generate bid accepted event
            let accept_event = SystemEvent::BidAccepted {
                aggregator_id: 3,
                bess_id: 123,
                final_price: 24.0,
                energy_amount: 20.0,
            };
            event_gateway.broadcast_event(accept_event).await.unwrap();
            
            sleep(Duration::from_secs(2)).await;
            
            // Generate system metrics
            let metrics_event = SystemEvent::SystemMetrics {
                total_auctions: auction_id,
                total_bids: bid_id * 3,
                avg_price_improvement_percent: 25.0 + (auction_id as f64 * 2.0),
                active_bess_nodes: 3,
                active_aggregators: 5,
            };
            event_gateway.broadcast_event(metrics_event).await.unwrap();
            
            auction_id += 1;
            bid_id += 1;
            
            sleep(Duration::from_secs(10)).await; // Wait before next cycle
        }
    });
    
    // Start the gateway (this will block)
    gateway.start().await?;

    Ok(())
}
