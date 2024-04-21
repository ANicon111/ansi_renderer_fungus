use std::{
    cell::RefCell,
    rc::{Rc, Weak},
    vec,
};

use crate::{
    color::Color,
    color_area::{ColorArea, ColorLayer},
    geometry::Dimension,
    misc::generic_dimension_calc,
    pixel::Pixel,
    renderer_object_style::{AlignmentX, AlignmentY, RendererObjectStyle},
};
pub(crate) struct RendererObjectValue {
    pub(crate) buffer: Vec<Vec<Pixel>>,

    pub(crate) x: Dimension,
    pub(crate) y: Dimension,
    pub(crate) width: Dimension,
    pub(crate) height: Dimension,

    pub(crate) absolute_x: i64,
    pub(crate) absolute_y: i64,

    pub(crate) parent_width: i64,
    pub(crate) parent_height: i64,

    pub(crate) renderer_width: i64,
    pub(crate) renderer_height: i64,

    pub(crate) calculated_width: i64,
    pub(crate) calculated_height: i64,

    pub(crate) update_size: bool,
    pub(crate) update_content: bool,

    pub(crate) default_character: char,

    pub(crate) text: Vec<Vec<char>>,

    pub(crate) pattern: Vec<Vec<char>>,

    pub(crate) animation: Vec<Vec<Vec<char>>>,
    pub(crate) current_animation_frame: i64,

    pub(crate) colors: Vec<ColorArea>,
    pub(crate) default_background_color: Color,
    pub(crate) default_foreground_color: Color,

    pub(crate) children: Vec<Rc<RefCell<RendererObjectValue>>>,

    pub(crate) style: RendererObjectStyle,

    pub(crate) parent: Option<Weak<RefCell<RendererObjectValue>>>,
    pub(crate) parent_location: usize,
}

impl RendererObjectValue {
    //getters and setters
    pub(crate) fn set_x(&mut self, x: Dimension) {
        if self.x != x {
            self.x = x;
            self.update_parent();
        }
    }

    pub(crate) fn get_x(&self) -> Dimension {
        self.x.clone()
    }

    pub(crate) fn set_y(&mut self, y: Dimension) {
        if self.y != y {
            self.y = y;
            self.update_parent();
        }
    }

    pub(crate) fn get_y(&self) -> Dimension {
        self.y.clone()
    }

    pub(crate) fn set_width(&mut self, width: Dimension) {
        if self.width != width {
            self.width = width;
            self.update_size = true;
            self.update();
        }
    }

    pub(crate) fn get_width(&self) -> Dimension {
        self.width.clone()
    }

    pub(crate) fn set_height(&mut self, height: Dimension) {
        if self.height != height {
            self.height = height;
            self.update_size = true;
            self.update();
        }
    }

    pub(crate) fn get_height(&self) -> Dimension {
        self.height.clone()
    }

    pub(crate) fn set_geometry(
        &mut self,
        (x, y, width, height): (Dimension, Dimension, Dimension, Dimension),
    ) {
        self.set_x(x);
        self.set_y(y);
        self.set_width(width);
        self.set_height(height);
    }

    pub(crate) fn get_geometry(&self) -> (Dimension, Dimension, Dimension, Dimension) {
        (
            self.get_x(),
            self.get_y(),
            self.get_width(),
            self.get_height(),
        )
    }

    pub(crate) fn set_default_background_color(&mut self, color: Color) {
        if self.default_background_color != color {
            self.default_background_color = color;
            self.update();
        }
    }

    pub(crate) fn get_default_background_color(&self) -> Color {
        self.default_background_color
    }

    pub(crate) fn set_default_foreground_color(&mut self, color: Color) {
        if self.default_foreground_color != color {
            self.default_foreground_color = color;
            self.update();
        }
    }

    pub(crate) fn get_default_foreground_color(&self) -> Color {
        self.default_foreground_color
    }

    pub(crate) fn set_default_character(&mut self, character: char) {
        if self.default_character != character {
            self.default_character = character;
            self.update();
        }
    }

