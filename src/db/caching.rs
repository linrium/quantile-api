use std::sync::Arc;
use parking_lot::Mutex;
use std::collections::HashMap;

type Child = Arc<Mutex<HashMap<String, (f64, usize)>>>;
type DataSource = HashMap<i32, Child>;

#[derive(Clone)]
pub struct Caching {
    data: DataSource,
}

impl Caching {
    pub fn new() -> Self {
        Self {
            data: HashMap::new()
        }
    }

    pub fn set(&mut self, id: i32, percentile: f64, value: (f64, usize)) {
        let data = self.data.get(&id);
        let key = format!("{}", percentile);

        if let Some(v) = data {
            v.lock().insert(key, value);
        } else {
            let mut map = HashMap::new();
            map.insert(key, value);
            self.data.insert(id, Arc::new(Mutex::new(map)));
        }
    }

    pub fn get(&self, id: i32, percentile: f64) -> Option<(f64, usize)> {
        let data = self.data.get(&id);
        let key = format!("{}", percentile).to_string();

        if let Some(v) = data {
            v.lock().get(&key).cloned()
        } else {
            None
        }
    }

    pub fn remove(&mut self, id: i32) {
        self.data.remove(&id);
    }
}

#[cfg(test)]
mod tests {
    use crate::db;

    #[test]
    fn test_set_success() {
        let mut caching = db::Caching::new();
        caching.set(1, 50.0, (1.0, 2));

        let values = caching.get(1, 50.0);
        assert_eq!(values, Some((1.0, 2)));
    }

    #[test]
    fn test_get_success() {
        let mut caching = db::Caching::new();
        caching.set(1, 50.0, (1.0, 2));

        let values = caching.get(1, 50.0);
        assert_eq!(values, Some((1.0, 2)));
    }

    #[test]
    fn test_get_failed() {
        // test get
        let caching = db::Caching::new();
        let values = caching.get(1, 50.0);
        assert_eq!(values, None);

        // mock set
        let mut caching = db::Caching::new();
        caching.set(1, 50.0, (1.0, 2));

        // test get
        let values = caching.get(1, 51.0);
        assert_eq!(values, None);
    }

    #[test]
    fn test_remove_success() {
        let mut caching = db::Caching::new();

        // mock set
        caching.set(1, 50.0, (1.0, 2));

        // mock remove
        caching.remove(1);

        let values = caching.get(1, 50.0);
        assert_eq!(values, None);
    }
}