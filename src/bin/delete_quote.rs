extern crate diesel;
extern crate rtss;

use self::diesel::prelude::*;
use self::rtss::*;
use std::env::args;

fn main() {
    use rtss::schema::quotes::dsl::*;

    let arg_id = args()
        .nth(1)
        .expect("Expected an ID identify Quote to delete.");

    let num_id = arg_id.parse::<i32>().expect("Expected numeric ID.");

    let connection = establish_connection();

    let num_deleted = diesel::delete(quotes.filter(id.eq(num_id)))
        .execute(&connection)
        .expect(&format!("Error deleting quote with id: {}", arg_id));

    println!("Deleted {} quotes.", num_deleted);
}
