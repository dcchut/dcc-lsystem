use std::f32::consts::FRAC_PI_3;

use crate::turtle::{BaseTurtle, MovingTurtle, Stack};

pub struct Lattice {
    x_direction: (f32, f32),
    y_direction: (f32, f32),
}

impl Lattice {
    pub fn new(x_direction: (f32, f32), y_direction: (f32, f32)) -> Self {
        Self {
            x_direction,
            y_direction,
        }
    }

    pub fn point(&self, x: i32, y: i32) -> (f32, f32) {
        (
            self.x_direction.0 * (x as f32) + self.y_direction.0 * (y as f32),
            self.x_direction.1 * (x as f32) + self.y_direction.1 * (y as f32),
        )
    }
}

pub struct LatticeTurtle {
    inner: BaseTurtle,
    lattice: Lattice,
    x: i32,
    y: i32,
    stack: Vec<(i32, i32)>,
}

impl LatticeTurtle {
    pub fn new(lattice: Lattice) -> Self {
        Self {
            inner: BaseTurtle::new(),
            lattice,
            x: 0,
            y: 0,
            stack: Vec::new(),
        }
    }

    /// Creates a grid-based latttice
    pub fn grid() -> Self {
        Self::new(Lattice::new((1.0, 0.0), (0.0, 1.0)))
    }

    /// Creates an equiangular lattice.
    pub fn equiangular() -> Self {
        Self::by_angle(FRAC_PI_3)
    }

    /// Creates a turtle based on a lattice where the angle between the two basis vectors
    /// is `angle` (in radians).
    pub fn by_angle(angle: f32) -> Self {
        Self::new(Lattice::new((1.0, 0.0), (angle.cos(), angle.sin())))
    }
}

impl MovingTurtle for LatticeTurtle {
    type Item = (i32, i32);

    fn inner(&self) -> &BaseTurtle {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut BaseTurtle {
        &mut self.inner
    }

    fn forward(&mut self, distance: (i32, i32)) {
        // set the turtles initial position
        let (rx, ry) = self.lattice.point(self.x, self.y);
        self.inner.set_position(rx as i32, ry as i32);

        // compute the delta between old and new position
        let (dx, dy) = distance;
        let (rdx, rdy) = self.lattice.point(dx, dy);
        self.inner.delta_move(rdx as i32, rdy as i32);

        // update our coordinates wrt. the lattice
        self.x += dx;
        self.y += dy;
    }
}

impl Stack for LatticeTurtle {
    fn push(&mut self) {
        self.stack.push((self.x, self.y));
    }

    fn pop(&mut self) {
        let (x, y) = self.stack.pop().expect("Called pop on empty stack");
        self.x = x;
        self.y = y;
    }
}
