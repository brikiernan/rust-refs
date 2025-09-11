fn eat_meal(mut meal: String) -> String {
    meal.clear();
    meal.push_str("Tuna");

    meal
}

fn main() {
    //  -- challenge 1
    let is_concert = true;
    let is_event = is_concert;
    // No, rust will not move ownership of the boolean value
    println!("is_concert: {}, is_event: {}", is_concert, is_event);

    let sushi = "Salmon";
    let dinner = sushi;
    // No, rust will not move ownership of the string slice/literal
    println!("sushi: {}, dinner: {}", sushi, dinner);

    let sushi = String::from("Salmon");
    let dinner = sushi.clone();
    // Yes, rust will move ownership of the String
    println!("sushi: {}, dinner: {}", sushi, dinner);

    // ownership of sushi is moved to eat_meal, then new ownership
    // is created for the returned String and assigned to meal
    let meal = eat_meal(sushi); // should be using a (&String) reference here, but this is for learning purposes
    println!("Eating {}", meal);

    // -- challenge 2

    let mut trip = start_trip();

    visit_philadelphia(&mut trip);
    trip.push_str(" and ");

    visit_new_york(&mut trip);
    trip.push_str(" and ");

    visit_boston(&mut trip);
    trip.push_str(".");

    show_itinerary(&trip);

    // --------
    // let person = String::from("Alice");
    // let genius = person.clone();
    // println!("Person: {}", person);
    // println!("Genius: {}", genius);

    // let my_stack_value = 5;
    // let my_integer_reference = &my_stack_value;

    // let my_heap_value = String::from("Hello");
    // let my_heap_reference = &my_heap_value;

    // println!("Stack value: {}", my_stack_value);
    // // (*) is dereferencing the reference to get the value,
    // // but it is not necessary for values with the display trait
    // println!("Stack value (reference): {}", *my_integer_reference);
    // println!("Heap value: {}", my_heap_value);
    // println!("Heap value (reference): {}", *my_heap_reference);

    // let static_str = "Hello";
    // println!("Static str: {}", static_str);

    // let value = String::from("some string");
    // print_my_value(&value);
    // println!("Value {} has been moved and cannot be used here.", value);
}

fn start_trip() -> String {
    String::from("The plan is...")
}

fn visit_philadelphia(itinerary: &mut String) {
    itinerary.push_str("Philadelphia");
}

fn visit_new_york(itinerary: &mut String) {
    itinerary.push_str("New York");
}

fn visit_boston(itinerary: &mut String) {
    itinerary.push_str("Boston");
}

fn show_itinerary(itinerary: &String) {
    println!("{}", itinerary);
}

// fn print_my_value(value: &String) {
//     println!("Value: {}", value);
// }
