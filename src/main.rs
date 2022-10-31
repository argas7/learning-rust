use std::io;

fn _parse_int(data: String) -> i32 {
    let integer_value: i32 = data
        .trim() // remove initial and final spaces
        .parse::<i32>() // parse from string to int
        .unwrap(); // uncapsule result - discouraged use

    return integer_value;
}

fn _hello_world_code() {
    let my_st_var = "Hello World";
    let my_nd_var_typed: &str = "Hello World";

    println!("Not typed var: {}", my_st_var);
    println!("Typed var {}", my_nd_var_typed);
}

fn _variables_and_types_code() {
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
}

fn _conditional_code() {
    let st_number: i8 = 10;
    let nd_number: i8 = 20;

    if st_number > nd_number {
        println!("{} é maior que {}", st_number, nd_number);
    } else if nd_number > st_number {
        println!("{} é menor que {}", st_number, nd_number);
    } else {
        println!("{} e {} são iguais", st_number, nd_number);
    }

    println!("Digite o valor de x: ");
    let mut data_input_1 = String::new();
    io::stdin()
        .read_line(&mut data_input_1) // try to read some input or throws error
        .expect("Error while reading data input number 1"); // handling with error throwed

    let x = _parse_int(data_input_1);
    println!("Digite o valor de y: ");
    let mut data_input_2 = String::new();
    io::stdin()
        .read_line(&mut data_input_2)
        .expect("Error while reading data input number 2");
    let y = _parse_int(data_input_2);

    if x > y {
        println!("{}:x é maior que {}:y", x, y);
    } else if y > x {
        println!("{}:x é menor que {}:y", x, y);
    } else {
        println!("{}:x e {}:y são iguais", x, y);
    }
}

fn _sum_digits_func() {
    let mut sum = 0;
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error while reading data input");
    let mut input_integer = _parse_int(input);

    while input_integer != 0 {
        let rest: i32 = input_integer % 10;
        sum += rest;
        input_integer = input_integer / 10;
    }

    print!("Valor da soma dos dígitos é {}\n", sum);
}

fn _factorial_func() {
    let mut input_f = String::new();
    io::stdin()
        .read_line(&mut input_f)
        .expect("Error while reading data input");

    let mut factorial_input: i32 = _parse_int(input_f);
    let mut factorial: i32 = 1;
    let initial_factor = factorial_input;

    while factorial_input != 1 {
        factorial *= factorial_input;
        factorial_input -= 1;
    }

    print!("{} fatorial é {}\n", initial_factor, factorial);
}

fn main() {
  // _hello_world_code();  // uncomment if want to test
  // _variables_and_types_code();  // uncomment if want to test
  // _conditional_code();  // uncomment if want to test
  // _sum_digits_func();  // uncomment if want to test
  _factorial_func();
}
