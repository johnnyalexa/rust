fn main() {
    let principal = 280478.91;
    let interest_rate = 0.048;
    let term_months = 323;
    let payment = monthly_payment(principal, interest_rate, term_months);
    let interest = monthly_interest(principal, interest_rate);

    println!("Monthly payment: {:.2}, interest {:.2}, principal = {:.2}", payment, interest, payment-interest);

}


fn monthly_payment(principal: f64, interest_rate: f64, term_months: i32) -> f64 {
    let r = interest_rate / 12.0;
    let n = term_months as f64;
    (r * principal) / (1.0 - (1.0 + r).powf(-n))
}

fn monthly_interest(principal: f64, interest_rate: f64) -> f64 {
    let r = interest_rate / 12.0;
    (r * principal)
}