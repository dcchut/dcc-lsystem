use crate::turtle::{BaseTurtle, Turtle};

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
}

impl LatticeTurtle {
    pub fn new(lattice: Lattice) -> Self {
        Self {
            inner: BaseTurtle::new(),
            lattice,
            x: 0,
            y: 0,
        }
    }

    pub fn grid() -> Self {
        Self::new(Lattice::new((1.0, 0.0), (0.0, 1.0)))
    }
}

impl Turtle<(i32, i32)> for LatticeTurtle {
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
