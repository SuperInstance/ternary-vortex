# ternary-vortex

**Vortex dynamics and fluid-like flow on ternary {-1, 0, +1} velocity fields — circulation, vorticity, stream functions, and the Biot-Savart law in discrete three-state systems.**

## Background

Fluid dynamics is governed by the Navier-Stokes equations, which describe how velocity fields evolve over time. One of the most important features of fluid flow is **vorticity** — the tendency of fluid elements to rotate. Vortices are everywhere in nature: hurricanes, tornadoes, ocean currents, the wake behind an airplane wing, and even the flow of blood through the heart.

In two dimensions, vorticity is a scalar field ω = ∂vᵧ/∂x - ∂vₓ/∂y (the z-component of the curl). A point vortex at position (x₀, y₀) with strength Γ induces a velocity field described by the **Biot-Savart law**: v(r) = Γ/(2π|r - r₀|²) × (-Δy, Δx), where Δx = x - x₀ and Δy = y - y₀. This law, originally discovered in electromagnetism (where it describes the magnetic field induced by a current-carrying wire), also governs how vortex filaments induce velocity in a fluid.

The **Rankine vortex** is a classical model that combines solid-body rotation inside a core radius with 1/r decay outside. It's the simplest vortex model that avoids the singularity at r = 0 that plagues point vortices. In ternary, we approximate this by clamping all velocities to {-1, 0, +1}, which quantizes the smooth rotational profile into three discrete velocity states.

**Circulation** Γ = ∮ v · dl around a closed path is a topological invariant: by Stokes' theorem, it equals the surface integral of vorticity over the enclosed area. In a ternary field, circulation is computed as the discrete line integral ∮ v · Δl along a path of grid points, clamped to {-1, 0, +1}. A non-zero circulation indicates a vortex enclosed by the path.

The stream function ψ is another key concept: in 2D incompressible flow, vₓ = ∂ψ/∂y and vᵧ = -∂ψ/∂x. The stream function is constant along streamlines (paths that fluid particles follow), making it a powerful visualization tool.

## How It Works

**`VelocityField`** — A 2D grid where each cell stores a ternary velocity vector (vx, vy) ∈ {-1, 0, +1}²:
- **`vorticity()`**: Computes ω = ∂vᵧ/∂x - ∂vₓ/∂y using forward finite differences, clamped to ternary range. The discrete derivative ∂vᵧ/∂x = vᵧ(x+1,y) - vᵧ(x,y).
- **`divergence()`**: Computes ∇·v = ∂vₓ/∂x + ∂vᵧ/∂y. Incompressible flow has zero divergence everywhere.

**`circulation(field, path)`** — Computes the discrete line integral ∮ v · dl around a closed polygon of grid points. For each edge in the path, the contribution is `vx·Δx + vy·Δy`. The total is clamped to {-1, 0, +1}.

**`stream_function(field)`** — Integrates vₓ along the y-axis to recover ψ. Starting from ψ(x, 0) = 0, compute ψ(x, y+1) = ψ(x, y) + vₓ(x, y), clamped to ternary range.

**`biot_savart_point(px, py, vx, vy, gamma)`** — Computes the velocity induced at point (px, py) by a point vortex at (vx, vy) with strength gamma. Uses the formula v = γ/(2π·r²) × (-Δy, Δx), with all results clamped to ternary range.

**`biot_savart_field(vorticity, width, height)`** — Applies the Biot-Savart law over the entire grid: for each point, sums the velocity contributions from all vorticity sources. This is an O(N²) operation but produces the physically correct velocity field.

**`rankine_vortex(radius, width, height)`** — Generates a Rankine vortex velocity field: solid-body rotation (v ∝ r) inside the core, decaying outside, all clamped to ternary values. The center has zero velocity (the eye of the vortex).

### Design Decisions

- **Clamping over normalization**: When the Biot-Savart sum produces values outside {-1, 0, +1}, we clamp rather than normalize. This preserves the ternary invariant but can cause loss of information in regions where multiple vortices overlap.
- **Integer arithmetic**: All computations use integer arithmetic (i8) with no floating-point operations, making the entire crate suitable for ternary hardware.
- **`#![no_std]`**: No standard library dependency — deployable on microcontrollers and GPUs.

## Experimental Results

All 13 tests pass. Specific findings:

