use super::bigdecimal;
use super::chrono;

// Queryable assumes the order for models is
// the same as defined in schema file.
#[derive(Queryable)]
pub struct Quote {
    pub id: i32,
    pub created_at: chrono::NaiveDateTime,
    pub price: bigdecimal::BigDecimal,
}
