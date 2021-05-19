use std::collections::HashMap;

use crate::turtle::TurtleContainer;
use crate::{ArenaId, LSystem};

#[cfg(feature = "image_renderer")]
pub use crate::image_renderer::ImageRendererOptionsBuilder;

#[cfg(feature = "image_renderer")]
pub use crate::image_renderer::VideoRendererOptionsBuilder;

pub trait Renderer<S> {
    /// The output of the rendering operation
    type Output;

    /// Renders the system, consuming the renderer.
    fn render(self, system: &LSystem, options: &S) -> Self::Output;
}

pub struct TurtleRenderer<Q: TurtleContainer> {
    pub(crate) state: Q,
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

    pub(crate) fn compute(&mut self, system_state: &[ArenaId]) {
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
}

/// A version of ImageRendererOptions but intended for data only rendering (no image).
/// For symmetry reasons and future proofing, it is implemented as an empty struct.
#[derive(Default)]
pub struct DataRendererOptions {}

impl<Q: TurtleContainer> Renderer<DataRendererOptions> for TurtleRenderer<Q> {
    type Output = Vec<(f64, f64, f64, f64)>;

    fn render(mut self, system: &LSystem, _options: &DataRendererOptions) -> Self::Output {
        // Setup our state machine based on the LSystem state
        self.compute(system.get_state());

        // TODO: find a way to move lines() instead of cloning it with to_vec()
        self.state.inner().inner().lines().to_vec()
    }
}
