fn main() {
    println!("Hello, world!");

    let sum1: u32 = sum(12,134);
    if sum1 > 20 {
        println!("{}", sum1);
    } else if sum1 < 20 {
        println!("Value is less than 20");
    } else {
        println!("Something else");
    }
    // If is an expression
    let expression_variable: u32 = if sum1 > 20 { 32 } else { 20 };
    println!("{}",expression_variable);

    // Loops


}
fn sum(x: u32, y:u32) -> u32 {
    x + y
}