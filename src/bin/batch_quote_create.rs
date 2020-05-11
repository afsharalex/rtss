/* This will by default, create quotes sequentially. I may add a flag to create them for the same time in the future.
 */

extern crate bigdecimal;
extern crate chrono;
extern crate diesel;
extern crate rand;
extern crate rtss;

use self::rtss::*;
use bigdecimal::{BigDecimal, ToPrimitive};
use chrono::{NaiveDate, NaiveDateTime};
use rand::Rng;
use std::env::args;
use std::io::{stdin, Read};
use std::str::FromStr;

fn main() {
    use rtss::schema::quotes::dsl::*;

    let arg_num = args()
        .nth(1)
        .expect("Expected a number of quotes to create.");

    let num_quotes = arg_num
        .parse::<i32>()
        .expect("Incorrect input for Number of Quotes.");

    let arg_base_price = args().nth(2).expect("Expected a base price for quotes.");

    let mut base_price =
        BigDecimal::from_str(&arg_base_price).expect("Incorrect input for Base Price of Quotes.");

    println!(
        "\nCreating {} Quotes from base price {:?}",
        num_quotes,
        base_price.to_f32().unwrap()
    );
}
