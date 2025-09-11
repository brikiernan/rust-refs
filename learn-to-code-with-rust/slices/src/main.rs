fn main() {
    let mut cereals = [
        String::from("Cookie Crisp"),
        String::from("Cinnamon Toast Crunch"),
        String::from("Frosted Flakes"),
        String::from("Cocoa Puffs"),
        String::from("Captain Crunch"),
    ];

    let first_two = &cereals[0..2];
    println!("First two cereals: {:?}", first_two);

    let mid_three = &cereals[1..4];
    println!("Middle three cereals: {:?}", mid_three);

    let last_three = &mut cereals[2..5];
    println!("Last three cereals: {:?}", last_three);

    last_three[2] = String::from("Lucky Charms");
    println!("Updated cereals: {:?}", cereals);

    let cookie_crisp = &cereals[0];
    let cookie = &cookie_crisp[0..6];
    println!("Slice of 'Cookie': {}", cookie);

    let cocoa_puffs = &cereals[3];
    let puffs = &cocoa_puffs[6..];
    println!("Slice of 'Puffs': {}", puffs);
}
