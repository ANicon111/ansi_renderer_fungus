use std::time::{Duration, Instant};

use crate::{
    colors::{Color, ColorArea, ColorLayer, Colors},
    geometry::{Dimension, Dimensions},
    renderer::Renderer,
    renderer_object::RendererObject,
    renderer_object_border::Borders,
    renderer_object_style::{AlignmentX, AlignmentY, RendererObjectStyle},
};

pub fn run_tests() {
    string_colors();
    html_colors();
    color_setters();
    color_overlay();
    color_inverted();
    html_dimension();
    renderer_test();
}

/* spellchecker: disable */
fn string_colors() {
    assert_eq!(
        Color::from_rgb(95, 158, 160),
        Color::from_name(" caDeT_Blue ").unwrap()
    );
}

fn html_colors() {
    assert_eq!(
        Color::from_rgb(95, 158, 160),
        Color::from_html(" caDeT_Blue ").unwrap()
    );
    assert_eq!(
        Color::from_rgba(95, 158, 160, 254.0 / 255.0),
        Color::from_html(" # 5f9Ea0fe ").unwrap()
    );
    assert_eq!(
        Color::from_rgb(95, 158, 160),
        Color::from_html(" # 5f9Ea0 ").unwrap()
    );
    assert_eq!(
        Color::from_rgba(0x66, 0x88, 0xaa, 204.0 / 255.0),
        Color::from_html(" # 68ac ").unwrap()
    );
    assert_eq!(
        Color::from_rgb(0x66, 0x88, 0xaa),
        Color::from_html(" # 68a ").unwrap()
    );
    assert_eq!(
        Color::from_rgba(95, 158, 160, 0.997),
        Color::from_html(" rgba ( 95 , 158 , 160 , 0.997 ) ").unwrap()
    );
    assert_eq!(
        Color::from_rgba(95, 158, 160, 0.997),
        Color::from_html(" rgba ( 95 , 158 , 160 , 99.7 % ) ").unwrap()
    );
    assert_eq!(
        Color::from_rgb(95, 158, 160),
        Color::from_html(" rgb ( 95 , 158 , 160 ) ").unwrap()
    );
    assert_eq!(
        Color::from_hsla(120.0, 0.6, 0.6, 0.997),
        Color::from_html(" hsla ( 120.0 , 0.6 , 0.6, 0.997 ) ").unwrap()
    );
    assert_eq!(
        Color::from_hsl(120.0, 0.6, 0.6),
        Color::from_html(" hsl (120.0 , 0.6 , 0.6 ) ").unwrap()
    );
}

fn color_setters() {
    assert_eq!(
        Color::from_rgba(156, 78, 231, 0.997),
        Color::from_rgba(0, 78, 231, 0.997).with_red(156),
    );
    assert_eq!(
        Color::from_rgba(156, 78, 231, 0.997),
        Color::from_rgba(156, 0, 231, 0.997).with_green(78),
    );
    assert_eq!(
        Color::from_rgba(156, 78, 231, 0.997),
        Color::from_rgba(156, 78, 0, 0.997).with_blue(231),
    );
    assert_eq!(
        Color::from_rgba(156, 78, 231, 0.997),
        Color::from_rgba(156, 78, 231, 0.0).with_alpha(0.997),
    );
    assert_eq!(
        Color::from_hsla(120.0, 0.6, 0.6, 0.997),
        Color::from_hsla(0.0, 0.6, 0.6, 0.997).with_hue(120.0),
    );
    assert_eq!(
        Color::from_hsla(120.0, 0.5, 0.5, 0.997),
        Color::from_hsla(120.0, 0.25, 0.5, 0.997).with_saturation(0.5),
    );
    assert_eq!(
        Color::from_hsla(120.0, 0.5, 0.5, 0.997),
        Color::from_hsla(120.0, 0.5, 0.25, 0.997).with_luminosity(0.5),
    );
}

fn color_overlay() {
    assert_eq!(
        Color::from_rgba(127, 127, 0, 0.75),
        Color::from_rgba(254, 0, 0, 0.5).with_overlay(Color::from_rgba(0, 254, 0, 0.5)),
    );
}

fn color_inverted() {
    assert_eq!(Colors::MAGENTA, Colors::LIME.inverted());
}

