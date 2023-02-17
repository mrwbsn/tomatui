use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct ConfigToml {
    duration: Option<ConfigTomlDuration>,
}

#[derive(Deserialize)]
struct ConfigTomlDuration {
    work: Option<i32>,
    short_break: Option<i32>,
    long_break: Option<i32>,
}

#[derive(Clone, Copy)]
pub struct Config {
    pub work_duration: i32,
    pub short_break_duration: i32,
    pub long_break_duration: i32,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    pub fn new() -> Self {
        let config_paths = ["./config.toml", "./Config.toml"];

        let mut path = "".to_owned();
        for config_path in config_paths {
            let result = fs::read_to_string(config_path);
            if let Ok(r) = result {
                path = r;
            }
        }

        let config_toml = toml::from_str(&path).unwrap_or(ConfigToml { duration: None });

        let (work, short_break, long_break) = if let Some(duration) = config_toml.duration {
            let work_dur = duration.work.unwrap_or_default();
            let short_break_dur = duration.short_break.unwrap_or_default();
            let long_break_dur = duration.long_break.unwrap_or_default();

            (work_dur, short_break_dur, long_break_dur)
        } else {
            (0, 0, 0)
        };

        Config {
            work_duration: work,
            short_break_duration: short_break,
            long_break_duration: long_break,
        }
    }
}
