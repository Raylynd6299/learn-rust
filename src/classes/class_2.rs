fn main() {
    let mut name: String = String::new(); // Cadena de texto, mnos optimo pero mas accesible
    let mut age: String = String::new();

    println!("Introduce tu nombre: ");
    //Recibir data del user
    std::io::stdin().read_line(&mut name).unwrap();
    name = name.trim().to_string();

    println!("Introduce tu edad: ");
    std::io::stdin().read_line(&mut age).unwrap();

    //  convertir a num
    let age_int:u8 = age.trim().parse().unwrap();

    println!("Hola Bienvenido {name} - {age_int}")
}