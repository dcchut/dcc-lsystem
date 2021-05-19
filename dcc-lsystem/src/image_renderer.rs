use crate::dcc_lsystem::LSystem;
use crate::image::{draw_line_mut, fill_mut};
use crate::renderer::{Renderer, TurtleRenderer};
use crate::turtle::TurtleContainer;
use crate::LSystemError;
use gifski::progress::{NoProgress, ProgressReporter};
use gifski::{CatResult, Collector, Repeat};
use image::{ImageBuffer, Rgb};
use mtpng::encoder::{Encoder, Options};
use mtpng::{ColorType, Header};
use pbr::ProgressBar;
use std::fs::File;
use std::io::Stdout;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;

pub struct ImageRendererOptionsBuilder {
    options: ImageRendererOptions,
}

impl ImageRendererOptionsBuilder {
    pub fn new() -> Self {
        Self {
            options: ImageRendererOptions {
                padding: 20,
                thickness: 15.0,
                fill_color: Rgb([255, 255, 255]),
                line_color: Rgb([0, 0, 0]),
            },
        }
    }

    pub fn padding(&mut self, padding: u32) -> &mut Self {
        self.options.padding = padding;
        self
    }

    pub fn thickness(&mut self, thickness: f64) -> &mut Self {
        self.options.thickness = thickness;
        self
    }

    pub fn fill_color(&mut self, fill_color: Rgb<u8>) -> &mut Self {
        self.options.fill_color = fill_color;
        self
    }

    pub fn line_color(&mut self, line_color: Rgb<u8>) -> &mut Self {
        self.options.line_color = line_color;
        self
    }

    pub fn build(&mut self) -> ImageRendererOptions {
        self.options.clone()
    }
}

impl Default for ImageRendererOptionsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone)]
pub struct ImageRendererOptions {
    padding: u32,
    thickness: f64,
    fill_color: Rgb<u8>,
    line_color: Rgb<u8>,
}

impl ImageRendererOptions {
    pub fn padding(&self) -> u32 {
        self.padding
    }

    pub fn thickness(&self) -> f64 {
        self.thickness
    }

    pub fn fill_color(&self) -> Rgb<u8> {
        self.fill_color
    }

    pub fn line_color(&self) -> Rgb<u8> {
        self.line_color
    }
}

pub struct VideoRendererOptionsBuilder {
    options: VideoRendererOptions,
}

impl VideoRendererOptionsBuilder {
    pub fn new() -> Self {
        Self {
            options: VideoRendererOptions {
                filename: String::from("render.gif"),
                fps: 20,
                skip_by: 0,
                padding: 20,
                thickness: 15.0,
                fill_color: Rgb([255, 255, 255]),
                line_color: Rgb([0, 0, 0]),
                progress_bar: false,
            },
        }
    }

    pub fn filename<T: Into<String>>(&mut self, filename: T) -> &mut Self {
        self.options.filename = filename.into();
        self
    }

    pub fn fps(&mut self, fps: usize) -> &mut Self {
        self.options.fps = fps;
        self
    }

    pub fn skip_by(&mut self, skip_by: usize) -> &mut Self {
        self.options.skip_by = skip_by;
        self
    }

    pub fn padding(&mut self, padding: u32) -> &mut Self {
        self.options.padding = padding;
        self
    }

    pub fn thickness(&mut self, thickness: f64) -> &mut Self {
        self.options.thickness = thickness;
        self
    }

    pub fn fill_color(&mut self, fill_color: Rgb<u8>) -> &mut Self {
        self.options.fill_color = fill_color;
        self
    }

    pub fn line_color(&mut self, line_color: Rgb<u8>) -> &mut Self {
        self.options.line_color = line_color;
        self
    }

    pub fn progress_bar(&mut self, progress_bar: bool) -> &mut Self {
        self.options.progress_bar = progress_bar;
        self
    }

    pub fn build(&mut self) -> VideoRendererOptions {
        self.options.clone()
    }
}

impl Default for VideoRendererOptionsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone)]
pub struct VideoRendererOptions {
    filename: String,
    fps: usize,
    skip_by: usize,
    padding: u32,
    thickness: f64,
    fill_color: Rgb<u8>,
    line_color: Rgb<u8>,
    progress_bar: bool,
}

impl VideoRendererOptions {
    pub fn filename(&self) -> &String {
        &self.filename
    }

    pub fn fps(&self) -> usize {
        self.fps
    }

    pub fn skip_by(&self) -> usize {
        self.skip_by
    }

    pub fn padding(&self) -> u32 {
        self.padding
    }

    pub fn thickness(&self) -> f64 {
        self.thickness
    }

    pub fn fill_color(&self) -> Rgb<u8> {
        self.fill_color
    }

    pub fn line_color(&self) -> Rgb<u8> {
        self.line_color
    }

    pub fn progress_bar(&self) -> bool {
        self.progress_bar
    }
}

struct Lodecoder {
    frames: Vec<PathBuf>,
    fps: usize,
}

impl Lodecoder {
    pub fn new(frames: Vec<PathBuf>, fps: usize) -> Self {
        Self { frames, fps }
    }

    fn total_frames(&self) -> u64 {
        self.frames.len() as u64
    }

