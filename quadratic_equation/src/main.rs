use std::io;

fn main() {
    //ax^2 + bx + c = 0

    let mut a_str = String::new();
    let mut b_str = String::new();
    let mut c_str = String::new();

    let mut i = 0;

    loop {
        println!("Solve the quadratic equation");

        println!("Please, enter a");
        match io::stdin().read_line(&mut a_str) {
            Ok(_) => {}
            Err(e) => println!("Input error - {}", e),
        }

        println!("Please, enter b");
        match io::stdin().read_line(&mut b_str) {
            Ok(_) => {}
            Err(e) => println!("Input error - {}", e),
        }

        println!("Please, enter c");
        match io::stdin().read_line(&mut c_str) {
            Ok(_) => {}
            Err(e) => println!("Input error - {}", e),
        }

        let a: f64 = a_str.trim().parse().unwrap();
        let b: f64 = b_str.trim().parse().unwrap();
        let c: f64 = c_str.trim().parse().unwrap();

        let discriminator: f64 = (b * b) - 4.0 * (a * c);

        if discriminator > 0.0 {
            let x1 = ((-b) + discriminator.sqrt()) / (2.0 * a);
            let x2 = ((-b) - discriminator.sqrt()) / (2.0 * a);
            println!(
                "Solution:\n Discriminator => {}\n Has 2 roots. Root1 => {}, Root2 => {} \n",
                discriminator, x1, x2
            );
        }
        if discriminator == 0.0 {
            let x = (-b) / (2.0 * a);
            println!(
                "Solution:\n Discriminator => 0\n Has 1 root. Root  => {}",
                x
            );
        }
        if discriminator < 0.0 {
            println!(
                "Solution:\n Discriminator => {}. Discriminator < 0, there is no roots! ",
                discriminator
            );
        }
    }
}
