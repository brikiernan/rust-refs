use std::collections::HashMap;

fn main() {
    let mut sauces_to_meals = HashMap::from([
        ("Ketchup", vec!["French Fries", "Burgers", "Hot Dogs"]),
        ("Mayonnaise", vec!["Sandwiches", "Burgers", "Coleslaw"]),
    ]);

    sauces_to_meals.insert("Mustard", vec!["Hot dog", "Burgers", "Pretzels"]);

    if let Some(removed_meals) = sauces_to_meals.remove("Mayonnaise") {
        println!("Removed Mayonnaise meals: {:?}", removed_meals);
    }

    if let Some(mustard_meals) = sauces_to_meals.get("Mustard") {
        println!("Mustard meals: {:?}", mustard_meals);
    }

    sauces_to_meals
        .entry("Soy Sauce")
        .or_insert(vec!["Sushi", "Dumplings"]);

    println!("Final sauces to meals: {:?}", sauces_to_meals);
}
