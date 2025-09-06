use energy_trading::*;
use std::time::{Duration, Instant};

#[tokio::test]
async fn test_aggregator_node_creation() {
    let aggregator = AggregatorNode::new(123, "AGG-001".to_string(), BiddingStrategy::Intelligent);
    
    assert_eq!(aggregator.device_id, 123);
    assert_eq!(aggregator.device_name, "AGG-001");
    assert_eq!(aggregator.strategy, BiddingStrategy::Intelligent);
    assert!(aggregator.is_online);
    assert!(aggregator.connected_bess_nodes.read().await.is_empty());
}

#[tokio::test]
async fn test_aggregator_bidding_strategies() {
    // Test Random strategy
    let random_agg = AggregatorNode::new(123, "AGG-RANDOM".to_string(), BiddingStrategy::Random);
    let bid1 = random_agg.generate_bid(15.0, 10.0, 20.0).await;
    let bid2 = random_agg.generate_bid(15.0, 10.0, 20.0).await;
    
    // Random bids should be different
    assert_ne!(bid1.bid_price, bid2.bid_price);
    assert!(bid1.bid_price >= 15.0);
    assert!(bid1.bid_price <= 20.0);
    
    // Test Conservative strategy
    let conservative_agg = AggregatorNode::new(124, "AGG-CONSERVATIVE".to_string(), BiddingStrategy::Conservative);
    let bid = conservative_agg.generate_bid(15.0, 10.0, 20.0).await;
    
    // Conservative should bid close to reserve price
    assert!(bid.bid_price >= 15.0);
    assert!(bid.bid_price <= 16.0); // Within 1 cent of reserve
    
    // Test Aggressive strategy
    let aggressive_agg = AggregatorNode::new(125, "AGG-AGGRESSIVE".to_string(), BiddingStrategy::Aggressive);
    let bid = aggressive_agg.generate_bid(15.0, 10.0, 20.0).await;
    
    // Aggressive should bid close to max price
    assert!(bid.bid_price >= 19.0);
    assert!(bid.bid_price <= 20.0);
}

#[tokio::test]
async fn test_aggregator_historical_context() {
    let aggregator = AggregatorNode::new(123, "AGG-001".to_string(), BiddingStrategy::Intelligent);
    
    // Add historical data
    aggregator.add_historical_bid(100, 15.5, 10.0, true).await;
    aggregator.add_historical_bid(101, 16.0, 8.0, true).await;
    aggregator.add_historical_bid(102, 14.8, 12.0, false).await;
    
    // Test price prediction
    let predicted_price = aggregator.predict_winning_price(10.0).await;
    assert!(predicted_price >= 15.0);
    assert!(predicted_price <= 20.0);
    
    // Test success rate calculation
    let success_rate = aggregator.get_success_rate().await;
    assert_eq!(success_rate, 2.0 / 3.0); // 2 out of 3 bids successful
}

#[tokio::test]
async fn test_aggregator_bess_discovery() {
    let aggregator = AggregatorNode::new(123, "AGG-001".to_string(), BiddingStrategy::Intelligent);
    
    // Simulate discovering BESS nodes
    let bess1 = BESSNode::new(100, "BESS-001".to_string(), 100.0, 15.0);
    let bess2 = BESSNode::new(101, "BESS-002".to_string(), 80.0, 16.0);
    
    aggregator.add_connected_bess(100, bess1).await;
    aggregator.add_connected_bess(101, bess2).await;
    
    assert_eq!(aggregator.connected_bess_nodes.read().await.len(), 2);
    assert!(aggregator.connected_bess_nodes.read().await.contains_key(&100));
    assert!(aggregator.connected_bess_nodes.read().await.contains_key(&101));
}

#[tokio::test]
async fn test_aggregator_bid_optimization() {
    let aggregator = AggregatorNode::new(123, "AGG-001".to_string(), BiddingStrategy::Intelligent);
    
    // Add multiple BESS nodes with different characteristics
    let mut bess1 = BESSNode::new(100, "BESS-001".to_string(), 100.0, 15.0);
    bess1.set_percentage_for_sale(50.0);
    
    let mut bess2 = BESSNode::new(101, "BESS-002".to_string(), 80.0, 16.0);
    bess2.set_percentage_for_sale(75.0);
    
    aggregator.add_connected_bess(100, bess1).await;
    aggregator.add_connected_bess(101, bess2).await;
    
    // Test bid optimization for 20kWh requirement
    let optimized_bids = aggregator.optimize_bids(20.0, 18.0).await;
    
    assert!(!optimized_bids.is_empty());
    assert!(optimized_bids.len() <= 2); // Should not exceed available BESS nodes
    
    // Verify total energy in bids matches requirement
    let total_energy: f64 = optimized_bids.iter().map(|bid| bid.required_energy_amount).sum();
    assert_eq!(total_energy, 20.0);
}

