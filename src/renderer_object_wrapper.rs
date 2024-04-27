use std::sync::{Arc, RwLock};

use crate::{
    color::Color,
    color_area::ColorArea,
    colors::Colors,
    geometry::Dimension,
    renderer_object_style::RendererObjectStyle,
    renderer_object_value::{RendererObjectValue, UpdateValueSignaler},
};

#[derive(Clone)]
pub struct RendererObject {
    pub(crate) value: Arc<RwLock<RendererObjectValue>>,
    new_value: Arc<RwLock<RendererObjectValue>>,
}

impl RendererObject {
    //getters and setters
    pub fn set_x(&mut self, x: Dimension) -> &mut Self {
        self.value
            .read()
            .unwrap()
            .update_value_signal
            .write()
            .unwrap()
            .update();
        self.new_value.write().unwrap().x = x;
        self
    }

    pub fn get_x(&self) -> Dimension {
        self.new_value.read().unwrap().x
    }

    pub fn set_y(&mut self, y: Dimension) -> &mut Self {
        self.value
            .read()
            .unwrap()
            .update_value_signal
            .write()
            .unwrap()
            .update();
        self.new_value.write().unwrap().y = y;
        self
    }

    pub fn get_y(&self) -> Dimension {
        self.new_value.read().unwrap().y
    }

    pub fn set_width(&mut self, width: Dimension) -> &mut Self {
        self.value
            .read()
            .unwrap()
            .update_value_signal
            .write()
            .unwrap()
            .update();
        self.new_value.write().unwrap().width = width;
        self
    }

    pub fn get_width(&self) -> Dimension {
        self.new_value.read().unwrap().width
    }

    pub fn set_height(&mut self, height: Dimension) -> &mut Self {
        self.value
            .read()
            .unwrap()
            .update_value_signal
            .write()
            .unwrap()
            .update();
        self.new_value.write().unwrap().height = height;
        self
    }

    pub fn get_height(&self) -> Dimension {
        self.new_value.read().unwrap().height
    }

    pub fn set_geometry(
        &mut self,
        (x, y, width, height): (Dimension, Dimension, Dimension, Dimension),
    ) -> &mut Self {
        {
            self.value
                .read()
                .unwrap()
                .update_value_signal
                .write()
                .unwrap()
                .update();
            let mut val = self.new_value.write().unwrap();

            (val.x, val.y, val.width, val.height) = (x, y, width, height);
        }
        self
    }

    pub fn get_geometry(&self) -> (Dimension, Dimension, Dimension, Dimension) {
        let val = self.new_value.read().unwrap();
        (val.x, val.y, val.width, val.height)
    }

    pub fn set_default_background_color(&mut self, color: Color) -> &mut Self {
        self.value
            .read()
            .unwrap()
            .update_value_signal
            .write()
            .unwrap()
            .update();
        self.new_value.write().unwrap().default_background_color = color;
        self
    }

    pub fn get_default_background_color(&self) -> Color {
        self.new_value.read().unwrap().default_background_color
    }

    pub fn set_default_foreground_color(&mut self, color: Color) -> &mut Self {
        self.value
            .read()
            .unwrap()
            .update_value_signal
            .write()
            .unwrap()
            .update();
        self.new_value.write().unwrap().default_foreground_color = color;
        self
    }

    pub fn get_default_foreground_color(&self) -> Color {
        self.new_value.read().unwrap().default_foreground_color
    }

    pub fn set_default_character(&mut self, character: char) -> &mut Self {
        self.value
            .read()
            .unwrap()
            .update_value_signal
            .write()
            .unwrap()
            .update();
        self.new_value.write().unwrap().default_character = character;
        self
    }

    pub fn get_default_character(&self) -> char {
        self.new_value.read().unwrap().default_character
    }

    pub fn set_text(&mut self, text: &str) -> &mut Self {
        {
            self.value
                .read()
                .unwrap()
                .update_value_signal
                .write()
                .unwrap()
                .update();
            let mut val = self.new_value.write().unwrap();
            val.text = text
                .replace("\r\n", "\n")
                .split('\n')
                .map(|s| s.to_string().chars().collect())
                .collect();
            val.changed_text = true;
        }
        self
    }

    pub fn get_text(&self) -> String {
        self.new_value
            .read()
            .unwrap()
            .text
            .iter()
            .map(|val| val.iter().collect())
            .collect::<Vec<String>>()
            .join("\n")
    }

    pub fn set_pattern(&mut self, pattern: &str) -> &mut Self {
        {
            self.value
                .read()
                .unwrap()
                .update_value_signal
                .write()
                .unwrap()
                .update();
            let mut val = self.new_value.write().unwrap();
            val.pattern = pattern
                .replace("\r\n", "\n")
                .split('\n')
                .map(|s| s.to_string().chars().collect())
                .collect();
            val.changed_pattern = true;
        }
        self
    }

    pub fn get_pattern(&self) -> String {
        self.new_value
            .read()
            .unwrap()
            .pattern
            .iter()
            .map(|val| val.iter().collect())
            .collect::<Vec<String>>()
            .join("\n")
    }

