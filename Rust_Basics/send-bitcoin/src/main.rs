use std::io;
use rand::Rng;

fn main() {
    console()
}

fn console() {
    println!("\nLet's have fun with Bitcoin!\n");
    println!("Do you want to send (s) or receive (r) Bitcoin?\n");

    let mut command = String::new();

    io::stdin().read_line(&mut command);

    if command.trim().eq("s") {
        send_bitcoin()
    } else if command.trim().eq("r") {
        receive_bitcoin()
    } else {
        exit_console()
    }
}

fn send_bitcoin() {
    println!("\nWe're going to send some Bitcoin!\n");

    let clients = vec!["Homer", "Marge", "Bart", "Lisa"];

    println!("Who do you want to send Bitcoin to?\n");

    for client in &clients {
        println!("{} ", client);
    }

    println!("\n");

    let mut recipient = String::new();

    io::stdin().read_line(&mut recipient);

    if clients.contains(&recipient.trim()) {
        println!("How many Bitcoin do you want to send?\n");

        let mut amount = String::new();

        io::stdin().read_line(&mut amount);

        println!("\nYou sent {} Bitcoin to {}!\n", amount.trim(), recipient.trim());
    } else {
        println!("{} is not in your contacts!", recipient.trim());
    }
}

fn receive_bitcoin() {
    println!("\nWe're going to receive some Bitcoin!\n");

    let amount = rand::thread_rng().gen_range(1, 10);

    println!("You just received {} Bitcoin!\n", amount);
}

fn exit_console() {
    println!("Invalid option, must be (s) or (r)")
}



