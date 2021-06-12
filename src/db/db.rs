use std::sync::Arc;
use parking_lot::Mutex;
use std::collections::HashMap;
use crate::db::errors::DbError;
use crate::db::status::UpdateStatus;

type DataSource = Arc<Mutex<HashMap<i32, Vec<i32>>>>;

#[derive(Clone)]
pub struct Db {
    data: DataSource
}

impl Db {
    pub fn new() -> Self {
        Db {
            data: Arc::new(Mutex::new(HashMap::new()))
        }
    }

    pub async fn find_by_id(&self, id: i32) -> Option<Vec<i32>> {
        let data = self.data.lock();
        data.get(&id).cloned()
    }

    pub async fn update_by_id(&mut self, id: i32, values: Vec<i32>, upsert: bool) -> anyhow::Result<UpdateStatus, DbError> {
        let pool_values = match self.find_by_id(id).await {
            Some(v) => Some(v),
            None if upsert == true => None,
            None =>  return Err(DbError::PoolNotFound)
        };

        let mut data = self.data.lock();
        if let Some(v) = pool_values {
            data.insert(id, [v, values].concat());

            return Ok(UpdateStatus::Updated)
        }

        data.insert(id, values);

        Ok(UpdateStatus::Inserted)
    }
}
