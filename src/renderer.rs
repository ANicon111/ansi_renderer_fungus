use std::{
    io::{self, Write},
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

    pub fn set_object(&mut self, renderer_object: Option<&RendererObject>) {
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
        if let Some(object) = &mut self.object {
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

                object.process_geometry(
                    self.width,
                    self.height,
                    self.width,
                    self.height,
                    self.padding,
                );

                let object_x: i64 = generic_dimension_calc(
                    &object.get_x(),
                    self.width,
                    self.height,
                    self.width,
                    self.height,
                    true,
                );
                let object_y: i64 = generic_dimension_calc(
                    &object.get_y(),
                    self.width,
                    self.height,
                    self.width,
                    self.height,
                    false,
                );
                let object_width: i64 = object.get_calculated_width();
                let object_height: i64 = object.get_calculated_height();

                let style: RendererObjectStyle = object.get_style();
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
                let current_buffer: Vec<Vec<Pixel>> =
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
}
