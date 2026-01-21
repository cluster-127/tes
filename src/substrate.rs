//! Substrate - the main simulation runner
//!
//! Combines Space and Shapes into a tick-based simulation.

use crate::space::Space;
use crate::shape::Shape;

/// The TES substrate - combines space and shapes.
pub struct Substrate {
    /// The topographic space
    space: Space,
    /// All shapes in the substrate
    shapes: Vec<Shape>,
    /// Current tick count
    tick_count: u64,
    /// Next shape ID
    next_id: u64,
}

impl Substrate {
    /// Create a new substrate.
    pub fn new(width: usize, height: usize, decay_rate: u32, threshold: u32) -> Self {
        Self {
            space: Space::new(width, height, decay_rate, threshold),
            shapes: Vec::new(),
            tick_count: 0,
            next_id: 1,
        }
    }

    /// Spawn a new shape at position.
    ///
    /// Returns the shape ID, or None if position is not habitable.
    pub fn spawn(
        &mut self,
        x: usize,
        y: usize,
        lifetime: u32,
        contribution: u32,
    ) -> Option<u64> {
        // A1: Check habitability
        if !self.space.is_habitable(x, y) {
            return None;
        }

        let id = self.next_id;
        self.next_id += 1;

        let mut shape = Shape::new(id, x, y, 100, lifetime, 1.0, contribution);
        shape.attach_payload(); // Assume payload on spawn

        self.shapes.push(shape);
        Some(id)
    }

    /// Run one tick of the simulation.
    ///
    /// This is the core loop:
    /// 1. Each alive shape with payload contributes trace
    /// 2. Each shape loses one tick of lifetime
    /// 3. Dead shapes are removed
    /// 4. Space decay is applied
    pub fn tick(&mut self) {
        self.tick_count += 1;

        // Phase 1: Contribution (A3)
        for shape in &self.shapes {
            if shape.can_produce_trace() {
                let (x, y) = shape.pos;
                self.space.contribute(x, y, shape.contribution);
            }
        }

        // Phase 2: Lifetime decay (A2)
        for shape in &mut self.shapes {
            shape.tick();
        }

        // Phase 3: Remove dead shapes (A5)
        // Ghost trace prevention: dead shapes can't contribute (A1 ∧ A3)
        self.shapes.retain(|s| s.is_alive());

        // Phase 4: Space decay (A4)
        self.space.tick();
    }

    /// Run multiple ticks.
    pub fn run(&mut self, ticks: u64) {
        for _ in 0..ticks {
            self.tick();
        }
    }

    /// Get current tick count.
    pub fn tick_count(&self) -> u64 {
        self.tick_count
    }

    /// Get number of living shapes.
    pub fn shape_count(&self) -> usize {
        self.shapes.len()
    }

    /// Get reference to space.
    pub fn space(&self) -> &Space {
        &self.space
    }

    /// Get density map as flat vector (for visualization).
    pub fn density_map(&self) -> Vec<u32> {
        let (w, h) = self.space.dimensions();
        let mut map = Vec::with_capacity(w * h);
        for y in 0..h {
            for x in 0..w {
                map.push(self.space.density(x, y));
            }
        }
        map
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid::Regime;

    #[test]
    fn test_substrate_simulation() {
        let mut sub = Substrate::new(10, 10, 5, 100);

        // Spawn a shape
        let id = sub.spawn(5, 5, 20, 10);
        assert!(id.is_some());
        assert_eq!(sub.shape_count(), 1);

        // Run simulation
        sub.run(10);

        // Check trace accumulation
        let density = sub.space().density(5, 5);
        // 10 contributions of 10, minus 10 decays of 5 = 50
        assert_eq!(density, 50);
    }

    #[test]
    fn test_habitability_blocking() {
        let mut sub = Substrate::new(10, 10, 0, 50); // No decay, threshold 50

        // First spawn succeeds
        assert!(sub.spawn(3, 3, 100, 60).is_some());

        // After one tick, density = 60 > threshold 50
        sub.tick();

        // Second spawn at same position should fail
        assert!(sub.spawn(3, 3, 100, 10).is_none());
    }

    #[test]
    fn test_death_removes_shapes() {
        let mut sub = Substrate::new(10, 10, 0, 1000);

        // Spawn with lifetime 5
        sub.spawn(0, 0, 5, 1);
        assert_eq!(sub.shape_count(), 1);

        // After 5 ticks, shape should be dead
        sub.run(5);
        assert_eq!(sub.shape_count(), 0);
    }

    #[test]
    fn test_ghost_trace_prevention() {
        let mut sub = Substrate::new(10, 10, 0, 1000);

        // Spawn with lifetime 3
        sub.spawn(2, 2, 3, 10);

        // Run 3 ticks (shape dies)
        sub.run(3);

        // Density should be 30 (3 contributions)
        let density = sub.space().density(2, 2);
        assert_eq!(density, 30);

        // Run more ticks - no more contribution (ghost trace prevented)
        sub.run(3);
        assert_eq!(sub.space().density(2, 2), 30); // Same, no new contribution
    }

    #[test]
    fn test_regime_evolution() {
        let mut sub = Substrate::new(10, 10, 0, 100);

        // Initially Gas
        assert_eq!(sub.space().regime(5, 5), Regime::Gas);

        // Spawn and accumulate
        sub.spawn(5, 5, 100, 20);
        sub.run(3); // density = 60 > liquid (50)

        assert_eq!(sub.space().regime(5, 5), Regime::Liquid);

        sub.run(3); // density = 120 > solid (100)
        assert_eq!(sub.space().regime(5, 5), Regime::Solid);
    }
}
