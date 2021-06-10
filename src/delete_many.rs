use bson::Document;
use mongodb::options::DeleteOptions;
use mongodb::Collection;

impl super::Database {
    pub async fn delete_many(
        &self,
        collection: &str,
        query: Document,
        options: Option<DeleteOptions>,
    ) -> Result<i64, String> {
        let collection: Collection = self.db.collection(&String::from(collection));

        match collection.delete_many(query, options).await {
            Ok(deleted) => Ok(deleted.deleted_count),
            Err(e) => Err(e.to_string()),
        }
    }
}
