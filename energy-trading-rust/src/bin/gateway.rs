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
    
    // Start the gateway
    let gateway_clone = gateway.clone();
    tokio::spawn(async move {
        if let Err(e) = gateway_clone.start().await {
            eprintln!("❌ Failed to start WebSocket gateway: {}", e);
        }
    });
    
    // Wait a moment for the gateway to start
    sleep(Duration::from_secs(2)).await;
    
    // Spawn event generator task
    tokio::spawn(async move {
        sleep(Duration::from_secs(5)).await; // Wait for clients to connect
        
        println!("🎯 Starting enhanced event generator...");
        
        let mut auction_id = 1;
        
        loop {
            // Generate auction events
            let total_energy = 100.0 + (auction_id as f64 * 2.0) % 20.0; // 100-120 kWh
            let reserve_price = 5.0 + (auction_id as f64 * 0.1) % 10.0; // 5-15 c/kWh (realistic Australian FiT)
            
            // Broadcast auction started event
            let auction_event = SystemEvent::AuctionStarted {
                auction_id: auction_id as u64,
                total_energy,
                reserve_price,
            };
            event_gateway.broadcast_event(auction_event).await.unwrap();
            println!("🎯 Auction #{} started: {:.1} kWh at {:.1}¢/kWh", auction_id, total_energy, reserve_price);

            // Step 1: Aggregators query BESS nodes for energy availability (once per auction)
            println!("🔍 Aggregators querying BESS nodes for energy availability...");
            for i in 1..=5 {
                for bess_id in 101..=103 {
                    let query_event = SystemEvent::QuerySent {
                        aggregator_id: i as u64,
                        bess_id: bess_id as u64,
                    };
                    event_gateway.broadcast_event(query_event).await.unwrap();
                    println!("❓ Aggregator {} queries BESS Node {}", i, bess_id);
                    
                    // Random micro-delay between queries (50-200ms)
                    let micro_delay = 50.0 + ((i * bess_id) as f64 * 0.1) % 150.0;
                    sleep(Duration::from_millis(micro_delay as u64)).await;
                }
            }

            // Step 2: BESS nodes respond with energy availability
            println!("📡 BESS nodes responding with energy availability...");
            for bess_id in 101..=103 {
                let energy_available = 8.0 + ((bess_id - 101) as f64 * 2.0); // 8-12 kWh
                let percentage_for_sale = 70.0 + ((bess_id - 101) as f64 * 10.0); // 70-90%
                
                let response_event = SystemEvent::QueryResponse {
                    bess_id: bess_id as u64,
                    energy_available,
                    percentage_for_sale,
                };
                event_gateway.broadcast_event(response_event).await.unwrap();
                println!("📊 BESS Node {}: {:.1} kWh available ({:.0}% for sale)", bess_id, energy_available, percentage_for_sale);
                sleep(Duration::from_millis(300)).await;
            }

            // Step 3: Aggregators place informed bids based on query responses
            println!("💰 Aggregators placing informed bids...");
            for i in 1..=5 {
                let bid_price = 5.0 + ((auction_id * 11 + i as u64 * 17) as f64 * 0.47) % 25.0; // 5-30 c/kWh
                let energy_amount = 2.0 + (i as f64 * 1.0); // 2-6 kWh
                let target_bess = 101 + (i % 3); // Distribute bids across BESS nodes

                let bid_event = SystemEvent::BidPlaced {
                    auction_id: auction_id as u64,
                    aggregator_id: i as u64,
                    bess_id: target_bess as u64,
                    bid_price,
                    energy_amount,
                };
                event_gateway.broadcast_event(bid_event).await.unwrap();
                println!("💰 Aggregator {} bid {:.1}¢/kWh for {:.1} kWh to BESS Node {}", i, bid_price, energy_amount, target_bess);
                
                sleep(Duration::from_secs(1)).await;
            }

            // Accept the best bid (highest price)
            let final_price = 8.0 + ((auction_id * 19 + 23) as f64 * 0.61) % 22.0; // 8-30 c/kWh
            let energy_amount = 4.0; // 4 kWh

            let accept_event = SystemEvent::BidAccepted {
                auction_id: auction_id as u64,
                aggregator_id: 3,
                bess_id: 123,
                final_price,
                energy_amount,
            };
            event_gateway.broadcast_event(accept_event).await.unwrap();
            println!("✅ Auction #{} completed: {:.1} kWh at {:.1}¢/kWh", auction_id, energy_amount, final_price);

            // Step 4: BESS nodes evaluate and respond to bids (realistic rejection logic)
            println!("⚖️ BESS nodes evaluating bids...");
            for i in 1..=5 {
                if i != 3 { // Don't reject the accepted bid
                    let target_bess = 101 + (i % 3);
                    let energy_requested = 2.0 + (i as f64 * 1.0);
                    let bid_price = 5.0 + ((auction_id * 11 + i as u64 * 17) as f64 * 0.47) % 25.0;
                    
                    // Realistic rejection reasons based on query responses
                    let reason = if bid_price < 8.0 {
                        "Bid price below reserve price"
                    } else if energy_requested > 10.0 {
                        "Insufficient energy available"
                    } else if i % 2 == 0 {
                        "BESS node capacity exceeded"
                    } else {
                        "Bid not competitive enough"
                    };
                    
                    let reject_event = SystemEvent::BidRejected {
                        aggregator_id: i as u64,
                        bess_id: target_bess as u64,
                        reason: reason.to_string(),
                    };
                    event_gateway.broadcast_event(reject_event).await.unwrap();
                    println!("❌ Aggregator {} bid rejected by BESS Node {}: {}", i, target_bess, reason);
                    sleep(Duration::from_millis(500)).await;
                }
            }

            // Broadcast BESS node status updates
            for device_id in 101..=103 {
                let energy_available = 8.0 + ((device_id - 101) as f64 * 2.0); // 8-12 kWh (realistic for 15kWh battery)
                let battery_health = (device_id - 101) as u64; // 0, 1, 2
                
                let status_event = SystemEvent::BESSNodeStatus {
                    device_id: device_id as u64,
                    energy_available,
                    battery_health: battery_health as u8,
                    is_online: true,
                };
                event_gateway.broadcast_event(status_event).await.unwrap();
            }

            // Broadcast aggregator status updates
            for device_id in 201..=205 {
                let strategy = match device_id % 4 {
                    0 => "Random",
                    1 => "Conservative", 
                    2 => "Aggressive",
                    _ => "Intelligent",
                };
                let success_rate = 65.0 + (device_id as f64 * 5.0) % 25.0; // 65-90%
                let total_bids = (auction_id * 3) as u64; // 3 bids per auction
                let successful_bids = (total_bids as f64 * (success_rate / 100.0)) as u64; // Calculate successful bids
                let total_energy_bought = successful_bids as f64 * 4.0; // 4 kWh per successful bid
                let average_bid_price = 5.0 + (device_id as f64 * 2.0) % 20.0; // 5-25 c/kWh

                let status_event = SystemEvent::AggregatorStatus {
                    device_id: device_id as u64,
                    strategy: strategy.to_string(),
                    success_rate,
                    total_bids,
                    successful_bids,
                    total_energy_bought,
                    average_bid_price,
                    is_online: true,
                };
                event_gateway.broadcast_event(status_event).await.unwrap();
            }

            // Broadcast system metrics
            let metrics_event = SystemEvent::SystemMetrics {
                total_auctions: auction_id as u64,
                total_bids: (auction_id * 3) as u64, // 3 bids per auction
                avg_price_improvement_percent: 200.0 + (auction_id as f64 * 5.0) % 100.0, // 200-300%
                active_bess_nodes: 3,
                active_aggregators: 5,
            };
            event_gateway.broadcast_event(metrics_event).await.unwrap();

            auction_id += 1;
            // Random delay between 2-10 seconds before next auction
            let delay_seconds = 2.0 + ((auction_id * 7) as f64 * 0.13) % 8.0; // 2-10 seconds
            println!("⏳ Waiting {:.1} seconds before next auction...", delay_seconds);
            sleep(Duration::from_secs_f64(delay_seconds)).await;
        }
    });
    
    // Keep the main thread alive
    tokio::signal::ctrl_c().await?;
    println!("Shutting down gateway...");
    
    Ok(())
}