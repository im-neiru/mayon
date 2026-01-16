#[derive(Clone, Copy, Debug, thiserror::Error)]
#[error("Buffer overflow error")]
pub struct BufferOverflowError;
