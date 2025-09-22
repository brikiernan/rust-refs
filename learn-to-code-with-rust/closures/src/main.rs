#[derive(Debug)]
struct SupermarketItem {
    name: String,
    price: f64,
}

#[derive(Debug)]
struct ShoppingCart {
    items: Vec<SupermarketItem>,
}

impl ShoppingCart {
    fn traverse_items<F>(&mut self, mut operation: F)
    where
        F: FnMut(&mut SupermarketItem),
    {
        let mut index = 0;
        while index < self.items.len() {
            operation(&mut self.items[index]);
            index += 1;
        }
    }

    fn checkout<F>(self, operation: F)
    where
        F: FnOnce(ShoppingCart),
    {
        operation(self);
    }
}

fn main() {
    let mut cart = ShoppingCart {
        items: vec![
            SupermarketItem {
                name: "APPLE".to_string(),
                price: 3.99,
            },
            SupermarketItem {
                name: "BANANA".to_string(),
                price: 2.99,
            },
        ],
    };

    cart.traverse_items(|item| {
        item.price *= 0.85;
    });

    cart.traverse_items(|item| {
        item.name = item.name.to_lowercase();
    });

    let mut total_price = 0.0;
    cart.checkout(|mut cart| {
        println!("Final shopping cart: {:?}", cart);

        cart.traverse_items(|item| {
            total_price += item.price;
        });
    });

    println!("Total price at checkout: ${:.2}", total_price);
}