    pub fn set_animation(&mut self, animation: &Vec<&str>) -> &mut Self {
        {
            self.value
                .read()
                .unwrap()
                .update_value_signal
                .write()
                .unwrap()
                .update();
            let mut val = self.new_value.write().unwrap();
            val.animation = animation
                .iter()
                .map(|text| {
                    text.replace("\r\n", "\n")
                        .split('\n')
                        .map(|s| s.to_string().chars().collect())
                        .collect()
                })
                .collect();
            val.changed_animation = true;
        }
        self
    }

    pub fn set_animation_from_string(&mut self, animation: &str) -> &mut Self {
        {
            self.value
                .read()
                .unwrap()
                .update_value_signal
                .write()
                .unwrap()
                .update();
            let mut val = self.new_value.write().unwrap();
            val.animation = animation
                .replace("\r\n", "\n")
                .split("\n<FrameSeparator>\n")
                .map(|text| {
                    text.split('\n')
                        .map(|s| s.to_string().chars().collect())
                        .collect()
                })
                .collect();
            val.changed_animation = true;
        }
        self
    }

    pub fn get_animation(&self) -> Vec<String> {
        self.new_value
            .read()
            .unwrap()
            .animation
            .iter()
            .map(|val| {
                val.iter()
                    .map(|val| val.iter().collect())
                    .collect::<Vec<String>>()
                    .join("\n")
            })
            .collect()
    }

    pub fn set_animated_pattern(&mut self, animated_pattern: &Vec<&str>) -> &mut Self {
        {
            self.value
                .read()
                .unwrap()
                .update_value_signal
                .write()
                .unwrap()
                .update();
            let mut val = self.new_value.write().unwrap();
            val.animated_pattern = animated_pattern
                .iter()
                .map(|text| {
                    text.replace("\r\n", "\n")
                        .split('\n')
                        .map(|s| s.to_string().chars().collect())
                        .collect()
                })
                .collect();
            val.changed_animated_pattern = true;
        }
        self
    }

    pub fn set_animated_pattern_from_string(&mut self, animated_pattern: &str) -> &mut Self {
        {
            self.value
                .read()
                .unwrap()
                .update_value_signal
                .write()
                .unwrap()
                .update();
            let mut val = self.new_value.write().unwrap();
            val.animated_pattern = animated_pattern
                .replace("\r\n", "\n")
                .split("\n<FrameSeparator>\n")
                .map(|text| {
                    text.split('\n')
                        .map(|s| s.to_string().chars().collect())
                        .collect()
                })
                .collect();
            val.changed_animated_pattern = true;
        }
        self
    }

    pub fn get_animated_pattern(&self) -> Vec<String> {
        self.new_value
            .read()
            .unwrap()
            .animated_pattern
            .iter()
            .map(|val| {
                val.iter()
                    .map(|val| val.iter().collect())
                    .collect::<Vec<String>>()
                    .join("\n")
            })
            .collect()
    }

    pub fn get_animated_pattern_as_string(&self) -> String {
        self.new_value
            .read()
            .unwrap()
            .animated_pattern
            .iter()
            .map(|val| {
                val.iter()
                    .map(|val| val.iter().collect())
                    .collect::<Vec<String>>()
                    .join("\n")
            })
            .collect::<Vec<String>>()
            .join("\n<FrameSeparator>\n")
    }

    pub fn set_current_animation_frame(&mut self, frame: i64) -> &mut Self {
        self.value
            .read()
            .unwrap()
            .update_value_signal
            .write()
            .unwrap()
            .update();
        self.new_value.write().unwrap().current_animation_frame = frame;
        self
    }

    pub fn shift_current_animation_frame(&mut self, shift_val: i64) -> &mut Self {
        self.value
            .read()
            .unwrap()
            .update_value_signal
            .write()
            .unwrap()
            .update();
        self.new_value.write().unwrap().current_animation_frame += shift_val;
        self
    }

    pub fn get_current_animation_frame(&self) -> i64 {
        self.new_value.read().unwrap().current_animation_frame
    }

    pub fn set_current_animated_pattern_frame(&mut self, frame: i64) -> &mut Self {
        self.value
            .read()
            .unwrap()
            .update_value_signal
            .write()
            .unwrap()
            .update();
        self.new_value
            .write()
            .unwrap()
            .current_animated_pattern_frame = frame;
        self
    }

    pub fn shift_current_animated_pattern_frame(&mut self, shift_val: i64) -> &mut Self {
        self.value
            .read()
            .unwrap()
            .update_value_signal
            .write()
            .unwrap()
            .update();
        self.new_value
            .write()
            .unwrap()
            .current_animated_pattern_frame += shift_val;
        self
    }

    pub fn get_current_animated_pattern_frame(&self) -> i64 {
        self.new_value
            .read()
            .unwrap()
            .current_animated_pattern_frame
    }

    pub fn set_style(&mut self, style: RendererObjectStyle) -> &mut Self {
        {
            self.value
                .read()
                .unwrap()
                .update_value_signal
                .write()
                .unwrap()
                .update();
            let mut val = self.new_value.write().unwrap();
            val.style = style;
            val.changed_style = true;
        }
        self
    }

