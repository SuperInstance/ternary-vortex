# Ternary Vortex

**Ternary Vortex** implements fluid dynamics on ternary grids — velocity fields where each component is constrained to {−1, 0, +1}. It computes vorticity (curl), divergence, circulation, and Biot-Savart induction, enabling the study of vortex-like flow patterns in ternary state spaces.

## Why It Matters

Fluid dynamics provides a powerful mathematical framework for understanding continuous flow: vortices, turbulence, laminar regimes. But what happens when the velocity field is discretized to ternary values? This question is both physically motivated (quantized flows in superconductors, discrete models of turbulence) and computationally practical (ternary arithmetic is cheaper than floating-point). In the SuperInstance framework, agent decision fields can be modeled as flows: constructive decisions (+1) are positive flow, avoidant decisions (−1) are negative flow, and neutral (0) is stillness. Where these flows circle, we have vortices — stable, self-sustaining patterns of behavior. Where they diverge, we have sources or sinks — agents entering or leaving a state. The Biot-Savart law tells us how a vortex at one point influences the field everywhere else.

## How It Works

### Ternary Velocity Field

```rust
struct VelocityField {
    width: usize,
    height: usize,
    vx: Vec<i8>,  // horizontal velocity: {-1, 0, +1} per cell
    vy: Vec<i8>,  // vertical velocity: {-1, 0, +1} per cell
}
```

The field is a 2D grid where each cell stores (vx, vy) ∈ {−1, 0, +1}², giving nine possible velocity directions per cell.

### Vorticity (Curl)

The scalar vorticity in 2D is:

```
ω = ∂v_y/∂x − ∂v_x/∂y
```

On the ternary grid, partial derivatives are forward differences clamped to {−1, 0, +1}:

```
∂v_y/∂x = clamp(v_y(x+1, y) − v_y(x, y), -1, +1)
∂v_x/∂y = clamp(v_x(x, y+1) − v_x(x, y), -1, +1)
ω = clamp(∂v_y/∂x − ∂v_x/∂y, -1, +1)
```

Positive ω indicates counterclockwise rotation; negative ω indicates clockwise.

### Divergence

```
∇·v = ∂v_x/∂x + ∂v_y/∂y
```

Non-zero divergence indicates sources (+1) or sinks (−1) — locations where flow appears or disappears.

### Circulation

Circulation around a closed loop is the line integral of velocity along the path:

```
Γ = ∮ v · dl
```

On the ternary grid, this becomes a discrete sum:

```
Γ = Σ v(path_i) · (path_{i+1} − path_i)
```

By Stokes' theorem, Γ equals the total vorticity enclosed by the loop. The ternary clamping makes Γ a coarse approximation but preserves its sign and rough magnitude.

### Biot-Savart Law

The velocity induced at point **r** by a vortex of strength Γ at point **r'**:

```
v(r) = Γ / (2π) · ẑ × (r − r') / |r − r'|²
```

On the ternary grid, this is computed discretely and clamped to {−1, 0, +1}, giving a coarse long-range interaction model.

### Complexity

| Operation | Time | Space |
|-----------|------|-------|
| Vorticity field | O(W·H) | O(W·H) |
| Divergence field | O(W·H) | O(W·H) |
| Circulation (loop of L points) | O(L) | O(1) |
| Biot-Savart (N vortices, M query points) | O(N·M) | O(M) |

## Quick Start

```rust
use ternary_vortex::VelocityField;

fn main() {
    let mut field = VelocityField::new(10, 10);

    // Create a simple rotation: center rotates counterclockwise
    field.set(5, 4, 1, 0);   // rightward
    field.set(6, 5, 0, 1);   // upward
    field.set(5, 6, -1, 0);  // leftward
    field.set(4, 5, 0, -1);  // downward

    // Compute vorticity
    let omega = field.vorticity();
    println!("Vorticity at center: {}", omega[5 * 10 + 5]); // expect +1

    // Compute divergence
    let div = field.divergence();
    println!("Divergence at center: {}", div[5 * 10 + 5]);
}
```

```bash
cargo build
cargo test
```

## API

| Type/Function | Method | Description |
|---------------|--------|-------------|
| `VelocityField` | `new(w, h)` | Create zero-initialized field |
| `VelocityField` | `get(x, y) → (i8, i8)` | Get velocity at point |
| `VelocityField` | `set(x, y, vx, vy)` | Set velocity (clamped to ternary) |
| `VelocityField` | `vorticity() → Vec<i8>` | Curl at every point |
| `VelocityField` | `divergence() → Vec<i8>` | Divergence at every point |
| `circulation(field, path) → i8` | — | Line integral around closed loop |

## Architecture Notes

Ternary Vortex models the **dynamics** of γ + η = C over space. A velocity field of γ (positive, +1) and η (negative, −1) creates flow patterns: vortices represent stable circular behaviors (self-sustaining competence C), divergence represents agents entering or leaving a strategy (source = new γ, sink = η elimination), and the Biot-Savart law captures long-range influence — a vortex of activity in one region affects agents far away. This connects local ternary decisions to global spatial dynamics. See [ARCHITECTURE.md](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md).

## References

1. Batchelor, G. K. (1967). *An Introduction to Fluid Dynamics*. Cambridge University Press. — Vorticity, circulation, and Biot-Savart.
2. Chorin, A. J., & Marsden, J. E. (1993). *A Mathematical Introduction to Fluid Mechanics*, 3rd ed. Springer.
3. Kada, K., et al. (2018). "Quantized Vortices in Superfluids." *Journal of Low Temperature Physics*. — On discrete/quantized vortex structures.

## License

MIT
