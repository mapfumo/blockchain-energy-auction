use crate::error::{ETPError, Result, SerializationError};
use serde::{Deserialize, Serialize};
use std::time::Instant;

/// Energy Trading Protocol (ETP) Message
/// 
/// Based on the research paper "Communication requirements for enabling real-time energy trading
/// among distributed energy storage systems and aggregators" by Antony Mapfumo.
/// 
/// The message contains 14 fields as specified in the research paper:
/// - message_type: u8 (0-9)
/// - message_id: u64 (unique identifier)
/// - device_id: u64 (sender device ID)
/// - ttl: u8 (time to live)
/// - bid_price: f64 (cents/kWh)
/// - sale_price: f64 (final price)
/// - energy_total: f64 (kWh available)
/// - percentage_for_sale: f64 (% available for trading)
/// - required_energy_amount: f64 (kWh required)
/// - termination_code: u8 (reason for termination)
/// - remaining_battery_energy: f64 (kWh remaining)
/// - battery_health_status_code: u8 (health status)
/// - battery_voltage: f64 (voltage level)
/// - discharge_rate: f64 (discharge rate)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ETPMessage {
    pub message_type: u8,
    pub message_id: u64,
    pub device_id: u64,
    pub ttl: u8,
    pub bid_price: f64,
    pub sale_price: f64,
    pub energy_total: f64,
    pub percentage_for_sale: f64,
    pub required_energy_amount: f64,
    pub termination_code: u8,
    pub remaining_battery_energy: f64,
    pub battery_health_status_code: u8,
    pub battery_voltage: f64,
    pub discharge_rate: f64,
}

impl ETPMessage {
    /// Create a new ETP message with the specified type
    pub fn new_with_type(message_type: u8, message_id: u64, bid_price: f64, energy_amount: f64) -> Self {
        Self {
            message_type,
            message_id,
            device_id: 0, // Will be set by sender
            ttl: 5, // Default TTL
            bid_price,
            sale_price: 0.0,
            energy_total: 0.0,
            percentage_for_sale: 0.0,
            required_energy_amount: energy_amount,
            termination_code: 0,
            remaining_battery_energy: 0.0,
            battery_health_status_code: 0,
            battery_voltage: 0.0,
            discharge_rate: 0.0,
        }
    }

    /// Create a new Bid message (type 3)
    pub fn new_bid(message_id: u64, bid_price: f64, energy_amount: f64) -> Self {
        Self::new_with_type(3, message_id, bid_price, energy_amount)
    }

    /// Create a new Register message (type 0)
    pub fn new_register(message_id: u64, device_id: u64) -> Self {
        Self {
            message_type: 0,
            message_id,
            device_id,
            ttl: 5,
            bid_price: 0.0,
            sale_price: 0.0,
            energy_total: 0.0,
            percentage_for_sale: 0.0,
            required_energy_amount: 0.0,
            termination_code: 0,
            remaining_battery_energy: 0.0,
            battery_health_status_code: 0,
            battery_voltage: 0.0,
            discharge_rate: 0.0,
        }
    }

    /// Create a new Query message (type 1)
    pub fn new_query(message_id: u64, device_id: u64) -> Self {
        Self {
            message_type: 1,
            message_id,
            device_id,
            ttl: 5,
            bid_price: 0.0,
            sale_price: 0.0,
            energy_total: 0.0,
            percentage_for_sale: 0.0,
            required_energy_amount: 0.0,
            termination_code: 0,
            remaining_battery_energy: 0.0,
            battery_health_status_code: 0,
            battery_voltage: 0.0,
            discharge_rate: 0.0,
        }
    }

    /// Create a new QueryResponse message (type 2)
    pub fn new_query_response(message_id: u64, device_id: u64, energy_total: f64, percentage_for_sale: f64) -> Self {
        Self {
            message_type: 2,
            message_id,
            device_id,
            ttl: 5,
            bid_price: 0.0,
            sale_price: 0.0,
            energy_total,
            percentage_for_sale,
            required_energy_amount: 0.0,
            termination_code: 0,
            remaining_battery_energy: 0.0,
            battery_health_status_code: 0,
            battery_voltage: 0.0,
            discharge_rate: 0.0,
        }
    }

