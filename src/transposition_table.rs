
pub struct TranspositionTable<T> {
    data: Vec<Option<(T, u64)>>,
    pub collisions: u64,
    pub used: u64,
}


impl<T> TranspositionTable<T> {
    pub fn new(size: u64) -> Self<> {
        let mut data = Vec::new();
        data.reserve_exact(size as usize);
        for _ in 0..size {
            data.push(None)
        }
        Self {
            data,
            collisions: 0,
            used: 0,
        }
    }

    pub fn insert(&mut self, hash: u64, value: T) {
        let len = self.data.len();
        let a = &mut self.data[(hash % len as u64) as usize];
        if a.is_some() {
            self.collisions += 1;
        } else {
            self.used += 1;
        }
        *a = Some((value, hash));
    }

    pub fn get(&self, hash: u64) -> Option<&T> {
        let len = self.data.len();
        let res = self.data[(hash % len as u64) as usize].as_ref();
        if let Some((i, found_hash)) = res {
            if *found_hash != hash {
                return None
            } else {
                return Some(i)
            }
        } else {
            None
        }
    }
}