use bson::Document;
use mongodb::options::FindOneAndDeleteOptions;
use mongodb::Collection;

impl super::Database {
    pub async fn find_one_and_delete<R>(
        &self,
        collection: &str,
        filter: Document,
        options: Option<FindOneAndDeleteOptions>,
    ) -> Result<R, String>
    where
        for<'r> R: serde::Deserialize<'r>,
    {
        let collection: Collection = self.db.collection(&String::from(collection));
        match match collection.find_one_and_delete(filter, options).await {
            Ok(doc) => doc,
            Err(e) => return Err(e.to_string()),
        } {
            Some(deleted_doc) => {
                let data: R = match bson::from_bson(bson::Bson::Document(deleted_doc)) {
                    Ok(from_bson) => from_bson,
                    Err(e) => return Err(e.to_string()),
                };
                Ok(data)
            }
            _ => return Err("cannot_find_document".to_string()),
        }
    }
}
