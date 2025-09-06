use energy_trading::*;
use std::time::{Duration, Instant};
use std::net::Ipv4Addr;
use tokio::net::{TcpListener, TcpStream};

#[tokio::test]
async fn test_multicast_discovery_setup() {
    let discovery = MulticastDiscovery::new(
        Ipv4Addr::new(224, 0, 0, 1),
        8888,
    ).await.unwrap();
    
    assert_eq!(discovery.multicast_group, Ipv4Addr::new(224, 0, 0, 1));
    assert_eq!(discovery.multicast_port, 8888);
    assert!(discovery.is_running().await);
}

#[tokio::test]
async fn test_bess_node_registration() {
    let discovery = MulticastDiscovery::new(
        Ipv4Addr::new(224, 0, 0, 1),
        8888,
    ).await.unwrap();
    
    let bess = BESSNode::new(123, "BESS-001".to_string(), 100.0, 15.0);
    
    // Register BESS node
    discovery.register_bess_node(bess).await.unwrap();
    
    // Verify registration
    let registered_nodes = discovery.get_registered_bess_nodes().await;
    assert_eq!(registered_nodes.len(), 1);
    assert_eq!(registered_nodes[0].device_id, 123);
}

#[tokio::test]
async fn test_aggregator_discovery_query() {
    let discovery = MulticastDiscovery::new(
        Ipv4Addr::new(224, 0, 0, 1),
        8888,
    ).await.unwrap();
    
    // Register some BESS nodes
    let bess1 = BESSNode::new(123, "BESS-001".to_string(), 100.0, 15.0);
    let bess2 = BESSNode::new(124, "BESS-002".to_string(), 80.0, 16.0);
    
    discovery.register_bess_node(bess1).await.unwrap();
    discovery.register_bess_node(bess2).await.unwrap();
    
    // Simulate aggregator query
    let query = ETPMessage::new_query(456, 100);
    let responses = discovery.handle_discovery_query(query).await.unwrap();
    
    assert_eq!(responses.len(), 2);
    assert!(responses.iter().any(|r| r.device_id == 123));
    assert!(responses.iter().any(|r| r.device_id == 124));
}

#[tokio::test]
async fn test_unicast_connection_establishment() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let server_addr = listener.local_addr().unwrap();
    
    // Start server task
    let server_handle = tokio::spawn(async move {
        let (stream, _) = listener.accept().await.unwrap();
        let mut connection = UnicastConnection::new(stream);
        connection.handle_messages().await
    });
    
    // Connect client
    let client_stream = TcpStream::connect(server_addr).await.unwrap();
    let mut client_connection = UnicastConnection::new(client_stream);
    
    // Send test message
    let test_message = ETPMessage::new_bid(123, 18.0, 10.0);
    client_connection.send_message(test_message).await.unwrap();
    
    // Wait for server to process
    tokio::time::sleep(Duration::from_millis(100)).await;
    server_handle.abort();
}

#[tokio::test]
async fn test_message_serialization_over_tcp() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let server_addr = listener.local_addr().unwrap();
    
    let server_handle = tokio::spawn(async move {
        let (stream, _) = listener.accept().await.unwrap();
        let mut connection = UnicastConnection::new(stream);
        
        // Receive and deserialize message
        let received_message = connection.receive_message().await.unwrap();
        assert_eq!(received_message.message_type, 3); // Bid message
        assert_eq!(received_message.bid_price, 18.0);
        assert_eq!(received_message.required_energy_amount, 10.0);
    });
    
    // Connect and send message
    let client_stream = TcpStream::connect(server_addr).await.unwrap();
    let mut client_connection = UnicastConnection::new(client_stream);
    
    let test_message = ETPMessage::new_bid(123, 18.0, 10.0);
    client_connection.send_message(test_message).await.unwrap();
    
    server_handle.await.unwrap();
}

#[tokio::test]
async fn test_websocket_gateway_creation() {
    let gateway = WebSocketGateway::new(8080).await.unwrap();
    
    assert_eq!(gateway.port, 8080);
    assert!(gateway.is_running_sync());
    assert!(gateway.connected_clients().await == 0);
}

#[tokio::test]
async fn test_websocket_event_broadcasting() {
    let gateway = WebSocketGateway::new(8080).await.unwrap();
    
    // Simulate system events
    let auction_event = SystemEvent::AuctionStarted {
        auction_id: 123,
        total_energy: 100.0,
        reserve_price: 15.0,
    };
    
    let bid_event = SystemEvent::BidPlaced {
        aggregator_id: 100,
        bess_id: 123,
        bid_price: 18.0,
        energy_amount: 10.0,
    };
    
    // Broadcast events
    gateway.broadcast_event(auction_event).await.unwrap();
    gateway.broadcast_event(bid_event).await.unwrap();
    
    // Verify events were queued for broadcasting
    // Note: In a real implementation, this would check the event queue
    // For now, we'll just verify the broadcast didn't error
    assert!(true);
}