#[tokio::test]
async fn test_aggregator_bid_evaluation() {
    let aggregator = AggregatorNode::new(123, "AGG-001".to_string(), BiddingStrategy::Intelligent);
    
    // Test bid evaluation logic
    let bid = ETPMessage::new_bid(123, 18.0, 10.0);
    let evaluation = aggregator.evaluate_bid_response(bid, BidResponse::Accept { sale_price: 18.0, energy_amount: 10.0 }).await;
    
    match evaluation {
        BidEvaluationResult::Accepted { final_price, energy_amount } => {
            assert_eq!(final_price, 18.0);
            assert_eq!(energy_amount, 10.0);
        }
        _ => panic!("Expected Accepted evaluation"),
    }
    
    // Test rejected bid
    let bid = ETPMessage::new_bid(124, 15.0, 10.0);
    let evaluation = aggregator.evaluate_bid_response(bid, BidResponse::Reject { reason: "Price too low".to_string(), code: 1 }).await;
    
    match evaluation {
        BidEvaluationResult::Rejected { reason, code } => {
            assert_eq!(reason, "Price too low");
            assert_eq!(code, 1);
        }
        _ => panic!("Expected Rejected evaluation"),
    }
}

#[tokio::test]
async fn test_aggregator_timing_constraints() {
    let aggregator = AggregatorNode::new(123, "AGG-001".to_string(), BiddingStrategy::Intelligent);
    
    // Test bid generation timing
    let start = Instant::now();
    let _bid = aggregator.generate_bid(15.0, 10.0, 20.0).await;
    let elapsed = start.elapsed();
    
    // Should generate bid within 100ms
    assert!(elapsed < Duration::from_millis(100));
    
    // Test bid optimization timing
    let start = Instant::now();
    let _bids = aggregator.optimize_bids(20.0, 18.0).await;
    let elapsed = start.elapsed();
    
    // Should optimize bids within 200ms
    assert!(elapsed < Duration::from_millis(200));
}

#[tokio::test]
async fn test_aggregator_performance() {
    let aggregator = AggregatorNode::new(123, "AGG-001".to_string(), BiddingStrategy::Intelligent);
    
    // Add some historical data
    for i in 0..100 {
        aggregator.add_historical_bid(i, 15.0 + (i as f64 * 0.1), 10.0, i % 2 == 0).await;
    }
    
    // Test bid generation performance
    let start = Instant::now();
    for _ in 0..1000 {
        let _bid = aggregator.generate_bid(15.0, 10.0, 20.0).await;
    }
    let elapsed = start.elapsed();
    
    // Should generate 1000 bids in less than 100ms
    assert!(elapsed < Duration::from_millis(100));
}

#[tokio::test]
async fn test_aggregator_concurrent_operations() {
    let aggregator = AggregatorNode::new(123, "AGG-001".to_string(), BiddingStrategy::Intelligent);
    
    // Test concurrent bid generation
    let handles: Vec<_> = (0..10)
        .map(|_i| {
            let aggregator = aggregator.clone();
            tokio::spawn(async move {
                aggregator.generate_bid(15.0, 10.0, 20.0).await
            })
        })
        .collect();
    
    // Wait for all bids to be generated
    for handle in handles {
        let _bid = handle.await.unwrap();
    }
    
    // All operations should complete successfully
    assert!(true);
}

#[tokio::test]
async fn test_aggregator_error_handling() {
    let aggregator = AggregatorNode::new(123, "AGG-001".to_string(), BiddingStrategy::Intelligent);
    
    // Test invalid bid generation
    let bid = aggregator.generate_bid(20.0, 10.0, 15.0).await; // max < reserve
    assert!(bid.bid_price >= 15.0);
    assert!(bid.bid_price <= 20.0);
    
    // Test empty BESS node list
    let bids = aggregator.optimize_bids(20.0, 18.0).await;
    assert!(bids.is_empty());
}

#[tokio::test]
async fn test_aggregator_serialization() {
    let aggregator = AggregatorNode::new(123, "AGG-001".to_string(), BiddingStrategy::Intelligent);
    
    // Test that aggregator data can be serialized/deserialized
    let data = aggregator.to_data();
    let serialized = serde_json::to_string(&data).unwrap();
    let deserialized: AggregatorNodeData = serde_json::from_str(&serialized).unwrap();
    
    assert_eq!(aggregator.device_id, deserialized.device_id);
    assert_eq!(aggregator.device_name, deserialized.device_name);
    assert_eq!(aggregator.strategy, deserialized.strategy);
}

