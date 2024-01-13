use crate::event::Event;

use std::env;
use std::fs::OpenOptions;
use std::io::Write;

pub(crate) fn log(input: String) -> Result<(), String> {
    let events = input.as_str().split(';');
    let mut to_write = vec![];

    for e in events {
        let event = Event::try_from(e)?;

        let serialized = serde_json::to_string(&event).map_err(|e| {
            format!(
                "Failed to serialize event for writing. Event: {:?}, Error: {}",
                event, e
            )
        })?;

        to_write.push(serialized);
    }

    let filename = env::var("AUSTRIAN_OAK_JOURNAL_LOCATION").map_err(|_| {
        "Failed to find a journal location; \
            please ensure the `AUSTRIAN_OAK_JOURNAL_LOCATION` environment variable is set."
    })?;
    let mut file = OpenOptions::new()
        .append(true)
        .open(&filename)
        .map_err(|_| format!("Failed to open file for appending: {}", filename))?;

    for line in to_write {
        file.write(format!("{}\n", line).as_bytes())
            .map_err(|_| format!("Failed to write to file: {}", filename))?;

        println!("Successfully wrote event to log: {}", line);
    }

    Ok(())
}
