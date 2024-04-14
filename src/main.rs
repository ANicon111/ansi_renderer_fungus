use tests::run_tests;

pub mod colors;
pub mod geometry;
pub mod misc;
pub mod pixel;
pub mod renderer;
pub mod renderer_object;
pub mod renderer_object_style;
pub mod tests;
pub mod renderer_object_border;

fn main() {
    run_tests();
}