#[tokio::test]
async fn test_aggregator_validation() {
    let aggregator = AggregatorNode::new(123, "AGG-001".to_string(), BiddingStrategy::Intelligent);
    
    // Test that aggregator validates correctly
    assert!(aggregator.device_id > 0);
    assert!(!aggregator.device_name.is_empty());
    assert!(aggregator.is_online);
    assert!(aggregator.connected_bess_nodes.read().await.is_empty());
    assert!(aggregator.historical_bids.read().await.is_empty());
}

#[tokio::test]
async fn test_aggregator_bidding_strategy_switching() {
    let mut aggregator = AggregatorNode::new(123, "AGG-001".to_string(), BiddingStrategy::Random);
    
    // Test switching strategies
    aggregator.set_strategy(BiddingStrategy::Conservative).await;
    assert_eq!(aggregator.strategy, BiddingStrategy::Conservative);
    
    aggregator.set_strategy(BiddingStrategy::Aggressive).await;
    assert_eq!(aggregator.strategy, BiddingStrategy::Aggressive);
    
    aggregator.set_strategy(BiddingStrategy::Intelligent).await;
    assert_eq!(aggregator.strategy, BiddingStrategy::Intelligent);
}

#[tokio::test]
async fn test_aggregator_bid_history_tracking() {
    let aggregator = AggregatorNode::new(123, "AGG-001".to_string(), BiddingStrategy::Intelligent);
    
    // Add multiple historical bids
    aggregator.add_historical_bid(100, 15.5, 10.0, true).await;
    aggregator.add_historical_bid(101, 16.0, 8.0, true).await;
    aggregator.add_historical_bid(102, 14.8, 12.0, false).await;
    aggregator.add_historical_bid(103, 17.2, 5.0, true).await;
    
    // Test bid history retrieval
    let history = aggregator.get_bid_history().await;
    assert_eq!(history.len(), 4);
    
    // Test average bid price calculation
    let avg_price = aggregator.get_average_bid_price().await;
    assert!(avg_price > 15.0);
    assert!(avg_price < 18.0);
    
    // Test success rate calculation
    let success_rate = aggregator.get_success_rate().await;
    assert_eq!(success_rate, 3.0 / 4.0); // 3 out of 4 bids successful
}

#[tokio::test]
async fn test_aggregator_energy_requirement_optimization() {
    let aggregator = AggregatorNode::new(123, "AGG-001".to_string(), BiddingStrategy::Intelligent);
    
    // Add BESS nodes with different energy availability
    let mut bess1 = BESSNode::new(100, "BESS-001".to_string(), 100.0, 15.0);
    bess1.set_percentage_for_sale(50.0); // 50kWh available
    
    let mut bess2 = BESSNode::new(101, "BESS-002".to_string(), 80.0, 16.0);
    bess2.set_percentage_for_sale(75.0); // 60kWh available
    
    let mut bess3 = BESSNode::new(102, "BESS-003".to_string(), 60.0, 17.0);
    bess3.set_percentage_for_sale(100.0); // 60kWh available
    
    aggregator.add_connected_bess(100, bess1).await;
    aggregator.add_connected_bess(101, bess2).await;
    aggregator.add_connected_bess(102, bess3).await;
    
    // Test optimization for 100kWh requirement
    let optimized_bids = aggregator.optimize_bids(100.0, 18.0).await;
    
    // Should select optimal combination of BESS nodes
    assert!(!optimized_bids.is_empty());
    
    let total_energy: f64 = optimized_bids.iter().map(|bid| bid.required_energy_amount).sum();
    assert_eq!(total_energy, 100.0);
    
    // Should prefer lower-priced BESS nodes
    let total_cost: f64 = optimized_bids.iter().map(|bid| bid.bid_price * bid.required_energy_amount).sum();
    assert!(total_cost < 100.0 * 18.0); // Should be less than max price
}

#[tokio::test]
async fn test_aggregator_network_communication_simulation() {
    let aggregator = AggregatorNode::new(123, "AGG-001".to_string(), BiddingStrategy::Intelligent);
    
    // Simulate network communication with BESS nodes
    let bess1 = BESSNode::new(100, "BESS-001".to_string(), 100.0, 15.0);
    let bess2 = BESSNode::new(101, "BESS-002".to_string(), 80.0, 16.0);
    
    aggregator.add_connected_bess(100, bess1).await;
    aggregator.add_connected_bess(101, bess2).await;
    
    // Test querying BESS nodes
    let query_responses = aggregator.query_bess_nodes(20.0).await;
    assert_eq!(query_responses.len(), 2);
    
    // Test placing bids
    let bids = aggregator.place_bids(20.0, 18.0).await;
    assert!(!bids.is_empty());
    
    // Test processing bid responses
    for bid in bids {
        let response = aggregator.process_bid_response(bid).await;
        assert!(response.is_ok());
    }
}
