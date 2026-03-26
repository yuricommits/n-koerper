# n-körper

A real-time n-body gravitational simulator written in Rust, implementing the Barnes-Hut algorithm for O(n log n) force computation. Bodies are randomly spawned with physically correct orbital velocities, rendered with per-body color and mass-scaled radius, and displayed in a live minifb window with a dynamic CoM-anchored grid.

![n-körper screenshot](screenshot.png)

---

## Table of Contents

- [Background](#background)
  - [The Two-Body Problem](#the-two-body-problem)
  - [The Three-Body Problem](#the-three-body-problem)
  - [The N-Body Problem](#the-n-body-problem)
- [The Barnes-Hut Algorithm](#the-barnes-hut-algorithm)
  - [The Quadtree](#the-quadtree)
  - [The θ Parameter](#the-θ-parameter)
  - [Complexity](#complexity)
- [Physics](#physics)
  - [Gravitational Force](#gravitational-force)
  - [Numerical Integration](#numerical-integration)
  - [Softening](#softening)
  - [Momentum Conservation](#momentum-conservation)
- [Project Structure](#project-structure)
- [Constants and Tuning](#constants-and-tuning)
- [Building and Running](#building-and-running)
- [Controls](#controls)
- [Dependencies](#dependencies)

---

## Background

### The Two-Body Problem

When two bodies interact gravitationally — a planet orbiting a star, two stars orbiting a common center — the system has an **exact analytical solution**. Given initial positions and velocities, you can write down a closed-form equation that tells you where each body will be at any future time. The orbits are always conic sections: circles, ellipses, parabolas, or hyperbolas.

This is the foundation of classical orbital mechanics and the reason we can launch probes to distant planets with extraordinary precision.

### The Three-Body Problem

Add a third body and the system becomes **chaotic**. No general closed-form solution exists. The future state of the system cannot be computed from a formula — only approximated by numerically integrating the equations of motion step by step.

Henri Poincaré proved in 1887 that the three-body problem is non-integrable in general. Small differences in initial conditions grow exponentially over time — a hallmark of chaotic systems. The bodies can form stable configurations for a time, then suddenly eject one member, sending it flying off to infinity.

This is not a failure of computation — it is a fundamental property of the mathematics. The chaos is real.

### The N-Body Problem

The three-body problem generalizes to any number of bodies. With N bodies, each exerts a gravitational force on every other, giving N(N-1)/2 pairwise interactions per time step. Simulating a galaxy of a million stars with the naïve approach requires 500 billion force computations per step — completely infeasible in real time.

This simulator uses the **Barnes-Hut algorithm** to reduce this to O(n log n), making large particle counts tractable.

---

## The Barnes-Hut Algorithm

### The Quadtree

Barnes-Hut uses a **quadtree** — a recursive spatial data structure that subdivides 2D space into four quadrants (NE, NW, SE, SW), then recursively subdivides each quadrant as needed. Each node stores:

- The **bounding region** of that subdivision
- The **total mass** of all bodies within it
- The **center of mass** of all bodies within it
- Up to four **child nodes**

A leaf node contains exactly one body. An internal node contains the aggregated mass data for its entire subtree.

```
+----------+----------+
|          |    |     |
|    NW    | NE | NE  |
|          +----|-----+
|          |    |     |
+----------+----------+
|          |          |
|    SW    |    SE    |
|          |          |
+----------+----------+
```

The tree is rebuilt from scratch every simulation step since bodies move.

### The θ Parameter

When computing the force on a body, the algorithm walks the quadtree. At each node it applies the **θ criterion**:

```
s / d < θ
```

Where `s` is the width of the node's region and `d` is the distance from the body to the node's center of mass.

- If the ratio is **below θ** — the node is far enough away relative to its size that its entire mass can be treated as a single point at the center of mass. No recursion needed.
- If the ratio is **above θ** — the node is too close or too large to approximate safely. Recurse into its children.

A smaller θ means more accurate forces but more computation. A larger θ means faster simulation with more approximation error. `θ = 0.5` is the standard default and the value used in this simulator.

### Complexity

| Algorithm | Force computation | Practical limit |
|-----------|-------------------|-----------------|
| Naïve     | O(n²)             | ~10,000 bodies  |
| Barnes-Hut| O(n log n)        | ~10,000,000 bodies |

---

## Physics

### Gravitational Force

Newton's law of universal gravitation:

```
F = G * m₁ * m₂ / r²
```

Where `G = 6.674 × 10⁻¹¹` N m² kg⁻², `m₁` and `m₂` are the masses, and `r` is the distance between them. The force is directed along the line connecting the two bodies.

In component form, the acceleration of body A due to body B:

```
dx = B.x - A.x
dy = B.y - A.y
r² = dx² + dy² + ε²       (ε = softening)
r  = sqrt(r²)
ax = G * M_B * dx / (r² * r)
ay = G * M_B * dy / (r² * r)
```

Note that this computes **acceleration directly**, not force — dividing by `m_A` cancels, which is why all bodies fall at the same rate regardless of their own mass.

### Numerical Integration

This simulator uses the **symplectic Euler** (semi-implicit Euler) integrator:

```
vel += acceleration * dt
pos += vel * dt
```

Velocity is updated before position. This is a first-order method — simple and fast, but accumulates energy error over time. For higher accuracy at a performance cost, a Leapfrog or Runge-Kutta 4 integrator could be substituted.

Numerical stability requires that bodies do not move more than a small fraction of their orbital radius per step. The simulation uses `DT = 1e-7` with `SUBSTEPS = 50` per frame — 50 physics steps per rendered frame.

### Softening

At very close range, the gravitational force between two bodies approaches infinity as `r → 0`. This causes numerical explosions when bodies pass near each other.

The **softening parameter** ε adds a small constant to the denominator:

```
r² = dx² + dy² + ε²
```

This prevents the force from diverging while having negligible effect at distances much larger than ε. The value `SOFTENING = 1e-3` is used here.

### Momentum Conservation

Barnes-Hut is an approximation — Newton's third law is not perfectly symmetric between approximated and non-approximated force pairs. This introduces a small net momentum drift over time, causing the center of mass to slowly accelerate.

To correct this, after each integration step the mass-weighted mean velocity is computed and subtracted from all bodies:

```
mean_vx = Σ(body.vel_x * body.mass) / total_mass
mean_vy = Σ(body.vel_y * body.mass) / total_mass

for each body:
    body.vel_x -= mean_vx
    body.vel_y -= mean_vy
```

This keeps the system in the center-of-mass frame and prevents all bodies from slowly drifting off screen.

---

## Project Structure

```
src/
├── main.rs        # Entry point, simulation loop, DT and SUBSTEPS constants
├── body.rs        # Body struct — position, velocity, mass, radius, color, id
├── sim.rs         # Simulation — spawn, quadtree integration, step()
├── quadtree.rs    # Barnes-Hut quadtree — BoundingBox, insert(), compute_force()
└── renderer.rs    # minifb renderer — draw(), grid, world_to_screen, draw_circle
```

### `body.rs`

Defines the `Body` struct. Radius is cube-root scaled from mass so larger bodies appear visually larger without dominating the screen:

```rust
radius = (mass / 1e12).cbrt() * 0.005
```

### `sim.rs`

`Simulation::new()` spawns a central heavy body at the origin and `n_orbiting` smaller bodies at random radii and angles. Each orbiting body is given the exact circular orbital velocity for its distance from the central mass:

```
v = sqrt(G * M / r)
```

A small random perturbation is added to each velocity to break perfect circular symmetry and produce interesting elliptical and chaotic trajectories.

`step()` rebuilds the quadtree each call, computes accelerations via Barnes-Hut, integrates positions and velocities, then applies the CoM velocity correction.

### `quadtree.rs`

The quadtree is an enum with three variants:

- `Empty(BoundingBox)` — unoccupied region
- `Leaf { body, region }` — single body
- `Internal { region, total_mass, center_of_mass, children }` — aggregated node

`insert()` places a body into the tree, splitting leaf nodes into internal nodes as needed and updating aggregated mass data up the tree.

`compute_force()` walks the tree applying the θ criterion, returning the gravitational acceleration vector on the queried body.

### `renderer.rs`

Uses `minifb` to manage a pixel buffer. Each frame:

1. Buffer cleared to black
2. Center of mass computed from body positions and masses
3. View scale set to the 90th percentile body distance × 1.5 — prevents ejected outliers from collapsing the zoom
4. Grid drawn with power-of-10 snapped spacing
5. Bodies drawn as filled circles with per-body color and mass-scaled radius

---

## Constants and Tuning

| Constant | Location | Default | Effect |
|----------|----------|---------|--------|
| `G` | `sim.rs` | `6.674e-11` | Gravitational constant — scale of all forces |
| `SOFTENING` | `sim.rs` | `1e-3` | Prevents force singularities at close range — increase to dampen close encounters |
| `THETA` | `sim.rs` | `0.5` | Barnes-Hut accuracy — lower = more accurate, slower |
| `DT` | `main.rs` | `1e-7` | Time step — smaller = more stable, slower |
| `SUBSTEPS` | `main.rs` | `50` | Physics steps per frame — more = smoother at cost of CPU |
| `central_mass` | `sim.rs` | `1e12..1e13` | Mass of the central body — drives orbital velocities |
| `n_orbiting` | `sim.rs` | `100..500` | Number of orbiting bodies |

---

## Building and Running

Requires Rust and Cargo. Tested on Linux.

```bash
git clone https://github.com/yuricommits/n-koerper
cd n-koerper
cargo run --release
```

Use `--release` for significantly better performance — debug builds are substantially slower due to unoptimized floating-point and recursive tree traversal.

---

## Controls

| Key | Action |
|-----|--------|
| `Escape` | Exit |

---

## Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| `minifb` | `0.28.0` | Window creation and pixel buffer rendering |
| `rand` | `0.10.0` | Random number generation for body spawn |
| `glam` | `0.32.1` | Vector math utilities |
| `rayon` | `1.11.0` | Parallel iteration for force calculations |

---

## LICENSE

MIT
