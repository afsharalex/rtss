use super::bigdecimal;
use super::chrono;
use super::schema::quotes;

// Queryable assumes the order for models is
// the same as defined in schema file.
#[derive(Queryable)]
pub struct Quote {
    pub id: i32,
    pub created_at: chrono::NaiveDateTime,
    pub price: bigdecimal::BigDecimal,
}

#[derive(Insertable)]
#[table_name = "quotes"]
pub struct NewQuote {
    pub created_at: chrono::NaiveDateTime,
    pub price: bigdecimal::BigDecimal,
}
