mod find_one_and_delete;

mod aggregate;
mod count;
mod delete_many;
mod find;
mod find_one;
mod find_one_and_update;
mod insert_many;
mod insert_one;
mod update_many;

#[derive(Clone)]
pub struct Database {
    pub db: mongodb::Database,
}
