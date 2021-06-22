use std::sync::Arc;
use dashmap::DashMap;

type Child = Arc<DashMap<String, (f32, usize)>>;
type DataSource = Arc<DashMap<i32, Child>>;

#[derive(Clone)]
pub struct Caching {
    data: DataSource,
}

impl Caching {
    pub fn new() -> Self {
        Self {
            data: Arc::new(DashMap::new())
        }
    }

    pub fn set(&mut self, id: i32, percentile: f32, value: (f32, usize)) {
        let key = format!("{}", percentile);
        match self.data.get(&id) {
            Some(v) => {
                v.insert(key, value);
            },
            None => {
                let map = DashMap::new();
                map.insert(key, value);
                self.data.insert(id, Arc::new(map));
            }
        }
    }

    pub fn get(&self, id: i32, percentile: f32) -> Option<(f32, usize)> {
        let key = format!("{}", percentile).to_string();

        if let Some(v) = self.data.get(&id) {
            if let Some(nested) = v.get(&key) {
                return Some(nested.value().clone());
            }

            return None;
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