    /// Create a new BidAccept message (type 4)
    pub fn new_bid_accept(message_id: u64, device_id: u64, sale_price: f64, energy_amount: f64) -> Self {
        Self {
            message_type: 4,
            message_id,
            device_id,
            ttl: 5,
            bid_price: 0.0,
            sale_price,
            energy_total: 0.0,
            percentage_for_sale: 0.0,
            required_energy_amount: energy_amount,
            termination_code: 0,
            remaining_battery_energy: 0.0,
            battery_health_status_code: 0,
            battery_voltage: 0.0,
            discharge_rate: 0.0,
        }
    }

    /// Create a new BidConfirm message (type 5)
    pub fn new_bid_confirm(message_id: u64, device_id: u64, sale_price: f64, energy_amount: f64) -> Self {
        Self {
            message_type: 5,
            message_id,
            device_id,
            ttl: 5,
            bid_price: 0.0,
            sale_price,
            energy_total: 0.0,
            percentage_for_sale: 0.0,
            required_energy_amount: energy_amount,
            termination_code: 0,
            remaining_battery_energy: 0.0,
            battery_health_status_code: 0,
            battery_voltage: 0.0,
            discharge_rate: 0.0,
        }
    }

    /// Create a new BidReject message (type 6)
    pub fn new_bid_reject(message_id: u64, device_id: u64, termination_code: u8) -> Self {
        Self {
            message_type: 6,
            message_id,
            device_id,
            ttl: 5,
            bid_price: 0.0,
            sale_price: 0.0,
            energy_total: 0.0,
            percentage_for_sale: 0.0,
            required_energy_amount: 0.0,
            termination_code,
            remaining_battery_energy: 0.0,
            battery_health_status_code: 0,
            battery_voltage: 0.0,
            discharge_rate: 0.0,
        }
    }

    /// Create a new Terminate message (type 7)
    pub fn new_terminate(message_id: u64, device_id: u64, termination_code: u8) -> Self {
        Self {
            message_type: 7,
            message_id,
            device_id,
            ttl: 5,
            bid_price: 0.0,
            sale_price: 0.0,
            energy_total: 0.0,
            percentage_for_sale: 0.0,
            required_energy_amount: 0.0,
            termination_code,
            remaining_battery_energy: 0.0,
            battery_health_status_code: 0,
            battery_voltage: 0.0,
            discharge_rate: 0.0,
        }
    }

    /// Create a new DeviceFailure message (type 8)
    pub fn new_device_failure(message_id: u64, device_id: u64, termination_code: u8) -> Self {
        Self {
            message_type: 8,
            message_id,
            device_id,
            ttl: 5,
            bid_price: 0.0,
            sale_price: 0.0,
            energy_total: 0.0,
            percentage_for_sale: 0.0,
            required_energy_amount: 0.0,
            termination_code,
            remaining_battery_energy: 0.0,
            battery_health_status_code: 0,
            battery_voltage: 0.0,
            discharge_rate: 0.0,
        }
    }

    /// Create a new BESSStatus message (type 9)
    pub fn new_bess_status(message_id: u64, device_id: u64, remaining_energy: f64, health_code: u8, voltage: f64, discharge_rate: f64) -> Self {
        Self {
            message_type: 9,
            message_id,
            device_id,
            ttl: 5,
            bid_price: 0.0,
            sale_price: 0.0,
            energy_total: 0.0,
            percentage_for_sale: 0.0,
            required_energy_amount: 0.0,
            termination_code: 0,
            remaining_battery_energy: remaining_energy,
            battery_health_status_code: health_code,
            battery_voltage: voltage,
            discharge_rate,
        }
    }

    /// Serialize the message to binary format
    pub fn serialize(&self) -> Result<Vec<u8>> {
        bincode::serialize(self)
            .map_err(|e| ETPError::Serialization(SerializationError::Serialization(e)))
    }

    /// Deserialize the message from binary format
    pub fn deserialize(data: &[u8]) -> Result<Self> {
        bincode::deserialize(data)
            .map_err(|e| ETPError::Serialization(SerializationError::Deserialization(e)))
    }

