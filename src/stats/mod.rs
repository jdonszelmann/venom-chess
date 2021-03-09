use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::sync::Arc;
use std::collections::HashMap;
use std::path::PathBuf;
use serde::Serialize;
use std::fs::{File, create_dir_all};
use std::sync::mpsc::{Sender, channel, Receiver};
use parking_lot::Mutex;
use std::{thread, io};
use std::io::Write;

#[derive(Clone, Serialize, Debug)]
pub struct TranspositionTableStats {
    collisions: u64,
    colisionless_insertions: u64,
    hits: u64,
}

impl TranspositionTableStats {
    pub fn new() -> Self {
        Self {
            colisionless_insertions: 0,
            collisions: 0,
            hits: 0
        }
    }

    pub fn collision(&mut self) {
        self.collisions += 1;
    }

    pub fn hit(&mut self) {
        self.hits += 1;
    }

    pub fn colisionless_insert(&mut self) {
        self.colisionless_insertions += 1;
    }
}

#[derive(Clone, Serialize, Debug)]
pub struct StatsEntry {
    time_created: SystemTime,
    time_finished: Option<SystemTime>,

    num_states_seen: u64,
    evaluation: i64,

    #[serde(flatten)]
    transposition: Option<TranspositionTableStats>,

    #[serde(flatten)]
    int_entries: HashMap<&'static str, i64>,
    #[serde(flatten)]
    float_entries: HashMap<&'static str, f64>,
}


impl StatsEntry {
    pub fn new() -> Self {
        Self {
            time_created: SystemTime::now(),
            time_finished: None,

            num_states_seen: 0,
            evaluation: 0,

            transposition: None,

            int_entries: Default::default(),
            float_entries: Default::default()
        }
    }

    pub fn custom_int_entry(&mut self, name: &'static str, value: i64) {
        self.int_entries.insert(name, value);
    }

    pub fn custom_int_entry_add(&mut self, name: &'static str) {
        *self.int_entries.entry(name).or_insert(0) += 1;
    }

    pub fn custom_int_entry_sub(&mut self, name: &'static str) {
        *self.int_entries.entry(name).or_insert(0) -= 1;
    }


    pub fn custom_float_entry(&mut self, name: &'static str, value: f64) {
        self.float_entries.insert(name, value);
    }

    pub fn evaluation(&mut self, value: i64) {
        self.evaluation = value;
    }

    pub fn seen_state(&mut self) {
        self.num_states_seen += 1;
    }

    pub fn end(&mut self) {
        self.time_finished = Some(SystemTime::now());
    }

    pub fn transposition(&mut self) -> &mut TranspositionTableStats {
        self.transposition.get_or_insert(TranspositionTableStats::new())
    }


    pub fn duration(&self) -> Option<Duration> {
        self.time_finished.map(|i| i.duration_since(self.time_created).expect("time went backwards!"))
    }
}

#[derive(Clone, Serialize)]
pub struct Metadata {
    algorithm_name: &'static str,

    // None if variable/not applicable
    search_depth: Option<u64>,

    // Size of the transposition table (in number of entries)
    transposition_table_size: Option<u64>,
}

#[derive(Clone)]
pub struct Stats {
    channel: Sender<StatsEntry>,

    metadata: Metadata,

    stats_folder: PathBuf,

    last_entry: Arc<Mutex<Option<StatsEntry>>>,
}

impl Stats {
    pub fn new(algorithm_name: &'static str, search_depth: Option<u64>, transposition_table_size: Option<u64>, stats_folder: String, log_stats: bool) -> Self {
        let path = PathBuf::from(stats_folder);

        let metadata = Metadata {
            algorithm_name,
            search_depth,
            transposition_table_size,
        };

        let (tx, rx) = channel();

        if log_stats {
            if !path.exists() || !path.is_dir() {
                create_dir_all(&path).expect("failed create directory");
            }

            let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("time went backwards").as_millis();
            let filename = path.join(format!("stats_{}_{}.dat", now, algorithm_name));
            println!("writing statistics to {:?}", filename);

            let mut file = io::LineWriter::new(File::create(filename).expect("couldn't open statistics file"));
            file.write_all(serde_json::to_string(&metadata).expect("failed to serialize").as_bytes()).expect("failed to write");
            file.write(&[b'\n']).expect("failed to write");

            thread::spawn(|| {
                Self::write_entries(file, rx)
            });
        }


        Self {
            channel: tx,
            metadata,
            stats_folder: path,
            last_entry: Arc::new(Mutex::new(None))
        }
    }

    pub fn write_entries(mut stats_file: io::LineWriter<File>, receiver: Receiver<StatsEntry>) {
        while let Ok(i) = receiver.recv() {
            stats_file.write_all(serde_json::to_string(&i).expect("failed to serialize").as_bytes()).expect("failed to write");
            stats_file.write(&[b'\n']).expect("failed to write");
        }
    }

    pub fn last_entry(&self) -> Option<StatsEntry> {
        self.last_entry.lock().clone()
    }

    pub fn new_entry(&self) -> StatsEntry {
        StatsEntry::new()
    }

    pub fn finish_entry(&self, mut entry: StatsEntry) {
        entry.end();
        *self.last_entry.lock() = Some(entry.clone());
        let _ = self.channel.send(entry);
    }
}