fn html_dimension() {
    assert_eq!(
        Dimension::from_html(" 10 px ").unwrap(),
        Dimension::Pixel(10)
    );
    assert_eq!(
        Dimension::from_html(" 10.5 % ").unwrap(),
        Dimension::Percent(10.5)
    );
    assert_eq!(
        Dimension::from_html(" 10.5 vW ").unwrap(),
        Dimension::VW(10.5)
    );
    assert_eq!(
        Dimension::from_html(" 10.5 vH ").unwrap(),
        Dimension::VH(10.5)
    );
    assert_eq!(
        Dimension::from_html(" 10.5 vmIn ").unwrap(),
        Dimension::VMin(10.5)
    );
    assert_eq!(
        Dimension::from_html(" 10.5 vmAx ").unwrap(),
        Dimension::VMax(10.5)
    );
    assert_eq!(
        Dimension::from_html(" 10.5 pW ").unwrap(),
        Dimension::PW(10.5)
    );
    assert_eq!(
        Dimension::from_html(" 10.5 pH ").unwrap(),
        Dimension::PH(10.5)
    );
    assert_eq!(
        Dimension::from_html(" 10.5 pMin ").unwrap(),
        Dimension::PMin(10.5)
    );
    assert_eq!(
        Dimension::from_html(" 10.5 pMax ").unwrap(),
        Dimension::PMax(10.5)
    );
    assert_eq!(Dimension::from_html(" 10 ").unwrap(), Dimension::Pixel(10));
    assert_eq!(Dimension::from_html(" auto ").unwrap(), Dimension::Auto);
}