    pub(crate) fn get_default_character(&self) -> char {
        self.default_character
    }

    pub(crate) fn set_text(&mut self, val: &str) {
        self.text = val
            .replace("\r\n", "\n")
            .split('\n')
            .map(|s| s.to_string().chars().collect())
            .collect();
        self.update();
    }

    pub(crate) fn get_text(&self) -> String {
        self.text
            .iter()
            .map(|val| val.iter().collect())
            .collect::<Vec<String>>()
            .join("\n")
    }

    pub(crate) fn set_pattern(&mut self, val: &str) {
        self.pattern = val
            .replace("\r\n", "\n")
            .split('\n')
            .map(|s| s.to_string().chars().collect())
            .collect();
        self.update();
    }

    pub(crate) fn get_pattern(&self) -> String {
        self.pattern
            .iter()
            .map(|val| val.iter().collect())
            .collect::<Vec<String>>()
            .join("\n")
    }

    pub(crate) fn set_animation(&mut self, val: &Vec<&str>) {
        self.animation = val
            .iter()
            .map(|val| {
                val.replace("\r\n", "\n")
                    .split('\n')
                    .map(|s| s.to_string().chars().collect())
                    .collect()
            })
            .collect();
        self.update();
    }

    pub(crate) fn get_animation(&self) -> Vec<String> {
        self.animation
            .iter()
            .map(|val| {
                val.iter()
                    .map(|val| val.iter().collect())
                    .collect::<Vec<String>>()
                    .join("\n")
            })
            .collect()
    }

    pub(crate) fn set_current_frame(&mut self, current_animation_frame: i64) {
        if self.current_animation_frame != current_animation_frame {
            self.current_animation_frame = current_animation_frame;
            self.update();
        }
    }

    pub(crate) fn shift_current_frame(&mut self, shift_val: i64) {
        if shift_val != 0 {
            self.current_animation_frame += shift_val;
            self.update();
        }
    }

    pub(crate) fn get_current_frame(&self) -> i64 {
        self.current_animation_frame
    }

    pub(crate) fn set_style(&mut self, style: RendererObjectStyle) {
        self.style = style;
        self.update();
    }

    pub(crate) fn get_style(&self) -> RendererObjectStyle {
        self.style.clone()
    }

    pub(crate) fn set_colors(&mut self, mut val: Vec<ColorArea>) {
        for i in 0..val.len() {
            val[i].renderer_object_index = i;
        }
        self.colors = val;
        self.update();
    }

    pub(crate) fn get_colors(&self) -> Vec<ColorArea> {
        self.colors.clone()
    }

    pub(crate) fn add_color(&mut self, val: &mut ColorArea) {
        val.renderer_object_index = self.colors.len();
        self.colors.push(val.clone());
        self.update();
    }

    pub(crate) fn remove_color(&mut self, val: &ColorArea) {
        self.colors.remove(val.renderer_object_index);
        self.update();
    }
    //end of getters/setters

