use std::{
    sync::{Arc, RwLock},
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

#[derive(Clone)]
pub(crate) struct UpdateValueSignaler {
    pub(crate) update: bool,
    pub(crate) parent: Option<Arc<RwLock<UpdateValueSignaler>>>,
}

impl UpdateValueSignaler {
    pub(crate) fn update(&mut self) {
        if !self.update {
            self.update = true;
            self.update_parent();
        }
    }

    fn update_parent(&mut self) {
        if let Some(parent) = self.parent.clone() {
            parent.write().unwrap().update();
        }
    }

    pub(crate) fn new() -> UpdateValueSignaler {
        UpdateValueSignaler {
            update: false,
            parent: None,
        }
    }
}

#[derive(Clone)]
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
    pub(crate) update_value_signal: Arc<RwLock<UpdateValueSignaler>>,

    pub(crate) default_character: char,

    pub(crate) text: Vec<Vec<char>>,
    pub(crate) changed_text: bool,

    pub(crate) pattern: Vec<Vec<char>>,
    pub(crate) changed_pattern: bool,

    pub(crate) animation: Vec<Vec<Vec<char>>>,
    pub(crate) changed_animation: bool,
    pub(crate) current_animation_frame: i64,

    pub(crate) animated_pattern: Vec<Vec<Vec<char>>>,
    pub(crate) changed_animated_pattern: bool,
    pub(crate) current_animated_pattern_frame: i64,

    pub(crate) colors: Vec<ColorArea>,
    pub(crate) changed_colors: bool,
    pub(crate) default_background_color: Color,
    pub(crate) default_foreground_color: Color,

    pub(crate) children: Vec<Arc<RwLock<RendererObjectValue>>>,
    pub(crate) changed_children: bool,

    pub(crate) style: RendererObjectStyle,
    pub(crate) changed_style: bool,

    pub(crate) parent_location: usize,

    pub(crate) new_self: Option<Arc<RwLock<RendererObjectValue>>>,
}

impl RendererObjectValue {
    ///returns update_parent
    pub(crate) fn update_value(&mut self) -> bool {
        {
            if !self.update_value_signal.read().unwrap().update {
                return false;
            }
        }
        let mut update = false;
        let mut update_parent = false;
        {
            let binding = self.new_self.clone().unwrap();
            let mut new_self = binding.write().unwrap();

            self.parent_location = new_self.parent_location;

            if self.x != new_self.x {
                self.x = new_self.x;
                update_parent = true;
            }

            if self.y != new_self.y {
                self.y = new_self.y;
                update_parent = true;
            }

            if self.width != new_self.width {
                self.width = new_self.width;
                update = true;
            }

            if self.height != new_self.height {
                self.height = new_self.height;
                update = true;
            }

            if self.default_character != new_self.default_character {
                self.default_character = new_self.default_character;
                update = true;
            }

            if self.default_background_color != new_self.default_background_color {
                self.default_background_color = new_self.default_background_color;
                update = true;
            }

            if self.default_foreground_color != new_self.default_foreground_color {
                self.default_foreground_color = new_self.default_foreground_color;
                update = true;
            }

            if self.current_animation_frame != new_self.current_animation_frame {
                self.current_animation_frame = new_self.current_animation_frame;
                update = true;
            }

            if self.current_animated_pattern_frame != new_self.current_animated_pattern_frame {
                self.current_animated_pattern_frame = new_self.current_animated_pattern_frame;
                update = true;
            }

            if new_self.changed_animated_pattern {
                self.animated_pattern = new_self.animated_pattern.clone();
                new_self.changed_animated_pattern = false;
                update = true;
            }

            if new_self.changed_text {
                self.text = new_self.text.clone();
                new_self.changed_text = false;
                update = true;
            }

            if new_self.changed_pattern {
                self.pattern = new_self.pattern.clone();
                new_self.changed_pattern = false;
                update = true;
            }

            if new_self.changed_animation {
                self.animation = new_self.animation.clone();
                new_self.changed_animation = false;
                update = true;
            }

            if new_self.changed_style {
                self.style = new_self.style.clone();
                new_self.changed_style = false;
                update = true;
            }

            if new_self.changed_colors {
                self.colors = new_self.colors.clone();
                new_self.changed_colors = false;
                update = true;
            }

            if new_self.changed_children {
                self.children = new_self.children.clone();
                new_self.changed_children = false;
                update = true;
            }
        }

        for i in 0..self.children.len() {
            let mut child = self.children[i].write().unwrap();
            update = child.update_value() || update;
        }

        if update {
            self.update();
            update_parent = true;
        }
        self.update_value_signal.write().unwrap().update = false;
        return update_parent;
    }

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
                let child_ref = child.try_read().unwrap();
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
                            child_ref.calculated_width
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
                let child_ref = child.try_read().unwrap();
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
                            child_ref.calculated_height
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
                let mut child = child_cell.write().unwrap();
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
                let mut child = child_cell.write().unwrap();
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

