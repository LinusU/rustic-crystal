#[cfg(not(feature = "legacy"))]
pub mod parties;
#[cfg(feature = "legacy")]
pub mod parties_legacy;
#[cfg(feature = "legacy")]
pub use parties_legacy as parties;