    fn calculate_geometry(
        &self,
        parent_width: i64,
        parent_height: i64,
        renderer_width: i64,
        renderer_height: i64,
    ) -> (i64, i64) {
        let mut width = generic_dimension_calc(
            &self.width,
            parent_width,
            parent_height,
            renderer_width,
            renderer_height,
            true,
        );

        if self.width == Dimension::Auto {
            for child in &self.children {
                let child_ref: &RendererObjectValue = &RefCell::borrow(child);
                match (&child_ref.x, &child_ref.width) {
                    //supported dimension types
                    (
                        Dimension::Auto
                        | Dimension::Pixel(_)
                        | Dimension::VW(_)
                        | Dimension::VH(_)
                        | Dimension::VMin(_)
                        | Dimension::VMax(_),
                        Dimension::Auto
                        | Dimension::Pixel(_)
                        | Dimension::VW(_)
                        | Dimension::VH(_)
                        | Dimension::VMin(_)
                        | Dimension::VMax(_),
                    ) => {
                        width = width.max(
                            child.borrow().calculated_width
                                + generic_dimension_calc(
                                    &child_ref.x,
                                    0,
                                    0,
                                    renderer_width,
                                    renderer_height,
                                    true,
                                ),
                        )
                    }
                    _ => (),
                }
            }
            for line in &self.text {
                width = width.max(line.len() as i64);
            }
            for frame in &self.animation {
                for line in frame {
                    width = width.max(line.len() as i64);
                }
            }
        }

        let mut height = generic_dimension_calc(
            &self.height,
            parent_width,
            parent_height,
            renderer_width,
            renderer_height,
            false,
        );

        if self.height == Dimension::Auto {
            for child in &self.children {
                let child_ref = RefCell::borrow(child);
                match (&child_ref.y, &child_ref.height) {
                    //supported dimension types
                    (
                        Dimension::Auto
                        | Dimension::Pixel(_)
                        | Dimension::VW(_)
                        | Dimension::VH(_)
                        | Dimension::VMin(_)
                        | Dimension::VMax(_),
                        Dimension::Auto
                        | Dimension::Pixel(_)
                        | Dimension::VW(_)
                        | Dimension::VH(_)
                        | Dimension::VMin(_)
                        | Dimension::VMax(_),
                    ) => {
                        height = height.max(
                            child.borrow().calculated_height
                                + generic_dimension_calc(
                                    &child_ref.y,
                                    0,
                                    0,
                                    renderer_width,
                                    renderer_height,
                                    false,
                                ),
                        )
                    }
                    _ => (),
                }
            }
            height = height.max(self.text.len() as i64);
            for frame in &self.animation {
                height = height.max(frame.len() as i64);
            }
        }

        (width, height)
    }

    fn draw_text(&mut self, renderer_padding: i64) {
        let start_x: i64 = (-self.absolute_x - renderer_padding)
            .min(self.calculated_width)
            .max(0);
        let end_x: i64 = (self.absolute_x + renderer_padding + self.renderer_width)
            .min(self.calculated_width)
            .max(0);
        let start_y: i64 = (-self.absolute_y - renderer_padding)
            .min(self.calculated_height)
            .max(0);
        let end_y: i64 = (self.absolute_y + renderer_padding + self.renderer_height)
            .min(self.calculated_height)
            .max(0);

        let text_height: usize = self.text.len();
        let alignment_offset_y: i64 = match self.style.internal_alignment_y {
            AlignmentY::Top => 0,
            AlignmentY::Center => self.calculated_height / 2 - text_height as i64 / 2,
            AlignmentY::Bottom => self.calculated_height - text_height as i64,
        };
        let text_start_y = start_y.max(alignment_offset_y);
        let text_end_y = end_y.min(alignment_offset_y + text_height as i64);

        for i in text_start_y..text_end_y {
            let line_width: usize = self.text[(i - alignment_offset_y) as usize].len();
            let alignment_offset_x: i64 = match self.style.internal_alignment_x {
                AlignmentX::Left => 0,
                AlignmentX::Center => self.calculated_width / 2 - line_width as i64 / 2,
                AlignmentX::Right => self.calculated_width - line_width as i64,
            };
            let line_start_x = start_x.max(alignment_offset_x);
            let line_end_x = end_x.min(alignment_offset_x + line_width as i64);
            for j in line_start_x..line_end_x {
                let new_val =
                    self.text[(i - alignment_offset_y) as usize][(j - alignment_offset_x) as usize];
                if new_val != '\0' {
                    self.buffer[i as usize][j as usize].value = new_val;
                }
            }
        }
    }

