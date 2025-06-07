#[derive(Debug, Clone)]
pub enum ErrorCache {
    KeyNotFound,
    CapacityExceeded,
    SerializationError(String),
    LockError(String),
}

impl std::fmt::Display for ErrorCache {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorCache::KeyNotFound => write!(f, "Key not found in cache"),
            ErrorCache::CapacityExceeded => write!(f, "Cache capacity exceeded"),
            ErrorCache::SerializationError(msg) => write!(f, "Serialize error: {}", msg),
            ErrorCache::LockError(msg) => write!(f, "Lock error: {}", msg),
        }
    }
}

impl std::error::Error for CacheError {}