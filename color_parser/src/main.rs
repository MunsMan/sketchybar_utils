use serde::Deserialize;
use std::env;
use std::fs;
use std::path::Path;

#[derive(Deserialize, Debug)]
struct ColorScheme {
    palette: ColorPalette,
}

#[derive(Deserialize, Debug)]
struct ColorPalette {
    base00: String,
    base01: String,
    base02: String,
    base03: String,
    base04: String,
    base05: String,
    base06: String,
    base07: String,
    base08: String,
    base09: String,
    #[serde(rename = "base0A")]
    base0_a: String,
    #[serde(rename = "base0B")]
    base0_b: String,
    #[serde(rename = "base0C")]
    base0_c: String,
    #[serde(rename = "base0D")]
    base0_d: String,
    #[serde(rename = "base0E")]
    base0_e: String,
    #[serde(rename = "base0F")]
    base0_f: String,
}

fn main() -> Result<(), &'static str> {
    if env::args().len() < 2 {
        return Err("Please provide the color key");
    }
    let color = env::args().collect::<Vec<_>>()[1].clone();
    let color_scheme = env::var("COLOR_SCHEME").unwrap();
    let color_schemes_dir = env::var("COLOR_SCHEMES_DIR").unwrap();

    let path = Path::new(&color_schemes_dir).join(Path::new(&format!("{}.yaml", color_scheme)));

    let color_scheme = read_color(&path)?;
    print!("{}", get_color(&color, &color_scheme)?);
    Ok(())
}

fn get_color<'a>(color: &str, color_scheme: &'a ColorScheme) -> Result<&'a str, &'static str> {
    match color.to_lowercase().as_str() {
        "base00" | "background" | "bg" => Ok(&color_scheme.palette.base00),
        "base01" | "lighter_background" | "lighter_bg" => Ok(&color_scheme.palette.base01),
        "base02" | "selection_background" | "selection_bg" => Ok(&color_scheme.palette.base02),
        "base03" | "comments" | "Invisibles" | "line_highlighting" => {
            Ok(&color_scheme.palette.base03)
        }
        "base04" | "dark_foreground" | "dark_fg" | "status_bar" => Ok(&color_scheme.palette.base04),
        "base05" | "foreground" | "fg" | "caret" | "delimiters" | "operators" => {
            Ok(&color_scheme.palette.base05)
        }
        "base06" | "light_foreground" | "light_fg" => Ok(&color_scheme.palette.base06),
        "base07" | "light_background" | "light_bg" => Ok(&color_scheme.palette.base07),
        "base08" | "variables" | "tags" | "links" => Ok(&color_scheme.palette.base08),
        "base09" | "integers" | "boolean" | "constants" | "url" => Ok(&color_scheme.palette.base09),
        "base0a" | "class" | "bold" | "search_background" | "search_bg" => {
            Ok(&color_scheme.palette.base0_a)
        }
        "base0b" | "string" | "code" => Ok(&color_scheme.palette.base0_b),
        "base0c" | "support" | "quotes" | "escape_characters" => Ok(&color_scheme.palette.base0_c),
        "base0d" | "function" | "method" | "heading" => Ok(&color_scheme.palette.base0_d),
        "base0e" | "keyword" | "storage" | "italic" => Ok(&color_scheme.palette.base0_e),
        "base0f" | "deprecated" => Ok(&color_scheme.palette.base0_f),
        _ => Err("Unknown Color"),
    }
}

fn read_color(path: &Path) -> Result<ColorScheme, &'static str> {
    let color_file = fs::read_to_string(path);
    let color_file = match color_file {
        Ok(color_file) => color_file,
        Err(_) => return Err("unable to read file!"),
    };
    match serde_yaml::from_str::<ColorScheme>(&color_file) {
        Ok(scheme) => Ok(scheme),
        Err(_) => Err("Invalid Base16 color scheme!"),
    }
}
