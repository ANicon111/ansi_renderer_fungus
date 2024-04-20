use crate::{
    color::Color,
    geometry::Dimension,
    renderer_object_style::{AlignmentX, AlignmentY},
};

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum ColorLayer {
    Background,
    Foreground,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct ColorArea {
    pub color: Color,
    pub layer: ColorLayer,
    pub x: Dimension,
    pub y: Dimension,
    pub width: Dimension,
    pub height: Dimension,
    pub external_alignment_x: Option<AlignmentX>,
    pub external_alignment_y: Option<AlignmentY>,
    pub renderer_object_index: usize,
}

impl ColorArea {
    pub fn new(color: Color, layer: ColorLayer) -> ColorArea {
        ColorArea {
            color,
            layer,
            x: Dimension::Auto,
            y: Dimension::Auto,
            width: Dimension::Auto,
            height: Dimension::Auto,
            external_alignment_x: None,
            external_alignment_y: None,
            renderer_object_index: 0,
        }
    }

    pub fn set_geometry(
        &mut self,
        (x, y, width, height): (Dimension, Dimension, Dimension, Dimension),
    ) -> &mut Self {
        (self.x, self.y, self.width, self.height) = (x, y, width, height);
        self
    }

    pub fn set_alignment(
        &mut self,
        external_alignment_x: Option<AlignmentX>,
        external_alignment_y: Option<AlignmentY>,
    ) -> &mut Self {
        (self.external_alignment_x, self.external_alignment_y) =
            (external_alignment_x, external_alignment_y);
        self
    }
}
