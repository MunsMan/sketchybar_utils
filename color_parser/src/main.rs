use serde::Deserialize;
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Deserialize, Debug)]
struct ColorScheme {
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
        "base00" | "background" | "bg" => Ok(&color_scheme.base00),
        "base01" | "lighter_background" | "lighter_bg" => Ok(&color_scheme.base01),
        "base02" | "selection_background" | "selection_bg" => Ok(&color_scheme.base02),
        "base03" | "comments" | "Invisibles" | "line_highlighting" => Ok(&color_scheme.base03),
        "base04" | "dark_foreground" | "dark_fg" | "status_bar" => Ok(&color_scheme.base04),
        "base05" | "foreground" | "fg" | "caret" | "delimiters" | "operators" => {
            Ok(&color_scheme.base05)
        }
        "base06" | "light_foreground" | "light_fg" => Ok(&color_scheme.base06),
        "base07" | "light_background" | "light_bg" => Ok(&color_scheme.base07),
        "base08" | "variables" | "tags" | "links" => Ok(&color_scheme.base08),
        "base09" | "integers" | "boolean" | "constants" | "url" => Ok(&color_scheme.base09),
        "base0a" | "class" | "bold" | "search_background" | "search_bg" => {
            Ok(&color_scheme.base0_a)
        }
        "base0b" | "string" | "code" => Ok(&color_scheme.base0_b),
        "base0c" | "support" | "quotes" | "escape_characters" => Ok(&color_scheme.base0_c),
        "base0d" | "function" | "method" | "heading" => Ok(&color_scheme.base0_d),
        "base0e" | "keyword" | "storage" | "italic" => Ok(&color_scheme.base0_e),
        "base0f" | "deprecated" => Ok(&color_scheme.base0_f),
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

fn set_env(color_scheme: &ColorScheme) {
    Command::new("export").arg(format!("base00={}", color_scheme.base00));
    Command::new("export").arg(format!("base01={}", color_scheme.base01));
    Command::new("export").arg(format!("base02={}", color_scheme.base02));
    Command::new("export").arg(format!("base03={}", color_scheme.base03));
    Command::new("export").arg(format!("base04={}", color_scheme.base04));
    Command::new("export").arg(format!("base05={}", color_scheme.base05));
    Command::new("export").arg(format!("base06={}", color_scheme.base06));
    Command::new("export").arg(format!("base07={}", color_scheme.base07));
    Command::new("export").arg(format!("base08={}", color_scheme.base08));
    Command::new("export").arg(format!("base09={}", color_scheme.base09));
    Command::new("export").arg(format!("base0A={}", color_scheme.base0_a));
    Command::new("export").arg(format!("base0B={}", color_scheme.base0_b));
    Command::new("export").arg(format!("base0C={}", color_scheme.base0_c));
    Command::new("export").arg(format!("base0D={}", color_scheme.base0_d));
    Command::new("export").arg(format!("base0E={}", color_scheme.base0_e));
    Command::new("export").arg(format!("base0F={}", color_scheme.base0_b));
}
