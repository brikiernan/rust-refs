#[derive(Debug)]
struct Food {
    name: String,
}

#[derive(Debug)]
struct Restaurant {
    reservations: u32,
    has_mice_infestation: bool,
}

impl Restaurant {
    fn chef_special(&self) -> Option<Food> {
        if self.has_mice_infestation {
            None
        } else if self.reservations < 12 {
            Some(Food {
                name: "Uni Sashimi".to_string(),
            })
        } else {
            Some(Food {
                name: "Strip Steak".to_string(),
            })
        }
    }

    fn deliver_burger(&self, address: &str) -> Result<Food, String> {
        if self.has_mice_infestation {
            Err("Sorry, we have a mice problem".to_string())
        } else if address.is_empty() {
            Err("No delivery address specified".to_string())
        } else {
            Ok(Food {
                name: "Burger".to_string(),
            })
        }
    }
}

fn main() {
    let restaurant_with_mice = Restaurant {
        reservations: 11,
        has_mice_infestation: true,
    };

    println!("{:?}", restaurant_with_mice.chef_special());
    println!(
        "{:?}",
        restaurant_with_mice.deliver_burger("123 Elm Street")
    );

    let restaurant_without_mice = Restaurant {
        reservations: 15,
        has_mice_infestation: false,
    };

    println!("{:?}", restaurant_without_mice.chef_special());
    println!("{:?}", restaurant_without_mice.deliver_burger(""));
    println!(
        "{:?}",
        restaurant_without_mice.deliver_burger("456 Oak Avenue")
    );
}
