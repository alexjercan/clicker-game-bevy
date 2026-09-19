pub mod autopilot;
pub mod predicate;

pub mod prelude {
    pub use crate::{
        autopilot::{
            AutopilotPlugin, StepBuilder, AUTOPILOT_ENV, DEADLINE_ENV, DEFAULT_DEADLINE_SECS,
        },
        predicate::{
            and, any_entity, elapsed, entity_count, not, or, resource_where, state_is, Predicate,
        },
    };
}
