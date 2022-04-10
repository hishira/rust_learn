pub mod front_of_house {
    enum Fruit{
        Apple,
        Banana,
    }
    pub struct Breakfast{
        pub toast: String,
        seasonal_fruit: Fruit,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast{
                toast: String::from(toast),
                seasonal_fruit: Fruit::Apple,
            }
        }
    }
    pub mod hosting {
        pub fn add_to_waitlist(){}
        fn seat_at_table() {}
    }
    mod serving {
        pub fn take_order(){}
        fn serve_order(){}
        fn take_payment(){}
    }
    fn other(){
        super::super::something_another();
    }
    fn from_module(){
        serving::take_order()
    }
}