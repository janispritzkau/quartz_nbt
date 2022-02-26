#[cfg(not(feature = "preserve_order"))]
pub use std::collections::HashMap as Map;

#[cfg(feature = "preserve_order")]
pub use indexmap::IndexMap as Map;
