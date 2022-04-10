fn main() {
    let  s = "Hello";
    // string
    {
        let mut s2 = String::from("Roman");
        println!("{}", s2);
        s2.push_str(", siema");
        println!("{}", s2)
    }

    // Let make other string
    {
        let mut s1 = String::from("Hello");
        let  s2 = &s1;
        println!("{}",s1);
    }
    // Clone pointer to heap
    {
        let s1 = String::from("Hi from value");
        let s2 = s1.clone();
        println!("{}", s1);
    }

    {
        let s = String::from("Roman");
        some_function(s);
        //println!("{}", s);
    }

    // Mutate string
    {
        let s = String::from("Roman");
        some_change_function(s);
        //println!("{}",s);
    }
}

fn some_function(s: String) {
    println!("{}", s);
}
fn some_change_function(mut s: String) {
    s = String::from("Marian");
    println!("{}", s);
}
