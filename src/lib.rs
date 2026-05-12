//! Utahnetes — swarm core blueprint: thermodynamic load signals + libp2p fabric + WASM execution.

pub mod compute;
pub mod swarm;

pub use compute::mitosis::WasmCell;
pub use swarm::network::{UtahnetesBehavior, UtahnetesSwarmEvent};
pub use swarm::thermodynamics::NodeThermodynamics;