    pub fn get_style(&self) -> RendererObjectStyle {
        self.new_value.read().unwrap().style.clone()
    }

    pub fn set_colors(&mut self, colors: Vec<ColorArea>) -> &mut Self {
        {
            self.value
                .read()
                .unwrap()
                .update_value_signal
                .write()
                .unwrap()
                .update();
            let mut val = self.new_value.write().unwrap();
            val.colors = colors;
            val.changed_colors = true;
        }
        self
    }

    pub fn get_colors(&self) -> Vec<ColorArea> {
        self.new_value.read().unwrap().colors.clone()
    }

    pub fn add_color(&mut self, color: &mut ColorArea) -> &mut Self {
        {
            self.value
                .read()
                .unwrap()
                .update_value_signal
                .write()
                .unwrap()
                .update();
            let mut val = self.new_value.write().unwrap();
            color.renderer_object_index = val.colors.len();
            val.colors.push(color.clone());
            val.changed_colors = true;
        }
        self
    }

    pub fn remove_color(&mut self, color: &ColorArea) -> &mut Self {
        self.value
            .read()
            .unwrap()
            .update_value_signal
            .write()
            .unwrap()
            .update();
        self.remove_color_at(color.renderer_object_index);
        self
    }

    pub fn remove_color_at(&mut self, index: usize) -> &mut Self {
        {
            self.value
                .read()
                .unwrap()
                .update_value_signal
                .write()
                .unwrap()
                .update();
            let mut val = self.new_value.write().unwrap();
            val.colors.remove(index);
            for i in 0..val.colors.len() {
                val.colors[i].renderer_object_index = i;
            }
            val.changed_colors = true;
        }
        self
    }

    pub fn set_children(&mut self, children: Vec<RendererObject>) {
        self.value
            .read()
            .unwrap()
            .update_value_signal
            .write()
            .unwrap()
            .update();
        let mut current_value = self.new_value.write().unwrap();
        for i in 0..children.len() {
            {
                let rw_lock = &children[i].new_value;
                let mut child = rw_lock.write().unwrap();
                child.parent_location = i;
                child.update_value_signal.write().unwrap().parent =
                    Some(current_value.update_value_signal.clone());
            }
            current_value.children.push(children[i].value.clone());
        }
        current_value.changed_children = true;
    }

    pub fn get_children(&self) -> Vec<RendererObject> {
        self.value
            .read()
            .unwrap()
            .update_value_signal
            .write()
            .unwrap()
            .update();
        self.new_value
            .read()
            .unwrap()
            .children
            .iter()
            .map(|val| RendererObject {
                value: val.clone(),
                new_value: val.read().unwrap().new_self.clone().unwrap(),
            })
            .collect()
    }

    pub fn add_child(&mut self, child: RendererObject) {
        self.value
            .read()
            .unwrap()
            .update_value_signal
            .write()
            .unwrap()
            .update();
        let mut current_value = self.new_value.write().unwrap();
        {
            let mut child_value = child.value.write().unwrap();
            child_value.update_value_signal.write().unwrap().parent =
                Some(current_value.update_value_signal.clone());
            child_value.parent_location = current_value.children.len();
        }
        current_value.children.push(child.value);
        current_value.changed_children = true;
    }

    pub fn remove_child(&mut self, renderer_object: &RendererObject) {
        self.value
            .read()
            .unwrap()
            .update_value_signal
            .write()
            .unwrap()
            .update();
        self.remove_child_at(renderer_object.value.read().unwrap().parent_location);
    }

    pub fn remove_child_at(&mut self, index: usize) {
        self.value
            .read()
            .unwrap()
            .update_value_signal
            .write()
            .unwrap()
            .update();
        let mut val = self.new_value.write().unwrap();
        val.children.remove(index);

        let children = val.children.clone();
        for i in 0..children.len() {
            children[i].write().unwrap().parent_location = i;
        }

        val.changed_children = true;
    }
    //end of getters/setters

    pub fn new() -> RendererObject {
        let mut renderer_object_value = RendererObjectValue {
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
            update_value_signal: Arc::new(RwLock::new(UpdateValueSignaler::new())),
            parent_location: 0,
            default_character: '\0',
            text: Vec::new(),
            changed_text: false,
            pattern: Vec::new(),
            changed_pattern: false,
            animation: Vec::new(),
            changed_animation: false,
            current_animation_frame: 0,
            animated_pattern: Vec::new(),
            changed_animated_pattern: false,
            current_animated_pattern_frame: 0,
            colors: Vec::new(),
            changed_colors: false,
            default_background_color: Colors::TRANSPARENT,
            default_foreground_color: Colors::WHITE,
            children: Vec::new(),
            changed_children: false,
            style: RendererObjectStyle::new(),
            changed_style: false,
            new_self: None,
        };
        renderer_object_value.new_self = Some(Arc::new(RwLock::new(renderer_object_value.clone())));

        RendererObject {
            new_value: renderer_object_value.new_self.clone().unwrap(),
            value: Arc::new(RwLock::new(renderer_object_value)),
        }
    }
}
