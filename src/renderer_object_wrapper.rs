use std::sync::{Arc, RwLock};

use crate::{
    color::Color, color_area::ColorArea, colors::Colors, geometry::Dimension, pixel::Pixel,
    renderer_object_style::RendererObjectStyle, renderer_object_value::RendererObjectValue,
};

#[derive(Clone)]
pub struct RendererObject {
    value: Arc<RwLock<RendererObjectValue>>,
}

impl RendererObject {
    //getters and setters
    pub fn set_x(&mut self, x: Dimension) -> &mut Self {
        self.value.try_write().unwrap().set_x(x);
        self
    }

    pub fn get_x(&self) -> Dimension {
        self.value.try_read().unwrap().get_x()
    }

    pub fn set_y(&mut self, y: Dimension) -> &mut Self {
        self.value.try_write().unwrap().set_y(y);
        self
    }

    pub fn get_y(&self) -> Dimension {
        self.value.try_read().unwrap().get_y()
    }

    pub fn set_width(&mut self, width: Dimension) -> &mut Self {
        self.value.try_write().unwrap().set_width(width);
        self
    }

    pub fn get_width(&self) -> Dimension {
        self.value.try_read().unwrap().get_width()
    }

    pub fn get_calculated_width(&self) -> i64 {
        self.value.try_read().unwrap().calculated_width
    }

    pub fn set_height(&mut self, height: Dimension) -> &mut Self {
        self.value.try_write().unwrap().set_height(height);
        self
    }

    pub fn get_height(&self) -> Dimension {
        self.value.try_read().unwrap().get_height()
    }

    pub fn get_calculated_height(&self) -> i64 {
        self.value.try_read().unwrap().calculated_height
    }

    pub fn set_geometry(
        &mut self,
        (x, y, width, height): (Dimension, Dimension, Dimension, Dimension),
    ) -> &mut Self {
        self.value
            .try_write()
            .unwrap()
            .set_geometry((x, y, width, height));
        self
    }

    pub fn get_geometry(&self) -> (Dimension, Dimension, Dimension, Dimension) {
        self.value.try_read().unwrap().get_geometry()
    }

    pub fn set_default_background_color(&mut self, color: Color) -> &mut Self {
        self.value
            .try_write()
            .unwrap()
            .set_default_background_color(color);
        self
    }

    pub fn get_default_background_color(&self) -> Color {
        self.value
            .try_read()
            .unwrap()
            .get_default_background_color()
    }

    pub fn set_default_foreground_color(&mut self, color: Color) -> &mut Self {
        self.value
            .try_write()
            .unwrap()
            .set_default_foreground_color(color);
        self
    }

    pub fn get_default_foreground_color(&self) -> Color {
        self.value
            .try_read()
            .unwrap()
            .get_default_foreground_color()
    }

    pub fn set_default_character(&mut self, character: char) -> &mut Self {
        self.value
            .try_write()
            .unwrap()
            .set_default_character(character);
        self
    }

    pub fn get_default_character(&self) -> char {
        self.value.try_read().unwrap().get_default_character()
    }

    pub fn set_text(&mut self, text: &str) -> &mut Self {
        self.value.try_write().unwrap().set_text(text);
        self
    }

    pub fn get_text(&self) -> String {
        self.value.try_read().unwrap().get_text()
    }

    pub fn set_pattern(&mut self, pattern: &str) -> &mut Self {
        self.value.try_write().unwrap().set_pattern(pattern);
        self
    }

    pub fn get_pattern(&self) -> String {
        self.value.try_read().unwrap().get_pattern()
    }

    pub fn set_animation(&mut self, animation: &Vec<&str>) -> &mut Self {
        self.value.try_write().unwrap().set_animation(animation);
        self
    }

    pub fn get_animation(&self) -> Vec<String> {
        self.value.try_read().unwrap().get_animation()
    }

    pub fn set_current_frame(&mut self, frame: i64) -> &mut Self {
        self.value.try_write().unwrap().set_current_frame(frame);
        self
    }

    pub fn shift_current_frame(&mut self, shift_val: i64) -> &mut Self {
        self.value
            .try_write()
            .unwrap()
            .shift_current_frame(shift_val);
        self
    }

