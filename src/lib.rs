/// Lib
// region:      --- Modules
pub mod args;
pub mod connectorx_df;
pub mod consts;
pub mod sqlx_trans;
pub mod vecstruct_df;

pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>; // For early dev

// endregion:   --- Modules
