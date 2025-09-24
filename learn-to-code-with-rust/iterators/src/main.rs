#![allow(unused, dead_code)]
#[derive(Debug, PartialEq, Eq, Hash)]
enum Product {
    Blender,
    Microwave,
    Toaster,
    Fridge,
}

#[derive(Debug)]
struct CustomerOrder {
    product: Product,
    quantity: u32,
    shipped: bool,
}

impl CustomerOrder {
    fn new(product: Product, quantity: u32, shipped: bool) -> Self {
        Self {
            product,
            quantity,
            shipped,
        }
    }
}

#[derive(Debug)]
struct Customer {
    id: u32,
    orders: Vec<CustomerOrder>,
}

fn main() {
    let mut orders = vec![
        CustomerOrder::new(Product::Blender, 3, false),
        CustomerOrder::new(Product::Microwave, 1, true),
        CustomerOrder::new(Product::Toaster, 2, false),
        CustomerOrder::new(Product::Microwave, 5, true),
        CustomerOrder::new(Product::Blender, 1, false),
        CustomerOrder::new(Product::Fridge, 10, false),
    ];

    let customer_ids_by_order = [2, 1, 2, 3, 4, 1];

    //-- 1
    let blender_orders: Vec<&CustomerOrder> = orders
        .iter()
        .filter(|order| order.product == Product::Blender)
        .collect();
    println!("Blender Orders: {:#?}", blender_orders);

    // -- 2
    let total_microwave_quantity: u32 = orders
        .iter()
        .filter(|order| order.product == Product::Microwave)
        .map(|order| order.quantity)
        .sum();
    println!("Total Microwave Quantity: {}", total_microwave_quantity);

    // -- 2a
    let total_microwave_quantity_filter_map: u32 = orders
        .iter()
        .filter_map(|order| {
            if order.product == Product::Microwave {
                Some(order.quantity)
            } else {
                None
            }
        })
        .sum();
    println!(
        "Total Microwave Quantity (filter_map): {}",
        total_microwave_quantity_filter_map
    );

    // -- 3
    use std::env;
    let min_quantity: u32 = env::args()
        .skip(1)
        .take(1)
        .map(|quantity| quantity.parse::<u32>().unwrap_or(2))
        .next()
        .unwrap_or(2);
    let filtered_orders: Vec<&CustomerOrder> = orders
        .iter()
        .filter(|order| order.quantity >= min_quantity)
        .collect();
    println!(
        "Orders with quantity >= {}: {:#?}",
        min_quantity, filtered_orders
    );

    // -- 4
    use std::collections::HashMap;

    let inventory =
        orders
            .iter()
            .filter(|order| !order.shipped)
            .fold(HashMap::new(), |mut acc, order| {
                *acc.entry(&order.product).or_insert(0) += order.quantity;
                acc
            });
    println!("Inventory for unshipped orders (fold): {:#?}", inventory);

    let mut inventory: HashMap<&Product, u32> = HashMap::new();
    for order in orders.iter().filter(|order| !order.shipped) {
        *inventory.entry(&order.product).or_insert(0) += order.quantity;
    }
    println!(
        "Inventory for unshipped orders (for loop): {:#?}",
        inventory
    );

    // -- 5
    if let Some(order) = orders.iter_mut().find(|order| !order.shipped) {
        order.shipped = true;
    }
    println!("Customer Orders: {:#?}", orders);

    // -- 6
    let mut customers = orders
        .into_iter()
        .zip(customer_ids_by_order)
        .fold(HashMap::new(), |mut acc, (order, customer_id)| {
            acc.entry(customer_id).or_insert(vec![]).push(order);
            acc
        })
        .into_iter()
        .map(|(id, orders)| Customer { id, orders })
        .collect::<Vec<Customer>>();

    // let mut customers_map: HashMap<u32, Vec<CustomerOrder>> = HashMap::new();
    // for (order, &customer_id) in orders.iter().zip(customer_ids_by_order.iter()) {
    //     customers_map
    //         .entry(customer_id)
    //         .or_insert_with(Vec::new)
    //         .push(CustomerOrder {
    //             product: order.product.clone(),
    //             quantity: order.quantity,
    //             shipped: order.shipped,
    //         });
    // }
    // let mut customers: Vec<Customer> = customers_map
    //     .into_iter()
    //     .map(|(id, orders)| Customer { id, orders })
    //     .collect();
    customers.sort_by_key(|customer| customer.id);
    println!("Customers: {:#?}", customers);
}
