pub mod base_stats;
pub mod egg_moves;

#[cfg(not(feature = "legacy"))]
pub mod evos_attacks;

#[cfg(feature = "legacy")]
pub mod evos_attacks_legacy;

#[cfg(feature = "legacy")]
pub use evos_attacks_legacy as evos_attacks;
