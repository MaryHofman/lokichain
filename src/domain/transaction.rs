#[derive(Debug, Clone, Serrialize)]

struct Transaction{
    sender: String,
    resiver: String,
    amount: f64,
}