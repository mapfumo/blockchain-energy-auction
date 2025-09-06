use energy_trading::*;
use std::time::{Duration, Instant};

#[tokio::test]
async fn test_bess_node_creation_and_properties() {
    let bess = BESSNode::new(123, "BESS-001".to_string(), 100.0, 15.0);
    
    assert_eq!(bess.device_id, 123);
    assert_eq!(bess.device_name, "BESS-001");
    assert_eq!(bess.total_energy_capacity, 100.0);
    assert_eq!(bess.current_energy_level, 80.0); // 80% of capacity
    assert_eq!(bess.reserve_price, 15.0);
    assert!(bess.is_online);
    assert!(bess.last_heartbeat.is_some());
}

#[tokio::test]
async fn test_available_energy_calculation() {
    let mut bess = BESSNode::new(123, "BESS-001".to_string(), 100.0, 15.0);
    
    // Test with 50% available for sale
    bess.set_percentage_for_sale(50.0);
    assert_eq!(bess.get_available_energy(), 40.0); // 80 * 0.5
    
    // Test with 25% available for sale
    bess.set_percentage_for_sale(25.0);
    assert_eq!(bess.get_available_energy(), 20.0); // 80 * 0.25
    
    // Test with 100% available for sale
    bess.set_percentage_for_sale(100.0);
    assert_eq!(bess.get_available_energy(), 80.0); // 80 * 1.0
}

#[tokio::test]
async fn test_can_provide_energy() {
    let mut bess = BESSNode::new(123, "BESS-001".to_string(), 100.0, 15.0);
    bess.set_percentage_for_sale(50.0); // 40kWh available
    
    // Should be able to provide 30kWh
    assert!(bess.can_provide_energy(30.0));
    
    // Should be able to provide exactly 40kWh
    assert!(bess.can_provide_energy(40.0));
    
    // Should NOT be able to provide 50kWh
    assert!(!bess.can_provide_energy(50.0));
    
    // Should NOT be able to provide negative energy
    assert!(!bess.can_provide_energy(-10.0));
}

#[tokio::test]
async fn test_bid_evaluation_accept() {
    let bess = BESSNode::new(123, "BESS-001".to_string(), 100.0, 15.0);
    
    // Test accepting a bid above reserve price
    let evaluation = bess.evaluate_bid(20.0, 10.0);
    
    match evaluation {
        BidEvaluation::Accept { sale_price, energy_amount } => {
            assert_eq!(sale_price, 20.0);
            assert_eq!(energy_amount, 10.0);
        }
        _ => panic!("Expected Accept evaluation"),
    }
}

#[tokio::test]
async fn test_bid_evaluation_reject_price_too_low() {
    let bess = BESSNode::new(123, "BESS-001".to_string(), 100.0, 15.0);
    
    // Test rejecting a bid below reserve price
    let evaluation = bess.evaluate_bid(10.0, 10.0);
    
    match evaluation {
        BidEvaluation::Reject { reason, code } => {
            assert_eq!(reason, "Bid price below reserve price");
            assert_eq!(code, 1);
        }
        _ => panic!("Expected Reject evaluation"),
    }
}

#[tokio::test]
async fn test_bid_evaluation_reject_insufficient_energy() {
    let mut bess = BESSNode::new(123, "BESS-001".to_string(), 100.0, 15.0);
    bess.set_percentage_for_sale(10.0); // Only 8kWh available
    
    // Test rejecting a bid for more energy than available
    let evaluation = bess.evaluate_bid(20.0, 50.0);
    
    match evaluation {
        BidEvaluation::Reject { reason, code } => {
            assert_eq!(reason, "Insufficient energy available");
            assert_eq!(code, 2);
        }
        _ => panic!("Expected Reject evaluation"),
    }
}

#[tokio::test]
async fn test_bid_evaluation_reject_offline() {
    let mut bess = BESSNode::new(123, "BESS-001".to_string(), 100.0, 15.0);
    bess.is_online = false;
    
    // Test rejecting a bid when offline
    let evaluation = bess.evaluate_bid(20.0, 10.0);
    
    match evaluation {
        BidEvaluation::Reject { reason, code } => {
            assert_eq!(reason, "BESS is offline");
            assert_eq!(code, 3);
        }
        _ => panic!("Expected Reject evaluation"),
    }
}

