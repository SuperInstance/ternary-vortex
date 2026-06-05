# ternary-vortex

**Whirlpools in ternary space. Vorticity, circulation, and the Biot-Savart law on discrete grids.**

A vortex is a point where fluid rotates: the center of a whirlpool, the eye of a hurricane. In continuous fluid dynamics, vortices are described by the vorticity field (curl of velocity) and connected to velocity by the Biot-Savart law. In ternary space, every quantity is {-1, 0, +1} — the vortex dynamics become discrete algebra, not differential equations.

## What's Inside

- **`VelocityField`** — 2D velocity grid with ternary components
- **`vorticity()`** — curl of velocity field
- **`divergence()`** — divergence of velocity field
- **`circulation()`** — line integral around a closed path
- **`stream_function()`** — integrate velocity to get streamlines
- **`biot_savart_point()`** — velocity induced by a point vortex
- **`biot_savart_field()`** — full velocity field from vorticity distribution
- **`rankine_vortex()`** — classic vortex profile (solid core + irrotational decay)

## The Deeper Truth

**In ternary, there are no vortices — only the algebraic ghost of rotation.** A continuous vortex has infinite resolution at its center: the velocity increases without bound as r→0. In ternary, velocity is clamped to {-1, 0, +1}, so the vortex is always bounded. The circulation around any closed loop is exactly -1, 0, or +1. Stokes' theorem (circulation = integral of vorticity) becomes exact arithmetic: the sum of ternary vorticities inside the loop equals the ternary circulation around it.

## See Also

- **ternary-field** — scalar field dynamics on grids
- **ternary-kuramoto** — synchronization (the opposite of vorticity)
- **ternary-drift** — drift dynamics on grids
- **ternary-dynamics** — general dynamical systems
- **ternary-warp** — spatial warping and flow

## Install

```bash
cargo add ternary-vortex
```

## License

MIT