    pub fn get_current_frame(&self) -> i64 {
        self.value.try_read().unwrap().get_current_frame()
    }

    pub fn set_style(&mut self, style: RendererObjectStyle) -> &mut Self {
        self.value.try_write().unwrap().set_style(style);
        self
    }

    pub fn get_style(&self) -> RendererObjectStyle {
        self.value.try_read().unwrap().get_style()
    }

    pub fn set_colors(&mut self, colors: Vec<ColorArea>) -> &mut Self {
        self.value.try_write().unwrap().set_colors(colors);
        self
    }

    pub fn get_colors(&self) -> Vec<ColorArea> {
        self.value.try_read().unwrap().get_colors()
    }

    pub fn add_color(&mut self, color: &mut ColorArea) -> &mut Self {
        self.value.try_write().unwrap().add_color(color);
        self
    }

    pub fn remove_color(&mut self, color: &ColorArea) -> &mut Self {
        self.value.try_write().unwrap().remove_color(color);
        self
    }

    pub fn process_geometry(
        &mut self,
        renderer_width: i64,
        renderer_height: i64,
        parent_width: i64,
        parent_height: i64,
        renderer_padding: i64,
    ) {
        self.value.try_write().unwrap().process_geometry(
            renderer_width,
            renderer_height,
            parent_width,
            parent_height,
            renderer_padding,
        );
    }

    pub fn get_buffer(
        &mut self,
        absolute_x: i64,
        absolute_y: i64,
        renderer_padding: i64,
    ) -> Vec<Vec<Pixel>> {
        self.value
            .try_write()
            .unwrap()
            .get_buffer(absolute_x, absolute_y, renderer_padding)
            .clone()
    }
    //end of getters/setters
    pub fn new() -> RendererObject {
        let renderer_object_value = RendererObjectValue {
            buffer: Vec::new(),
            x: Dimension::Auto,
            y: Dimension::Auto,
            width: Dimension::Auto,
            height: Dimension::Auto,
            calculated_width: 0,
            calculated_height: 0,
            absolute_x: 0,
            absolute_y: 0,
            parent_width: 0,
            parent_height: 0,
            renderer_width: 0,
            renderer_height: 0,
            update_size: true,
            update_content: true,
            parent: None,
            parent_location: 0,
            default_character: '\0',
            text: Vec::new(),
            pattern: Vec::new(),
            animation: Vec::new(),
            current_animation_frame: 0,
            colors: Vec::new(),
            default_background_color: Colors::TRANSPARENT,
            default_foreground_color: Colors::WHITE,
            children: Vec::new(),
            style: RendererObjectStyle::new(),
        };

        RendererObject {
            value: Arc::new_cyclic(|_| RwLock::new(renderer_object_value)),
        }
    }

    pub fn add_child(&mut self, child: &RendererObject) {
        let mut current_value = self.value.try_write().unwrap();
        let mut child_value = child.value.try_write().unwrap();
        child_value.parent = Some(Arc::downgrade(&self.value.clone()));
        child_value.parent_location = current_value.children.len();

        current_value.children.push(child.value.clone());

        current_value.update();
    }

    pub fn set_children(&mut self, children: &Vec<RendererObject>) {
        let mut current_value = self.value.try_write().unwrap();
        current_value.children = children.iter().map(|val| val.value.clone()).collect();
        let children = current_value.children.clone();
        for i in 0..children.len() {
            children[i].try_write().unwrap().parent_location = i;
            children[i].try_write().unwrap().parent = Some(Arc::downgrade(&self.value.clone()));
        }
        current_value.update();
    }

    pub fn get_children(&self) -> Vec<RendererObject> {
        self.value
            .try_read()
            .unwrap()
            .children
            .iter()
            .map(|val| RendererObject { value: val.clone() })
            .collect()
    }

    pub fn remove_child(&mut self, renderer_object: &RendererObject) {
        self.value
            .try_write()
            .unwrap()
            .children
            .remove(renderer_object.value.try_read().unwrap().parent_location);

        let children = self.value.try_write().unwrap().children.clone();
        for i in 0..children.len() {
            children[i].try_write().unwrap().parent_location = i;
        }

        self.value.try_write().unwrap().update();
    }
}