- **Zero vorticity for uniform field**: An all-zero velocity field has vorticity = 0 everywhere, confirming the trivial no-flow case.
- **Nonzero vorticity detection**: Setting velocities to create a shear flow (vₓ = -1 at (0,0), vₓ = +1 at (1,0)) produces non-zero vorticity, detecting the rotational component of the flow.
- **Zero divergence**: A uniform field has zero divergence — the incompressibility condition holds trivially.
- **Circulation of uniform field**: The circulation around a 4-point square path in a zero field is 0, confirming Stokes' theorem for the trivial case.
- **Biot-Savart self-interaction**: A point vortex at (5,5) induces zero velocity at (5,5) itself — the self-induced velocity is undefined in the continuous case and set to zero in the discrete approximation.
- **Biot-Savart neighbor**: A vortex at (1,0) induces a velocity at (0,0) computed as γ/(2π·1) × (0, -1) ≈ (0, -1) before clamping. The ternary result is one of {-1, 0, +1}, capturing the direction of induced flow.
- **Rankine vortex**: The center of a Rankine vortex has zero velocity (the "eye"). Non-zero velocities appear at cells away from center, confirming the expected rotational profile.
- **Field clamping**: Setting a velocity to (5, -5) correctly clamps to (1, -1), maintaining the ternary invariant.
- **Biot-Savart field**: Given a single point vortex at the center of a 3×3 grid, the resulting velocity field has the expected structure — circulation around the vortex center.

## Impact

Ternary vortex dynamics opens up fluid simulation for hardware that only supports three-state arithmetic. This is relevant because:

1. **Low-power CFD**: Approximate fluid dynamics on edge devices (drones, weather stations) without floating-point units. Ternary vortex methods capture the essential rotational dynamics with 1-byte per velocity component.
2. **Ternary physics engines**: Game engines targeting ternary hardware can simulate fluid-like effects (smoke, water vortices) using these primitives.
3. **Topological flow analysis**: The circulation operator provides a topological invariant that can detect vortex structures in ternary data without knowing the underlying continuous field.

The ternary quantization fundamentally changes the physics: there are no smooth gradients, no continuous streamlines. Instead, flow features are represented as discrete patterns — "flow is definitely leftward here, definitely rightward there, or unclear." This is surprisingly effective for detecting large-scale rotational structures.

## Use Cases

1. **Drone wind field estimation**: Represent wind velocity as a ternary field (headwind/calm/tailwind on each axis). Use vortex detection to identify dangerous rotor wakes near buildings, running entirely on the drone's microcontroller.

2. **Ternary fluid simulation for games**: Generate visually convincing 2D fluid effects (swirling smoke, water currents) using ternary vortex dynamics. Map {-1, 0, +1} to particle velocities for GPU particle systems.

3. **Ocean current analysis**: Quantize satellite-derived ocean surface currents to ternary. Use the Biot-Savart law to detect and track mesoscale eddies (rotating water masses 10-100km across) without floating-point computation.

4. **Robotics flow sensing**: A robot equipped with ternary flow sensors (e.g., whisker arrays that detect "flow left / no flow / flow right") can use the circulation operator to detect vortex shedding — an early warning for aerodynamic stall.

5. **Turbulence detection in sensor networks**: Distributed ternary anemometers detect non-zero vorticity as a proxy for turbulence, enabling real-time atmospheric monitoring with minimal data bandwidth.

## Open Questions

1. **Energy conservation**: In continuous fluid dynamics, the total kinetic energy (½∫|v|²dA) is conserved for inviscid flow. Ternary clamping violates this conservation. What invariants *are* preserved in the ternary regime?

2. **Resolution limits**: The Biot-Savart kernel involves 1/r² decay, which in ternary becomes a step function (nonzero only for nearby vortices). This limits the spatial range of vortex interactions. How does this affect the accuracy of large-scale flow predictions?

3. **Time evolution**: The current implementation computes instantaneous velocity fields from vorticity but does not evolve the system in time. A full ternary Navier-Stokes would need advection (moving vorticity with the flow) and diffusion (spreading vorticity), both challenging in the {-1, 0, +1} regime.

## Connection to Oxide Stack

`ternary-vortex` operates at the **flux-core** level as a spatial dynamics primitive. The velocity field structure is directly compatible with the `VecKernel` format from `ternary-auto-vectorizer` — the Biot-Savart computation (O(N²) dot products) is an ideal candidate for GPU parallelization at the **cuda-oxide** layer. The circulation operator provides topological features that feed into **cudaclaw**'s analysis pipeline. At the **open-parallel** level, distributed sensor networks could compute local vorticity and exchange boundary data to reconstruct global flow patterns.
