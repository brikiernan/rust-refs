#[derive(Debug)]
struct Flight {
    origin: String,
    destination: String,
    price: f64,
    passengers: u32,
}

impl Flight {
    fn new(origin: String, destination: String, price: f64, passengers: u32) -> Self {
        Self {
            origin,
            destination,
            price,
            passengers,
        }
    }

    fn change_destination(&mut self, new_destination: String) {
        self.destination = new_destination;
    }

    fn increase_price(&mut self) {
        self.price *= 1.20;
    }

    fn itinerary(&self) {
        println!("{} -> {}", self.origin, self.destination);
    }
}

fn main() {
    let mut flight1 = Flight::new(
        String::from("New York"),
        String::from("Los Angeles"),
        300.0,
        150,
    );

    flight1.itinerary();
    flight1.increase_price();
    flight1.change_destination(String::from("San Francisco"));

    println!("{:?}", flight1);

    let flight2 = Flight {
        origin: String::from("Chicago"),
        destination: String::from("Miami"),
        ..flight1
    };

    println!("{:#?}", flight2);
}