#[tokio::test]
async fn test_battery_status_update() {
    let mut bess = BESSNode::new(123, "BESS-001".to_string(), 100.0, 15.0);
    
    // Update battery status
    bess.update_battery_status(75.0, 12.4, 2);
    
    assert_eq!(bess.current_energy_level, 75.0);
    assert_eq!(bess.battery_voltage, 12.4);
    assert_eq!(bess.battery_health_status, 2);
    assert!(bess.last_heartbeat.is_some());
}

#[tokio::test]
async fn test_percentage_for_sale_clamping() {
    let mut bess = BESSNode::new(123, "BESS-001".to_string(), 100.0, 15.0);
    
    // Test clamping to 0-100 range
    bess.set_percentage_for_sale(-10.0);
    assert_eq!(bess.percentage_for_sale, 0.0);
    
    bess.set_percentage_for_sale(150.0);
    assert_eq!(bess.percentage_for_sale, 100.0);
    
    bess.set_percentage_for_sale(50.0);
    assert_eq!(bess.percentage_for_sale, 50.0);
}

#[tokio::test]
async fn test_generate_status_message() {
    let bess = BESSNode::new(123, "BESS-001".to_string(), 100.0, 15.0);
    let status_msg = bess.generate_status_message(456);
    
    assert_eq!(status_msg.message_type, 9); // BESSStatus
    assert_eq!(status_msg.message_id, 456);
    assert_eq!(status_msg.device_id, 123);
    assert_eq!(status_msg.remaining_battery_energy, 80.0);
    assert_eq!(status_msg.battery_health_status_code, 1);
    assert_eq!(status_msg.battery_voltage, 12.6);
    assert_eq!(status_msg.discharge_rate, 5.0);
}

#[tokio::test]
async fn test_generate_query_response() {
    let bess = BESSNode::new(123, "BESS-001".to_string(), 100.0, 15.0);
    let response_msg = bess.generate_query_response(789, 999);
    
    assert_eq!(response_msg.message_type, 2); // QueryResponse
    assert_eq!(response_msg.message_id, 789);
    assert_eq!(response_msg.device_id, 123);
    assert_eq!(response_msg.energy_total, 80.0);
    assert_eq!(response_msg.percentage_for_sale, 50.0);
}

#[tokio::test]
async fn test_bess_node_manager_creation() {
    let manager = BESSNodeManager::new(
        std::net::Ipv4Addr::new(224, 0, 0, 1),
        8888,
    );
    
    // Manager should be created successfully
    assert!(manager.nodes.read().await.is_empty());
}

#[tokio::test]
async fn test_bess_node_manager_add_node() {
    let manager = BESSNodeManager::new(
        std::net::Ipv4Addr::new(224, 0, 0, 1),
        8888,
    );
    
    let bess = BESSNode::new(123, "BESS-001".to_string(), 100.0, 15.0);
    
    // Add node
    manager.add_node(bess).await.unwrap();
    
    // Verify node was added
    let retrieved_node = manager.get_node(123).await;
    assert!(retrieved_node.is_some());
    
    let node = retrieved_node.unwrap();
    assert_eq!(node.device_id, 123);
    assert_eq!(node.device_name, "BESS-001");
}

#[tokio::test]
async fn test_bess_node_manager_get_nonexistent_node() {
    let manager = BESSNodeManager::new(
        std::net::Ipv4Addr::new(224, 0, 0, 1),
        8888,
    );
    
    // Try to get non-existent node
    let retrieved_node = manager.get_node(999).await;
    assert!(retrieved_node.is_none());
}

#[tokio::test]
async fn test_bess_node_manager_start() {
    let mut manager = BESSNodeManager::new(
        std::net::Ipv4Addr::new(224, 0, 0, 1),
        8888,
    );
    
    // Start manager on localhost
    let bind_addr = "127.0.0.1:0".parse().unwrap();
    manager.start(bind_addr).await.unwrap();
    
    // Manager should be started
    assert!(manager.listener.is_some());
}

