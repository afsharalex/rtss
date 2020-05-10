extern crate bigdecimal;
extern crate chrono;
#[macro_use]
extern crate diesel;
extern crate dotenv;

use self::models::{NewQuote, Quote};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");

    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}.", database_url))
}

pub fn create_quote(
    conn: &PgConnection,
    d_time: chrono::NaiveDateTime,
    price: bigdecimal::BigDecimal,
) -> Quote {
    use schema::quotes;

    let new_quote = NewQuote {
        created_at: d_time,
        price: price,
    };

    diesel::insert_into(quotes::table)
        .values(&new_quote)
        .get_result(conn)
        .expect("Error saving new quote.")
}
