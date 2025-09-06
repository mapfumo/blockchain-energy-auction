use energy_trading::bess_node::BESSNode;
use energy_trading::bess_tcp_server::BESSTCPServer;
use energy_trading::etp_message::ETPMessage;
use energy_trading::network::unicast_connection::UnicastConnection;
use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use tokio::time::{timeout, Duration};

/// Test BESS TCP Server functionality
/// 
/// These tests follow TDD approach - write failing tests first,
/// then implement the functionality to make them pass.

#[tokio::test]
async fn test_bess_tcp_server_startup() {
    // Test that BESS TCP server can start and bind to a port
    let bess = BESSNode::new(123, "BESS-001".to_string(), 100.0, 15.0);
    let mut server = BESSTCPServer::new(bess, "127.0.0.1:0".parse().unwrap()).await.unwrap();
    
    assert!(!server.is_running().await);
    assert!(server.local_addr().is_ok());
}

#[tokio::test]
async fn test_bess_tcp_server_handles_multiple_connections() {
    // Test that BESS server can handle multiple concurrent connections
    let bess = BESSNode::new(123, "BESS-001".to_string(), 100.0, 15.0);
    let mut server = BESSTCPServer::new(bess, "127.0.0.1:0".parse().unwrap()).await.unwrap();
    let server_addr = server.local_addr().unwrap();
    
    // Start the server
    let server_handle = tokio::spawn(async move {
        server.start().await.unwrap();
    });
    
    // Give server time to start
    tokio::time::sleep(Duration::from_millis(100)).await;
    
    // Connect multiple clients
    let client1 = TcpStream::connect(server_addr).await.unwrap();
    let client2 = TcpStream::connect(server_addr).await.unwrap();
    let client3 = TcpStream::connect(server_addr).await.unwrap();
    
    // All connections should succeed
    assert!(client1.peer_addr().is_ok());
    assert!(client2.peer_addr().is_ok());
    assert!(client3.peer_addr().is_ok());
    
    server_handle.abort();
}

#[tokio::test]
async fn test_bess_tcp_server_processes_query_messages() {
    // Test that BESS server processes query messages and responds
    let bess = BESSNode::new(123, "BESS-001".to_string(), 100.0, 15.0);
    let mut server = BESSTCPServer::new(bess, "127.0.0.1:0".parse().unwrap()).await.unwrap();
    let server_addr = server.local_addr().unwrap();
    
    // Start the server
    let server_handle = tokio::spawn(async move {
        server.start().await.unwrap();
    });
    
    // Give server time to start
    tokio::time::sleep(Duration::from_millis(100)).await;
    
    // Connect and send query message
    let client_stream = TcpStream::connect(server_addr).await.unwrap();
    let mut client_connection = UnicastConnection::new(client_stream);
    
    let query = ETPMessage::new_query(456, 789); // aggregator_id=789
    client_connection.send_message(query).await.unwrap();
    
    // Should receive query response within 500ms (timing constraint)
    let response = timeout(Duration::from_millis(500), client_connection.receive_message()).await;
    assert!(response.is_ok());
    
    let response_message = response.unwrap().unwrap();
    assert_eq!(response_message.message_type, 2); // QueryResponse
    assert_eq!(response_message.device_id, 123); // BESS device ID
    
    server_handle.abort();
}

#[tokio::test]
async fn test_bess_tcp_server_processes_bid_messages() {
    // Test that BESS server processes bid messages and responds
    let bess = BESSNode::new(123, "BESS-001".to_string(), 100.0, 15.0);
    let mut server = BESSTCPServer::new(bess, "127.0.0.1:0".parse().unwrap()).await.unwrap();
    let server_addr = server.local_addr().unwrap();
    
    // Start the server
    let server_handle = tokio::spawn(async move {
        server.start().await.unwrap();
    });
    
    // Give server time to start
    tokio::time::sleep(Duration::from_millis(100)).await;
    
    // Connect and send bid message
    let client_stream = TcpStream::connect(server_addr).await.unwrap();
    let mut client_connection = UnicastConnection::new(client_stream);
    
    let bid = ETPMessage::new_bid(789, 18.0, 10.0); // bid_price=18.0, energy=10.0
    client_connection.send_message(bid).await.unwrap();
    
    // Should receive bid response within 500ms (timing constraint)
    let response = timeout(Duration::from_millis(500), client_connection.receive_message()).await;
    assert!(response.is_ok());
    
    let response_message = response.unwrap().unwrap();
    assert!(response_message.message_type == 4 || response_message.message_type == 6); // Accept or Reject
    assert_eq!(response_message.device_id, 123); // BESS device ID
    
    server_handle.abort();
}

#[tokio::test]
async fn test_bess_tcp_server_timing_constraints() {
    // Test that BESS server respects timing constraints for different message types
    let bess = BESSNode::new(123, "BESS-001".to_string(), 100.0, 15.0);
    let mut server = BESSTCPServer::new(bess, "127.0.0.1:0".parse().unwrap()).await.unwrap();
    let server_addr = server.local_addr().unwrap();
    
    // Start the server
    let server_handle = tokio::spawn(async move {
        server.start().await.unwrap();
    });
    
    // Give server time to start
    tokio::time::sleep(Duration::from_millis(100)).await;
    
    // Test DeviceFailure message (200ms constraint) - no response expected
    let client_stream = TcpStream::connect(server_addr).await.unwrap();
    let mut client_connection = UnicastConnection::new(client_stream);
    
    let device_failure = ETPMessage::new_device_failure(789, 123, 1);
    client_connection.send_message(device_failure).await.unwrap();
    
    // DeviceFailure messages don't get responses, just test that server processes them quickly
    // by sending a query message after and measuring response time
    let query = ETPMessage::new_query(456, 789);
    client_connection.send_message(query).await.unwrap();
    
    let start = std::time::Instant::now();
    let response = timeout(Duration::from_millis(300), client_connection.receive_message()).await;
    let elapsed = start.elapsed();
    
    assert!(response.is_ok());
    assert!(elapsed.as_millis() <= 500, "Query response took {}ms, should be ≤500ms", elapsed.as_millis());
    
    server_handle.abort();
}

#[tokio::test]
async fn test_bess_tcp_server_graceful_shutdown() {
    // Test that BESS server can shut down gracefully
    let bess = BESSNode::new(123, "BESS-001".to_string(), 100.0, 15.0);
    let server = BESSTCPServer::new(bess, "127.0.0.1:0".parse().unwrap()).await.unwrap();
    
    // Test that server is not running initially
    assert!(!server.is_running().await);
    
    // Test shutdown on non-running server
    server.shutdown().await.unwrap();
    
    // Server should still not be running
    assert!(!server.is_running().await);
}

#[tokio::test]
async fn test_bess_tcp_server_error_handling() {
    // Test that BESS server handles malformed messages gracefully
    let bess = BESSNode::new(123, "BESS-001".to_string(), 100.0, 15.0);
    let mut server = BESSTCPServer::new(bess, "127.0.0.1:0".parse().unwrap()).await.unwrap();
    let server_addr = server.local_addr().unwrap();
    
    // Start the server
    let server_handle = tokio::spawn(async move {
        server.start().await.unwrap();
    });
    
    // Give server time to start
    tokio::time::sleep(Duration::from_millis(100)).await;
    
    // Connect and send malformed data
    let mut client_stream = TcpStream::connect(server_addr).await.unwrap();
    client_stream.write_all(b"invalid data").await.unwrap();
    
    // Server should still be running and handle the error gracefully
    tokio::time::sleep(Duration::from_millis(100)).await;
    
    server_handle.abort();
}

