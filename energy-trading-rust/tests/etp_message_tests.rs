use energy_trading::*;
use std::time::{Duration, Instant};

#[test]
fn test_etp_message_creation() {
    let msg = ETPMessage::new_bid(123, 15.5, 10.0);
    
    assert_eq!(msg.message_type, 3); // Bid message type
    assert_eq!(msg.message_id, 123);
    assert_eq!(msg.bid_price, 15.5);
    assert_eq!(msg.required_energy_amount, 10.0);
    assert_eq!(msg.device_id, 0); // Will be set by sender
    assert_eq!(msg.ttl, 5); // Default TTL
}

#[test]
fn test_etp_message_serialization_roundtrip() {
    let original = ETPMessage::new_bid(123, 15.5, 10.0);
    let serialized = original.serialize().unwrap();
    let deserialized = ETPMessage::deserialize(&serialized).unwrap();
    
    assert_eq!(original, deserialized);
}

#[test]
fn test_etp_message_all_fields_serialization() {
    let original = ETPMessage {
        message_type: 3,
        message_id: 12345,
        device_id: 100,
        ttl: 5,
        bid_price: 15.5,
        sale_price: 16.0,
        energy_total: 20.0,
        percentage_for_sale: 50.0,
        required_energy_amount: 10.0,
        termination_code: 0,
        remaining_battery_energy: 18.5,
        battery_health_status_code: 1,
        battery_voltage: 12.6,
        discharge_rate: 2.5,
    };
    
    let serialized = original.serialize().unwrap();
    let deserialized = ETPMessage::deserialize(&serialized).unwrap();
    
    assert_eq!(original, deserialized);
}

#[test]
fn test_message_type_validation() {
    // Valid message types (0-9)
    for msg_type in 0..=9 {
        let msg = ETPMessage::new_with_type(msg_type, 123, 15.5, 10.0);
        assert!(msg.validate().is_ok());
    }
    
    // Invalid message type
    let invalid_msg = ETPMessage::new_with_type(10, 123, 15.5, 10.0);
    assert!(invalid_msg.validate().is_err());
}

#[test]
fn test_all_message_types() {
    // Test all 10 message types from research paper
    let message_types = [
        (0, "Register"),
        (1, "Query"),
        (2, "QueryResponse"),
        (3, "Bid"),
        (4, "BidAccept"),
        (5, "BidConfirm"),
        (6, "BidReject"),
        (7, "Terminate"),
        (8, "DeviceFailure"),
        (9, "BESSStatus"),
    ];
    
    for (msg_type, _name) in message_types {
        let msg = ETPMessage::new_with_type(msg_type, 123, 15.5, 10.0);
        assert_eq!(msg.message_type, msg_type);
        assert!(msg.validate().is_ok());
    }
}

#[test]
fn test_timing_constraints() {
    let msg = ETPMessage::new_with_type(8, 123, 15.5, 10.0); // DeviceFailure
    let max_delay = msg.get_max_delay_ms();
    assert_eq!(max_delay, 200); // DeviceFailure has highest priority
    
    let bid_msg = ETPMessage::new_with_type(4, 123, 15.5, 10.0); // BidAccept
    let bid_max_delay = bid_msg.get_max_delay_ms();
    assert_eq!(bid_max_delay, 500); // Bid operations have 500ms limit
    
    let status_msg = ETPMessage::new_with_type(9, 123, 15.5, 10.0); // BESSStatus
    let status_max_delay = status_msg.get_max_delay_ms();
    assert_eq!(status_max_delay, 2000); // Status updates have 2000ms limit
}

#[test]
fn test_serialization_performance() {
    let msg = ETPMessage::new_bid(123, 15.5, 10.0);
    
    // Test serialization speed
    let start = Instant::now();
    for _ in 0..1000 {
        let _serialized = msg.serialize().unwrap();
    }
    let elapsed = start.elapsed();
    
    // Should serialize 1000 messages in less than 10ms (reasonable for debug build)
    assert!(elapsed < Duration::from_millis(10));
}

#[test]
fn test_deserialization_performance() {
    let msg = ETPMessage::new_bid(123, 15.5, 10.0);
    let serialized = msg.serialize().unwrap();
    
    // Test deserialization speed
    let start = Instant::now();
    for _ in 0..1000 {
        let _deserialized = ETPMessage::deserialize(&serialized).unwrap();
    }
    let elapsed = start.elapsed();
    
    // Should deserialize 1000 messages in less than 10ms (reasonable for debug build)
    assert!(elapsed < Duration::from_millis(10));
}

