use bson::Document;
use mongodb::options::UpdateOptions;
use mongodb::Collection;

impl super::Database {
    pub async fn update_many(
        &self,
        collection: &str,
        query: Document,
        update: Document,
        options: Option<UpdateOptions>,
    ) -> Result<i64, String> {
        let collection: Collection = self.db.collection(&String::from(collection));

        match collection.update_many(query, update, options).await {
            Ok(updated) => Ok(updated.modified_count),
            Err(e) => Err(e.to_string()),
        }
    }
}
