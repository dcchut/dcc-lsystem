use std::collections::HashMap;

use image::{ImageBuffer, Rgb};
use pbr::ProgressBar;

use crate::image::{draw_line_mut, fill_mut};
use crate::turtle::TurtleContainer;
use crate::{ArenaId, LSystem};

pub trait Renderer<S> {
    /// The output of the rendering operation
    type Output;

    /// Renders the system, consuming the renderer.
    fn render(self, system: &LSystem, options: &S) -> Self::Output;
}

pub struct ImageRendererOptions {
    pub padding: u32,
    pub thickness: f32,
    pub fill_color: Rgb<u8>,
    pub line_color: Rgb<u8>,
}

impl ImageRendererOptions {
    pub fn new(padding: u32, thickness: f32, fill_color: Rgb<u8>, line_color: Rgb<u8>) -> Self {
        Self {
            padding,
            thickness,
            fill_color,
            line_color,
        }
    }

    pub fn padding(&self) -> u32 {
        self.padding
    }

    pub fn thickness(&self) -> f32 {
        self.thickness
    }

    pub fn fill_color(&self) -> Rgb<u8> {
        self.fill_color
    }

    pub fn line_color(&self) -> Rgb<u8> {
        self.line_color
    }
}

pub struct VideoRendererOptions {
    pub filename: String,
    pub fps: usize,
    pub skip_by: usize,
    pub padding: u32,
    pub thickness: f32,
    pub fill_color: Rgb<u8>,
    pub line_color: Rgb<u8>,
    pub progress_bar: bool,
}

impl VideoRendererOptions {
    pub fn new<S: Into<String>>(
        filename: S,
        fps: usize,
        skip_by: usize,
        padding: u32,
        thickness: f32,
        fill_color: Rgb<u8>,
        line_color: Rgb<u8>,
        progress_bar: bool,
    ) -> Self {
        Self {
            filename: filename.into(),
            fps,
            skip_by,
            padding,
            thickness,
            fill_color,
            line_color,
            progress_bar,
        }
    }
}

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
}

impl<Q: TurtleContainer> Renderer<VideoRendererOptions> for TurtleRenderer<Q> {
    type Output = ();

    // ffmpeg -r 24 -f image2 -i frame-%8d.png -vcodec libx264 -crf 20 -pix_fmt yuv420p output.mp4
    fn render(mut self, system: &LSystem, options: &VideoRendererOptions) -> Self::Output {
        // Setup our state machine based on the system state
        self.compute(system.get_state());

        let (turtle_width, turtle_height, min_x, min_y) = self.state.inner().inner().bounds();

        // We add some padding to the width reported by our turtle to make
        // our final image look a little nicer.
        let width = 2 * options.padding + turtle_width;
        let height = 2 * options.padding + turtle_height;

        let mut buffer = ImageBuffer::new(width, height);
        fill_mut(&mut buffer, options.fill_color);

        let mut files = Vec::new();

        // Helper functions for converting between the coordinate system used
        // by the image crate and our coordinate system.  These functions also
        // take care of the padding for us.
        let xp = |x: i32| -> u32 { (x - min_x + options.padding as i32) as u32 };

        let yp = |y: i32| -> u32 {
            (i64::from(height) - i64::from(y - min_y + options.padding as i32)) as u32
        };

        let mut frame_counter = 0;
        let mut absolute_frame_counter = 0;
        let total_frame_counter = self.state.inner().inner().lines().len();

        let mut pb = if options.progress_bar {
            Some(ProgressBar::new(total_frame_counter as u64))
        } else {
            None
        };

        let dir = tempfile::tempdir().unwrap();
        let mut workers = Vec::new();

        for (x1, y1, x2, y2) in self.state.inner().inner().lines() {
            draw_line_mut(
                &mut buffer,
                xp(*x1),
                yp(*y1),
                xp(*x2),
                yp(*y2),
                options.thickness,
                options.line_color,
            );

            if options.progress_bar {
                pb.as_mut().unwrap().inc();
            }

            if options.skip_by == 0 || frame_counter % options.skip_by == 0 {
                // TODO: estimate number of digits we need (for correct padding of filenames)
                // for the moment we just use 8.
                let filename = dir
                    .path()
                    .join(format!("frame-{:08}.png", absolute_frame_counter));
                absolute_frame_counter += 1;
                files.push(filename.clone());

                let local_buffer = buffer.clone();

                // spawn a thread to do this work
                workers.push(std::thread::spawn(move || {
                    local_buffer.save(filename).unwrap();
                }));
            }
            frame_counter += 1;
        }

        for child in workers {
            child.join().expect("Failure");
            if options.progress_bar {
                pb.as_mut().unwrap().inc();
            }
        }

        if options.progress_bar {
            pb.unwrap().finish();
        }

        let mut args = Vec::new();
        args.push(String::from("-o"));
        args.push(String::from(&options.filename));
        args.push(String::from("--fps"));
        args.push(options.fps.to_string());
        args.push(format!("{}/frame*.png", dir.path().display()));

        let _output = std::process::Command::new("gifski")
            .args(&args)
            .output()
            .expect("failed to execute process");

        // Now delete the temporary files
        drop(dir);
    }
}

impl<Q: TurtleContainer> Renderer<ImageRendererOptions> for TurtleRenderer<Q> {
    type Output = ImageBuffer<Rgb<u8>, Vec<u8>>;

    fn render(mut self, system: &LSystem, options: &ImageRendererOptions) -> Self::Output {
        // Setup our state machine based on the LSystem state
        self.compute(system.get_state());

        let (turtle_width, turtle_height, min_x, min_y) = self.state.inner().inner().bounds();
        let width = 2 * options.padding + turtle_width;
        let height = 2 * options.padding + turtle_height;

        let mut buffer = ImageBuffer::new(width, height);
        fill_mut(&mut buffer, options.fill_color);

        // Helper functions for converting between the coordinate system used
        // by the image crate and our coordinate system.  These functions also
        // take care of the padding for us.
        let xp = |x: i32| -> u32 { (x - min_x + options.padding as i32) as u32 };

        let yp = |y: i32| -> u32 {
            (i64::from(height) - i64::from(y - min_y + options.padding as i32)) as u32
        };

        for (x1, y1, x2, y2) in self.state.inner().inner().lines() {
            draw_line_mut(
                &mut buffer,
                xp(*x1),
                yp(*y1),
                xp(*x2),
                yp(*y2),
                options.thickness,
                options.line_color,
            );
        }

        buffer
    }
}
