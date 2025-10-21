//! Cellular automata rules

mod rule;
mod conway;
mod highlife;
mod seeds;
mod daynight;

pub use rule::Rule;
pub use conway::Conway;
pub use highlife::HighLife;
pub use seeds::Seeds;
pub use daynight::DayAndNight;