    fn collect(&mut self, mut dest: Collector) -> CatResult<()> {
        for (i, frame) in self.frames.drain(..).enumerate() {
            dest.add_frame_png_file(i, frame, i as f64 / self.fps as f64)?;
        }
        Ok(())
    }
}

impl<Q: TurtleContainer> Renderer<VideoRendererOptions> for TurtleRenderer<Q> {
    type Output = Result<(), LSystemError>;

    fn render(mut self, system: &LSystem, options: &VideoRendererOptions) -> Self::Output {
        // Setup our state machine based on the system state
        self.compute(system.get_state());

        let (turtle_width, turtle_height, min_x, min_y) = self.state.inner().inner().bounds();

        let padding = options.padding as f64;

        // We add some padding to the width reported by our turtle to make
        // our final image look a little nicer.
        let width = (2.0 * padding) + turtle_width;
        let height = (2.0 * padding) + turtle_height;

        let mut buffer = ImageBuffer::new(width.ceil() as u32, height.ceil() as u32);
        fill_mut(&mut buffer, options.fill_color);

        let mut files = Vec::new();

        // Helper functions for converting between the coordinate system used
        // by the image crate and our coordinate system.  These functions also
        // take care of the padding for us.
        let xp = |x: f64| -> f64 { x - min_x + padding };

        let yp = |y: f64| -> f64 { height - (y - min_y + padding) };

        let mut absolute_frame_counter = 0;
        let total_frame_counter = self.state.inner().inner().lines().len();

        let mut pb = if options.progress_bar {
            Some(ProgressBar::new(total_frame_counter as u64))
        } else {
            None
        };

        let dir = tempfile::tempdir()?;
        let mut workers = Vec::new();

        for (frame_counter, (x1, y1, x2, y2)) in
            self.state.inner().inner().lines().iter().enumerate()
        {
            draw_line_mut(
                &mut buffer,
                xp(*x1),
                yp(*y1),
                xp(*x2),
                yp(*y2),
                options.thickness,
                options.line_color,
            );

            if let Some(pb) = pb.as_mut() {
                pb.inc();
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
                workers.push(std::thread::spawn(move || -> Result<(), LSystemError> {
                    save_png(&local_buffer, filename.as_path())
                }));
            }
        }

        for child in workers {
            child
                .join()
                .map_err(|_| LSystemError::RenderError("failure in worker thread"))??;

            if let Some(pb) = pb.as_mut() {
                pb.inc();
            }
        }

        if let Some(pb) = pb.as_mut() {
            pb.finish();
        }

        let settings = gifski::Settings {
            width: None,
            height: None,
            quality: 100,
            fast: false,
            repeat: Repeat::Infinite,
        };

        let mut decoder = Box::new(Lodecoder::new(files, options.fps));

        let mut progress: Box<dyn ProgressReporter> = if !options.progress_bar {
            Box::new(NoProgress {})
        } else {
            let mut pb: ProgressBar<Stdout> = ProgressBar::new(decoder.total_frames());
            pb.set_max_refresh_rate(Some(Duration::from_millis(250)));
            Box::new(pb)
        };

        let (collector, writer) = gifski::new(settings)?;
        let decode_thread = thread::spawn(move || decoder.collect(collector));

        let file = File::create(&options.filename)?;
        writer.write(file, &mut *progress)?;
        let _ = decode_thread
            .join()
            .map_err(|_| LSystemError::RenderError("failure in decode thread"))?;
        progress.done(&format!("Output written to {}", options.filename));

        // Now delete the temporary files
        drop(dir);

        Ok(())
    }
}

impl<Q: TurtleContainer> Renderer<ImageRendererOptions> for TurtleRenderer<Q> {
    type Output = ImageBuffer<Rgb<u8>, Vec<u8>>;

    fn render(mut self, system: &LSystem, options: &ImageRendererOptions) -> Self::Output {
        // Setup our state machine based on the LSystem state
        self.compute(system.get_state());

        let (turtle_width, turtle_height, min_x, min_y) = self.state.inner().inner().bounds();

        let padding = options.padding as f64;

        let width = 2.0 * padding + turtle_width;
        let height = 2.0 * padding + turtle_height;

        let buffer_width = width.ceil() as u32;
        let buffer_height = height.ceil() as u32;

        let mut buffer = ImageBuffer::new(buffer_width, buffer_height);
        fill_mut(&mut buffer, options.fill_color);

        // Helper functions for converting between the coordinate system used
        // by the image crate and our coordinate system.  These functions also
        // take care of the padding for us.
        let xp = |x: f64| -> f64 { x - min_x + padding };
        let yp = |y: f64| -> f64 { height - (y - min_y + padding) };

        // Determine the pixels we want to draw
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

/// Convenience function for saving image renderer output.  This uses the [`mtpng`] crate which
/// is significantly faster than calling [`image::ImageBuffer::save`] directly.
pub fn save_png(buffer: &ImageBuffer<Rgb<u8>, Vec<u8>>, path: &Path) -> Result<(), LSystemError> {
    let file = File::create(path)?;

    let options = Options::new();
    let mut encoder = Encoder::new(file, &options);
    let mut header = Header::new();
    header.set_size(buffer.width(), buffer.height())?;
    header.set_color(ColorType::Truecolor, 8)?;
    encoder.write_header(&header)?;
    encoder.write_image_rows(buffer.as_raw())?;
    encoder.finish()?;

    Ok(())
}
