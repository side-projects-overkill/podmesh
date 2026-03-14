use thiserror::Error;

#[derive(Debug, Error)]
pub enum PodMeshError {
    #[error("Podman API error: {message}")]
    PodmanApi {
        status: u16,
        message: String,
    },

    #[error("Node unreachable: {node_id}")]
    NodeUnreachable { node_id: String },

    #[error("Resource not found: {kind} {id}")]
    NotFound { kind: String, id: String },

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Authentication error: {0}")]
    Auth(String),

    #[error("Permission denied: {0}")]
    Forbidden(String),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

impl PodMeshError {
    pub fn status_code(&self) -> u16 {
        match self {
            Self::NotFound { .. } => 404,
            Self::Validation(_) => 400,
            Self::Auth(_) => 401,
            Self::Forbidden(_) => 403,
            Self::NodeUnreachable { .. } => 502,
            Self::PodmanApi { status, .. } => *status,
            _ => 500,
        }
    }
}

pub type Result<T> = std::result::Result<T, PodMeshError>;
