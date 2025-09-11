fn main() {
    let num: i32 = 1_337;
    let num_i16 = num as i16;

    let float_num: f32 = 3.14159;
    println!("{:.3}", float_num);

    let with_milk: bool = true;
    let with_sugar: bool = false;
    let is_my_type_of_coffee = with_milk && with_sugar;
    let is_acceptable_coffee = with_milk || with_sugar;

    let arr: [i8; 4] = [1, 2, 3, 4];
    println!("{:#?}", arr);

    let tuple = (num, float_num, with_milk, arr);
    dbg!(tuple);
}
