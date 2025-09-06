use thiserror::Error;

#[derive(Error, Debug)]
pub enum SerializationError {
    #[error("Serialization failed: {0}")]
    Serialization(bincode::Error),
    
    #[error("Deserialization failed: {0}")]
    Deserialization(bincode::Error),
    
    #[error("Invalid message type: {0}")]
    InvalidMessageType(u8),
    
    #[error("Invalid message size: expected {expected}, got {actual}")]
    InvalidMessageSize { expected: usize, actual: usize },
}

#[derive(Error, Debug)]
pub enum ETPError {
    #[error("Serialization error: {0}")]
    Serialization(#[from] SerializationError),
    
    #[error("Message validation failed: {0}")]
    Validation(String),
    
    #[error("Timing constraint violated: {message_type} took {elapsed_ms}ms, max allowed {max_ms}ms")]
    TimingViolation {
        message_type: u8,
        elapsed_ms: u64,
        max_ms: u64,
    },
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Network error: {0}")]
    Network(String),
    
    #[error("BESS node error: {0}")]
    BESSNode(String),
    
    #[error("Insufficient energy available")]
    InsufficientEnergy,
    
    #[error("JSON serialization error: {0}")]
    JsonSerialization(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, ETPError>;
