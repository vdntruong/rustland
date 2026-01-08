use chrono::naive::{NaiveDate};
use chrono::offset::{Local};
use std::ops::Add;

#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
}

#[derive(Debug)]
struct Marriage {
    husband: Person,
    wife: Person,
    location: String,
    date: NaiveDate,
}

impl Add for Person {
    // define the return type for the add operator (Marriage)
    type Output = Marriage;

    fn add(self, other: Person) ->  Self::Output {
        Marriage {
            husband: self,
            wife: other,
            location: "Arizona".to_string(),
            date: Local::now().naive_local().date(),
        }
    }
}

#[derive(Debug)]
struct GroceryItem {
    name: String,
    price: f32,
}
#[derive(Debug)]
struct GroceryBill{
    items: Vec<GroceryItem>,
    tax_rate: f32,
}

impl GroceryBill {
    fn calculate_total(&self) -> f32 {
        let total = self.items.iter().fold(0.0, |acc, item| acc + item.price);
        let tax = total * self.tax_rate;
        total + tax
    }
}

impl Add<GroceryItem> for GroceryBill {
    type Output = GroceryBill;

    fn add(self, other: GroceryItem) -> Self::Output {
        let mut bill = self;
        bill.items.push(other);
        bill
    }
}

fn main() {
    // Marriage example
    let person1 = Person {
        first_name: String::from("John"),
        last_name: String::from("Smith"),
    };
    let person2 = Person {
        first_name: String::from("Nancy"),
        last_name: String::from("Jones"),
    };

    let marriage = person1 + person2;
    println!("{:#?}", marriage);

    // Grocery example
    let carrot = GroceryItem{name: String::from("Carrot"), price: 2.0};
    let paper = GroceryItem{name: String::from("Paper"), price: 1.5};
    let cheese = GroceryItem{name: String::from("Cheese"), price: 3.5};

    let bill = GroceryBill{items: vec![], tax_rate: 0.05};
    let bill = bill + carrot + paper + cheese;
    let total = bill.calculate_total();
    println!("Total {:#?}", total);
}