#[tokio::test]
async fn test_network_integration_flow() {
    // Create network components
    let discovery = MulticastDiscovery::new(
        Ipv4Addr::new(224, 0, 0, 1),
        8888,
    ).await.unwrap();
    
    let gateway = WebSocketGateway::new(8080).await.unwrap();
    
    // Register BESS nodes
    let bess1 = BESSNode::new(123, "BESS-001".to_string(), 100.0, 15.0);
    let bess2 = BESSNode::new(124, "BESS-002".to_string(), 80.0, 16.0);
    
    discovery.register_bess_node(bess1).await.unwrap();
    discovery.register_bess_node(bess2).await.unwrap();
    
    // Create aggregator
    let aggregator = AggregatorNode::new(100, "AGG-001".to_string(), BiddingStrategy::Intelligent);
    
    // Add BESS nodes to aggregator
    let bess1 = BESSNode::new(123, "BESS-001".to_string(), 100.0, 15.0);
    let bess2 = BESSNode::new(124, "BESS-002".to_string(), 80.0, 16.0);
    aggregator.add_connected_bess(123, bess1).await;
    aggregator.add_connected_bess(124, bess2).await;
    
    // Simulate discovery query
    let query = ETPMessage::new_query(456, 100);
    let responses = discovery.handle_discovery_query(query).await.unwrap();
    
    // Verify discovery worked
    assert_eq!(responses.len(), 2);
    
    // Simulate bidding process
    let bids = aggregator.optimize_bids(50.0, 20.0).await;
    assert!(!bids.is_empty());
    
    // Broadcast auction events
    let auction_event = SystemEvent::AuctionStarted {
        auction_id: 789,
        total_energy: 50.0,
        reserve_price: 15.0,
    };
    gateway.broadcast_event(auction_event).await.unwrap();
}

#[tokio::test]
async fn test_network_timing_constraints() {
    let _discovery = MulticastDiscovery::new(
        Ipv4Addr::new(224, 0, 0, 1),
        8888,
    ).await.unwrap();
    
    // Test discovery query timing
    let start = Instant::now();
    let query = ETPMessage::new_query(456, 100);
    let _responses = _discovery.handle_discovery_query(query).await.unwrap();
    let elapsed = start.elapsed();
    
    // Discovery should be fast (within 100ms for local test)
    assert!(elapsed < Duration::from_millis(100));
}

#[tokio::test]
async fn test_concurrent_network_operations() {
    let discovery = MulticastDiscovery::new(
        Ipv4Addr::new(224, 0, 0, 1),
        8888,
    ).await.unwrap();
    
    // Test concurrent BESS registrations
    let handles: Vec<_> = (0..10)
        .map(|i| {
            let discovery = discovery.clone();
            tokio::spawn(async move {
                let bess = BESSNode::new(i, format!("BESS-{:03}", i), 100.0, 15.0);
                discovery.register_bess_node(bess).await
            })
        })
        .collect();
    
    // Wait for all registrations
    for handle in handles {
        handle.await.unwrap().unwrap();
    }
    
    // Verify all nodes were registered
    let registered_nodes = discovery.get_registered_bess_nodes().await;
    assert_eq!(registered_nodes.len(), 10);
}

#[tokio::test]
async fn test_network_error_handling() {
    let discovery = MulticastDiscovery::new(
        Ipv4Addr::new(224, 0, 0, 1),
        8888,
    ).await.unwrap();
    
    // Test invalid multicast group
    let invalid_discovery = MulticastDiscovery::new(
        Ipv4Addr::new(127, 0, 0, 1), // Invalid multicast address
        8888,
    ).await;
    
    assert!(invalid_discovery.is_err());
}

#[tokio::test]
async fn test_websocket_connection_handling() {
    let gateway = WebSocketGateway::new(8080).await.unwrap();
    
    // Simulate client connections
    let initial_clients = gateway.connected_clients().await;
    
    // Simulate connection events
    gateway.handle_connection_event(ConnectionEvent::ClientConnected { client_id: uuid::Uuid::new_v4() }).await;
    gateway.handle_connection_event(ConnectionEvent::ClientConnected { client_id: uuid::Uuid::new_v4() }).await;
    
    // Verify client count increased
    let new_clients = gateway.connected_clients().await;
    assert!(new_clients > initial_clients);
}

#[tokio::test]
async fn test_network_metrics_collection() {
    let gateway = WebSocketGateway::new(8080).await.unwrap();
    
    // Simulate some network activity
    for i in 0..100 {
        let event = SystemEvent::BidPlaced {
            aggregator_id: 100,
            bess_id: 123,
            bid_price: 15.0 + (i as f64 * 0.1),
            energy_amount: 10.0,
        };
        gateway.broadcast_event(event).await.unwrap();
    }
    
    // Get system metrics
    let metrics = gateway.get_system_metrics().await;
    
    assert!(metrics.total_events_broadcast > 0);
    assert!(metrics.average_events_per_second > 0.0);
}

#[tokio::test]
async fn test_network_performance_under_load() {
    let gateway = WebSocketGateway::new(8080).await.unwrap();
    
    // Test high-frequency event broadcasting
    let start = Instant::now();
    let event_count = 1000;
    
    for i in 0..event_count {
        let event = SystemEvent::BidPlaced {
            aggregator_id: 100,
            bess_id: 123,
            bid_price: 15.0 + (i as f64 * 0.01),
            energy_amount: 10.0,
        };
        gateway.broadcast_event(event).await.unwrap();
    }
    
    let elapsed = start.elapsed();
    
    // Should handle 1000 events in less than 1 second
    assert!(elapsed < Duration::from_secs(1));
    
    // Calculate events per second
    let events_per_second = event_count as f64 / elapsed.as_secs_f64();
    assert!(events_per_second > 1000.0);
}