    fn draw_pattern(&mut self, renderer_padding: i64) {
        let start_x: i64 = (-self.absolute_x - renderer_padding)
            .min(self.calculated_width)
            .max(0);
        let end_x: i64 = (-self.absolute_x + renderer_padding + self.renderer_width)
            .min(self.calculated_width)
            .max(0);
        let start_y: i64 = (-self.absolute_y - renderer_padding)
            .min(self.calculated_height)
            .max(0);
        let end_y: i64 = (-self.absolute_y + renderer_padding + self.renderer_height)
            .min(self.calculated_height)
            .max(0);

        for i in start_y..end_y {
            let chars: &Vec<char> = &self.pattern[i as usize % self.pattern.len()];
            if chars.len() > 0 {
                for j in start_x..end_x {
                    let new_val = chars[j as usize % chars.len()];
                    if new_val != '\0' {
                        self.buffer[(i - start_y) as usize][(j - start_x) as usize].value = new_val;
                    }
                }
            }
        }
    }

    fn draw_animation(&mut self, renderer_padding: i64, current_animation_frame: i64) {
        let start_x: i64 = (-self.absolute_x - renderer_padding)
            .min(self.calculated_width)
            .max(0);
        let end_x: i64 = (-self.absolute_x + renderer_padding + self.renderer_width)
            .min(self.calculated_width)
            .max(0);
        let start_y: i64 = (-self.absolute_y - renderer_padding)
            .min(self.calculated_height)
            .max(0);
        let end_y: i64 = (-self.absolute_y + renderer_padding + self.renderer_height)
            .min(self.calculated_height)
            .max(0);

        let frame = &self.animation[current_animation_frame as usize % self.animation.len()];

        let frame_height: usize = frame.len();
        let alignment_offset_y: i64 = match self.style.internal_alignment_y {
            AlignmentY::Top => 0,
            AlignmentY::Center => self.calculated_height / 2 - frame_height as i64 / 2,
            AlignmentY::Bottom => self.calculated_height - frame_height as i64,
        };
        let frame_start_y = start_y.max(alignment_offset_y) as usize;
        let frame_end_y = end_y.min(alignment_offset_y + frame_height as i64) as usize;

        for j in frame_start_y..frame_end_y as usize {
            let line_width: usize = frame[j - alignment_offset_y as usize].len();
            let alignment_offset_x: i64 = match self.style.internal_alignment_x {
                AlignmentX::Left => 0,
                AlignmentX::Center => self.calculated_width / 2 - line_width as i64 / 2,
                AlignmentX::Right => self.calculated_width - line_width as i64,
            };
            let line_start_x = start_x.max(alignment_offset_x) as usize;
            let line_end_x = end_x.min(alignment_offset_x + line_width as i64) as usize;
            for i in line_start_x..line_end_x {
                self.buffer[j][i].value = frame[(j as i64 - alignment_offset_y) as usize]
                    [(i as i64 - alignment_offset_x) as usize];
            }
        }
    }

