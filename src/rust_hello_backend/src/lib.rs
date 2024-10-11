#[ic_cdk::query]
fn greet(number1: i64, number2: i64) -> String {
    let number = number1 + number2;
    format!("Hello My Friend, the number is {}!", number)
}
