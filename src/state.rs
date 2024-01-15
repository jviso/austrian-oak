use crate::event::{Event, EventDetails, EventType};

use std::collections::{BTreeMap, HashMap};

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub(crate) struct StateByEvent {
    days_by_event: HashMap<EventType, BTreeMap<NaiveDate, Vec<EventDetails>>>,
}

impl StateByEvent {
    pub(crate) fn new() -> Self {
        Self {
            days_by_event: HashMap::new(),
        }
    }

    pub(crate) fn add_event(&mut self, event: &Event) {
        self.days_by_event
            .entry(event.r#type)
            .and_modify(|e| {
                e.entry(event.date)
                    .and_modify(|d| {
                        d.push(event.details);
                    })
                    .or_insert_with(|| vec![event.details]);
            })
            .or_insert_with(|| {
                let mut map = BTreeMap::new();
                map.insert(event.date, vec![event.details]);
                map
            });
    }
}
