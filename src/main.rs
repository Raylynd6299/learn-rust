fn main() {
    let mut names: Vec<String> = Vec::new();


    for _index in 1..5 {
        println!("Porfavor introduce un nombre: ");

        let mut name: String = String::new();
        std::io::stdin().read_line(&mut name).unwrap();

        name = name.trim().to_string();

        names.push(name);
    }

    println!("{:?} {} ",names, names.len());

    for internal_name in names{
        println!("{internal_name}")
    }
}
