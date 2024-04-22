use std::{
    io::{self, Write},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::{self, JoinHandle},
    time::{Duration, Instant},
    vec,
};

use terminal_size::terminal_size;

use crate::{
    colors::Colors,
    misc::{generic_dimension_calc, BufferedConsole},
    pixel::Pixel,
    renderer_object_style::{AlignmentX, AlignmentY, RendererObjectStyle},
    renderer_object_wrapper::RendererObject,
};

pub struct Renderer {
    previous_buffer: Vec<Vec<Pixel>>,
    width: i64,
    height: i64,
    padding: i64,
    object: Option<RendererObject>,
    console: BufferedConsole,
    drawing: bool,

    pub disable_output: bool,
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {
            previous_buffer: Vec::new(),
            width: 0,
            height: 0,
            object: None,
            padding: 5,
            console: BufferedConsole::new(),
            drawing: false,
            disable_output: false,
        }
    }

    pub fn hide_cursor(&self) {
        let mut stdout = io::stdout().lock();
        stdout.write_all(b"\x1b[?25l").unwrap();
        stdout.flush().unwrap();
    }

    pub fn show_cursor(&self) {
        let mut stdout = io::stdout().lock();
        stdout.write_all(b"\x1b[?25h").unwrap();
        stdout.flush().unwrap();
    }

    pub fn set_object(&mut self, renderer_object: Option<RendererObject>) {
        if let Some(obj) = renderer_object {
            self.object = Some(obj.clone());
        } else {
            self.object = None
        }
    }

    pub fn get_object(&self) -> Option<RendererObject> {
        self.object.clone()
    }

    pub fn set_padding(&mut self, buffer: i64) {
        self.padding = buffer;
    }

    ///returns the actual frame time
    pub fn draw(&mut self, force_update: bool) {
        if self.drawing {
            return;
        }
        self.drawing = true;
        if let Some(object_wrapper) = &mut self.object {
            {
                let (terminal_width, terminal_height) = match terminal_size() {
                    Some(val) => (val.0 .0 as i64, val.1 .0 as i64),
                    None => (0, 0),
                };
                if self.width != terminal_width || self.height != terminal_height {
                    self.width = terminal_width;
                    self.height = terminal_height;
                    self.previous_buffer = vec::from_elem(
                        vec::from_elem(
                            Pixel {
                                value: '\0',
                                background: Colors::INVALID,
                                foreground: Colors::INVALID,
                            },
                            self.width as usize,
                        ),
                        self.height as usize,
                    );
                }
                let mut object = object_wrapper.value.try_write().unwrap();

                object.update_value();

                object.process_geometry(
                    self.width,
                    self.height,
                    self.width,
                    self.height,
                    self.padding,
                );

                let object_x: i64 = generic_dimension_calc(
                    &object.x,
                    self.width,
                    self.height,
                    self.width,
                    self.height,
                    true,
                );
                let object_y: i64 = generic_dimension_calc(
                    &object.y,
                    self.width,
                    self.height,
                    self.width,
                    self.height,
                    false,
                );
                let (object_width, object_height) =
                    (object.calculated_width, object.calculated_height);

                let style: &RendererObjectStyle = &object.style;
                let alignment_offset_x: i64 = if let Some(style) = style.external_alignment_x {
                    match style {
                        AlignmentX::Left => 0,
                        AlignmentX::Center => self.width / 2 - object_width / 2,
                        AlignmentX::Right => self.width - object_width,
                    }
                } else {
                    0
                };
                let alignment_offset_y: i64 = if let Some(style) = style.external_alignment_y {
                    match style {
                        AlignmentY::Top => 0,
                        AlignmentY::Center => self.height / 2 - object_height / 2,
                        AlignmentY::Bottom => self.height - object_height,
                    }
                } else {
                    0
                };
                let current_buffer: &Vec<Vec<Pixel>> =
                    object.get_buffer(alignment_offset_x, alignment_offset_y, self.padding);

                let start_x: i64 = (alignment_offset_x + object_x).max(0).min(self.width);
                let end_x: i64 = (alignment_offset_x + object_x + object_width as i64)
                    .max(0)
                    .min(self.width);
                let start_y: i64 = (alignment_offset_y + object_y).max(0).min(self.height);
                let end_y: i64 = (alignment_offset_y + object_y + object_height as i64)
                    .max(0)
                    .min(self.height);

                let mut last_i: i64 = -1;
                for i in start_y..end_y {
                    let mut last_j: i64 = -1;
                    for j in start_x..end_x {
                        let padding: i64 = (self.width - terminal_width).max(0).min(self.padding);
                        let current_pixel: Pixel = current_buffer[(i - start_y + padding) as usize]
                            [(j - start_x + padding) as usize]
                            .clone();
                        if self.previous_buffer[i as usize][j as usize] != current_pixel
                            || force_update
                        {
                            if j != last_j + 1 || i != last_i {
                                self.console.set_cursor_position(j, i);
                                last_i = i;
                                last_j = j;
                            }
                            self.previous_buffer[i as usize][j as usize] = current_pixel.clone();
                            self.console.print(current_pixel);
                        }
                    }
                }

                self.console.set_cursor_position(end_x, end_y);
                if self.disable_output {
                    self.console.clear();
                } else {
                    self.console.flush();
                }
            }
        }
        self.drawing = false;
    }

    pub fn run(self, target_frame_time: Duration) -> RunningRenderer {
        let running = Arc::new(AtomicBool::new(true));
        let mut running_renderer = RunningRenderer {
            thread: None,
            running: running.clone(),
        };
        running_renderer.thread = Some(thread::spawn(move || {
            let mut renderer = self;
            let target_frame_time = target_frame_time;
            let running = running;

            renderer.draw(true);

            while running.load(Ordering::Relaxed) {
                let start_time = Instant::now();
                renderer.draw(false);
                let elapsed_time = start_time.elapsed();
                if elapsed_time < target_frame_time {
                    thread::sleep(target_frame_time - elapsed_time);
                }
            }
            renderer
        }));
        running_renderer
    }

    pub(crate) fn _debug_run(self) -> _RunningRendererDebug {
        let running = Arc::new(AtomicBool::new(true));
        let mut running_renderer = _RunningRendererDebug {
            thread: None,
            running: running.clone(),
        };
        running_renderer.thread = Some(thread::spawn(move || {
            let mut renderer = self;
            let running = running;

            renderer.draw(true);

            let mut debug_values = _RunningRendererDebugValues {
                frame_count: 0,
                min_frames_per_second: i64::MAX,
                max_frames_per_second: 0,
                min_frames_per_fourth: i64::MAX,
                max_frames_per_fourth: 0,
                first_frame_time: Duration::ZERO,
                min_frame_time: Duration::MAX,
                max_frame_time: Duration::ZERO,
            };

            let mut first_frame = true;
            let mut time_step_second = Instant::now();
            let mut time_step_fourth = Instant::now();
            let mut last_frames_per_second = 0;
            let mut last_frames_per_fourth = 0;

            while running.load(Ordering::Relaxed) {
                let start_time = Instant::now();
                debug_values.frame_count += 1;
                renderer.draw(false);
                let elapsed_time = start_time.elapsed();

                if first_frame {
                    debug_values.first_frame_time = elapsed_time;
                    first_frame = false;
                } else {
                    debug_values.min_frame_time = debug_values.min_frame_time.min(elapsed_time);
                    debug_values.max_frame_time = debug_values.max_frame_time.max(elapsed_time);
                }

                if time_step_second.elapsed() > Duration::from_millis(1000) {
                    time_step_second = Instant::now();
                    let diff_frames_per_second = debug_values.frame_count - last_frames_per_second;
                    last_frames_per_second = debug_values.frame_count;
                    debug_values.min_frames_per_second = debug_values
                        .min_frames_per_second
                        .min(diff_frames_per_second);
                    debug_values.max_frames_per_second = debug_values
                        .max_frames_per_second
                        .max(diff_frames_per_second);
                }

                if time_step_fourth.elapsed() > Duration::from_millis(250) {
                    time_step_fourth = Instant::now();
                    let diff_frames_per_fourth = debug_values.frame_count - last_frames_per_fourth;
                    last_frames_per_fourth = debug_values.frame_count;
                    debug_values.min_frames_per_fourth = debug_values
                        .min_frames_per_fourth
                        .min(diff_frames_per_fourth);
                    debug_values.max_frames_per_fourth = debug_values
                        .max_frames_per_fourth
                        .max(diff_frames_per_fourth);
                }
            }
            debug_values
        }));
        running_renderer
    }
}

pub struct RunningRenderer {
    thread: Option<JoinHandle<Renderer>>,
    running: Arc<AtomicBool>,
}

impl RunningRenderer {
    pub fn stop(self) -> Renderer {
        self.running.store(false, Ordering::Relaxed);
        self.thread.unwrap().join().unwrap()
    }
}

pub(crate) struct _RunningRendererDebugValues {
    pub(crate) frame_count: i64,
    pub(crate) min_frames_per_second: i64,
    pub(crate) max_frames_per_second: i64,
    pub(crate) min_frames_per_fourth: i64,
    pub(crate) max_frames_per_fourth: i64,
    pub(crate) first_frame_time: Duration,
    pub(crate) min_frame_time: Duration,
    pub(crate) max_frame_time: Duration,
}

pub(crate) struct _RunningRendererDebug {
    thread: Option<JoinHandle<_RunningRendererDebugValues>>,
    running: Arc<AtomicBool>,
}

impl _RunningRendererDebug {
    pub(crate) fn _stop(self) -> _RunningRendererDebugValues {
        self.running.store(false, Ordering::Relaxed);
        self.thread.unwrap().join().unwrap()
    }
}
