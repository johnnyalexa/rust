mod front_of_house;

pub use crate::front_of_house::hosting;


//pub use crate::front_of_house::hosting;

mod customer {
    pub use crate::front_of_house::hosting;
    use crate::back_of_house::Appetizer;
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();

        //front_of_house::hosting::add_to_waitlist();

        let mut meal = super::back_of_house::Breakfast::summer("Rye");
        meal.toast = String::from("Wheat");
        println!("I'd like {} toast please", meal.toast);

        //meal.seasonal_fruit = String::from("blueberries");

        let order1 = Appetizer::Soup;
        let order2 = Appetizer::Salad;
    }
}

fn deliver_order() { }

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }
    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}