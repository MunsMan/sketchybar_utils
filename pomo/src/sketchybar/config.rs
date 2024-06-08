use std::env;

type Color = String;

pub(crate) struct Colors {
    pub(crate) black: Color,
    pub(crate) white: Color,
    pub(crate) red: Color,
    pub(crate) green: Color,
    pub(crate) blue: Color,
    pub(crate) yellow: Color,
    pub(crate) orange: Color,
    pub(crate) magenta: Color,
    pub(crate) gray: Color,
    pub(crate) transparent: Color,
    pub(crate) bg0: Color,
    pub(crate) bg1: Color,
    pub(crate) bg2: Color,
    pub(crate) bar: Color,
    pub(crate) bar_border: Color,
    pub(crate) background_1: Color,
    pub(crate) background_2: Color,
    pub(crate) icon: Color,
    pub(crate) label: Color,
    pub(crate) popup_background: Color,
    pub(crate) popup_border: Color,
    pub(crate) shadow: Color,
}

impl Default for Colors {
    fn default() -> Self {
        let bg0 = "0xEE1E2127".to_string();
        let bg1 = "0xCC4B5263".to_string();
        let bg2 = "0x60494d64".to_string();
        let black = "0xFF1E2127".to_string();
        let white = "0xFFABB2BF".to_string();
        Self {
            black: black.clone(),
            white: white.clone(),
            red: "0xFFE06C75".to_string(),
            green: "0xFF98C379".to_string(),
            blue: "0xFF61AFEF".to_string(),
            yellow: "0xFFD19A66".to_string(),
            orange: "0xFFF5A97F".to_string(),
            magenta: "0xFFC678DD".to_string(),
            gray: "0xFF5C6370".to_string(),
            transparent: "0x00000000".to_string(),
            bg0: bg0.clone(),
            bg1: bg1.clone(),
            bg2: bg2.clone(),
            bar: bg0.clone(),
            bar_border: bg2.clone(),
            background_1: bg1,
            background_2: bg2,
            icon: white.clone(),
            label: white.clone(),
            popup_background: "0xD01E2127".to_string(),
            popup_border: white,
            shadow: black,
        }
    }
}

pub(crate) fn load_colors() -> Colors {
    let mut colors = Colors::default();
    load_env_colors(&mut colors);
    colors
}

fn is_color(color: &String) -> bool {
    if color[..2].to_string() != "0x" && color.len() != 10 {
        return false;
    }
    if let Ok(value) = color[2..4].to_string().parse::<i32>() {
        if !(value < u8::MAX as i32 || value > u8::MIN as i32) {
            return false;
        }
    }
    if let Ok(value) = color[4..6].to_string().parse::<i32>() {
        if !(value < u8::MAX as i32 || value > u8::MIN as i32) {
            return false;
        }
    }
    if let Ok(value) = color[6..8].to_string().parse::<i32>() {
        if !(value < u8::MAX as i32 || value > u8::MIN as i32) {
            return false;
        }
    }
    if let Ok(value) = color[8..10].to_string().parse::<i32>() {
        if !(value < u8::MAX as i32 || value > u8::MIN as i32) {
            return false;
        }
    }
    true
}

fn load_env_colors(colors: &mut Colors) {
    for color in env::vars() {
        match color.0.as_ref() {
            "BLACK" => {
                if is_color(&color.1) {
                    colors.black = color.1
                }
            }
            "WHITE" => {
                if is_color(&color.1) {
                    colors.white = color.1
                }
            }
            "RED" => {
                if is_color(&color.1) {
                    colors.red = color.1
                }
            }
            "GREEN" => {
                if is_color(&color.1) {
                    colors.green = color.1
                }
            }
            "BLUE" => {
                if is_color(&color.1) {
                    colors.blue = color.1
                }
            }
            "YELLOW" => {
                if is_color(&color.1) {
                    colors.yellow = color.1
                }
            }
            "ORANGE" => {
                if is_color(&color.1) {
                    colors.orange = color.1
                }
            }
            "MAGENTA" => {
                if is_color(&color.1) {
                    colors.magenta = color.1
                }
            }
            "GRAY" => {
                if is_color(&color.1) {
                    colors.gray = color.1
                }
            }
            "TRANSPARENT" => {
                if is_color(&color.1) {
                    colors.transparent = color.1
                }
            }
            "BG0" => {
                if is_color(&color.1) {
                    colors.bg0 = color.1
                }
            }
            "BG1" => {
                if is_color(&color.1) {
                    colors.bg1 = color.1
                }
            }
            "BG2" => {
                if is_color(&color.1) {
                    colors.bg2 = color.1
                }
            }
            "BAR_COLOR" => {
                if is_color(&color.1) {
                    colors.bar = color.1
                }
            }
            "BAR_BORDER_COLOR" => {
                if is_color(&color.1) {
                    colors.bar_border = color.1
                }
            }
            "BACKGROUND_1" => {
                if is_color(&color.1) {
                    colors.background_1 = color.1
                }
            }
            "BACKGROUND_2" => {
                if is_color(&color.1) {
                    colors.background_2 = color.1
                }
            }
            "ICON_COLOR" => {
                if is_color(&color.1) {
                    colors.icon = color.1
                }
            }
            "LABEL_COLOR" => {
                if is_color(&color.1) {
                    colors.label = color.1
                }
            }
            "POPUP_BACKGROUND_COLOR" => {
                if is_color(&color.1) {
                    colors.popup_background = color.1
                }
            }
            "POPUP_BORDER_COLOR" => {
                if is_color(&color.1) {
                    colors.popup_border = color.1
                }
            }
            "SHADOW_COLOR" => {
                if is_color(&color.1) {
                    colors.shadow = color.1
                }
            }
            _ => {}
        }
    }
}