    /// Validate the message structure
    pub fn validate(&self) -> Result<()> {
        // Validate message type (0-9)
        if self.message_type > 9 {
            return Err(ETPError::Validation(format!("Invalid message type: {}", self.message_type)));
        }

        // Validate TTL
        if self.ttl == 0 {
            return Err(ETPError::Validation("Message TTL is 0".to_string()));
        }

        // Validate energy values are non-negative
        if self.energy_total < 0.0 || self.percentage_for_sale < 0.0 || self.required_energy_amount < 0.0 {
            return Err(ETPError::Validation("Energy values cannot be negative".to_string()));
        }

        // Validate percentage is within 0-100 range
        if self.percentage_for_sale > 100.0 {
            return Err(ETPError::Validation("Percentage for sale cannot exceed 100%".to_string()));
        }

        Ok(())
    }

    /// Get the maximum allowed delay in milliseconds for this message type
    pub fn get_max_delay_ms(&self) -> u64 {
        match self.message_type {
            8 => 200,  // DeviceFailure - highest priority
            4 | 5 | 6 => 500,  // BidAccept, BidConfirm, BidReject - high priority
            2 => 500,  // QueryResponse - high priority
            9 => 2000, // BESSStatus - medium priority
            0 => 5000, // Register - low priority
            _ => 1000, // Default for other message types
        }
    }

    /// Get the priority level for this message type (lower number = higher priority)
    pub fn get_priority(&self) -> u8 {
        match self.message_type {
            8 => 0,  // DeviceFailure - very high priority
            4 | 5 | 6 => 5,  // BidAccept, BidConfirm, BidReject - high priority
            2 => 50, // QueryResponse - medium priority
            9 => 60, // BESSStatus - medium priority
            0 => 80, // Register - low priority
            _ => 50, // Default for other message types
        }
    }

    /// Decrement the TTL
    pub fn decrement_ttl(&mut self) {
        if self.ttl > 0 {
            self.ttl -= 1;
        }
    }

    /// Check if the message has expired (TTL = 0)
    pub fn is_expired(&self) -> bool {
        self.ttl == 0
    }

    /// Process message with timing constraint validation
    pub fn process_with_timing<F, R>(&self, processor: F) -> Result<R>
    where
        F: FnOnce() -> Result<R>,
    {
        let start = Instant::now();
        let result = processor()?;
        let elapsed = start.elapsed().as_millis() as u64;
        let max_delay = self.get_max_delay_ms();

        if elapsed > max_delay {
            return Err(ETPError::TimingViolation {
                message_type: self.message_type,
                elapsed_ms: elapsed,
                max_ms: max_delay,
            });
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_creation() {
        let msg = ETPMessage::new_bid(123, 15.5, 10.0);
        assert_eq!(msg.message_type, 3);
        assert_eq!(msg.message_id, 123);
        assert_eq!(msg.bid_price, 15.5);
        assert_eq!(msg.required_energy_amount, 10.0);
    }

    #[test]
    fn test_serialization_roundtrip() {
        let original = ETPMessage::new_bid(123, 15.5, 10.0);
        let serialized = original.serialize().unwrap();
        let deserialized = ETPMessage::deserialize(&serialized).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_validation() {
        let valid_msg = ETPMessage::new_bid(123, 15.5, 10.0);
        assert!(valid_msg.validate().is_ok());

        let invalid_msg = ETPMessage::new_with_type(10, 123, 15.5, 10.0);
        assert!(invalid_msg.validate().is_err());
    }

    #[test]
    fn test_timing_constraints() {
        let device_failure = ETPMessage::new_device_failure(123, 100, 1);
        assert_eq!(device_failure.get_max_delay_ms(), 200);

        let bid_accept = ETPMessage::new_bid_accept(123, 100, 18.0, 5.0);
        assert_eq!(bid_accept.get_max_delay_ms(), 500);

        let bess_status = ETPMessage::new_bess_status(123, 100, 18.5, 1, 12.6, 2.5);
        assert_eq!(bess_status.get_max_delay_ms(), 2000);
    }

    #[test]
    fn test_ttl_handling() {
        let mut msg = ETPMessage::new_bid(123, 15.5, 10.0);
        assert_eq!(msg.ttl, 5);
        assert!(!msg.is_expired());

        msg.decrement_ttl();
        assert_eq!(msg.ttl, 4);

        for _ in 0..5 {
            msg.decrement_ttl();
        }
        assert!(msg.is_expired());
    }
}
