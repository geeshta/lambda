//! Module responsible for dealing with free and fresh variables
pub mod generator;
pub mod varmap;
pub mod varset;

pub use self::generator::VarGen;
pub use self::varmap::VarMap;
pub use self::varset::VarSet;
