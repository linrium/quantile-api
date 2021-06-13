use std::sync::Arc;
use parking_lot::Mutex;
use std::collections::HashMap;

type Child = Arc<Mutex<HashMap<String, (f64, usize)>>>;
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

    fn get_parent(&self, id: i32) -> Child {
        match self.data.lock().get(&id) {
            Some(v) => {
                v.clone()
            },
            None => Arc::new(Mutex::new(HashMap::new()))
        }
    }

    pub fn set(&self, id: i32, percentile: f64, value: (f64, usize)) {
        let data = self.get_parent(id);

        data.clone().lock().insert(format!("{}", percentile), value);
        self.data.lock().insert(id, data);
    }

    pub fn get(&self, id: i32, percentile: f64) -> Option<(f64, usize)> {
        let data = self.get_parent(id);

        data.clone().lock().get(&format!("{}", percentile).to_string()).cloned()
    }

    pub fn remove(&self, id: i32) {
        self.data.lock().remove(&id);
    }
}

#[cfg(test)]
mod tests {
    use crate::db;

    #[test]
    fn test_set_success() {
        let caching = db::Caching::new();
        caching.set(1, 50.0, (1.0, 2));

        let values = caching.get(1, 50.0);
        assert_eq!(values, Some((1.0, 2)));
    }

    #[test]
    fn test_get_success() {
        let caching = db::Caching::new();
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
        let caching = db::Caching::new();
        caching.set(1, 50.0, (1.0, 2));

        // test get
        let values = caching.get(1, 51.0);
        assert_eq!(values, None);
    }

    #[test]
    fn test_remove_success() {
        let caching = db::Caching::new();

        // mock set
        caching.set(1, 50.0, (1.0, 2));

        // mock remove
        caching.remove(1);

        let values = caching.get(1, 50.0);
        assert_eq!(values, None);
    }
}