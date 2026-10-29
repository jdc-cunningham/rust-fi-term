#[derive(Debug)]
struct CreditCard {
    id: i32,
    name: String,
    balance: f32,
    min_due: i32,
    due_date: i32,
    is_closed: bool,
    apr: i32,
    credit_limit: f32
}
