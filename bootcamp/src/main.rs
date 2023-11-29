fn main() {
    println!("Welcome to the zkEVM Bootcamp!");
    println!("Number of fizz buzz:{}", fizz_buzz());
}

fn fizz_buzz() -> u32 {
    let mut counter: u32 = 0;

    for i in 0..=301 {

        if i % 3 == 0{
            println!("fizz");
        }
        
        if i % 5 == 0{
            println!("buzz");
        }

        if i % 3 == 0 && i % 5 == 0{
            println!("fizz buzz");
            counter += 1;
        }
    }
    counter
}
