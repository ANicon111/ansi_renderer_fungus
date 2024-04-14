pub struct Dimensions;

impl Dimensions {
    pub fn auto() -> (Dimension, Dimension, Dimension, Dimension) {
        (
            Dimension::Auto,
            Dimension::Auto,
            Dimension::Auto,
            Dimension::Auto,
        )
    }
    pub fn pixel(
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) -> (Dimension, Dimension, Dimension, Dimension) {
        (
            Dimension::Pixel(x),
            Dimension::Pixel(y),
            Dimension::Pixel(width),
            Dimension::Pixel(height),
        )
    }
    pub fn percent(
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    ) -> (Dimension, Dimension, Dimension, Dimension) {
        (
            Dimension::Percent(x),
            Dimension::Percent(y),
            Dimension::Percent(width),
            Dimension::Percent(height),
        )
    }
    pub fn pw(
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    ) -> (Dimension, Dimension, Dimension, Dimension) {
        (
            Dimension::PW(x),
            Dimension::PW(y),
            Dimension::PW(width),
            Dimension::PW(height),
        )
    }
    pub fn ph(
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    ) -> (Dimension, Dimension, Dimension, Dimension) {
        (
            Dimension::PH(x),
            Dimension::PH(y),
            Dimension::PH(width),
            Dimension::PH(height),
        )
    }
    pub fn pmin(
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    ) -> (Dimension, Dimension, Dimension, Dimension) {
        (
            Dimension::PMin(x),
            Dimension::PMin(y),
            Dimension::PMin(width),
            Dimension::PMin(height),
        )
    }
    pub fn pmax(
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    ) -> (Dimension, Dimension, Dimension, Dimension) {
        (
            Dimension::PMax(x),
            Dimension::PMax(y),
            Dimension::PMax(width),
            Dimension::PMax(height),
        )
    }
    pub fn vw(
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    ) -> (Dimension, Dimension, Dimension, Dimension) {
        (
            Dimension::VW(x),
            Dimension::VW(y),
            Dimension::VW(width),
            Dimension::VW(height),
        )
    }
    pub fn vh(
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    ) -> (Dimension, Dimension, Dimension, Dimension) {
        (
            Dimension::VH(x),
            Dimension::VH(y),
            Dimension::VH(width),
            Dimension::VH(height),
        )
    }
    pub fn vmin(
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    ) -> (Dimension, Dimension, Dimension, Dimension) {
        (
            Dimension::VMin(x),
            Dimension::VMin(y),
            Dimension::VMin(width),
            Dimension::VMin(height),
        )
    }
    pub fn vmax(
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    ) -> (Dimension, Dimension, Dimension, Dimension) {
        (
            Dimension::VMax(x),
            Dimension::VMax(y),
            Dimension::VMax(width),
            Dimension::VMax(height),
        )
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Dimension {
    Auto,
    Pixel(i32),
    Percent(f64),
    PW(f64),
    PH(f64),
    PMin(f64),
    PMax(f64),
    VW(f64),
    VH(f64),
    VMin(f64),
    VMax(f64),
}

/* spell-checker: disable */
impl Dimension {
    pub fn from_html(mut html: &str) -> Result<Dimension, &str> {
        html = html.trim();
        let lowercase_html = html.to_ascii_lowercase();
        match true {
            _ if lowercase_html == "auto" => Ok(Dimension::Auto),
            _ if lowercase_html.ends_with("px") => match lowercase_html[..lowercase_html.len() - 2]
                .trim()
                .parse::<i32>()
            {
                Ok(val) => Ok(Self::Pixel(val)),
                Err(_) => Err("Couldn't parse pixel dimension"),
            },
            _ if lowercase_html.ends_with('%') => match lowercase_html[..lowercase_html.len() - 1]
                .trim()
                .parse::<f64>()
            {
                Ok(val) => Ok(Self::Percent(val)),
                Err(_) => Err("Couldn't parse percent dimension"),
            },
            _ if lowercase_html.ends_with("vw") => {
                match lowercase_html[..lowercase_html.len() - 2]
                    .trim()
                    .trim()
                    .parse::<f64>()
                {
                    Ok(val) => Ok(Self::VW(val)),
                    Err(_) => Err("Couldn't parse viewport width dimension"),
                }
            }
            _ if lowercase_html.ends_with("vh") => {
                match lowercase_html[..lowercase_html.len() - 2]
                    .trim()
                    .trim()
                    .parse::<f64>()
                {
                    Ok(val) => Ok(Self::VH(val)),
                    Err(_) => Err("Couldn't parse viewport height dimension"),
                }
            }
            _ if lowercase_html.ends_with("vmin") => {
                match lowercase_html[..lowercase_html.len() - 4]
                    .trim()
                    .trim()
                    .parse::<f64>()
                {
                    Ok(val) => Ok(Self::VMin(val)),
                    Err(_) => Err("Couldn't parse viewport minimum dimension"),
                }
            }
            _ if lowercase_html.ends_with("vmax") => {
                match lowercase_html[..lowercase_html.len() - 4]
                    .trim()
                    .trim()
                    .parse::<f64>()
                {
                    Ok(val) => Ok(Self::VMax(val)),
                    Err(_) => Err("Couldn't parse viewport maximum dimension"),
                }
            }
            _ if lowercase_html.ends_with("pw") => {
                match lowercase_html[..lowercase_html.len() - 2]
                    .trim()
                    .trim()
                    .parse::<f64>()
                {
                    Ok(val) => Ok(Self::PW(val)),
                    Err(_) => Err("Couldn't parse parent width dimension"),
                }
            }
            _ if lowercase_html.ends_with("ph") => {
                match lowercase_html[..lowercase_html.len() - 2]
                    .trim()
                    .trim()
                    .parse::<f64>()
                {
                    Ok(val) => Ok(Self::PH(val)),
                    Err(_) => Err("Couldn't parse parent height dimension"),
                }
            }
            _ if lowercase_html.ends_with("pmin") => {
                match lowercase_html[..lowercase_html.len() - 4]
                    .trim()
                    .trim()
                    .parse::<f64>()
                {
                    Ok(val) => Ok(Self::PMin(val)),
                    Err(_) => Err("Couldn't parse parent minimum dimension"),
                }
            }
            _ if lowercase_html.ends_with("pmax") => {
                match lowercase_html[..lowercase_html.len() - 4]
                    .trim()
                    .trim()
                    .parse::<f64>()
                {
                    Ok(val) => Ok(Self::PMax(val)),
                    Err(_) => Err("Couldn't parse parent maximum dimension"),
                }
            }
            _ => match lowercase_html.parse::<i32>() {
                Ok(val) => Ok(Self::Pixel(val)),
                Err(_) => Err("Couldn't parse pixel value"),
            },
        }
    }
    /* spell-checker: enable */
}
