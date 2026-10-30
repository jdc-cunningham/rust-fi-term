#[derive(Debug)]
struct CreditCard {
    name: String,
    balance: f32,
    min_due: i32,
    due_date: i32,
    is_closed: bool,
    apr: f32,
    credit_limit: f32
}
