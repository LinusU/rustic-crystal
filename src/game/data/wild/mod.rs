pub mod flee_mons;

#[cfg(not(feature = "legacy"))]
pub mod johto_grass;
#[cfg(feature = "legacy")]
pub mod johto_grass_legacy;
#[cfg(feature = "legacy")]
pub use johto_grass_legacy as johto_grass;

#[cfg(not(feature = "legacy"))]
pub mod johto_water;
#[cfg(feature = "legacy")]
pub mod johto_water_legacy;
#[cfg(feature = "legacy")]
pub use johto_water_legacy as johto_water;

#[cfg(not(feature = "legacy"))]
pub mod kanto_grass;
#[cfg(feature = "legacy")]
pub mod kanto_grass_legacy;
#[cfg(feature = "legacy")]
pub use kanto_grass_legacy as kanto_grass;

#[cfg(not(feature = "legacy"))]
pub mod kanto_water;
#[cfg(feature = "legacy")]
pub mod kanto_water_legacy;
#[cfg(feature = "legacy")]
pub use kanto_water_legacy as kanto_water;

pub mod probabilities;
pub mod swarm_grass;
pub mod swarm_water;
