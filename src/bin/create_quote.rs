extern crate bigdecimal;
extern crate chrono;
extern crate diesel;
extern crate rtss;

use self::rtss::*;
use bigdecimal::BigDecimal;
use chrono::{NaiveDate, NaiveDateTime};
use std::io::{stdin, Read};
use std::str::FromStr;

fn main() {
    let connection = establish_connection();

    println!("Please enter a price:");
    let mut price_string = String::new();
    stdin().read_line(&mut price_string).unwrap();

    price_string = price_string[..(price_string.len() - 1)].to_string(); // Drop the newline char!

    let price = BigDecimal::from_str(&mut price_string).expect("Incorrect input.");

    let dt: NaiveDateTime = NaiveDate::from_ymd(2020, 1, 1).and_hms(10, 15, 0);

    let quote = create_quote(&connection, dt, price);

    println!(
        "\nSaved quote with \nid: {}\nprice: {}\ncreated_at: {}",
        quote.id, quote.price, quote.created_at
    );
}