#[tokio::test]
async fn test_bid_evaluation_edge_cases() {
    let bess = BESSNode::new(123, "BESS-001".to_string(), 100.0, 15.0);
    
    // Test exact reserve price (should accept)
    let evaluation = bess.evaluate_bid(15.0, 10.0);
    match evaluation {
        BidEvaluation::Accept { .. } => {}, // Should accept
        _ => panic!("Expected Accept for exact reserve price"),
    }
    
    // Test zero energy request (should accept if price is good)
    let evaluation = bess.evaluate_bid(20.0, 0.0);
    match evaluation {
        BidEvaluation::Accept { .. } => {}, // Should accept
        _ => panic!("Expected Accept for zero energy request"),
    }
    
    // Test negative energy request (should reject)
    let evaluation = bess.evaluate_bid(20.0, -10.0);
    match evaluation {
        BidEvaluation::Reject { reason, code } => {
            assert_eq!(reason, "Insufficient energy available");
            assert_eq!(code, 2);
        }
        _ => panic!("Expected Reject for negative energy request"),
    }
}

#[tokio::test]
async fn test_bess_node_performance() {
    let bess = BESSNode::new(123, "BESS-001".to_string(), 100.0, 15.0);
    
    // Test bid evaluation performance
    let start = Instant::now();
    for _ in 0..1000 {
        let _evaluation = bess.evaluate_bid(20.0, 10.0);
    }
    let elapsed = start.elapsed();
    
    // Should evaluate 1000 bids in less than 10ms
    assert!(elapsed < Duration::from_millis(10));
}

#[tokio::test]
async fn test_bess_node_concurrent_access() {
    let manager = BESSNodeManager::new(
        std::net::Ipv4Addr::new(224, 0, 0, 1),
        8888,
    );
    
    // Add multiple nodes concurrently
    let handles: Vec<_> = (0..10)
        .map(|i| {
            let manager = manager.clone();
            tokio::spawn(async move {
                let bess = BESSNode::new(i, format!("BESS-{:03}", i), 100.0, 15.0);
                manager.add_node(bess).await
            })
        })
        .collect();
    
    // Wait for all nodes to be added
    for handle in handles {
        handle.await.unwrap().unwrap();
    }
    
    // Verify all nodes were added
    let nodes = manager.nodes.read().await;
    assert_eq!(nodes.len(), 10);
    
    // Verify we can retrieve each node
    for i in 0..10 {
        assert!(nodes.contains_key(&i));
    }
}

#[tokio::test]
async fn test_bess_node_serialization() {
    let bess = BESSNode::new(123, "BESS-001".to_string(), 100.0, 15.0);
    
    // Test that BESS node can be serialized/deserialized
    let serialized = serde_json::to_string(&bess).unwrap();
    let deserialized: BESSNode = serde_json::from_str(&serialized).unwrap();
    
    assert_eq!(bess.device_id, deserialized.device_id);
    assert_eq!(bess.device_name, deserialized.device_name);
    assert_eq!(bess.total_energy_capacity, deserialized.total_energy_capacity);
    assert_eq!(bess.current_energy_level, deserialized.current_energy_level);
    assert_eq!(bess.reserve_price, deserialized.reserve_price);
}

#[tokio::test]
async fn test_bess_node_validation() {
    let bess = BESSNode::new(123, "BESS-001".to_string(), 100.0, 15.0);
    
    // Test that BESS node validates correctly
    assert!(bess.device_id > 0);
    assert!(!bess.device_name.is_empty());
    assert!(bess.total_energy_capacity > 0.0);
    assert!(bess.current_energy_level >= 0.0);
    assert!(bess.current_energy_level <= bess.total_energy_capacity);
    assert!(bess.reserve_price >= 0.0);
    assert!(bess.max_discharge_rate > 0.0);
    assert!(bess.battery_voltage > 0.0);
    assert!(bess.battery_health_status <= 3);
    assert!(bess.percentage_for_sale >= 0.0);
    assert!(bess.percentage_for_sale <= 100.0);
}
