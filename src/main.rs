extern crate core;

use std::io;

fn main() {
    println!("Entrer votre premier nombre entier");
    let a: u32 = read_var(&mut String::new())
                    .expect("Erreur de donnees saisies");

    println!("Entrer votre second nombre entier");
    let b: u32 = read_var(&mut String::new())
                    .expect("Erreur de donnees saisies");

    print!(" Le PGCD de {0} et {1} est {2}", a, b, pgcd(a, b));
}

fn read_var(value: &mut String) -> Result<u32, &str> {
    match io::stdin().read_line(value) {
        Ok(_n) => match value.trim().parse::<u32>() {
            Ok(value) => Ok(value),
            Err(error) => panic!("{}", error),
        },
        Err(error) => panic!("{}", error),
    }
}

fn pgcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        return a;
    }
    pgcd(b, a % b)
}
