use std::time::Instant;
use std::sync::Arc;
use parking_lot::RwLock;

pub struct StatsEntry {
    time_created: Instant,

}

impl StatsEntry {
    pub fn new() -> Self {
        Self {
            time_created: Instant::now(),
        }
    }

}

pub struct StatsInner {
    entries: Vec<StatsEntry>
}

impl StatsInner {
    pub fn new() -> Self {
        Self {
            entries: Vec::new()
        }
    }

    pub fn add_entry(&mut self, entry: StatsEntry) {
        self.entries.push(entry);
    }
}

#[derive(Clone)]
pub struct Stats {
    inner: Arc<RwLock<StatsInner>>
}

impl Stats {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(StatsInner::new()))
        }
    }

    pub fn new_entry(&self) -> StatsEntry {
        StatsEntry::new()
    }

    pub fn finish_entry(&self, entry: StatsEntry) {
        self.inner.write().add_entry(entry)
    }
}