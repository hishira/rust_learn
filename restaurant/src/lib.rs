mod front_of_house;

pub use crate::front_of_house::front_of_house::Breakfast;
fn something_another() {

}
pub fn eat_at_restaurant() {
    //crate::front_of_house::hosting::add_to_waitlist();
    //front_of_house::hosting::add_to_waitlist();
    let mut meal = Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("{}", meal.toast);
}