    fn draw_colors(&mut self, renderer_padding: i64) {
        let start_x: i64 = (-self.absolute_x - renderer_padding)
            .min(self.calculated_width)
            .max(0);
        let end_x: i64 = (-self.absolute_x + renderer_padding + self.renderer_width)
            .min(self.calculated_width)
            .max(0);
        let start_y: i64 = (-self.absolute_y - renderer_padding)
            .min(self.calculated_height)
            .max(0);
        let end_y: i64 = (-self.absolute_y + renderer_padding + self.renderer_height)
            .min(self.calculated_height)
            .max(0);

        for color_area in &self.colors {
            let color_x = generic_dimension_calc(
                &color_area.x,
                self.calculated_width,
                self.calculated_height,
                self.renderer_width,
                self.renderer_height,
                true,
            );
            let color_y = generic_dimension_calc(
                &color_area.y,
                self.calculated_width,
                self.calculated_height,
                self.renderer_width,
                self.renderer_height,
                false,
            );
            let mut color_width = generic_dimension_calc(
                &color_area.width,
                self.calculated_width,
                self.calculated_height,
                self.renderer_width,
                self.renderer_height,
                true,
            );

            if color_area.width == Dimension::Auto {
                color_width = self.calculated_width;
            }

            let mut color_height = generic_dimension_calc(
                &color_area.height,
                self.calculated_width,
                self.calculated_height,
                self.renderer_width,
                self.renderer_height,
                false,
            );

            if color_area.height == Dimension::Auto {
                color_height = self.calculated_height;
            }

            let alignment_offset_x: i64 = match match color_area.external_alignment_x {
                Some(val) => val,
                None => self.style.internal_alignment_x,
            } {
                AlignmentX::Left => 0,
                AlignmentX::Center => self.calculated_width / 2 - color_width as i64 / 2,
                AlignmentX::Right => self.calculated_width - color_width as i64,
            };

            let alignment_offset_y: i64 = match match color_area.external_alignment_y {
                Some(val) => val,
                None => self.style.internal_alignment_y,
            } {
                AlignmentY::Top => 0,
                AlignmentY::Center => self.calculated_height / 2 - color_height as i64 / 2,
                AlignmentY::Bottom => self.calculated_height - color_height as i64,
            };

            let color_start_x = start_x.max(alignment_offset_x + color_x);
            let color_end_x = end_x.min(alignment_offset_x + color_x + color_width);
            let color_start_y = start_y.max(alignment_offset_y + color_y);
            let color_end_y = end_y.min(alignment_offset_y + color_y + color_height);

            for i in color_start_y..color_end_y {
                for j in color_start_x..color_end_x {
                    match color_area.layer {
                        ColorLayer::Background => {
                            self.buffer[(i - start_y) as usize][(j - start_x) as usize].background =
                                self.buffer[(i - start_y) as usize][(j - start_x) as usize]
                                    .background
                                    .with_overlay(color_area.color)
                        }
                        ColorLayer::Foreground => {
                            self.buffer[(i - start_y) as usize][(j - start_x) as usize].foreground =
                                self.buffer[(i - start_y) as usize][(j - start_x) as usize]
                                    .foreground
                                    .with_overlay(color_area.color)
                        }
                    }
                }
            }
        }
    }

