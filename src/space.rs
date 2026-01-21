//! Space - the atemporal, passive topographic field
//!
//! Space does NOT change. Observations project through time.

use crate::grid::DensityGrid;

/// The topographic execution space.
///
/// # Formalization Mapping
///
/// ```text
/// Space := (V, ω, δ, τ)
/// ```
///
/// - `V` → grid dimensions (discrete approximation of ℝⁿ)
/// - `ω` → density values in grid (weight function)
/// - `δ` → `apply_decay()` (projection function)
/// - `τ` → habitability threshold (local lifetime function)
pub struct Space {
    /// Trace density grid
    pub(crate) trace: DensityGrid,
    /// Global habitability threshold
    threshold: u32,
}

impl Space {
    /// Create a new space.
    ///
    /// # Arguments
    ///
    /// * `width` - Space width
    /// * `height` - Space height
    /// * `decay_rate` - Trace decay per tick
    /// * `threshold` - Habitability threshold
    pub fn new(width: usize, height: usize, decay_rate: u32, threshold: u32) -> Self {
        let solid = threshold;
        let liquid = threshold / 2;
        let trace = DensityGrid::new(width, height, decay_rate, solid, liquid);
        Self { trace, threshold }
    }

    /// Check if position is habitable.
    ///
    /// From A1: `inhabits(s, space) ⟺ ... ∧ ω(pos(s)) < threshold(space)`
    #[inline]
    pub fn is_habitable(&self, x: usize, y: usize) -> bool {
        self.trace.is_habitable(x, y, self.threshold)
    }

    /// Get density at position.
    #[inline]
    pub fn density(&self, x: usize, y: usize) -> u32 {
        self.trace.density(x, y)
    }

    /// Get regime at position.
    #[inline]
    pub fn regime(&self, x: usize, y: usize) -> crate::grid::Regime {
        self.trace.regime(x, y)
    }

    /// Contribute trace at position (side-effect of presence).
    #[inline]
    pub fn contribute(&self, x: usize, y: usize, amount: u32) {
        self.trace.contribute(x, y, amount);
    }

    /// Apply decay projection.
    ///
    /// This is δ: (ω, t) → ω'
    /// Called once per tick.
    pub fn tick(&self) {
        self.trace.apply_decay();
    }

    /// Get dimensions.
    pub fn dimensions(&self) -> (usize, usize) {
        self.trace.dimensions()
    }

    /// Get threshold.
    pub fn threshold(&self) -> u32 {
        self.threshold
    }
}
