use mongodb::{bson::document::Document, options::FindOptions, Collection};

use futures::stream::StreamExt;

impl super::Database {
    pub async fn find<R>(
        &self,
        collection: &str,
        filter: Option<Document>,
        find_options: Option<FindOptions>,
    ) -> Result<Vec<R>, String>
    where
        for<'r> R: serde::Deserialize<'r>,
    {
        let collection: Collection = self.db.collection(&String::from(collection));

        match collection.find(filter, find_options).await {
            Ok(mut cursor) => {
                let mut result = vec![];
                while let Some(doc) = cursor.next().await {
                    let parsed_document =
                        match bson::de::from_bson::<R>(bson::Bson::Document(match doc {
                            Ok(doc) => doc,
                            Err(e) => return Err(e.to_string()),
                        })) {
                            Ok(document) => document,
                            Err(e) => return Err(e.to_string()),
                        };
                    result.push(parsed_document);
                }
                Ok(result)
            }
            Err(e) => Err(e.to_string()),
        }
    }
}