    fn draw_children(&mut self, renderer_padding: i64) {
        let start_x: i64 = (-self.absolute_x - renderer_padding)
            .min(self.calculated_width)
            .max(0);
        let end_x: i64 = (-self.absolute_x + renderer_padding + self.renderer_width)
            .min(self.calculated_width)
            .max(0);
        let start_y: i64 = (-self.absolute_y - renderer_padding)
            .min(self.calculated_height)
            .max(0);
        let end_y: i64 = (-self.absolute_y + renderer_padding + self.renderer_height)
            .min(self.calculated_height)
            .max(0);

        for child_cell in &self.children {
            let mut child = child_cell.borrow_mut();
            let child_x = generic_dimension_calc(
                &child.x,
                self.calculated_width,
                self.calculated_height,
                self.renderer_width,
                self.renderer_height,
                true,
            );
            let child_y = generic_dimension_calc(
                &child.y,
                self.calculated_width,
                self.calculated_height,
                self.renderer_width,
                self.renderer_height,
                false,
            );

            let child_width = child.calculated_width;
            let child_height = child.calculated_height;

            //use precalculated child
            let alignment_offset_x: i64 = match match child.style.external_alignment_x {
                Some(val) => val,
                None => self.style.internal_alignment_x,
            } {
                AlignmentX::Left => 0,
                AlignmentX::Center => self.calculated_width / 2 - child_width as i64 / 2,
                AlignmentX::Right => self.calculated_width - child_width as i64,
            };

            let alignment_offset_y: i64 = match match child.style.external_alignment_y {
                Some(val) => val,
                None => self.style.internal_alignment_y,
            } {
                AlignmentY::Top => 0, //IDK why, but child positioning is whack if this ain't 1
                AlignmentY::Center => self.calculated_height / 2 - child_height as i64 / 2,
                AlignmentY::Bottom => self.calculated_height - child_height as i64,
            };

            let child_buffer = child.get_buffer(
                self.absolute_x + alignment_offset_x + child_x,
                self.absolute_y + alignment_offset_y + child_y,
                renderer_padding,
            );

            let child_start_x = start_x.max(alignment_offset_x + child_x);
            let child_end_x = end_x.min(alignment_offset_x + child_x + child_width);
            let child_start_y = start_y.max(alignment_offset_y + child_y);
            let child_end_y = end_y.min(alignment_offset_y + child_y + child_height);
            for i in child_start_y..child_end_y {
                for j in child_start_x..child_end_x {
                    self.buffer[(i - start_y) as usize][(j - start_x) as usize] =
                        self.buffer[(i - start_y) as usize][(j - start_x) as usize].with_overlay(
                            &child_buffer[(i - child_start_y) as usize]
                                [(j - child_start_x) as usize],
                        );
                }
            }

            //draw border around object
            let child_top = alignment_offset_y + child_y;
            let child_bottom = alignment_offset_y + child_y + child_height - 1;
            let child_left = alignment_offset_x + child_x;
            let child_right = alignment_offset_x + child_x + child_width - 1;

            if child.style.border.top.value != '\0'
                && child_top - 1 >= start_y
                && child_top - 1 < end_y
            {
                for j in child_left.max(start_x)..=child_right.min(end_x - 1) {
                    self.buffer[(child_top - 1 - start_y) as usize][(j - start_x) as usize] =
                        self.buffer[(child_top - 1 - start_y) as usize][(j - start_x) as usize]
                            .with_overlay(&child.style.border.top);
                }
            }

            if child.style.border.bottom.value != '\0'
                && child_bottom + 1 >= start_y
                && child_bottom + 1 < end_y
            {
                for j in child_left.max(start_x)..=child_right.min(end_x - 1) {
                    self.buffer[(child_bottom + 1 - start_y) as usize][(j - start_x) as usize] =
                        self.buffer[(child_bottom + 1 - start_y) as usize][(j - start_x) as usize]
                            .with_overlay(&child.style.border.bottom);
                }
            }

            if child.style.border.left.value != '\0'
                && child_left - 1 >= start_x
                && child_left - 1 < end_x
            {
                for i in child_top.max(start_y)..=child_bottom.min(end_y - 1) {
                    self.buffer[(i - start_y) as usize][(child_left - 1 - start_x) as usize] =
                        self.buffer[(i - start_y) as usize][(child_left - 1 - start_x) as usize]
                            .with_overlay(&child.style.border.left);
                }
            }

            if child.style.border.right.value != '\0'
                && child_right + 1 >= start_x
                && child_right + 1 < end_x
            {
                for i in child_top.max(start_y)..=child_bottom.min(end_y - 1) {
                    self.buffer[(i - start_y) as usize][(child_right + 1 - start_x) as usize] =
                        self.buffer[(i - start_y) as usize][(child_right + 1 - start_x) as usize]
                            .with_overlay(&child.style.border.right);
                }
            }

            if child.style.border.top_left.value != '\0'
                && child_top - 1 >= start_y
                && child_top - 1 < end_y
                && child_left - 1 >= start_x
                && child_left - 1 < end_x
            {
                self.buffer[(child_top - 1 - start_y) as usize]
                    [(child_left - 1 - start_x) as usize] = self.buffer
                    [(child_top - 1 - start_y) as usize][(child_left - 1 - start_x) as usize]
                    .with_overlay(&child.style.border.top_left);
            }

            if child.style.border.top_right.value != '\0'
                && child_top - 1 >= start_y
                && child_top - 1 < end_y
                && child_right + 1 >= start_x
                && child_right + 1 < end_x
            {
                self.buffer[(child_top - 1 - start_y) as usize]
                    [(child_right + 1 - start_x) as usize] = self.buffer
                    [(child_top - 1 - start_y) as usize][(child_right + 1 - start_x) as usize]
                    .with_overlay(&child.style.border.top_right);
            }

            if child.style.border.bottom_right.value != '\0'
                && child_bottom + 1 >= start_y
                && child_bottom + 1 < end_y
                && child_right + 1 >= start_x
                && child_right + 1 < end_x
            {
                self.buffer[(child_bottom + 1 - start_y) as usize]
                    [(child_right + 1 - start_x) as usize] = self.buffer
                    [(child_bottom + 1 - start_y) as usize]
                    [(child_right + 1 - start_x) as usize]
                    .with_overlay(&child.style.border.bottom_right);
            }

            if child.style.border.bottom_left.value != '\0'
                && child_bottom + 1 >= start_y
                && child_bottom + 1 < end_y
                && child_left - 1 >= start_x
                && child_left - 1 < end_x
            {
                self.buffer[(child_bottom + 1 - start_y) as usize]
                    [(child_left - 1 - start_x) as usize] = self.buffer
                    [(child_bottom + 1 - start_y) as usize]
                    [(child_left - 1 - start_x) as usize]
                    .with_overlay(&child.style.border.bottom_left);
            }
        }
    }

