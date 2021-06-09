use bson::Bson;
use mongodb::options::InsertManyOptions;
use std::collections::HashMap;
use mongodb::Collection;

impl super::Database {
    pub async fn insert_many<D, R>(
        &self,
        collection: &str,
        list_document: Vec<D>,
        options: Option<InsertManyOptions>,
    ) -> Result<Vec<R>, String>
    where
        D: serde::Serialize,
        for<'r> R: serde::Deserialize<'r>,
    {
        let collection: Collection = self.db.collection(&String::from(collection));

        let mut list_entry_bson = vec![];

        for doc in list_document {
            let bson = match bson::ser::to_bson(&doc) {
                Ok(bson) => bson,
                Err(e) => return Err(e.to_string()),
            };

            list_entry_bson.push(match bson {
                bson::Bson::Document(doc) => doc,
                _ => return Err("invalid_bson_format".to_string()),
            });
        }
        match collection.insert_many(list_entry_bson, options).await {
            Ok(inserted_id_list) => {
                let list_ids: HashMap<usize, Bson> = inserted_id_list.inserted_ids;
                let list_ids_iter: Vec<Bson> = list_ids.values().cloned().collect();

                let inserted_id_result: Vec<R> = list_ids_iter
                    .iter()
                    .filter_map(
                        |inserted_id| match bson::de::from_bson(inserted_id.clone()) {
                            Ok(bson) => bson,
                            Err(_) => None,
                        },
                    )
                    .collect();

                Ok(inserted_id_result)
            }
            Err(e) => Err(e.to_string()),
        }
    }
}
