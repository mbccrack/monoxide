use futures::stream::StreamExt;
use mongodb::options::AggregateOptions;
use mongodb::Collection;

impl super::Database {
    pub async fn aggregate<R>(
        &self,
        collection: &str,
        pipeline: Vec<bson::Document>,
        options: Option<AggregateOptions>,
    ) -> Result<Vec<R>, String>
    where
        for<'r> R: serde::Deserialize<'r>,
    {
        let collection: Collection = self.db.collection(&String::from(collection));
        match collection.aggregate(pipeline, options).await {
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
