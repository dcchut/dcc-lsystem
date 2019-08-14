use std::collections::HashMap;

use image::{ImageBuffer, Rgb};

use crate::image::{draw_line_mut, fill_mut};
use crate::turtle::TurtleContainer;
use crate::{ArenaId, LSystem};

pub struct TurtleRenderer<Q: TurtleContainer> {
    state: Q,
    state_actions: HashMap<ArenaId, Box<dyn Fn(&mut Q)>>,
    aliases: HashMap<ArenaId, ArenaId>,
}

impl<Q: TurtleContainer> TurtleRenderer<Q> {
    pub fn new(state: Q) -> Self {
        Self {
            state,
            state_actions: HashMap::new(),
            aliases: HashMap::new(),
        }
    }

    pub fn register<F: 'static + Fn(&mut Q)>(&mut self, arena_id: ArenaId, modifier: F) {
        self.aliases.insert(arena_id, arena_id);
        self.state_actions.insert(arena_id, Box::from(modifier));
    }

    pub fn register_multiple<F: 'static + Fn(&mut Q)>(
        &mut self,
        arena_ids: &[ArenaId],
        modifier: F,
    ) {
        if let Some(id) = arena_ids.first() {
            // Alias each ID in the slice to the first one
            for aliased_id in arena_ids.iter() {
                self.aliases.insert(*aliased_id, *id);
            }

            // Register the mutator for the first id
            self.register(*id, modifier);
        }
    }

    fn compute(&mut self, system_state: &[ArenaId]) {
        for arena_id in system_state {
            if self.aliases.contains_key(arena_id) {
                // Find the arena id that the provided one points to
                let alias = self.aliases[arena_id];

                // If there is a function corresponding to the alias,
                // apply it
                if self.state_actions.contains_key(&alias) {
                    self.state_actions[&alias](&mut self.state);
                }
            }
        }
    }

    pub fn render(
        &mut self,
        system: &LSystem,
        padding: u32,
        thickness: f32,
        fill_color: Rgb<u8>,
        line_color: Rgb<u8>,
    ) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        // First modify the internal state using the system state
        self.compute(system.get_state());

        let (turtle_width, turtle_height, min_x, min_y) = self.state.inner().inner().bounds();

        // We add some padding to the width reported by our turtle to make
        // our final image look a little nicer.
        let width = 2 * padding + turtle_width;
        let height = 2 * padding + turtle_height;

        let mut buffer = ImageBuffer::new(width, height);
        fill_mut(&mut buffer, fill_color);

        // Helper functions for converting between the coordinate system used
        // by the image crate and our coordinate system.  These functions also
        // take care of the padding for us.
        let xp = |x: i32| -> u32 { (x - min_x + padding as i32) as u32 };

        let yp =
            |y: i32| -> u32 { (i64::from(height) - i64::from(y - min_y + padding as i32)) as u32 };

        for (x1, y1, x2, y2) in self.state.inner().inner().lines() {
            draw_line_mut(
                &mut buffer,
                xp(*x1),
                yp(*y1),
                xp(*x2),
                yp(*y2),
                thickness,
                line_color,
            );
        }

        buffer
    }
}
