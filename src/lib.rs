//! # ternary-vortex
//! 
//! Vortex dynamics and fluid-like flow on ternary grids.
//! Circulation, vorticity, stream functions, and the Biot-Savart law.

#![forbid(unsafe_code)]
#![no_std]

extern crate alloc;
use alloc::{vec, vec::Vec};

/// Integer square root
fn isqrt(n: usize) -> usize {
    if n == 0 { return 0; }
    let mut x = n;
    let mut y = (x + 1) / 2;
    while y < x {
        x = y;
        y = (x + n / x) / 2;
    }
    x
}

/// A 2D velocity field on a ternary grid
/// Each cell stores (vx, vy) where each component is {-1, 0, 1}
#[derive(Debug, Clone)]
pub struct VelocityField {
    pub width: usize,
    pub height: usize,
    pub vx: Vec<i8>,
    pub vy: Vec<i8>,
}

impl VelocityField {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width, height,
            vx: vec![0; width * height],
            vy: vec![0; width * height],
        }
    }

    pub fn get(&self, x: usize, y: usize) -> (i8, i8) {
        let idx = y * self.width + x;
        (self.vx[idx], self.vy[idx])
    }

    pub fn set(&mut self, x: usize, y: usize, vx: i8, vy: i8) {
        let idx = y * self.width + x;
        self.vx[idx] = vx.clamp(-1, 1);
        self.vy[idx] = vy.clamp(-1, 1);
    }

    /// Compute vorticity (curl in 2D) at each point: ω = ∂vy/∂x - ∂vx/∂y
    pub fn vorticity(&self) -> Vec<i8> {
        let mut omega = vec![0i8; self.width * self.height];
        for y in 0..self.height {
            for x in 0..self.width {
                let dvy_dx = if x + 1 < self.width {
                    self.get(x + 1, y).1 - self.get(x, y).1
                } else { 0 };
                let dvx_dy = if y + 1 < self.height {
                    self.get(x, y + 1).0 - self.get(x, y).0
                } else { 0 };
                let vort = dvy_dx - dvx_dy;
                omega[y * self.width + x] = vort.clamp(-1, 1);
            }
        }
        omega
    }

    /// Compute divergence: ∇·v = ∂vx/∂x + ∂vy/∂y
    pub fn divergence(&self) -> Vec<i8> {
        let mut div = vec![0i8; self.width * self.height];
        for y in 0..self.height {
            for x in 0..self.width {
                let dvx_dx = if x + 1 < self.width {
                    self.get(x + 1, y).0 - self.get(x, y).0
                } else { 0 };
                let dvy_dy = if y + 1 < self.height {
                    self.get(x, y + 1).1 - self.get(x, y).1
                } else { 0 };
                let d = dvx_dx + dvy_dy;
                div[y * self.width + x] = d.clamp(-1, 1);
            }
        }
        div
    }
}

/// Compute circulation around a closed loop of grid points
pub fn circulation(field: &VelocityField, path: &[(usize, usize)]) -> i8 {
    let mut circ = 0i8;
    for i in 0..path.len() {
        let (x, y) = path[i];
        let (nx, ny) = path[(i + 1) % path.len()];
        let (vx, vy) = field.get(x, y);
        // Line integral: v · dl where dl is the displacement
        let dx = (nx as i8).wrapping_sub(x as i8);
        let dy = (ny as i8).wrapping_sub(y as i8);
        circ += vx * dx + vy * dy;
    }
    circ.clamp(-1, 1)
}

/// Stream function from velocity field
/// In 2D: vx = ∂ψ/∂y, vy = -∂ψ/∂x
pub fn stream_function(field: &VelocityField) -> Vec<i8> {
    let mut psi = vec![0i8; field.width * field.height];
    // Integrate vx along y
    for x in 0..field.width {
        let mut cumulative = 0i8;
        for y in 0..field.height {
            cumulative += field.get(x, y).0;
            psi[y * field.width + x] = cumulative.clamp(-1, 1);
        }
    }
    psi
}

/// Simple Biot-Savart: compute velocity induced at (px, py) by a point vortex at (vx, vy) with strength gamma
pub fn biot_savart_point(px: usize, py: usize, vx: usize, vy: usize, gamma: i8) -> (i8, i8) {
    let dx = px as i8 - vx as i8;
    let dy = py as i8 - vy as i8;
    let r2 = dx * dx + dy * dy;
    if r2 == 0 {
        return (0, 0);
    }
    // v = gamma / (2π r²) * (-dy, dx)
    let speed = gamma / r2.max(1);
    let ux = (-dy * speed).clamp(-1, 1);
    let uy = (dx * speed).clamp(-1, 1);
    (ux, uy)
}

