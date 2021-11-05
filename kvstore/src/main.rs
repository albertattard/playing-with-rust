use crate::database::Database;

mod database;

fn main() {
    let mut args = std::env::args().skip(1);
    let key = args.next().expect("Key was not provided");
    let value = args.next().expect("Key was not provided");

    let mut database = Database::new("kv.db").expect("Failed to create database");
    database.insert(key, value);
    // The database is saved automatically when the database goes out of scope and is dropped
    // match database.flush() {
    //     Ok(()) => println!("Value added"),
    //     Err(_) => println!("Failed"),
    // }
}
