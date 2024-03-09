use std::collections::HashMap;

use chrono::{NaiveTime, Utc};

struct Record {
    value: String,
    timestamp: NaiveTime,
    expire_in_millisecs: Option<usize>,
}

pub struct Store {
    inner: HashMap<String, Record>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    pub fn set(&mut self, k: &str, v: &str, expire_in_millisecs: Option<usize>) {
        let timestamp = Utc::now().time();

        self.inner.insert(
            k.into(),
            Record {
                value: v.into(),
                timestamp,
                expire_in_millisecs,
            },
        );
    }

    pub fn get(&mut self, s: &str) -> Option<String> {
        let timestamp = Utc::now().time();

        let record = self.inner.get(s);

        if record.is_none() {
            return None;
        }

        let r = record.unwrap();

        if let Some(exp) = r.expire_in_millisecs {
            let diff = timestamp - r.timestamp;
            let diff = diff.num_milliseconds();

            if diff > exp.try_into().unwrap() {
                self.inner.remove(s);
                return None;
            }
        }

        Some(r.value.clone())
    }
}
