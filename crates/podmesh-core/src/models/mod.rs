mod container;
mod event;
mod image;
mod network;
mod node;
mod pod;
mod volume;

pub use container::*;
pub use event::*;
pub use image::*;
pub use network::*;
pub use node::*;
pub use pod::*;
pub use volume::*;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ApiResponse<T: Serialize> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn ok(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn err(message: impl Into<String>) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message.into()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PaginatedResponse<T: Serialize> {
    pub items: Vec<T>,
    pub total: u64,
    pub page: u64,
    pub per_page: u64,
}
