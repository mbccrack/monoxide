use bson::Document;
use mongodb::options::FindOneAndUpdateOptions;
use mongodb::Collection;

impl super::Database {
    pub async fn find_one_and_update<R>(
        &self,
        collection: &str,
        filter: Document,
        update: Document,
        options: Option<FindOneAndUpdateOptions>,
    ) -> Result<R, String>
    where
        for<'r> R: serde::Deserialize<'r>,
    {
        let collection: Collection = self.db.collection(&String::from(collection));
        match match collection.find_one_and_update(filter, update, options).await {
            Ok(doc) => doc,
            Err(e) => return Err(e.to_string()),
        } {
            Some(updated_doc) => {
                let data: R = match bson::from_bson(bson::Bson::Document(updated_doc)) {
                    Ok(from_bson) => from_bson,
                    Err(e) => return Err(e.to_string()),
                };
                Ok(data)
            }
            _ => return Err("cannot_find_document".to_string()),
        }
    }
}
