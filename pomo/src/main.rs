use crate::cli::{Cli, Commands};
use clap::Parser;
use std::{
    env::{self},
    fs,
    io::{self},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use self::sketchybar::sketchybar_handler;

mod cli;
mod sketchybar;

macro_rules! minutes {
    ($x:expr) => {
        Duration::new(60 * $x, 0)
    };
}

static STATE_FILE_NAME: &str = "pomo-cli-state";

fn main() -> Result<(), io::Error> {
    let cli = Cli::parse();
    let mut state = load_state();

    match &cli.command {
        Commands::Start {} => {
            let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
            state.start = start.as_secs();
            state.running = true;
        }
        Commands::Config(args) => {
            if let Some(duration) = args.work_duration {
                state.work_duration = minutes!(duration)
            }
            if let Some(duration) = args.break_short {
                state.break_short = minutes!(duration)
            }
            if let Some(duration) = args.break_long {
                state.break_long = minutes!(duration)
            }
            println!("{:#?}", state);
        }
        Commands::Update {} => println!("{}", update(&state)),
        Commands::Sketchybar(sub) => sketchybar_handler(&sub.commands, &mut state),
        Commands::Stop => state = State::default(),
    }
    save_state(&state)
}

enum CurrentStates {
    Work { duration: Duration },
    ShortBreak { duration: Duration },
    LongBreak { duration: Duration },
}

fn current_pomo_state(mut duration: Duration, state: &State) -> CurrentStates {
    loop {
        for _i in 1..state.work_blocks {
            duration = match duration.checked_sub(state.work_duration) {
                Some(duration) => duration,
                None => return CurrentStates::Work { duration },
            };
            duration = match duration.checked_sub(state.break_short) {
                Some(duration) => duration,
                None => return CurrentStates::ShortBreak { duration },
            };
        }
        duration = match duration.checked_sub(state.work_duration) {
            Some(duration) => duration,
            None => return CurrentStates::ShortBreak { duration },
        };
        duration = match duration.checked_sub(state.break_long) {
            Some(duration) => duration,
            None => return CurrentStates::LongBreak { duration },
        };
    }
}

fn update(state: &State) -> String {
    if state.running {
        let start_time = Duration::new(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
                - state.start,
            0,
        );
        let current_state = current_pomo_state(start_time, state);
        match current_state {
            CurrentStates::Work { duration } => format!(
                "Stay Focused: {:0>2}:{:0>2}",
                (state.work_duration.as_secs() - duration.as_secs()) / 60,
                (state.work_duration.as_secs() - duration.as_secs()) % 60
            ),
            CurrentStates::ShortBreak { duration } => format!(
                "Short Break: {:0>2}:{:0>2}",
                (state.break_short.as_secs() - duration.as_secs()) / 60,
                (state.break_short.as_secs() - duration.as_secs()) % 60
            ),
            CurrentStates::LongBreak { duration } => format!(
                "Short Break: {:0>2}:{:0>2}",
                (state.break_long.as_secs() - duration.as_secs()) / 60,
                (state.break_long.as_secs() - duration.as_secs()) % 60
            ),
        }
    } else {
        String::new()
    }
}

#[derive(Debug, bincode::Encode, bincode::Decode)]
struct State {
    work_duration: Duration,
    break_short: Duration,
    break_long: Duration,
    work_blocks: u8,
    running: bool,
    start: u64,
}

impl Default for State {
    fn default() -> Self {
        Self {
            work_duration: minutes!(25),
            break_short: minutes!(5),
            break_long: minutes!(30),
            work_blocks: 3,
            running: false,
            start: 0,
        }
    }
}

impl State {
    pub fn reset(&mut self) {
        self.start = 0;
        self.work_duration = minutes!(25);
        self.break_short = minutes!(5);
        self.break_long = minutes!(30);
        self.work_blocks = 5;
        self.running = false;
    }
}

fn load_state() -> State {
    let config = bincode::config::standard();
    let dir = env::temp_dir();
    let file = fs::read(dir.join(STATE_FILE_NAME));
    let state: State = match file {
        Ok(file) => {
            let (result, _size): (State, usize) =
                bincode::decode_from_slice(&file, config).unwrap();
            result
        }
        Err(_) => State::default(),
    };
    state
}

fn save_state(state: &State) -> Result<(), std::io::Error> {
    let config = bincode::config::standard();
    let encoded: Vec<u8> = bincode::encode_to_vec(state, config).unwrap();
    fs::write(env::temp_dir().join(STATE_FILE_NAME), encoded)
}
