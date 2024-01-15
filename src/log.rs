use crate::event::Event;
use crate::state::StateByEvent;

use std::env;
use std::fs::OpenOptions;
use std::io::{Read, Write};

pub(crate) fn log(input: String) -> Result<(), String> {
    let events = parse_events(input)?;
    write_to_journal(&events)?;
    update_or_insert_state(&events)?;

    Ok(())
}

fn parse_events(input: String) -> Result<Vec<Event>, String> {
    let events = input.as_str().split(';');
    // TODO: Refactor as a chain.
    let mut result = vec![];

    for e in events {
        let event = Event::try_from(e)?;
        result.push(event);
    }

    Ok(result)
}

fn write_to_journal(events: &[Event]) -> Result<(), String> {
    let journal_name = env::var("AUSTRIAN_OAK_JOURNAL_LOCATION").map_err(|_| {
        "Failed to find a journal location; \
            please ensure the `AUSTRIAN_OAK_JOURNAL_LOCATION` environment variable is set."
    })?;
    let mut journal_file = OpenOptions::new()
        .append(true)
        .open(&journal_name)
        .map_err(|_| format!("Failed to open file for appending: {}", journal_name))?;

    // TODO: Make writing of all events in a command atomic to minimize detailed repairing.
    for event in events {
        let serialized = serde_json::to_string(&event).map_err(|e| {
            format!(
                "Failed to serialize event for writing. Event: {:?}, Error: {}",
                event, e
            )
        })?;
        journal_file
            .write(format!("{}\n", serialized).as_bytes())
            .map_err(|_| format!("Failed to write to file: {}", journal_name))?;

        println!("Successfully wrote event to log: {}", serialized);
    }

    Ok(())
}

fn update_or_insert_state(events: &[Event]) -> Result<(), String> {
    // TODO: This could be a few functions.
    let state_name = env::var("AUSTRIAN_OAK_STATE_LOCATION").map_err(|_| {
        "Failed to find a journal location; \
            please ensure the `AUSTRIAN_OAK_STATE_LOCATION` environment variable is set."
    })?;
    let mut state_file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(&state_name)
        .map_err(|_| format!("Failed to open file: {}", state_name))?;
    let mut contents = String::new();
    state_file
        .read_to_string(&mut contents)
        .map_err(|e| format!("Failed to read state file: {}", e))?;
    let mut lines = contents.as_str().split('\n');

    let first = lines
        .next()
        .ok_or("Failed to read line from state file.".to_string())?;
    let state = match serde_json::from_str::<StateByEvent>(first) {
        Err(_) => build_state_by_event()?,
        Ok(mut state) => {
            for event in events {
                state.add_event(event);
            }
            state
        }
    };
    state_file.set_len(0).map_err(|e| format!("Failed to clear state file: {}", e))?;
    state_file
        .write_all(
            &serde_json::to_string(&state)
                .map_err(|e| format!("Failed to serialize state: {}", e))?
                .into_bytes(),
        )
        .map_err(|e| format!("Failed to write to state file: {}", e))?;

    Ok(())
}

fn build_state_by_event() -> Result<StateByEvent, String> {
    let journal_name = env::var("AUSTRIAN_OAK_JOURNAL_LOCATION").map_err(|_| {
        "Failed to find a journal location; \
            please ensure the `AUSTRIAN_OAK_JOURNAL_LOCATION` environment variable is set."
    })?;
    let mut journal_file = OpenOptions::new()
        .read(true)
        .open(&journal_name)
        .map_err(|_| format!("Failed to open file for reading: {}", journal_name))?;
    let mut contents = String::new();
    journal_file
        .read_to_string(&mut contents)
        .map_err(|e| format!("Failed to read state file: {}", e))?;
    let lines = contents.as_str().split('\n');
    let mut events = vec![];
    for line in lines {
        if line.len() > 0 {
            let event = serde_json::from_str::<Event>(line)
                .map_err(|e| format!("Failed to deserialize from journal file: {}", e))?;
            events.push(event);
        }
    }
    let mut state = StateByEvent::new();
    for event in events {
        state.add_event(&event);
    }

    Ok(state)
}
