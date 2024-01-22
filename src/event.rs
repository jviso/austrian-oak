use std::time::Duration;

use chrono::naive::NaiveDate;
use phf::phf_map;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Event {
    pub(crate) r#type: EventType,
    pub(crate) details: EventDetails,
    pub(crate) date: NaiveDate,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum EventType {
    Deadlift,
    Squat,
    FrontSquat,
    Bench,
    Press,
    PosteriorPress,
    CleanAndJerk,
    Snatch,
    Row,
    Curl,
    TricepExtension,
    TricepPushdown,
    Nordic,
    WidePulldown,
    NarrowPulldown,
    FacePull,
    LateralRaise,
    Conditioning,
    Sleep,
    Nutrition,
    WeighIn,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase", untagged)]
pub(crate) enum EventDetails {
    Strength {
        load: usize,
        reps: usize,
    },
    Sleep {
        quality: Quality,
        duration: Duration,
    },
    Nutrition {
        quality: Quality,
    },
    Conditioning {
        intensity: Intensity,
        duration: Duration,
    },
    WeighIn {
        mass: usize,
    },
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum Intensity {
    Low,
    Medium,
    High,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum Quality {
    Good,
    NotGood,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub(crate) struct StrengthSet {
    load: usize,
    reps: usize,
}

fn today() -> NaiveDate {
    chrono::Local::now().date_naive()
}

// TODO: Add "show shortcodes" command
static SHORTCODES: phf::Map<&'static str, &'static str> = phf_map! {
    "dl" => "deadlift",
    "sq" => "squat",
    "fs" => "frontsquat",
    "bp" => "bench",
    "pr" => "press",
    "pp" => "posteriorpress",
    "cj" => "cleanandjerk",
    "sn" => "snatch",
    "ro" => "row",
    "cu" => "curl",
    "te" => "tricepextension",
    "tp" => "triceppushdown",
    "no" => "nordic",
    "wp" => "widepulldown",
    "np" => "narrowpulldown",
    "fp" => "facepull",
    "lr" => "lateralraise",
    "co" => "conditioning",
    "sl" => "sleep",
    "nu" => "nutrition",
    "wi" => "weighin",
};

fn wrap_with_quotes(s: &str) -> String {
    format!("\"{}\"", s)
}

impl TryFrom<&str> for Event {
    type Error = String;

    // TODO: Consider better error messages when deserialization fails.
    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let mut parts = input.split(',');
        let mut tag = parts.next().ok_or("Input was empty.".to_string())?;
        if let Some(long) = SHORTCODES.get(tag) {
            tag = long;
        }
        let event_type =
            serde_json::from_str::<EventType>(&wrap_with_quotes(tag)).map_err(|e| e.to_string())?;
        let details = match event_type {
            EventType::Sleep => {
                let quality = serde_json::from_str(
                    wrap_with_quotes(
                        parts
                            .next()
                            .ok_or("Input had too few fields.".to_string())?,
                    )
                    .as_str(),
                )
                .map_err(|e| e.to_string())?;
                let duration = parts
                    .next()
                    .ok_or("Input had too few fields.".to_string())?
                    .parse::<humantime::Duration>()
                    .map_err(|e| e.to_string())?
                    .into();
                EventDetails::Sleep { quality, duration }
            }
            EventType::Nutrition => {
                let quality = serde_json::from_str(
                    wrap_with_quotes(
                        parts
                            .next()
                            .ok_or("Input had too few fields.".to_string())?,
                    )
                    .as_str(),
                )
                .map_err(|e| e.to_string())?;
                EventDetails::Nutrition { quality }
            }
            EventType::Conditioning => {
                let intensity = serde_json::from_str(
                    wrap_with_quotes(
                        parts
                            .next()
                            .ok_or("Input had too few fields.".to_string())?,
                    )
                    .as_str(),
                )
                .map_err(|e| e.to_string())?;
                let duration = parts
                    .next()
                    .ok_or("Input had too few fields.".to_string())?
                    .parse::<humantime::Duration>()
                    .map_err(|e| e.to_string())?
                    .into();
                EventDetails::Conditioning {
                    intensity,
                    duration,
                }
            }
            EventType::WeighIn => {
                let mass = serde_json::from_str(
                    parts
                        .next()
                        .ok_or("Input had too few fields.".to_string())?,
                )
                .map_err(|e| e.to_string())?;
                EventDetails::WeighIn { mass }
            }
            EventType::Deadlift
            | EventType::Squat
            | EventType::FrontSquat
            | EventType::Bench
            | EventType::Press
            | EventType::PosteriorPress
            | EventType::CleanAndJerk
            | EventType::Snatch
            | EventType::Row
            | EventType::Curl
            | EventType::TricepExtension
            | EventType::TricepPushdown
            | EventType::Nordic
            | EventType::WidePulldown
            | EventType::NarrowPulldown
            | EventType::FacePull
            | EventType::LateralRaise => {
                // TODO: Add opinionated parsing to protect against flipping these fields:
                //       - `reps` should not be > 100
                let load = serde_json::from_str(
                    parts.next().ok_or("Input had too few fields".to_string())?,
                )
                .map_err(|e| e.to_string())?;
                let reps = serde_json::from_str(
                    parts
                        .next()
                        .ok_or("Input had too few fields.".to_string())?,
                )
                .map_err(|e| e.to_string())?;
                EventDetails::Strength { load, reps }
            }
        };
        let date = if let Some(d) = parts.next() {
            serde_json::from_str(&wrap_with_quotes(d)).map_err(|e| e.to_string())?
        } else {
            today()
        };
        let event = Event {
            r#type: event_type,
            details,
            date,
        };

        Ok(event)
    }
}
