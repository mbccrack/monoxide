use mongodb::options::ClientOptions; 
use mongodb::Client;
use crate::Database;

impl super::Database {
    pub async fn new(
        &self,
        uri: &str,database:&str
    ) -> Result<Database, String> {
        match ClientOptions::parse(uri).await{
            Ok(client_options)=>{
                match Client::with_options(client_options){
                    Ok(client)=>{
                        Ok(Database{
                            db: client.database(database)
                        })
                    },
                    Err(e)=>Err(e.to_string())
                }
            },
            Err(e)=>Err(e.to_string())
        }
    
    }
    
}
