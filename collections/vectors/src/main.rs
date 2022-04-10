fn main() {
    println!("Hello, world!");
    let mut vec1: Vec<i32> = Vec::new();
    let vec2 = vec![1,2,3,4,5];
    vec1.push(1);
    vec1.push(2);
    vec1.push(3);
    vec1.push(4);
    vec1.push(5);
    vec1.push(6);
    vec1.push(7);

   for i in 0..vec1.len(){
       println!("{}", vec1.get(i).expect("Something goes bad"));
   }

    for i in &vec1 {
        println!("{}",i)
    }
}
