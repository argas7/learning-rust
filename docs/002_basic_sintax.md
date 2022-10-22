# Declarando variáveis

## Conceitos

- **let** ⇒ utilizado para indicar a criaçõa de uma variável;

## Dicas
- Em rust, o padrão é utilizar variáveis em snake_case;
  ```rust
  let userName = "Alyson"; // wrong
  let user_name = "Alyson"; // right
  ```

- Você pode explicitamente indicar o tipo da variável, mas caso não faça, isto será inferido pela linguagem;
  ```rust
  let my_st_var = "Hello World";
  let my_nd_var_typed: &str = "Hello World";
  ```

- É preciso utilizar **{}** para indicar o local onde a variável irá aparecer no print
  ```rust
  let name = "Alyson";

  println!("Hello {}!", name);
  ```

- Variáveis em rust são **imutáveis**; até que se diga o contrário...
  ```rust
  let name = "Alyson";
  name = "Renan";
  // wrong

  let mut age = 22;
  age += 1;
  // right
  ```

- Caso queira definir uma variável que não será utilizada, a prefixe com **_**
  ```rust
  let unused_var_with_warning = "Alyson"; // wrong
  let _unused_var_without_warning = "Renan"; // right
  ```

- É indicado que parenteses desnecessários não sejam utilizados
  ```rust
  if (st_number > nd_number) {} // warning
  if st_number > nd_number {} // good
  ```

## Tipos de dados

- Strings
  - &str

- Inteiros
  - i8 ⇒ 8 bits de memória
  - i16 ⇒ 16 bits...
  - i32 ⇒ 32 bits _(default)_
  - i64 ⇒ 64 bits
  - 128 ⇒ 128 bits

  - variação, utilização de __u__ *(unsigned)* para variáveis somente inteiras
    - u8; u16; u32; ...

- Pontos flutuantes
  - f32 ⇒ 32 bits
  - f64 ⇒ 64 bits _(default)_

- Booleanos
  - bool