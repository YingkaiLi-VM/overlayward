use thiserror::Error;

#[derive(Debug, Error)]
pub enum OverlaywardError {
    #[error("[{code}] {reason}")]
    GuardianDenied { code: String, reason: String },
    #[error("approval required: {approval_id} (timeout {timeout})")]
    ApprovalRequired { approval_id: String, timeout: String },
    #[error("not found: {resource}")]
    NotFound { resource: String },
    #[error("invalid argument: {message}")]
    InvalidArgument { message: String },
    #[error("resource exhausted: {message}")]
    ResourceExhausted { message: String },
    #[error("permission denied: {message}")]
    PermissionDenied { message: String },
    #[error("internal: {message}")]
    Internal { message: String },
    #[error("connection: {0}")]
    Connection(#[from] tonic::transport::Error),
    #[error("rpc: {0}")]
    Rpc(tonic::Status),
}

impl From<tonic::Status> for OverlaywardError {
    #[inline]
    fn from(s: tonic::Status) -> Self {
        match s.code() {
            tonic::Code::NotFound => Self::NotFound { resource: s.message().into() },
            tonic::Code::InvalidArgument => Self::InvalidArgument { message: s.message().into() },
            tonic::Code::PermissionDenied => Self::PermissionDenied { message: s.message().into() },
            tonic::Code::ResourceExhausted => Self::ResourceExhausted { message: s.message().into() },
            tonic::Code::FailedPrecondition => Self::InvalidArgument { message: s.message().into() },
            _ => Self::Rpc(s),
        }
    }
}
