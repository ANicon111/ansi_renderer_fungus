use crate::renderer_object_border::{Border, Borders};

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum AlignmentX {
    Left,
    Center,
    Right,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum AlignmentY {
    Top,
    Center,
    Bottom,
}

#[derive(Clone)]
pub struct RendererObjectStyle {
    pub border: Border,
    pub internal_alignment_x: AlignmentX,
    pub internal_alignment_y: AlignmentY,
    pub external_alignment_x: Option<AlignmentX>,
    pub external_alignment_y: Option<AlignmentY>,
    pub stack_children: bool,
    pub stack_colors: bool,
}

impl RendererObjectStyle {
    pub fn new() -> RendererObjectStyle {
        RendererObjectStyle {
            border: Borders::EMPTY,
            internal_alignment_x: AlignmentX::Left,
            internal_alignment_y: AlignmentY::Top,
            external_alignment_x: None,
            external_alignment_y: None,
            stack_children: false,
            stack_colors: false,
        }
    }
}
