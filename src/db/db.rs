use crate::db::errors::DbError;
use crate::db::status::UpdateStatus;
use std::sync::Arc;
use dashmap::DashMap;

type DataSource = Arc<DashMap<i32, Vec<i32>>>;

#[derive(Clone)]
pub struct Db {
    data: DataSource,
}

impl Db {
    pub fn new() -> Self {
        Db {
            data: Arc::new(DashMap::new()),
        }
    }

    pub async fn find_by_id(&self, id: i32) -> Option<Vec<i32>> {
        if let Some(v) = self.data.get(&id) {
            return Some(v.value().clone())
        }

        return None
    }

    pub async fn update_by_id(
        &mut self,
        id: i32,
        values: Vec<i32>,
    ) -> anyhow::Result<UpdateStatus, DbError> {
        let pool_values = match self.find_by_id(id).await {
            Some(v) => Some(v),
            None => None,
        };

        if let Some(v) = pool_values {
            let new_values = [v, values].concat();
            self.data.insert(id, new_values);

            return Ok(UpdateStatus::Appended);
        }

        self.data.insert(id, values);

        Ok(UpdateStatus::Inserted)
    }
}

#[cfg(test)]
mod tests {
    use crate::db;
    use crate::db::UpdateStatus;

    #[tokio::test]
    async fn test_find_by_id_success() -> Result<(), db::DbError> {
        let mut db = db::Db::new();

        // mock insert
        let values = vec![1, 2, 3];
        let _ = db.update_by_id(1, values.clone()).await?;

        // test find by id
        let result = db.find_by_id(1).await;

        assert_eq!(result, Some(values));

        Ok(())
    }

    #[tokio::test]
    async fn test_find_by_id_not_found() -> Result<(), db::DbError> {
        let db = db::Db::new();

        // test find by id
        let result = db.find_by_id(1).await;

        assert_eq!(result, None);

        Ok(())
    }

    #[tokio::test]
    async fn test_update_by_id_insert_success() -> Result<(), db::DbError> {
        let mut db = db::Db::new();

        // test insert
        let values = vec![1, 4, 2];
        let result = db.update_by_id(1, values.clone()).await?;

        assert_eq!(result, UpdateStatus::Inserted);

        // get values
        let result = db.find_by_id(1).await;

        assert_eq!(result, Some(vec![1, 4, 2]));

        Ok(())
    }

    #[tokio::test]
    async fn test_update_by_id_update_success() -> Result<(), db::DbError> {
        let mut db = db::Db::new();

        // test insert
        let values = vec![1, 4, 2];
        let result = db.update_by_id(1, values.clone()).await?;

        assert_eq!(result, UpdateStatus::Inserted);

        // test insert
        let values = vec![3, 7];
        let result = db.update_by_id(1, values.clone()).await?;

        assert_eq!(result, UpdateStatus::Appended);

        // get values
        let result = db.find_by_id(1).await;
        let expected = vec![1, 4, 2, 3, 7];

        assert_eq!(result, Some(expected));

        Ok(())
    }
}
