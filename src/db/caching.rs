use std::sync::Arc;
use parking_lot::Mutex;
use std::collections::HashMap;

type Child = Arc<Mutex<HashMap<String, (f32, usize)>>>;
type DataSource = Arc<Mutex<HashMap<i32, Child>>>;

#[derive(Clone)]
pub struct Caching {
    data: DataSource,
}

impl Caching {
    pub fn new() -> Self {
        Self {
            data: Arc::new(Mutex::new(HashMap::new()))
        }
    }

    pub fn set(&mut self, id: i32, percentile: f32, value: (f32, usize)) {
        let key = format!("{}", percentile);
        let data = self.data.lock().get(&id).cloned();
        match data {
            Some(v) => {
                v.lock().insert(key, value);
                self.data.lock().insert(id, v);
            },
            None => {
                let mut map = HashMap::new();
                map.insert(key, value);
                self.data.lock().insert(id, Arc::new(Mutex::new(map)));
            }
        }
    }

    pub fn get(&self, id: i32, percentile: f32) -> Option<(f32, usize)> {
        let key = format!("{}", percentile).to_string();

        if let Some(v) = self.data.lock().get(&id) {
            v.lock().get(&key).cloned()
        } else {
            None
        }
    }

    pub fn remove(&mut self, id: i32) {
        self.data.lock().remove(&id);
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