fn renderer_test() {
    let mut renderer = Renderer::new();
    let water_color = Colors::DARK_SLATE_BLUE.with_luminosity(0.1).with_alpha(0.7);

    let mut boat_right = RendererObject::new();
    boat_right.set_text(include_str!("test_assets/boat_right.txt"));
    boat_right.set_style({
        let mut style = RendererObjectStyle::new();
        style.external_alignment_x = Some(AlignmentX::Left);
        style
    });
    boat_right.set_colors(vec![
        ColorArea::with_geometry(
            Colors::SADDLE_BROWN.with_luminosity(0.15),
            ColorLayer::Foreground,
            Dimensions::pixel(0, 0, 18, 9),
        ),
        ColorArea::with_geometry(
            water_color,
            ColorLayer::Foreground,
            Dimensions::pixel(0, 8, 18, 1),
        ),
        ColorArea::with_geometry(
            Colors::SADDLE_BROWN.with_luminosity(0.2),
            ColorLayer::Foreground,
            Dimensions::pixel(5, 1, 1, 6),
        ),
        ColorArea::with_geometry(
            Colors::SADDLE_BROWN.with_luminosity(0.2),
            ColorLayer::Foreground,
            Dimensions::pixel(8, 0, 2, 7),
        ),
        ColorArea::with_geometry(
            Colors::SADDLE_BROWN.with_luminosity(0.2),
            ColorLayer::Foreground,
            Dimensions::pixel(13, 2, 1, 5),
        ),
        ColorArea::with_geometry(
            Color::from_rgb(140, 120, 100),
            ColorLayer::Foreground,
            Dimensions::pixel(5, 2, 1, 1),
        ),
        ColorArea::with_geometry(
            Color::from_rgb(140, 120, 100),
            ColorLayer::Foreground,
            Dimensions::pixel(5, 4, 1, 2),
        ),
        ColorArea::with_geometry(
            Color::from_rgb(140, 120, 100),
            ColorLayer::Foreground,
            Dimensions::pixel(9, 2, 3, 3),
        ),
        ColorArea::with_geometry(
            Color::from_rgb(140, 120, 100),
            ColorLayer::Foreground,
            Dimensions::pixel(13, 3, 2, 3),
        ),
        ColorArea::with_geometry(
            water_color,
            ColorLayer::Foreground,
            Dimensions::pixel(18, 7, 2, 1),
        ),
    ]);
    boat_right.set_y(Dimension::Pixel(-2));
    boat_right.set_default_foreground_color(Colors::INVALID);

    let mut boat_left = RendererObject::new();
    boat_left.set_text(include_str!("test_assets/boat_left.txt"));
    boat_left.set_style({
        let mut style = RendererObjectStyle::new();
        style.external_alignment_x = Some(AlignmentX::Right);
        style
    });
    boat_left.set_colors(vec![
        ColorArea::with_geometry(
            Colors::SADDLE_BROWN.with_luminosity(0.15),
            ColorLayer::Foreground,
            Dimensions::pixel(2, 0, 18, 9),
        ),
        ColorArea::with_geometry(
            water_color,
            ColorLayer::Foreground,
            Dimensions::pixel(2, 8, 18, 1),
        ),
        ColorArea::with_geometry(
            Colors::SADDLE_BROWN.with_luminosity(0.20),
            ColorLayer::Foreground,
            Dimensions::pixel(14, 1, 1, 6),
        ),
        ColorArea::with_geometry(
            Colors::SADDLE_BROWN.with_luminosity(0.20),
            ColorLayer::Foreground,
            Dimensions::pixel(10, 0, 2, 7),
        ),
        ColorArea::with_geometry(
            Colors::SADDLE_BROWN.with_luminosity(0.20),
            ColorLayer::Foreground,
            Dimensions::pixel(6, 2, 1, 5),
        ),
        ColorArea::with_geometry(
            Color::from_rgb(140, 120, 100),
            ColorLayer::Foreground,
            Dimensions::pixel(14, 2, 1, 1),
        ),
        ColorArea::with_geometry(
            Color::from_rgb(140, 120, 100),
            ColorLayer::Foreground,
            Dimensions::pixel(14, 4, 1, 2),
        ),
        ColorArea::with_geometry(
            Color::from_rgb(140, 120, 100),
            ColorLayer::Foreground,
            Dimensions::pixel(9, 2, 3, 3),
        ),
        ColorArea::with_geometry(
            Color::from_rgb(140, 120, 100),
            ColorLayer::Foreground,
            Dimensions::pixel(5, 3, 2, 3),
        ),
        ColorArea::with_geometry(
            water_color,
            ColorLayer::Foreground,
            Dimensions::pixel(0, 7, 2, 1),
        ),
    ]);
    boat_left.set_y(Dimension::Pixel(-2));
    boat_left.set_default_foreground_color(Colors::INVALID);

    let mut waves = RendererObject::new();
    waves.set_pattern(" ▁▂▃▃▄▄▄▃▃▂▁        ▁▃▅▆▇▇▇▆▅▃▁     ▁▂▃▄▄▅▅▅▄▄▃▂▁       ");
    waves.set_width(Dimension::PW(500.0));
    waves.set_height(Dimension::Pixel(1));
    waves.set_y(Dimension::Pixel(-3));
    waves.set_colors(vec![ColorArea::new(water_color, ColorLayer::Foreground)]);
    waves.set_default_foreground_color(Colors::INVALID);

    let mut moon = RendererObject::new();
    moon.set_animation(&mut vec![
        "▗▟▀▔
█▌
▝▜▄▁",
        "▗▟█▀
██
▝▜█▄",
        "▗▟█▛
███
▝▜█▙",
        "▗▟██▖
████▌
▝▜██▘",
        "▗▟██▙▖
██████
▝▜██▛▘",
        " ▗██▙▖
 ▐████
 ▝██▛▘",
        "  ▜█▙▖
   ███
  ▟█▛▘",
        "  ▀█▙▖
    ██
  ▄█▛▘",
        "  ▔▀▙▖
    ▐█
  ▁▄▛▘",
        "▗▞▔▔▚▖
▌    ▐
▝▚▁▁▞▘",
    ]);
    moon.set_colors(vec![ColorArea::new(
        Colors::GOLDENROD.with_luminosity(0.8),
        ColorLayer::Foreground,
    )]);

    let mut title = RendererObject::new();
    title.set_default_character(' ');
    title.set_text(
        "
   Night          Seascapes  
",
    );
    title.set_style({
        let mut style = RendererObjectStyle::new();
        style.external_alignment_x = Some(AlignmentX::Center);
        style.external_alignment_y = Some(AlignmentY::Center);
        style.internal_alignment_x = AlignmentX::Center;
        style.internal_alignment_y = AlignmentY::Center;
        let mut border = Borders::ROUNDED;
        border.set_background(Colors::BLACK.with_alpha(0.5));
        style.border = border;
        style
    });
    title.set_colors(vec![
        ColorArea::new(Colors::BLACK.with_alpha(0.5), ColorLayer::Background),
        ColorArea::with_geometry(
            Colors::SLATE_BLUE,
            ColorLayer::Foreground,
            Dimensions::pixel(8, 0, 9, 1),
        ),
        ColorArea::with_geometry(
            Colors::DARK_GOLDENROD,
            ColorLayer::Foreground,
            Dimensions::pixel(-9, 0, 5, 1),
        ),
    ]);
    title.set_children(&vec![moon.clone()]);

    let mut root = RendererObject::new();
    root.set_pattern(include_str!("test_assets/stars.txt"));
    root.set_width(Dimension::VW(100.0));
    root.set_height(Dimension::VH(100.0));
    root.set_style({
        let mut style = RendererObjectStyle::new();
        style.internal_alignment_x = AlignmentX::Center;
        style.internal_alignment_y = AlignmentY::Bottom;
        style.external_alignment_x = Some(AlignmentX::Center);
        style.external_alignment_y = Some(AlignmentY::Center);
        style
    });
    root.set_colors(vec![
        ColorArea::with_geometry(
            Colors::BLACK,
            ColorLayer::Background,
            (
                Dimension::Auto,
                Dimension::Auto,
                Dimension::Auto,
                Dimension::Auto,
            ),
        ),
        ColorArea::with_geometry(
            Colors::WHITE,
            ColorLayer::Foreground,
            (
                Dimension::Auto,
                Dimension::Auto,
                Dimension::Auto,
                Dimension::Auto,
            ),
        ),
        ColorArea::with_geometry(
            Colors::BLACK,
            ColorLayer::Foreground,
            (
                Dimension::Pixel(0),
                Dimension::Pixel(0),
                Dimension::Auto,
                Dimension::Pixel(1),
            ),
        ),
        ColorArea::with_geometry(
            Colors::BLACK.with_luminosity(0.3),
            ColorLayer::Foreground,
            (
                Dimension::Pixel(0),
                Dimension::Pixel(0),
                Dimension::Auto,
                Dimension::Pixel(3),
            ),
        ),
        ColorArea::with_geometry(
            water_color,
            ColorLayer::Foreground,
            (
                Dimension::Pixel(0),
                Dimension::Pixel(0),
                Dimension::Auto,
                Dimension::Pixel(4),
            ),
        ),
        ColorArea::with_geometry(
            water_color,
            ColorLayer::Background,
            (
                Dimension::Pixel(0),
                Dimension::Pixel(0),
                Dimension::Auto,
                Dimension::Pixel(3),
            ),
        ),
    ]);
    root.set_children(&mut vec![
        boat_left.clone(),
        boat_right.clone(),
        waves.clone(),
        title.clone(),
    ]);
    renderer.set_object(Some(&root));

    let mut frame_count = 0;

    let mut previous_frame_count = 0;
    let mut min_frames_per_second = i32::MAX;
    let mut max_frames_per_second = 0;

    let mut previous_frame_count_fourth = 0;
    let mut min_frames_per_fourth = i32::MAX;
    let mut max_frames_per_fourth = 0;

    let mut time_step = Instant::now();

    renderer.disable_output = false;
    for _second in 0..10 {
        for _quarter_second in 0..4 {
            while Instant::now().duration_since(time_step) < Duration::from_millis(250) {
                renderer.draw(false);
                frame_count += 1;
                moon.set_current_frame((frame_count % 200) as u64);
                boat_left.set_x(Dimension::PW(-(frame_count % 200) as f64 + 50.0));
                boat_right.set_x(Dimension::PW((frame_count % 200) as f64 - 50.0));
                waves.set_x(Dimension::PW(
                    ((200 - frame_count % 400) as f64).abs() / 2.0,
                ));
            }
            time_step = Instant::now();
            min_frames_per_fourth =
                min_frames_per_fourth.min(frame_count - previous_frame_count_fourth);
            max_frames_per_fourth =
                max_frames_per_fourth.max(frame_count - previous_frame_count_fourth);
            previous_frame_count_fourth = frame_count;
        }
        min_frames_per_second = min_frames_per_second.min(frame_count - previous_frame_count);
        max_frames_per_second = max_frames_per_second.max(frame_count - previous_frame_count);
        previous_frame_count = frame_count;
    }

    println!("\n\n\nFrames rendered in 10 seconds: {}", frame_count);
    println!("Minimum frames per 1/4s: {}", min_frames_per_fourth);
    println!("Maximum frames per 1/4s: {}", max_frames_per_fourth);
    println!("Minimum frames per 1s: {}", min_frames_per_second);
    println!("Maximum frames per 1s: {}", max_frames_per_second);
}
