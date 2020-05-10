extern crate diesel;
extern crate rtss;

use self::diesel::prelude::*;
use self::models::*;
use self::rtss::*;

fn main() {
    use rtss::schema::quotes::dsl::*; // This will import aliases automatically created for quotes

    let connection = establish_connection();
    let results = quotes
        .limit(10)
        .load::<Quote>(&connection)
        .expect("Error loading quotes.");

    println!("Displaying {} quotes", results.len());

    for quote in results {
        println!("--------------------");
        println!("{}", quote.id);
        println!("{}", quote.created_at);
        println!("{}", quote.price);
        println!("--------------------");
    }
}
