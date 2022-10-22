use std::io;

fn main() {
    // ########## HELLO WORLD ##########

    let my_st_var = "Hello World";
    let my_nd_var_typed: &str = "Hello World";

    println!("Not typed var: {}", my_st_var);
    println!("Typed var {}", my_nd_var_typed);

    // ########## VARIABLES ##########

    let unmutted_var = "Alyson";
    //  unmutted_var = "Renan"; => will thows an error

    let mut mutted_var = 22;
    println!("Person: {}", unmutted_var);
    println!("Age: {}", mutted_var);

    mutted_var += 1;

    println!("Person: {}", unmutted_var);
    println!("Age: {}", mutted_var);

    // ########## TYPES ##########

    let _string_var: &str = "Alyson";

    let _integer_var_8_bits_length: i8 = 8;
    let _integer_var_16_bits_length: i16 = 16;
    let _integer_var_32_bits_length = 32;
    let _integer_var_64_bits_length: i64 = 64;
    let _integer_var_128_bits_length: i128 = 128;

    let _integer_unsigned_var_8_bits_length: u8 = 8;
    // let integer_unsigned_var_8_bits_length: u8 = -8 => will thows an error

    let _floating_var_32_bits_length: f32 = 32.0;
    let _floating_var_32_bits_length = 64.0;

    let _boolean_var_truthy: bool = true;
    let _boolean_var_falsy: bool = false;

    // ########## CONDITIONAL FLOW ##########

    let st_number: i8 = 10;
    let nd_number: i8 = 20;

    if st_number > nd_number {
        println!("{} é maior que {}", st_number, nd_number);
    } else if nd_number > st_number {
        println!("{} é menor que {}", st_number, nd_number);
    } else {
        println!("{} e {} são iguais", st_number, nd_number);
    }

    // ########## COMPARING USER INPUT NUMBERS ##########

    println!("Digite o valor de x: ");
    let mut data_input_1 = String::new();
    io::stdin()
        .read_line(&mut data_input_1) // try to read some input or throws error
        .expect("Error while reading data input number 1"); // handling with error throwed

    let x = data_input_1
        .trim() // remove initial and final spaces
        .parse::<i32>() // parse from string to int
        .unwrap(); // uncapsule result - discouraged use

    println!("Digite o valor de y: ");
    let mut data_input_2 = String::new();
    io::stdin()
        .read_line(&mut data_input_2)
        .expect("Error while reading data input number 2");
    let y = data_input_2.trim().parse::<i32>().unwrap();

    if x > y {
        println!("{}:x é maior que {}:y", x, y);
    } else if y > x {
        println!("{}:x é menor que {}:y", x, y);
    } else {
        println!("{}:x e {}:y são iguais", x, y);
    }
}
