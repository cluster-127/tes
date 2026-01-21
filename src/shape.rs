//! Shape - the bounded-memory carrier
//!
//! Shapes do NOT act. They only exist.
//! They do not affect space; space affects them.

/// A carrier that inhabitsa the topographic space.
///
/// # Formalization Mapping
///
/// ```text
/// Shape := (id, pos, β, λ, σ)
/// ```
///
/// - `id` → unique identifier
/// - `pos` → position in space
/// - `β` → memory budget (bytes)
/// - `λ` → lifetime (ticks remaining)
/// - `σ` → sensitivity (how much space affects this shape)
#[derive(Debug, Clone)]
pub struct Shape {
    /// Unique identifier
    pub id: u64,
    /// Position in space (x, y)
    pub pos: (usize, usize),
    /// Memory budget (abstract, not enforced)
    pub budget: u32,
    /// Remaining lifetime (ticks)
    pub lifetime: u32,
    /// Sensitivity to space density
    pub sensitivity: f32,
    /// Contribution amount per tick
    pub contribution: u32,
    /// Whether shape has payload (for A0 temporal observation)
    pub has_payload: bool,
}

impl Shape {
    /// Create a new shape.
    pub fn new(
        id: u64,
        x: usize,
        y: usize,
        budget: u32,
        lifetime: u32,
        sensitivity: f32,
        contribution: u32,
    ) -> Self {
        Self {
            id,
            pos: (x, y),
            budget,
            lifetime,
            sensitivity,
            contribution,
            has_payload: false,
        }
    }

    /// Attach payload to shape (activates temporal observation).
    ///
    /// From A0: `temporal_observation(s) ⟺ payload(s) ≠ ∅ ∧ ...`
    pub fn attach_payload(&mut self) {
        self.has_payload = true;
    }

    /// Detach payload.
    pub fn detach_payload(&mut self) {
        self.has_payload = false;
    }

    /// Spend one tick of lifetime.
    ///
    /// From A2: `∀t₁ < t₂: λ(s, t₂) ≤ λ(s, t₁)`
    ///
    /// Returns `true` if still alive, `false` if dead.
    pub fn tick(&mut self) -> bool {
        if self.lifetime > 0 {
            self.lifetime = self.lifetime.saturating_sub(1);
        }
        self.is_alive()
    }

    /// Check if shape is alive.
    ///
    /// Part of A1: `inhabits(s, space) ⟺ β(s) > 0 ∧ λ(s) > 0 ∧ ...`
    #[inline]
    pub fn is_alive(&self) -> bool {
        self.budget > 0 && self.lifetime > 0
    }

    /// Check if shape can produce trace.
    ///
    /// From A0: temporal observation requires payload + relationality.
    /// For now, we only check payload.
    #[inline]
    pub fn can_produce_trace(&self) -> bool {
        self.is_alive() && self.has_payload
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lifetime_decay() {
        let mut shape = Shape::new(1, 5, 5, 100, 10, 1.0, 5);
        assert!(shape.is_alive());

        // Tick 10 times
        for _ in 0..10 {
            shape.tick();
        }

        // Should be dead
        assert!(!shape.is_alive());
    }

    #[test]
    fn test_payload_requirement() {
        let mut shape = Shape::new(1, 5, 5, 100, 10, 1.0, 5);

        // No payload = no trace
        assert!(!shape.can_produce_trace());

        // Attach payload
        shape.attach_payload();
        assert!(shape.can_produce_trace());
    }

    #[test]
    fn test_death_inevitability() {
        // A5: ∀s: ∃t*: λ(s, t*) = 0 ⟹ ¬inhabits(s, space)
        let mut shape = Shape::new(1, 0, 0, 100, 5, 1.0, 1);

        let mut ticks = 0;
        while shape.is_alive() {
            shape.tick();
            ticks += 1;
            if ticks > 100 {
                panic!("Death should be inevitable");
            }
        }

        assert_eq!(ticks, 5);
    }
}
