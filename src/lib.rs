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

pub fn batch_create_quotes(
    conn: &PgConnection,
    base_time: chrono::NaiveDateTime,
    base_price: bigdecimal::BigDecimal,
    num_quotes: i32,
) -> Vec<Quote> {
    // Create first quote with base price and time
    let mut quotes_vec: Vec<Quote> = vec![];

    let mut curr_time = base_time.clone();
    let mut curr_price = base_price.clone();

    let first_quote = create_quote(conn, base_time, base_price);
    quotes_vec.push(first_quote);

    // loop for num_quotes -1 generating random prices, and times
    // TODO: For now, the interval will be 15 minutes, but this should be an argument.
    for i in 1..(num_quotes - 1) {
        //TODO: Need to randomly choose whether price increase, decreases, or remains the same
        // For now, just increase the price by 5
        curr_price = curr_price + bigdecimal::BigDecimal::from(5);
        curr_time = curr_time + chrono::Duration::minutes(15);

        quotes_vec.push(create_quote(&conn, curr_time.clone(), curr_price.clone()));
    }

    quotes_vec
}
