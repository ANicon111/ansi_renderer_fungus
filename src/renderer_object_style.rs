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
}

impl RendererObjectStyle {
    pub fn new() -> RendererObjectStyle {
        RendererObjectStyle {
            border: Borders::EMPTY,
            internal_alignment_x: AlignmentX::Left,
            internal_alignment_y: AlignmentY::Top,
            external_alignment_x: None,
            external_alignment_y: None,
        }
    }

    pub fn set_external_alignment(
        &mut self,
        x: Option<AlignmentX>,
        y: Option<AlignmentY>,
    ) -> &mut Self {
        (self.external_alignment_x, self.external_alignment_y) = (x, y);
        self
    }

    pub fn set_internal_alignment(&mut self, x: AlignmentX, y: AlignmentY) -> &mut Self {
        (self.internal_alignment_x, self.internal_alignment_y) = (x, y);
        self
    }

    pub fn set_border(&mut self, border: Border) -> &mut Self {
        self.border = border;
        self
    }
}
