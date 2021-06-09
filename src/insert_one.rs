use mongodb::options::InsertOneOptions;
use mongodb::Collection;

impl super::Database {
    pub async fn insert_one<D, R>(
        &self,
        collection: &str,
        document: D,
        options: Option<InsertOneOptions>,
    ) -> Result<R, String>
    where
        D: serde::Serialize,
        for<'r> R: serde::Deserialize<'r>,
    {
        let collection: Collection = self.db.collection(&String::from(collection));

        let bson = match bson::ser::to_bson(&document) {
            Ok(bson) => bson,
            Err(e) => return Err(e.to_string()),
        };

        let entry_bson = match bson {
            bson::Bson::Document(doc) => doc,
            _ => return Err("invalid_bson_format".to_string()),
        };

        match collection.insert_one(entry_bson, options).await {
            Ok(inserted) => {
                let inserted_id: R = match bson::from_bson(inserted.inserted_id) {
                    Ok(inserted_id) => inserted_id,
                    Err(e) =>  return Err(e.to_string())
                };
                Ok(inserted_id)
            }
            Err(e) => Err(e.to_string()),
        }
    }
}
