mod find_one_and_delete;

mod find_one_and_update;
mod find_one;
mod count;
mod delete_many;
mod find;
mod insert_many;
mod aggregate;
mod insert_one;
mod update_many;



#[derive(Clone)]
pub struct Database {
    pub db: mongodb::Database,
}
