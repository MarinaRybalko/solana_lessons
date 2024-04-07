fn fizz_buzz() {
    let mut fizz_buzz_count = 0;

    for i in 1..=301 {
        match (i % 3, i % 5) {
            (0, 0) => {
                println!("fizz buzz");
                fizz_buzz_count += 1;
            }
            (0, _) => println!("fizz"),
            (_, 0) => println!("buzz"),
            _ => {}
        }
    }

    println!("Number of times 'fizz buzz' occurred: {}", fizz_buzz_count);
}

fn main() {
    fizz_buzz();
    println!("Welcome to the Bootcamp!");
}
