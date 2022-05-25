use std::fs;

use clap::Parser;

use list::List;

#[derive(Parser, Debug)]
struct CliArgs {
    /// relative path to the input file
    file_path: String,
}

fn main() {
    let mut list = List::new();

    let cli_args = CliArgs::parse();
    println!("\nreading file: {}", cli_args.file_path);

    let contents =
        fs::read_to_string(cli_args.file_path).expect("something went wrong reading the file!");
    println!("\nfile contents: {}", contents);

    // parse the string into individual items
    let mut elements: Vec<i32> = contents
        .split(",")
        .map(|x| x.parse::<i32>().unwrap()) // parse into i32
        .collect();
    elements.sort();

    println!("\n**** adding items to the linked list ****");
    for element in elements {
        list.push(element);
    }

    list.print();

    list.remove_at_index(1);
    println!("\n**** removed 2nd node ****");

    list.print();


    // print the command to transfer a token
    for token_value in list.iter() {
        println!("\n solana transfer --from <KEYPAIR> test_wallet {} --allow-unfunded-recipient --url https://api.devnet.solana.com --fee-payer <KEYPAIR>", token_value);
    }
}
