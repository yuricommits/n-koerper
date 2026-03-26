# Contributing to n-körper

Thank you for your interest in contributing. This document covers how to get started, what areas need work, and the standards expected for contributions.

---

## Getting Started

```bash
git clone https://github.com/yuricommits/n-koerper
cd n-koerper
cargo build
cargo run
```

Rust stable is sufficient. No nightly features are used.

---

## Areas Open for Contribution

### Physics

- **Leapfrog integrator** — replace symplectic Euler with a leapfrog (Störmer-Verlet) integrator for better long-term energy conservation
- **Body merging** — detect collisions when two bodies overlap and merge them into a single body conserving mass and momentum
- **3D extension** — extend positions and forces to three dimensions, adapting the quadtree to an octree

### Performance

- **Parallel force computation** — the per-body `compute_force` calls are independent and can be parallelized with `rayon`
- **Tree reuse** — instead of rebuilding the quadtree every step, explore incremental updates
- **SIMD** — vectorize the inner force calculation loop

### Rendering

- **Trails** — store a ring buffer of past positions per body and draw fading line trails
- **Glow effect** — accumulate brightness in a float buffer and apply bloom as a post-process pass
- **Quadtree overlay** — debug mode that draws the quadtree bounding boxes over the simulation
- **Coordinate labels** — render world-space coordinates at grid intersections using a pixel font

### Simulation

- **Multiple initial configurations** — named scenarios beyond the random default (galaxy collision, figure-eight orbit, Lagrange points)
- **Interactive spawn** — click to add bodies at runtime
- **Energy display** — compute and display total kinetic and potential energy to validate conservation

---

## Code Standards

- Run `cargo fmt` before committing — all code must be formatted with the default rustfmt settings
- Run `cargo clippy` and address warnings before opening a pull request
- Keep physics, simulation logic, and rendering strictly separated across their respective modules — `quadtree.rs` should never import `renderer.rs`, `sim.rs` should never import `minifb`
- Constants belong in the module that uses them, named in `SCREAMING_SNAKE_CASE`
- Commit messages follow the conventional commits format: `feat:`, `fix:`, `perf:`, `refactor:`, `docs:`

---

## Commit Message Format

```
type: short description in imperative mood

Optional longer explanation of why, not what.
```

Types used in this project:

| Type | When to use |
|------|-------------|
| `feat` | New feature or capability |
| `fix` | Bug fix |
| `perf` | Performance improvement |
| `refactor` | Code restructure without behavior change |
| `docs` | Documentation only |
| `chore` | Build system, dependencies |

---

## Pull Request Process

1. Fork the repository and create a branch named after your change: `feat/trails`, `fix/softening-blowup`
2. Make your changes with clear, focused commits
3. Ensure `cargo build`, `cargo fmt --check`, and `cargo clippy` all pass cleanly
4. Open a pull request with a description of what changed and why
5. Reference any relevant issues

---

## Physics Contributions

If your contribution touches the physics simulation, include a brief explanation of the physical principle involved and why your implementation is correct. This project values understanding over code — a change to `step()` or `compute_force()` should come with reasoning, not just working output.
