use crate::body::Body;

pub enum QuadTree {
    Empty(BoundingBox),
    Leaf {
        body: Body,
        region: BoundingBox,
    },
    Internal {
        region: BoundingBox,
        total_mass: f64,
        center_of_mass: [f64; 2],
        children: [Option<Box<QuadTree>>; 4],
    },
}

#[derive(Clone)]
pub struct BoundingBox {
    pub cx: f64,   // center x
    pub cy: f64,   // center y
    pub half: f64, // half-width (and half-height, assuming square regions)
} // half gives s in the θ criterion — s = 2 * half of the node's region

impl BoundingBox {
    pub fn quadrant(&self, index: usize) -> BoundingBox {
        // returns one of 4 sub-regions
        let quarter = self.half / 2.0;

        match index {
            0 => BoundingBox {
                cx: self.cx - quarter,
                cy: self.cy - quarter,
                half: quarter,
            },
            1 => BoundingBox {
                cx: self.cx + quarter,
                cy: self.cy - quarter,
                half: quarter,
            },
            2 => BoundingBox {
                cx: self.cx - quarter,
                cy: self.cy + quarter,
                half: quarter,
            },
            3 => BoundingBox {
                cx: self.cx + quarter,
                cy: self.cy + quarter,
                half: quarter,
            },
            _ => unreachable!(),
        }
    }
    pub fn contains(&self, body: &Body) -> bool {
        body.pos[0] >= self.cx - self.half
            && body.pos[0] < self.cx + self.half
            && body.pos[1] >= self.cy - self.half
            && body.pos[1] < self.cy + self.half
    }
    pub fn quadrant_index(&self, body: &Body) -> usize {
        let right = body.pos[0] >= self.cx; // east vs west
        let up = body.pos[1] >= self.cy; // north vs south
        match (right, up) {
            (false, false) => 0, // SW
            (true, false) => 1,  // SE
            (false, true) => 2,  // NW
            (true, true) => 3,   // NE
        }
    }
}

impl QuadTree {
    pub fn insert(&mut self, body: &Body) {
        match self {
            QuadTree::Empty(region) => {
                *self = QuadTree::Leaf {
                    body: body.clone(),
                    region: region.clone(), // carry the region forward
                };
            }
            QuadTree::Leaf {
                body: existing,
                region,
            } => {
                if !region.contains(body) {
                    return;
                }

                let mut children: [Option<Box<QuadTree>>; 4] = Default::default();

                let idx = region.quadrant_index(existing);
                if children[idx].is_some() {
                    children[idx].as_mut().unwrap().insert(existing);
                } else {
                    children[idx] = Some(Box::new(QuadTree::Empty(region.quadrant(idx))));
                    children[idx].as_mut().unwrap().insert(existing);
                }

                let idx = region.quadrant_index(body);
                if children[idx].is_some() {
                    children[idx].as_mut().unwrap().insert(body);
                } else {
                    children[idx] = Some(Box::new(QuadTree::Empty(region.quadrant(idx))));
                    children[idx].as_mut().unwrap().insert(body);
                }

                *self = QuadTree::Internal {
                    region: region.clone(),
                    total_mass: existing.mass + body.mass,
                    center_of_mass: [
                        (existing.pos[0] * existing.mass + body.pos[0] * body.mass)
                            / (existing.mass + body.mass),
                        (existing.pos[1] * existing.mass + body.pos[1] * body.mass)
                            / (existing.mass + body.mass),
                    ],
                    children,
                };
            }
            QuadTree::Internal {
                region,
                total_mass,
                center_of_mass,
                children,
            } => {
                if !region.contains(body) {
                    return; // body is outside this region
                }

                let idx = region.quadrant_index(body);
                *total_mass += body.mass;
                *center_of_mass = [
                    (center_of_mass[0] * (*total_mass - body.mass) + body.pos[0] * body.mass)
                        / *total_mass,
                    (center_of_mass[1] * (*total_mass - body.mass) + body.pos[1] * body.mass)
                        / *total_mass,
                ];

                if children[idx].is_some() {
                    // recurse
                    children[idx].as_mut().unwrap().insert(body);
                } else {
                    // initialize as Empty with correct sub-region, then recurse
                    children[idx] = Some(Box::new(QuadTree::Empty(region.quadrant(idx))));
                    children[idx].as_mut().unwrap().insert(body);
                }
            }
        }
    }

    pub fn compute_force(&self, body: &Body, theta_sq: f64, g: f64, softening_sq: f64) -> [f64; 2] {
        match self {
            QuadTree::Empty(_) => [0.0, 0.0],
            QuadTree::Leaf { body: existing, .. } => {
                if body.id == existing.id {
                    return [0.0, 0.0];
                }
                self.calculate_acceleration(body.pos, existing.pos, existing.mass, g, softening_sq)
            }
            QuadTree::Internal {
                region,
                total_mass,
                center_of_mass,
                children,
            } => {
                let dx = center_of_mass[0] - body.pos[0];
                let dy = center_of_mass[1] - body.pos[1];
                let d_sq = dx * dx + dy * dy + softening_sq;

                let s_sq = region.half * region.half * 4.0;

                // If (region_size)² < (opening_angle)² × (distance)²
                // Then: use multipole approximation (single mass)
                // Else: recurse into children (more detail)

                if s_sq < theta_sq * d_sq {
                    self.calculate_acceleration(
                        body.pos,
                        *center_of_mass,
                        *total_mass,
                        g,
                        softening_sq,
                    )
                } else {
                    let mut total_force = [0.0_f64; 2];
                    for child in children.iter().flatten() {
                        let f = child.compute_force(body, theta_sq, g, softening_sq);
                        total_force[0] += f[0];
                        total_force[1] += f[1];
                    }
                    total_force
                }
            }
        }
    }
    fn calculate_acceleration(
        &self,
        pos_a: [f64; 2],
        pos_b: [f64; 2],
        mass_b: f64,
        g: f64,
        softening_sq: f64,
    ) -> [f64; 2] {
        let dx = pos_b[0] - pos_a[0];
        let dy = pos_b[1] - pos_a[1];

        let dist_sq = dx * dx + dy * dy + softening_sq;
        let dist_cu = dist_sq * dist_sq.sqrt(); // Pythagoras: dx² + dy²
        let mag = (g * mass_b) / dist_cu;

        [dx * mag, dy * mag]
    }
}
