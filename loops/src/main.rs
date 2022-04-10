fn main() {
    // Loops examples
    {
        loop {
            println!("again");
            break;
        }
        let mut count = 10;
        'counting_loop: loop {
            println!("count: {}", count);
            let mut counter5 = 5;
            'counter5: loop {
                println!("5counter: {}", counter5);
                if counter5 <= 0 {
                    break 'counter5;
                }
                counter5 -= 1;
            }
            if count <= 0 {
                break 'counting_loop;
            }
            count -= 1;
        }
    }

    // Loops can return value
    {
        let mut counter_check = 10;
        let a = loop {
            counter_check = counter_check * 3 + 3;
            break number_eval(counter_check);
        };
        println!("a value after evaluate {}", a);
    }

    // While loop
    {
        let mut counter_while: i32 = 20;
        while counter_while >= 0 {
            println!("counter while value: {}", counter_while);
            counter_while -= 1;
        }
    }
    // For loops
    {
        let elements: [u32; 9] = [1, 2, 3, 3, 2, 2, 1, 4, 3];
        for element in elements {
            println!("element in elements array: {}", element);
        }
        for number in (1..10).rev() {
            println!("number: {}", number);
        }
    }
}

fn number_eval(val: u32) -> u32 {
    if val % 2 == 0 {10} else {20}
}
