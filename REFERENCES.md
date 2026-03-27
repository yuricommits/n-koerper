# References

Resources consulted during the design and implementation of n-körper.

---

## Foundational Physics

**Newton, I. (1687)**
*Philosophiæ Naturalis Principia Mathematica*
The original formulation of universal gravitation and the three laws of motion.
https://en.wikipedia.org/wiki/Philosophiæ_Naturalis_Principia_Mathematica

**Poincaré, H. (1890)**
*Sur le problème des trois corps et les équations de la dynamique*
Proof that the three-body problem has no general closed-form solution and the first mathematical treatment of chaos.
https://en.wikipedia.org/wiki/Henri_Poincaré

---

## The Barnes-Hut Algorithm

**Barnes, J. & Hut, P. (1986)**
*A hierarchical O(N log N) force-calculation algorithm*
Nature, 324, 446–449. The original paper introducing the quadtree-based approximation for n-body force computation.
https://doi.org/10.1038/324446a0

**Computer Science Department at Princeton University**
*COS 126 Programming Assignment: Barnes-Hut Galaxy Simulator*
https://www.cs.princeton.edu/courses/archive/fall03/cs126/assignments/barnes-hut.html
This assignment was developed by Tom Ventimiglia and Kevin Wayne.

**Wikipedia - Barnes-Hut simulation**
Accessible overview of the algorithm, quadtree construction, and the θ criterion with diagrams.
https://en.wikipedia.org/wiki/Barnes–Hut_simulation

---

## Numerical Integration

**Hairer, E., Lubich, C., Wanner, G.**
*Geometric Numerical Integration - Structure-Preserving Algorithms for Ordinary Differential Equations*
Reference for symplectic integrators and energy conservation properties of the leapfrog/Störmer-Verlet method.
https://link.springer.com/book/10.1007/3-540-30666-8

**Wikipedia - Symplectic Euler method**
Overview of the semi-implicit Euler integrator used in this simulator.
https://en.wikipedia.org/wiki/Semi-implicit_Euler_method

**Hockney, R.W. & Eastwood, J.W. (1988)**
*Computer Simulation Using Particles*
Original treatment of the leapfrog/KDK integrator in the context of particle simulations.
https://www.taylorfrancis.com/books/mono/10.1201/9780367806934

**Wikipedia - Leapfrog integration**
Clear derivation of the KDK and DKD forms and their symplectic properties.
https://en.wikipedia.org/wiki/Leapfrog_integration

---

## N-Body Simulation

**Aarseth, S. J. (2003)**
*Gravitational N-Body Simulations: Tools and Algorithms*
Cambridge University Press. Comprehensive treatment of n-body methods including softening, time step control, and regularization.
https://www.cambridge.org/core/books/gravitational-nbody-simulations/

**Wikipedia - N-body simulation**
General overview of n-body methods, complexity, and applications in astrophysics.
https://en.wikipedia.org/wiki/N-body_simulation

**Wikipedia - Gravitational N-body problem**
https://en.wikipedia.org/wiki/Gravitational_N-body_problem

---

## Softening

**Merritt, D. (1996)**
*Optimal Smoothing for N-Body Codes*
Discussion of how to choose the softening parameter ε to balance accuracy against numerical stability.
https://iopscience.iop.org/article/10.1086/133493

---

## Orbital Mechanics

**Wikipedia - Orbital speed**
Derivation of circular orbital velocity `v = sqrt(G * M / r)` used to initialize orbiting bodies.
https://en.wikipedia.org/wiki/Orbital_speed

**Wikipedia - Vis-viva equation**
General relationship between orbital speed and position for any conic orbit.
https://en.wikipedia.org/wiki/Vis-viva_equation

---

## Chaos Theory

**Wikipedia - Chaos theory**
Background on sensitive dependence on initial conditions - the mathematical foundation of why the three-body problem is non-integrable.
https://en.wikipedia.org/wiki/Chaos_theory

**Wikipedia - Three-body problem**
History, special solutions (figure-eight orbit, Lagrange points), and the general non-integrability result.
https://en.wikipedia.org/wiki/Three-body_problem