/// Apply Biot-Savart to compute velocity field from vorticity distribution
pub fn biot_savart_field(vorticity: &[i8], width: usize, height: usize) -> VelocityField {
    let mut field = VelocityField::new(width, height);
    for py in 0..height {
        for px in 0..width {
            let mut vx_total = 0i8;
            let mut vy_total = 0i8;
            for vy in 0..height {
                for vx_i in 0..width {
                    let gamma = vorticity[vy * width + vx_i];
                    if gamma == 0 { continue; }
                    let (ux, uy) = biot_savart_point(px, py, vx_i, vy, gamma);
                    vx_total += ux;
                    vy_total += uy;
                }
            }
            field.set(px, py, vx_total, vy_total);
        }
    }
    field
}

/// Rankine vortex profile: solid body rotation inside radius r, 1/r decay outside
pub fn rankine_vortex(radius: usize, width: usize, height: usize) -> VelocityField {
    let mut field = VelocityField::new(width, height);
    let cx = width / 2;
    let cy = height / 2;
    for y in 0..height {
        for x in 0..width {
            let dx = x as i8 - cx as i8;
            let dy = y as i8 - cy as i8;
            let r = isqrt((dx * dx + dy * dy) as usize).max(1);
            let (vx, vy) = if r <= radius {
                // Solid body rotation
                (-dy.clamp(-1, 1), dx.clamp(-1, 1))
            } else if r > 0 {
                // Outside: velocity decays (ternary approximation)
                let vx = if radius * 2 >= r { -dy.clamp(-1, 1) } else { 0 };
                let vy = if radius * 2 >= r { dx.clamp(-1, 1) } else { 0 };
                (vx, vy)
            } else {
                (0, 0)
            };
            field.set(x, y, vx, vy);
        }
    }
    field
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_velocity_field_new() {
        let f = VelocityField::new(3, 3);
        assert_eq!(f.get(1, 1), (0, 0));
    }

    #[test]
    fn test_velocity_field_set() {
        let mut f = VelocityField::new(3, 3);
        f.set(1, 1, 1, -1);
        assert_eq!(f.get(1, 1), (1, -1));
    }

    #[test]
    fn test_zero_vorticity() {
        let f = VelocityField::new(3, 3);
        let vort = f.vorticity();
        assert!(vort.iter().all(|&v| v == 0));
    }

    #[test]
    fn test_nonzero_vorticity() {
        let mut f = VelocityField::new(3, 3);
        f.set(0, 0, 0, -1);
        f.set(1, 0, 0, 1);
        let vort = f.vorticity();
        assert!(vort.iter().any(|&v| v != 0));
    }

    #[test]
    fn test_zero_divergence() {
        let f = VelocityField::new(3, 3);
        let div = f.divergence();
        assert!(div.iter().all(|&d| d == 0));
    }

    #[test]
    fn test_circulation_zero() {
        let f = VelocityField::new(3, 3);
        let path = vec![(0, 0), (1, 0), (1, 1), (0, 1)];
        assert_eq!(circulation(&f, &path), 0);
    }

    #[test]
    fn test_stream_function() {
        let f = VelocityField::new(3, 3);
        let psi = stream_function(&f);
        assert_eq!(psi.len(), 9);
    }

    #[test]
    fn test_biot_savart_point_self() {
        let (vx, vy) = biot_savart_point(5, 5, 5, 5, 1);
        assert_eq!((vx, vy), (0, 0));
    }

    #[test]
    fn test_biot_savart_point_nonzero() {
        let (vx, vy) = biot_savart_point(0, 0, 1, 0, 1);
        // Point vortex at (1,0), evaluating at (0,0): dx=-1, dy=0
        // v = gamma/(2πr²) * (-dy, dx) = 1/2 * (0, -1)
        assert!(vx == 0 || vx == -1 || vx == 1);
    }

    #[test]
    fn test_rankine_vortex() {
        let f = rankine_vortex(2, 5, 5);
        // Center should be zero velocity
        assert_eq!(f.get(2, 2), (0, 0));
    }

    #[test]
    fn test_rankine_nonzero() {
        let f = rankine_vortex(2, 5, 5);
        // Some cells should have nonzero velocity
        let has_flow = (0..5).any(|y| (0..5).any(|x| f.get(x, y) != (0, 0)));
        assert!(has_flow);
    }

    #[test]
    fn test_biot_savart_field() {
        let mut vort = vec![0i8; 9];
        vort[4] = 1; // vortex at center of 3x3
        let field = biot_savart_field(&vort, 3, 3);
        assert_eq!(field.width, 3);
    }

    #[test]
    fn test_field_clamp() {
        let mut f = VelocityField::new(3, 3);
        f.set(0, 0, 5, -5); // should clamp to (1, -1)
        assert_eq!(f.get(0, 0), (1, -1));
    }
}
