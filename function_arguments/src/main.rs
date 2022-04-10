fn main() {
    // Funtion example 1  
    {
        let s1 = String::from("asd");
        example_1(s1);
        println!("{}",s1);
    }

    // Function example 2
    {
        let s1 = String::from("asd");
        example_2(s1);
        println!("{}", s1);
    }

    // Function example 3
    {
        let mut s1 = String::from("asd");
        example_3(&mut s1);
        println!("{}", s1);
    }
}

fn example_1(value: String) {
    value = String::from("re");
}

fn example_2(mut value: String) {
    value = String::from("der");
}

fn example_3(value: &mut String) {
    value =  String::from("dser");
}