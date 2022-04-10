fn main() {

    // Reference
    {
        let s = String::from("Moja kawa i rabarbary");
        println!("{}", reference_function(&s));
        println!("{}", s);
    }

    // Mutable reference
    {
        let mut s = String::from("Aga");
        mutable_reference(&mut s);
        println!("{}", s);
        let r1 = &mut s;
        println!("{}", r1);
        let r2 = &mut s;
        println!("{}",r2);
    }
    //DAggiling
    {
        let s = string_return();
        println!("{}", s);
    }
}

fn reference_function(s: &String) ->usize {
    s.len()
}

fn mutable_reference(s: &mut String) {
    s.push_str(" Something else");
}
fn string_return()-> &String{
    return &String::from("astalavista");
}