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