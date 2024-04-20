use tests::run_tests;

pub mod color;
pub mod color_area;
pub mod colors;
pub mod geometry;
pub mod misc;
pub mod pixel;
pub mod renderer;
pub mod renderer_object;
pub mod renderer_object_border;
pub mod renderer_object_style;
pub mod tests;

fn main() {
    run_tests();
}