#[test]
fn test_message_size_consistency() {
    let msg = ETPMessage::new_bid(123, 15.5, 10.0);
    let serialized = msg.serialize().unwrap();
    
    // Message should be reasonably sized (not too large)
    assert!(serialized.len() < 200); // Should be well under 200 bytes
    
    // Message should have minimum expected size
    assert!(serialized.len() >= 50); // Should be at least 50 bytes for all fields
}

#[test]
fn test_error_handling() {
    // Test deserialization with invalid data
    let invalid_data = vec![0x42, 0x42, 0x42]; // Invalid data
    let result = ETPMessage::deserialize(&invalid_data);
    assert!(result.is_err());
    
    // Test deserialization with empty data
    let empty_data = vec![];
    let result = ETPMessage::deserialize(&empty_data);
    assert!(result.is_err());
}

#[test]
fn test_message_priority() {
    let device_failure = ETPMessage::new_with_type(8, 123, 15.5, 10.0);
    let bid_accept = ETPMessage::new_with_type(4, 123, 15.5, 10.0);
    let query_response = ETPMessage::new_with_type(2, 123, 15.5, 10.0);
    let bess_status = ETPMessage::new_with_type(9, 123, 15.5, 10.0);
    let register = ETPMessage::new_with_type(0, 123, 15.5, 10.0);
    
    // Test priority ordering (lower number = higher priority)
    assert!(device_failure.get_priority() < bid_accept.get_priority());
    assert!(bid_accept.get_priority() < query_response.get_priority());
    assert!(query_response.get_priority() < bess_status.get_priority());
    assert!(bess_status.get_priority() < register.get_priority());
}

#[test]
fn test_message_ttl_handling() {
    let mut msg = ETPMessage::new_bid(123, 15.5, 10.0);
    assert_eq!(msg.ttl, 5); // Default TTL
    
    // Test TTL decrement
    msg.decrement_ttl();
    assert_eq!(msg.ttl, 4);
    
    // Test TTL expiration
    for _ in 0..5 {
        msg.decrement_ttl();
    }
    assert!(msg.is_expired());
}

#[test]
fn test_message_creation_helpers() {
    // Test Register message
    let register = ETPMessage::new_register(123, 100);
    assert_eq!(register.message_type, 0);
    assert_eq!(register.device_id, 100);
    
    // Test Query message
    let query = ETPMessage::new_query(123, 200);
    assert_eq!(query.message_type, 1);
    assert_eq!(query.device_id, 200);
    
    // Test QueryResponse message
    let response = ETPMessage::new_query_response(123, 100, 20.0, 50.0);
    assert_eq!(response.message_type, 2);
    assert_eq!(response.energy_total, 20.0);
    assert_eq!(response.percentage_for_sale, 50.0);
    
    // Test BidAccept message
    let accept = ETPMessage::new_bid_accept(123, 100, 18.0, 5.0);
    assert_eq!(accept.message_type, 4);
    assert_eq!(accept.sale_price, 18.0);
    assert_eq!(accept.required_energy_amount, 5.0);
    
    // Test BidConfirm message
    let confirm = ETPMessage::new_bid_confirm(123, 200, 18.0, 5.0);
    assert_eq!(confirm.message_type, 5);
    assert_eq!(confirm.device_id, 200);
    
    // Test BidReject message
    let reject = ETPMessage::new_bid_reject(123, 100, 1); // Code 1 = price too low
    assert_eq!(reject.message_type, 6);
    assert_eq!(reject.termination_code, 1);
    
    // Test Terminate message
    let terminate = ETPMessage::new_terminate(123, 100, 0); // Code 0 = normal termination
    assert_eq!(terminate.message_type, 7);
    assert_eq!(terminate.termination_code, 0);
    
    // Test DeviceFailure message
    let failure = ETPMessage::new_device_failure(123, 100, 2); // Code 2 = battery failure
    assert_eq!(failure.message_type, 8);
    assert_eq!(failure.termination_code, 2);
    
    // Test BESSStatus message
    let status = ETPMessage::new_bess_status(123, 100, 18.5, 1, 12.6, 2.5);
    assert_eq!(status.message_type, 9);
    assert_eq!(status.remaining_battery_energy, 18.5);
    assert_eq!(status.battery_health_status_code, 1);
    assert_eq!(status.battery_voltage, 12.6);
    assert_eq!(status.discharge_rate, 2.5);
}
