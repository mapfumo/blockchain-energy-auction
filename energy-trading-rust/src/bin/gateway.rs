use energy_trading::network::websocket_gateway::{WebSocketGateway, SystemEvent};
use energy_trading::bess_node::EnergyStatus;
use energy_trading::database::{DatabaseConnection, Repository, NewBattery, NewAggregator};
use std::time::{Duration, Instant};
use tokio::time::sleep;
use sqlx::types::BigDecimal;

async fn initialize_database_data(repository: &Repository) -> Result<(), Box<dyn std::error::Error>> {
    // Create BESS nodes (batteries)
    let bess_nodes = vec![
        NewBattery {
            device_id: 101,
            owner_pubkey: "BESS101_OWNER_PUBKEY".to_string(),
            energy_total: BigDecimal::from(15i64), // 15 kWh capacity
            percentage_for_sale: BigDecimal::from(70i64), // 70% for sale
            reserve_price: BigDecimal::from(8i64), // 8 c/kWh
            health_status: 0, // Good health
            voltage: BigDecimal::from(48i64), // 48V
            discharge_rate: BigDecimal::from(5i64), // 5 kW
            status: "online".to_string(),
        },
        NewBattery {
            device_id: 102,
            owner_pubkey: "BESS102_OWNER_PUBKEY".to_string(),
            energy_total: BigDecimal::from(15i64),
            percentage_for_sale: BigDecimal::from(80i64),
            reserve_price: BigDecimal::from(7i64),
            health_status: 1,
            voltage: BigDecimal::from(24i64),
            discharge_rate: BigDecimal::from(4i64),
            status: "online".to_string(),
        },
        NewBattery {
            device_id: 103,
            owner_pubkey: "BESS103_OWNER_PUBKEY".to_string(),
            energy_total: BigDecimal::from(15i64),
            percentage_for_sale: BigDecimal::from(90i64),
            reserve_price: BigDecimal::from(9i64),
            health_status: 2,
            voltage: BigDecimal::from(48i64),
            discharge_rate: BigDecimal::from(6i64),
            status: "online".to_string(),
        },
    ];

    // Create aggregators
    let aggregators = vec![
        NewAggregator {
            device_id: 201,
            owner_pubkey: "AGG201_OWNER_PUBKEY".to_string(),
            max_bid_price: BigDecimal::from(30i64), // 30 c/kWh max
            energy_requirements: BigDecimal::from(50i64), // 50 kWh requirements
            reputation_score: 100,
            status: "online".to_string(),
        },
        NewAggregator {
            device_id: 202,
            owner_pubkey: "AGG202_OWNER_PUBKEY".to_string(),
            max_bid_price: BigDecimal::from(25i64),
            energy_requirements: BigDecimal::from(40i64),
            reputation_score: 95,
            status: "online".to_string(),
        },
        NewAggregator {
            device_id: 203,
            owner_pubkey: "AGG203_OWNER_PUBKEY".to_string(),
            max_bid_price: BigDecimal::from(28i64),
            energy_requirements: BigDecimal::from(45i64),
            reputation_score: 98,
            status: "online".to_string(),
        },
        NewAggregator {
            device_id: 204,
            owner_pubkey: "AGG204_OWNER_PUBKEY".to_string(),
            max_bid_price: BigDecimal::from(32i64),
            energy_requirements: BigDecimal::from(55i64),
            reputation_score: 100,
            status: "online".to_string(),
        },
        NewAggregator {
            device_id: 205,
            owner_pubkey: "AGG205_OWNER_PUBKEY".to_string(),
            max_bid_price: BigDecimal::from(26i64),
            energy_requirements: BigDecimal::from(35i64),
            reputation_score: 92,
            status: "online".to_string(),
        },
    ];

    // Insert BESS nodes (ignore duplicates)
    for battery in bess_nodes {
        if let Err(e) = repository.create_battery(battery).await {
            if e.to_string().contains("duplicate key") {
                println!("ℹ️  BESS node already exists, skipping...");
            } else {
                return Err(e.into());
            }
        }
    }
    println!("✅ BESS nodes ready in database");

    // Insert aggregators (ignore duplicates)
    for aggregator in aggregators {
        if let Err(e) = repository.create_aggregator(aggregator).await {
            if e.to_string().contains("duplicate key") {
                println!("ℹ️  Aggregator already exists, skipping...");
            } else {
                return Err(e.into());
            }
        }
    }
    println!("✅ Aggregators ready in database");

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    println!("Starting Energy Trading WebSocket Gateway...");

    // Try to initialize database (optional)
    let db_connection = match DatabaseConnection::new().await {
        Ok(db) => {
            println!("✅ Database connected successfully");
            
            // Initialize database with required data
            let repository = Repository::new(db.clone());
            if let Err(e) = initialize_database_data(&repository).await {
                println!("⚠️  Failed to initialize database data: {}. Running in mock mode.", e);
                None
            } else {
                println!("💾 Database initialized with BESS nodes and aggregators");
                Some(db)
            }
        }
        Err(e) => {
            println!("⚠️  Database connection failed: {}. Running in mock mode.", e);
            None
        }
    };

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
        let mut bess_energy_levels = [8.0, 10.0, 12.0]; // Starting energy levels for BESS 101, 102, 103
        let mut last_recharge_time = Instant::now();
        
        loop {
            // Calculate realistic total energy based on actual BESS availability
            let total_available_energy: f64 = bess_energy_levels.iter().sum();
            let total_energy = if total_available_energy > 0.0 {
                // Use actual available energy with some variation (80-100% of available)
                let variation = 0.8 + ((auction_id as f64 * 0.1) % 0.2); // 0.8-1.0
                (total_available_energy * variation).round()
            } else {
                0.0 // No energy available
            };
            let reserve_price = 5.0 + (auction_id as f64 * 0.1) % 10.0; // 5-15 c/kWh (realistic Australian FiT)
            
            // Broadcast auction started event
            let auction_event = SystemEvent::AuctionStarted {
                auction_id: auction_id as u64,
                total_energy,
                reserve_price,
            };
            event_gateway.broadcast_event(auction_event).await.unwrap();
            println!("🎯 Auction #{} started: {:.1} kWh at {:.1}¢/kWh", auction_id, total_energy, reserve_price);

            // Persist auction to database if available
            if let Some(db) = &db_connection {
                let repository = Repository::new(db.clone());
                use energy_trading::database::NewAuction;
                use sqlx::types::BigDecimal;
                
                let new_auction = NewAuction {
                    battery_id: 101, // Default battery_id for now
                    energy_amount: BigDecimal::from(total_energy as i64),
                    reserve_price: BigDecimal::from(reserve_price as i64),
                    status: "active".to_string(),
                };
                
                if let Err(e) = repository.create_auction(new_auction).await {
                    println!("⚠️  Failed to persist auction to database: {}", e);
                } else {
                    println!("💾 Auction #{} persisted to database", auction_id);
                }
            }

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

            // Step 2: BESS nodes respond with energy availability (with recharge simulation)
            println!("📡 BESS nodes responding with energy availability...");
            
            // Simulate energy recharge over time
            let current_time = Instant::now();
            let time_elapsed = current_time.duration_since(last_recharge_time).as_secs_f64();
            last_recharge_time = current_time;
            
            for bess_id in 101..=103 {
                let bess_index = (bess_id - 101) as usize;
                let energy_percentage = (bess_energy_levels[bess_index] / 15.0) * 100.0;
                
                // Enhanced recharge logic: 5% per second when below 10% capacity
                let recharge_rate = if energy_percentage < 10.0 {
                    0.75 // 5% of 15kWh = 0.75 kWh per second (fast recharge when critical)
                } else {
                    0.05 // 0.05 kWh per second (normal solar charging)
                };
                
                let recharge_amount = recharge_rate * time_elapsed;
                bess_energy_levels[bess_index] = (bess_energy_levels[bess_index] + recharge_amount).min(15.0); // Max 15 kWh
                
                let energy_available = bess_energy_levels[bess_index];
                let new_energy_percentage = (energy_available / 15.0) * 100.0;
                let percentage_for_sale = 70.0 + ((bess_id - 101) as f64 * 10.0); // 70-90%
                
                // Log enhanced recharge when critical
                if energy_percentage < 10.0 && new_energy_percentage > energy_percentage {
                    println!("⚡ BESS Node {} critical recharge: {:.1}% → {:.1}% ({:.1} kWh)", 
                             bess_id, energy_percentage, new_energy_percentage, energy_available);
                }
                
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
                
                // Random delay between 5-10 seconds after each bid
                let bid_delay = 5.0 + ((auction_id * 7 + i as u64 * 13) as f64 * 0.23) % 5.0; // 5-10 seconds
                println!("⏳ Aggregator {} waiting {:.1} seconds before next action...", i, bid_delay);
                sleep(Duration::from_secs_f64(bid_delay)).await;
            }

            // Accept the best bid (highest price) and deplete energy
            let final_price = 8.0 + ((auction_id * 19 + 23) as f64 * 0.61) % 22.0; // 8-30 c/kWh
            let energy_amount = 4.0; // 4 kWh
            let winning_bess = 101 + (auction_id % 3); // Rotate winning BESS
            let bess_index = (winning_bess - 101) as usize;

            // Deplete energy from winning BESS
            bess_energy_levels[bess_index] = (bess_energy_levels[bess_index] - energy_amount).max(0.0);
            let new_energy_percentage = (bess_energy_levels[bess_index] / 15.0) * 100.0;

            let accept_event = SystemEvent::BidAccepted {
                auction_id: auction_id as u64,
                aggregator_id: 3,
                bess_id: winning_bess as u64,
                final_price,
                energy_amount,
            };
            event_gateway.broadcast_event(accept_event).await.unwrap();
            
            // Send detailed auction completion event
            let total_value = final_price * energy_amount;
            let auction_duration_ms = 15000 + (auction_id as u64 * 2000) % 10000; // 15-25 seconds
            let completed_event = SystemEvent::AuctionCompleted {
                auction_id: auction_id as u64,
                winner_aggregator_id: 3,
                seller_bess_id: winning_bess as u64,
                energy_sold: energy_amount,
                final_price,
                total_value,
                auction_duration_ms,
            };
            event_gateway.broadcast_event(completed_event).await.unwrap();
            
            println!("✅ Auction #{} completed: {:.1} kWh at {:.1}¢/kWh (BESS {}: {:.1} kWh remaining)", 
                     auction_id, energy_amount, final_price, winning_bess, bess_energy_levels[bess_index]);
            println!("🏆 Winner: Aggregator {} bought {:.1} kWh from BESS {} for {:.1}¢/kWh (Total: ${:.2})", 
                     3, energy_amount, winning_bess, final_price, total_value / 100.0);

            // Check if BESS is depleted and send event
            if bess_energy_levels[bess_index] <= 0.1 {
                let depleted_event = SystemEvent::EnergyDepleted {
                    bess_id: winning_bess as u64,
                    final_energy: bess_energy_levels[bess_index],
                    energy_percentage: new_energy_percentage,
                };
                event_gateway.broadcast_event(depleted_event).await.unwrap();
                println!("🔋 BESS Node {} energy depleted! ({:.1}% remaining)", winning_bess, new_energy_percentage);
            }

            // Step 4: BESS nodes evaluate and respond to bids (realistic rejection logic)
            println!("⚖️ BESS nodes evaluating bids...");
            for i in 1..=5 {
                if i != 3 { // Don't reject the accepted bid
                    let target_bess = 101 + (i % 3);
                    let energy_requested = 2.0 + (i as f64 * 1.0);
                    let bid_price = 5.0 + ((auction_id * 11 + i as u64 * 17) as f64 * 0.47) % 25.0;
                    
                    // Get actual available energy for the target BESS
                    let bess_index = (target_bess - 101) as usize;
                    let available_energy = bess_energy_levels[bess_index];
                    
                    // Realistic rejection reasons based on actual energy availability
                    let reason = if bid_price < 8.0 {
                        "Bid price below reserve price"
                    } else if energy_requested > available_energy {
                        "BESS node capacity exceeded"
                    } else if energy_requested > (available_energy - 0.5) {
                        "BESS node capacity exceeded" // Too close to available energy
                    } else if energy_requested > 10.0 {
                        "Insufficient energy available"
                    } else {
                        "Bid not competitive enough"
                    };
                    
                    println!("❌ Rejecting Aggregator {}: {} (bid: {:.1}¢/kWh, requested: {:.1} kWh, available: {:.1} kWh)", 
                             i, reason, bid_price, energy_requested, available_energy);
                    
                    let reject_event = SystemEvent::BidRejected {
                        aggregator_id: i as u64,
                        bess_id: target_bess as u64,
                        reason: reason.to_string(),
                    };
                    event_gateway.broadcast_event(reject_event).await.unwrap();
                    println!("❌ Aggregator {} bid rejected by BESS Node {}: {}", i, target_bess, reason);
                    
                    // Random delay after rejection (2-5 seconds)
                    let rejection_delay = 2.0 + ((auction_id * 11 + i as u64 * 19) as f64 * 0.31) % 3.0; // 2-5 seconds
                    println!("⏳ Aggregator {} waiting {:.1} seconds after rejection...", i, rejection_delay);
                    sleep(Duration::from_secs_f64(rejection_delay)).await;
                }
            }

            // Broadcast BESS node status updates with actual energy levels
            for device_id in 101..=103 {
                let bess_index = (device_id - 101) as usize;
                let energy_available = bess_energy_levels[bess_index];
                let battery_health = (device_id - 101) as u64; // 0, 1, 2
                let energy_percentage = (energy_available / 15.0) * 100.0;
                
                let status_event = SystemEvent::BESSNodeStatus {
                    device_id: device_id as u64,
                    energy_available,
                    battery_health: battery_health as u8,
                    is_online: true,
                };
                event_gateway.broadcast_event(status_event).await.unwrap();
                
                // Send recharge event if energy increased significantly
                if energy_available > 12.0 && energy_percentage > 80.0 {
                    let recharge_event = SystemEvent::EnergyRecharged {
                        bess_id: device_id as u64,
                        energy_added: 0.0, // Will be calculated in real implementation
                        new_total: energy_available,
                        energy_percentage,
                    };
                    event_gateway.broadcast_event(recharge_event).await.unwrap();
                    println!("🔋 BESS Node {} recharged to {:.1} kWh ({:.1}%)", device_id, energy_available, energy_percentage);
                }
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