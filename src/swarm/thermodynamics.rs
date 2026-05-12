// [MODULE: UTAHNETES-THERMODYNAMICS]
// Decentralized load signal: nodes expose "heat" instead of a central scheduler.

use std::sync::atomic::{AtomicUsize, Ordering};

pub struct NodeThermodynamics {
    cpu_temp: AtomicUsize,
    max_capacity: usize,
}

impl NodeThermodynamics {
    pub fn new(capacity: usize) -> Self {
        Self {
            cpu_temp: AtomicUsize::new(0),
            max_capacity: capacity,
        }
    }

    pub fn current_heat_index(&self) -> f32 {
        let current = self.cpu_temp.load(Ordering::Relaxed) as f32;
        let max = self.max_capacity as f32;
        if max <= 0.0 {
            return 0.0;
        }
        (current / max).clamp(0.0, 1.0)
    }

    pub fn can_accept_liquid(&self) -> bool {
        self.current_heat_index() < 0.85
    }

    pub fn add_load(&self, weight: usize) {
        self.cpu_temp.fetch_add(weight, Ordering::Relaxed);
    }

    pub fn remove_load(&self, weight: usize) {
        self.cpu_temp.fetch_sub(weight, Ordering::Relaxed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heat_index_clamps_and_acceptance_threshold() {
        let t = NodeThermodynamics::new(100);
        assert_eq!(t.current_heat_index(), 0.0);
        assert!(t.can_accept_liquid());
        t.add_load(84);
        assert!((t.current_heat_index() - 0.84).abs() < f32::EPSILON);
        assert!(t.can_accept_liquid());
        t.add_load(1);
        assert!(!t.can_accept_liquid());
    }

    #[test]
    fn zero_capacity_is_cold() {
        let t = NodeThermodynamics::new(0);
        assert_eq!(t.current_heat_index(), 0.0);
        assert!(t.can_accept_liquid());
    }
}
