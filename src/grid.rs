//! Trace density grid - the heart of TES
//!
//! Trace is NOT a data structure, it's a scalar field.
//! No origin tracking (Source Amnesia), just density values.

use std::sync::atomic::{AtomicU32, Ordering};

/// A 2D grid of trace density values.
///
/// # Implementation Note
///
/// Uses `AtomicU32` for lock-free updates.
/// Density is stored as fixed-point (multiply by 0.001 for float value).
///
/// # Complexity
///
/// - Contribution: O(1)
/// - Decay: O(grid_size)
/// - Check habitability: O(1)
pub struct DensityGrid {
    width: usize,
    height: usize,
    cells: Vec<AtomicU32>,
    /// Decay rate per tick (fixed-point, 1000 = 1.0)
    decay_rate: u32,
    /// Solid threshold (above this = Solid regime)
    solid_threshold: u32,
    /// Liquid threshold (above this = Liquid regime)
    liquid_threshold: u32,
}

/// Phase regime at a given position
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Regime {
    /// High density - low permeability, fast saturation
    Solid,
    /// Medium density - medium permeability, diffusion
    Liquid,
    /// Low density - high permeability, no pattern forms
    Gas,
}

impl DensityGrid {
    /// Create a new density grid.
    ///
    /// # Arguments
    ///
    /// * `width` - Grid width
    /// * `height` - Grid height
    /// * `decay_rate` - Decay per tick (0.001 units, e.g., 50 = 0.05 decay)
    /// * `solid_threshold` - Density threshold for Solid regime
    /// * `liquid_threshold` - Density threshold for Liquid regime
    pub fn new(
        width: usize,
        height: usize,
        decay_rate: u32,
        solid_threshold: u32,
        liquid_threshold: u32,
    ) -> Self {
        let cells = (0..width * height)
            .map(|_| AtomicU32::new(0))
            .collect();

        Self {
            width,
            height,
            cells,
            decay_rate,
            solid_threshold,
            liquid_threshold,
        }
    }

    /// Contribute trace density at position (side-effect).
    ///
    /// This is the **only** way trace accumulates.
    /// No origin information is stored (Source Amnesia).
    #[inline]
    pub fn contribute(&self, x: usize, y: usize, amount: u32) {
        if let Some(cell) = self.get_cell(x, y) {
            cell.fetch_add(amount, Ordering::Relaxed);
        }
    }

    /// Get current density at position.
    #[inline]
    pub fn density(&self, x: usize, y: usize) -> u32 {
        self.get_cell(x, y)
            .map(|c| c.load(Ordering::Relaxed))
            .unwrap_or(0)
    }

    /// Get regime at position.
    #[inline]
    pub fn regime(&self, x: usize, y: usize) -> Regime {
        let d = self.density(x, y);
        if d > self.solid_threshold {
            Regime::Solid
        } else if d > self.liquid_threshold {
            Regime::Liquid
        } else {
            Regime::Gas
        }
    }

    /// Check if position is habitable (not saturated).
    ///
    /// This is the core habitability check from A1.
    #[inline]
    pub fn is_habitable(&self, x: usize, y: usize, threshold: u32) -> bool {
        self.density(x, y) < threshold
    }

    /// Apply global decay to entire grid.
    ///
    /// This is the δ projection function.
    /// Called once per tick.
    pub fn apply_decay(&self) {
        for cell in &self.cells {
            let current = cell.load(Ordering::Relaxed);
            let new_value = current.saturating_sub(self.decay_rate);
            cell.store(new_value, Ordering::Relaxed);
        }
    }

    /// Get dimensions
    pub fn dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    #[inline]
    fn get_cell(&self, x: usize, y: usize) -> Option<&AtomicU32> {
        if x < self.width && y < self.height {
            Some(&self.cells[y * self.width + x])
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contribution_and_decay() {
        let grid = DensityGrid::new(10, 10, 10, 1000, 500);

        // Contribute
        grid.contribute(5, 5, 100);
        assert_eq!(grid.density(5, 5), 100);

        // Decay
        grid.apply_decay();
        assert_eq!(grid.density(5, 5), 90);
    }

    #[test]
    fn test_regime_transitions() {
        let grid = DensityGrid::new(10, 10, 0, 1000, 500);

        // Gas initially
        assert_eq!(grid.regime(0, 0), Regime::Gas);

        // Contribute to Liquid
        grid.contribute(0, 0, 600);
        assert_eq!(grid.regime(0, 0), Regime::Liquid);

        // Contribute to Solid
        grid.contribute(0, 0, 500);
        assert_eq!(grid.regime(0, 0), Regime::Solid);
    }

    #[test]
    fn test_source_amnesia() {
        let grid = DensityGrid::new(10, 10, 0, 1000, 500);

        // Multiple contributions from "different sources"
        // But we have NO WAY to know who contributed what
        grid.contribute(3, 3, 50);
        grid.contribute(3, 3, 30);
        grid.contribute(3, 3, 20);

        // Only total density is known
        assert_eq!(grid.density(3, 3), 100);
        // Source information is LOST by design
    }
}
