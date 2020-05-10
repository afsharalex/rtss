use super::bigdecimal;
use super::chrono;

#[derive(Queryable)]
pub struct Quote {
    pub id: i32,
    pub created_at: chrono::NaiveDateTime,
    pub price: bigdecimal::BigDecimal,
}