    pub(crate) fn process_geometry(
        &mut self,
        renderer_width: i64,
        renderer_height: i64,
        parent_width: i64,
        parent_height: i64,
        renderer_padding: i64,
    ) {
        if renderer_width != self.renderer_width {
            self.renderer_width = renderer_width;
            match (&self.x, &self.width) {
                (
                    Dimension::VW(_) | Dimension::VH(_) | Dimension::VMin(_) | Dimension::VMax(_),
                    _,
                )
                | (
                    _,
                    Dimension::VW(_) | Dimension::VH(_) | Dimension::VMin(_) | Dimension::VMax(_),
                ) => self.update_size = true,
                _ => (),
            }
        }

        if renderer_height != self.renderer_height {
            self.renderer_height = renderer_height;
            match (&self.y, &self.height) {
                (
                    Dimension::VW(_) | Dimension::VH(_) | Dimension::VMin(_) | Dimension::VMax(_),
                    _,
                )
                | (
                    _,
                    Dimension::VW(_) | Dimension::VH(_) | Dimension::VMin(_) | Dimension::VMax(_),
                ) => self.update_size = true,
                _ => (),
            }
        }

        if parent_width != self.parent_width {
            self.parent_width = parent_width;
            match (&self.x, &self.width) {
                (
                    Dimension::Percent(_)
                    | Dimension::PW(_)
                    | Dimension::PH(_)
                    | Dimension::PMin(_)
                    | Dimension::PMax(_),
                    _,
                )
                | (
                    _,
                    Dimension::Percent(_)
                    | Dimension::PW(_)
                    | Dimension::PH(_)
                    | Dimension::PMin(_)
                    | Dimension::PMax(_),
                ) => self.update_size = true,
                _ => (),
            }
        }

        if parent_height != self.parent_height {
            self.parent_height = parent_height;
            match (&self.y, &self.height) {
                (
                    Dimension::Percent(_)
                    | Dimension::PW(_)
                    | Dimension::PH(_)
                    | Dimension::PMin(_)
                    | Dimension::PMax(_),
                    _,
                )
                | (
                    _,
                    Dimension::Percent(_)
                    | Dimension::PW(_)
                    | Dimension::PH(_)
                    | Dimension::PMin(_)
                    | Dimension::PMax(_),
                ) => self.update_size = true,
                _ => (),
            }
        }

        //update children independent on current object
        if self.update_content {
            for child_cell in &self.children {
                let mut child = child_cell.borrow_mut();
                match (child.width, child.height) {
                    //evil match that selects children with parent-dependent values
                    (
                        Dimension::Percent(_)
                        | Dimension::PW(_)
                        | Dimension::PH(_)
                        | Dimension::PMin(_)
                        | Dimension::PMax(_),
                        _,
                    )
                    | (
                        _,
                        Dimension::Percent(_)
                        | Dimension::PW(_)
                        | Dimension::PH(_)
                        | Dimension::PMin(_)
                        | Dimension::PMax(_),
                    ) => (),
                    _ => child.process_geometry(
                        renderer_width,
                        renderer_height,
                        0,
                        0,
                        renderer_padding,
                    ),
                }
            }
        }

        if self.update_content {
            if self.width == Dimension::Auto || self.height == Dimension::Auto {
                let (new_width, new_height) = self.calculate_geometry(
                    parent_width,
                    parent_height,
                    renderer_width,
                    renderer_height,
                );
                if (new_width, new_height) != (self.calculated_width, self.calculated_height) {
                    (self.calculated_width, self.calculated_height) = (new_width, new_height);
                    self.update_size = true;
                }
            }
        }

        if self.update_size {
            self.update_content = true;
            // skip guessing the geometry if either is Dimension::Auto
            // since it's calculated earlier in check_for_size_changes
            if self.width != Dimension::Auto && self.height != Dimension::Auto {
                (self.calculated_width, self.calculated_height) = self.calculate_geometry(
                    self.parent_width,
                    self.parent_height,
                    self.renderer_width,
                    self.renderer_height,
                );
            }
            let width: i64 = (-self.absolute_x + self.renderer_width)
                .min(self.calculated_width)
                .max(0)
                - (-self.absolute_x).min(self.calculated_width).max(0)
                + 2 * renderer_padding;
            let height: i64 = (-self.absolute_y + self.renderer_height)
                .min(self.calculated_height)
                .max(0)
                - (-self.absolute_y).min(self.calculated_height).max(0)
                + 2 * renderer_padding;
            self.buffer = vec::from_elem(
                vec::from_elem(
                    Pixel {
                        value: ' ',
                        background: self.default_background_color,
                        foreground: self.default_foreground_color,
                    },
                    width as usize,
                ),
                height as usize,
            );
            self.update_size = false;
        }

        //update children dependent on current object
        if self.update_content {
            for child_cell in &self.children {
                let mut child = child_cell.borrow_mut();
                match (child.width, child.height) {
                    (
                        Dimension::Percent(_)
                        | Dimension::PW(_)
                        | Dimension::PH(_)
                        | Dimension::PMin(_)
                        | Dimension::PMax(_),
                        _,
                    )
                    | (
                        _,
                        Dimension::Percent(_)
                        | Dimension::PW(_)
                        | Dimension::PH(_)
                        | Dimension::PMin(_)
                        | Dimension::PMax(_),
                    ) => child.process_geometry(
                        renderer_width,
                        renderer_height,
                        self.calculated_width,
                        self.calculated_height,
                        renderer_padding,
                    ),
                    _ => (),
                }
            }
        }
    }

