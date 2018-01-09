use std::io;

fn main() {

    println!("Welcome to Maeve!");

    loop {
        println!("Please select an option:");
        println!("1 - New Game");
        println!("2 - Load Game");
        println!("3 - Exit Game");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input.");

        let choice: u32 = match choice.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => println!("Welcome to Maeve, the hosts are here to serve you."),
            2 => println!("I see you've been a guest with us before."),
            3 => {
                println!("We look forward to your next visit.");
                break;
            },
            _ => println!("That is not how this works, choose again.")
        }
    }
}
