use bson::Document;
use mongodb::options::CountOptions;
use mongodb::Collection;

impl super::Database {
    pub async fn count(
        &self,
        collection: &str,
        filter: Option<Document>,
        count_options: Option<CountOptions>,
    ) -> Result<i64, String> {
        let collection: Collection = self.db.collection(&String::from(collection));

        return match collection.count_documents(filter, count_options).await {
            Ok(r) => Ok(r),
            Err(e) => Err(e.to_string()),
        };
    }
}
