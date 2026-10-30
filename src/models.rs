#[derive(Debug)]
pub struct CreditCard {
    pub name: String,
    pub balance: f32,
    pub min_due: i32,
    pub due_date: i32,
    pub is_closed: bool,
    pub apr: f32,
    pub credit_limit: f32
}
