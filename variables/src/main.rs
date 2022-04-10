fn main() {
    let mut x = 5;
    const THREE_HOUR_PER_SECOND: u32 = 60 * 80 * 90;
    println!("x value is {}", 5);
    x = 6;
    println!("x now is {}", x);

    // Shadowing

    let mut x = 6;
    println!("X value is {}",x);

    let x = 5;

    println!("X value is {}",x);
    {
        let x = x * 2;
        println!("X value is {}", x);
    }
    println!("X value is {}",x);
}
