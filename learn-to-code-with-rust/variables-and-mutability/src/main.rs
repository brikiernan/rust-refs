#![allow(unused_variables)] // Allow unused variables in whole module

const TAX_RATE: f64 = 0.0635;

const TOUCHDOWN_POINTS: i32 = 6;

type Meters = i32;

#[allow(unused_variables)] // Allow unused variables in function
fn main() {
    let apples = 5;
    let bananas = 10;
    let _fruits = apples + bananas;

    println!("I have {apples} apples and {bananas} bananas.");

    let mut gym_reps = 10;
    println!(
        "I did {0} reps at the gym today. {0} is a good number.",
        gym_reps
    );

    gym_reps = 15;

    println!("I increased my reps to {}.", gym_reps);

    let grams_of_protein = "100.245";

    let grams_of_protein = 100.245;

    let mut grams_of_protein = 100;

    grams_of_protein = 150;

    let coffee_price = 2.99;

    {
        println!("The price of coffee is ${coffee_price}.");
        let cookie_price = 1.99;
        println!("The price of a cookie is ${cookie_price}.");
    }

    // cookie price out of scope
    println!("The total price is ${}.", coffee_price);

    let total_price = (coffee_price * TAX_RATE) + coffee_price;

    println!("The total price is of coffee with tax is ${total_price}.");

    let mile_race_distance: Meters = 1609;
    #[allow(unused_variables)] // Allow unused variables to next line
    let two_mile_race_distance: Meters = 2 * mile_race_distance;

    let season: &str = "Summer";
    let mut points_scored: i32 = 28;
    println!("Initial points scored: {}", points_scored);
    points_scored = 35;
    let event_time: &str = "06:00";
    let event_time: i32 = 6;

    println!(
        "My favorite season is {season}. I scored {points_scored} points. A touchdown is worth {TOUCHDOWN_POINTS} points. The event starts at {event_time}:00."
    );

    let _favorite_beverage = "Arnold Palmer";
}
