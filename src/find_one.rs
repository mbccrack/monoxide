use bson::Document;
use mongodb::options::FindOneOptions;
use mongodb::Collection;

impl super::Database {
    pub async fn find_one<R>(
        &self,
        collection: &str,
        filter: Option<Document>,
        options: Option<FindOneOptions>,
    ) -> Result<Option<R>, String>
    where
        for<'r> R: serde::Deserialize<'r>,
    {
        let collection: Collection = self.db.collection(&String::from(collection));
        match match collection.find_one(filter, options).await {
            Ok(doc) => doc,
            Err(e) => return Err(e.to_string()),
        } {
            Some(doc) => {
                let data: R = match bson::from_bson(bson::Bson::Document(doc)) {
                    Ok(from_bson) => from_bson,
                    Err(e) => return Err(e.to_string()),
                };
                Ok(Some(data))
            }
            _ => Ok(None),
        }
    }
}
