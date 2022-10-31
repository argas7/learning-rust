# Trabalhando com estruturas de repetição

## while

1. Crie um código que receba um input numérico e execute a soma dos dígitos deste input:

```rust
let mut sum = 0;
let mut input = String::new();

io::stdin().read_line(&mut input).expect("Error while reading data input");

let mut input_integer = input.trim().parse::<i32>().unwrap();

while input_integer != 0 {
    let mut rest:i32 = input_integer % 10;
    sum = sum + rest;
    input_integer = input_integer / 10;
}

print!("Valor da soma dos dígitos é {}", sum);

```

2. Crie um código que receba um valor inteiro e calcule o fatorial deste valor:

```rust
let mut input_f = String::new();
io::stdin()
    .read_line(&mut input_f)
    .expect("Error while reading data input");

let mut factorial_input: i32 = input_f.trim().parse::<i32>().unwrap();
let mut factorial: i32 = 1;
let initial_factor = factorial_input;

while factorial_input != 1 {
    factorial *= factorial_input;
    factorial_input -= 1;
}

print!("{} fatorial é {}\n", initial_factor, factorial);
```

3. Faça os mesmos exemplos utilizando a sintaxe de loops do tipo **for**