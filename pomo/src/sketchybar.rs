use std::process::Command;

use crate::cli::SketchybarCommand;
use crate::sketchybar::config::load_colors;
use crate::{update, State};

mod config;

pub(crate) fn sketchybar_handler(command: &SketchybarCommand, state: &mut State) {
    match command {
        SketchybarCommand::Load { icon } => load_sketchybar(icon),
        SketchybarCommand::Unload => unload_sketchybar(state),
        SketchybarCommand::Update => update_sketchybar(state),
        SketchybarCommand::Show => draw(),
        SketchybarCommand::Hide => hide(),
    }
}

fn load_sketchybar(icon: &Option<char>) {
    let colors = load_colors();
    let command = [
        "--add",
        "item",
        "pomo",
        "center",
        "--set",
        "pomo",
        &format!("icon={}", icon.unwrap_or('󰚭')).to_string(),
        "update_freq=1",
        "updates=when_shown",
        "script=pomo sketchybar update",
        "click_script=sketchybar --set pomo popup.drawing=toggle",
        "label.align=left",
        "popup.blur_radius=50",
        "popup.align=center",
        "popup.horizontal=true",
        "popup.y_offset=3",
        "popup.blur_radius=5",
        &format!("popup.background.border_color={}", colors.transparent),
        &format!("popup.background.color={}", colors.popup_background),
    ];
    let status = Command::new("sketchybar")
        .args(command)
        .args(popup("Start", "pomo start"))
        .args(popup("Stop", "pomo sketchybar hide"))
        .status();
    println!("{:#?}", status);
}

fn popup(name: &str, function: &str) -> [String; 8] {
    let id = format!("pomo.{}", name.to_lowercase());
    [
        "--add",
        "item",
        &id,
        "popup.pomo",
        "--set",
        &id,
        &format!("label={}", name),
        &format!("click_script={}", function),
    ]
    .map(|x| x.to_string())
}

fn draw() {
    let status = Command::new("sketchybar")
        .args(["--set", "pomo", "drawing=on"])
        .status();
    println!("{:#?}", status);
}

fn hide() {
    let status = Command::new("sketchybar")
        .args(["--set", "pomo", "drawing=false", "popup.drawing=toggle"])
        .status();
    println!("{:#?}", status);
}

fn unload_sketchybar(state: &mut State) {
    state.reset();
    let status = Command::new("sketchybar")
        .args(["--remove", "pomo"])
        .status();
    println!("{:#?}", status);
}

fn update_sketchybar(state: &State) {
    let time = update(state);
    let _ = Command::new("sketchybar")
        .args(["--set", "pomo", &format!("label={}", time).to_string()])
        .status();
}