    fn draw_text(&mut self, renderer_padding: i64) {
        let start_x: i64 = (-self.absolute_x - renderer_padding)
            .min(self.calculated_width)
            .max(0);
        let start_y: i64 = (-self.absolute_y - renderer_padding)
            .min(self.calculated_height)
            .max(0);
        let end_x: i64 = (self.absolute_x + renderer_padding + self.renderer_width)
            .min(self.calculated_width)
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
        let start_y: i64 = (-self.absolute_y - renderer_padding)
            .min(self.calculated_height)
            .max(0);
        let end_x: i64 = (-self.absolute_x + renderer_padding + self.renderer_width)
            .min(self.calculated_width)
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
        let start_y: i64 = (-self.absolute_y - renderer_padding)
            .min(self.calculated_height)
            .max(0);
        let end_x: i64 = (-self.absolute_x + renderer_padding + self.renderer_width)
            .min(self.calculated_width)
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

    fn draw_animated_pattern(
        &mut self,
        renderer_padding: i64,
        current_animated_pattern_frame: i64,
    ) {
        let start_x: i64 = (-self.absolute_x - renderer_padding)
            .min(self.calculated_width)
            .max(0);
        let start_y: i64 = (-self.absolute_y - renderer_padding)
            .min(self.calculated_height)
            .max(0);
        let end_x: i64 = (-self.absolute_x + renderer_padding + self.renderer_width)
            .min(self.calculated_width)
            .max(0);
        let end_y: i64 = (-self.absolute_y + renderer_padding + self.renderer_height)
            .min(self.calculated_height)
            .max(0);

        let frame = &self.animated_pattern
            [current_animated_pattern_frame as usize % self.animated_pattern.len()];

        for i in start_y..end_y {
            let chars: &Vec<char> = &frame[i as usize % frame.len()];
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

    fn draw_colors(&mut self, renderer_padding: i64) {
        let start_x: i64 = (-self.absolute_x - renderer_padding)
            .min(self.calculated_width)
            .max(0);
        let rendered_width: i64 = (-self.absolute_x + renderer_padding + self.renderer_width
            - start_x)
            .min(self.calculated_width)
            .max(0);
        let start_y: i64 = (-self.absolute_y - renderer_padding)
            .min(self.calculated_height)
            .max(0);
        let rendered_height: i64 = (-self.absolute_y + renderer_padding + self.renderer_height
            - start_y)
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

            let color_start_x = alignment_offset_x + color_x;
            let color_start_y = alignment_offset_y + color_y;
            let color_end_x = alignment_offset_x + color_x + color_width;
            let color_end_y = alignment_offset_y + color_y + color_height;

            for i in color_start_y.max(0)..color_end_y.min(rendered_height) {
                for j in color_start_x.max(0)..color_end_x.min(rendered_width) {
                    match color_area.layer {
                        ColorLayer::Background => {
                            self.buffer[i as usize][j as usize].background = self.buffer[i as usize]
                                [j as usize]
                                .background
                                .with_overlay(color_area.color)
                        }
                        ColorLayer::Foreground => {
                            self.buffer[i as usize][j as usize].foreground = self.buffer[i as usize]
                                [j as usize]
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
        let start_y: i64 = (-self.absolute_y - renderer_padding)
            .min(self.calculated_height)
            .max(0);
        let end_x: i64 = (-self.absolute_x + renderer_padding + self.renderer_width)
            .min(self.calculated_width)
            .max(0);
        let end_y: i64 = (-self.absolute_y + renderer_padding + self.renderer_height)
            .min(self.calculated_height)
            .max(0);

        for child_cell in &self.children {
            let mut child = child_cell.write().unwrap();
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
                AlignmentY::Top => 0,
                AlignmentY::Center => self.calculated_height / 2 - child_height as i64 / 2,
                AlignmentY::Bottom => self.calculated_height - child_height as i64,
            };

            let child_buffer = child.get_buffer(
                self.absolute_x + alignment_offset_x + child_x,
                self.absolute_y + alignment_offset_y + child_y,
                renderer_padding,
            );

            //I have no idea why this correction works, it's a total bodge, but it's perfect
            let child_rendering_correction_x = child_width + renderer_padding * 2
                - child_buffer.first().unwrap_or(&vec![]).len() as i64
                - (child_width + child_x + alignment_offset_x - self.calculated_width)
                    .clamp(0, (child_width - self.calculated_width).max(0));
            let child_rendering_correction_y = child_height + renderer_padding * 2
                - child_buffer.len() as i64
                - (child_height + child_y + alignment_offset_y - self.calculated_height)
                    .clamp(0, (child_height - self.calculated_height).max(0));

            let child_top = alignment_offset_y + child_y;
            let child_bottom = alignment_offset_y + child_y + child_height - 1;
            let child_left = alignment_offset_x + child_x;
            let child_right = alignment_offset_x + child_x + child_width - 1;

            for i in (child_top + child_rendering_correction_y).max(start_y)
                ..=child_bottom.min(end_y - 1)
            {
                for j in (child_left + child_rendering_correction_x).max(start_x)
                    ..=child_right.min(end_x - 1)
                {
                    self.buffer[(i - start_y) as usize][(j - start_x) as usize] =
                        self.buffer[(i - start_y) as usize][(j - start_x) as usize].with_overlay(
                            &child_buffer[(i - child_top - child_rendering_correction_y) as usize]
                                [(j - child_left - child_rendering_correction_x) as usize],
                        );
                }
            }

            //draw border around object
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

            if self.animated_pattern.len() > 0 {
                self.draw_animated_pattern(renderer_padding, self.current_animated_pattern_frame);
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
        }
    }
}
