// Permite que o compilador não avise sobre variaveis não ultilizadas
#![allow(unused_variables)]

fn main() {
    println!("Hello, world!");

    // i para números com sinal
    let constante:i8 = -14;

    // u para números sem sinal
    let outro_numero:u8 = 47;

    // Rust também "Adivinha" o tipo
    let inferen = 50;

    // Geralmente usado para endereçamento de memória
    let numero_arquiteturadosistema_i: isize = 1024;
    let numero_arquiteturadosistema_u: usize = 1024;
    println!("Número baseado na arquitetura do sistema: {}", numero_arquiteturadosistema_i);

    // Float
    let numero_fracionario: f32 = 3.54;
    let numero_fracionario: f64 = 7.18;

    // boolean type
    let flag1: bool = true;
    let flag2: bool = false;
    println!("flag1 = {}", flag1);
    println!("flag2 = {}", flag2);

    // char type
    let character: char = 'z';
    println!("character = {}", character);

    // Array: [tipo;tamanho]
    let lista: [f32; 2] = [0.00, 0.00];
    // Para arrays muito grandes [valor padrão; quantidade]
    let array_gigante = [0; 10000];

    // Tupla
    let location = ("Brazil", 45.6786, 'B', 60);
    // Melhor forma de acessar os valores de uma tupla
    let (pais, lat, letra, inteiro) = location;
    println!("Estamos no {}, número fracionario: {}, char: {}, Inteiro: {}", location.0, location.1, location.2, location.3);
    println!("Estamos no {}, número fracionario: {}, char: {}, Inteiro: {}", pais, lat, letra, inteiro);

    // String and str or string slice
    // Slices são imutaveis
    let name = "Marcello";
    let name_string = name.to_string();
    let another_name = String::from("Marcello");

    // Trabalhando com uma slice na Heap
    let string_to_slice = &name_string;

    // Trabalhando com uma slice na Stack
    let string_to_slice2 = name_string.as_str();

    // Concatenando strings
    // Contatenação de strings sempre terá uma String mutavel com um &str imutavel
    let duck = "Duck";
    let airlines = "Airlines";
    let duck_airlines = [duck, airlines].concat();

    // Format method
    let duck_airlines2 = format!("{} {}", duck, airlines);
    println!("Name: {}, or {}", duck_airlines, duck_airlines2);

    let mut slogan = String::new();
    slogan.push_str("Fly with us");
    slogan.push('!');

    // Concatenando com o operador + precisa de um &str (slice)
    slogan += " every time";
    println!("Slogan: {}", slogan);

}
