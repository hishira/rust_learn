mod string{
    pub fn string_section_create() {
        // Tworzenie stringow
        let new_string = String::new();
        let s = "Display something good";
        let s2 = s.to_string();
        let s3 = String::from("To something from");
        println!("{}, {}, {}, {}",s,s2,s3,new_string);
    }

    pub fn string_section_update(){
        // Update stringow
        let mut s = String::from("My name is ");
        let s2 = "Giobani Giorgio";
        s.push_str(s2);
        let mut swithchar = String::from("Hello");
        let charone = 'z';
        println!("{}, {}", s,s2);
        swithchar.push(charone);
        println!("{}", swithchar);
    }

    pub fn concat_string(){
        // s1 tracimy, s2 zostaje
        let s1 = String::from("Hello ");
        let s2 = String::from("World");
        let s3 = s1 + &s2;
        println!("{},{}",s2,s3);

        {
            let s1 = String::from("Hello ");
            let s2 = String::from("World");
            let s3 = String::from(" from macro format!");
            let s4 = format!("{} {} {}",s1,s2,s3);
            println!("{}", s4);
        }
    }

    pub fn string_iterate(){
        let s = String::from("Hello world");
        for letter in s.chars() {
            println!("{}",letter);
        }
    }
}
fn main() {
    string::string_section_create();
    string::string_section_update();
    string::concat_string();
    string::string_iterate();
}