    pub(crate) fn get_buffer(
        &mut self,
        absolute_x: i64,
        absolute_y: i64,
        renderer_padding: i64,
    ) -> &Vec<Vec<Pixel>> {
        if (absolute_x - self.absolute_x).abs() >= renderer_padding {
            self.absolute_x = absolute_x;
            self.update_content = true;
        }

        if (absolute_y - self.absolute_y).abs() >= renderer_padding {
            self.absolute_y = absolute_y;
            self.update_content = true;
        }

        if self.update_content {
            for line in &mut self.buffer {
                line.fill(Pixel {
                    value: self.default_character,
                    background: self.default_background_color,
                    foreground: self.default_foreground_color,
                });
            }

            if self.pattern.len() > 0 {
                self.draw_pattern(renderer_padding);
            }

            self.draw_text(renderer_padding);

            if self.animation.len() > 0 {
                self.draw_animation(renderer_padding, self.current_animation_frame);
            }

            self.draw_colors(renderer_padding);

            if self.children.len() > 0 {
                self.draw_children(renderer_padding);
            }

            self.update_content = false;
        }

        &self.buffer
    }

    pub(crate) fn update(&mut self) {
        if !self.update_content {
            self.update_content = true;
            self.update_parent();
        }
    }

    pub(crate) fn update_parent(&self) {
        if self.parent.is_some() {
            let parent_ref = self.parent.as_ref().unwrap().upgrade();
            if parent_ref.is_some() {
                let parent_rc = &parent_ref.unwrap();
                let mut parent = parent_rc.borrow_mut();
                if !parent.update_content {
                    parent.update();
                }
            }
        }
    }
}
