//! This module handles the beta reduction of terms
pub mod memo;
pub mod reduction;

pub use self::memo::Memo;
pub use self::reduction::BetaReduction;
