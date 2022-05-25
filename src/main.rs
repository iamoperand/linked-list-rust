use std::fs;

use clap::Parser;

use list::List;

#[derive(Parser, Debug)]
struct CliArgs {
    /// relative path to the input file
    filepath: String,
}

fn parse_elements_from_file(path: String) -> Vec<i32> {
    let file_content = fs::read_to_string(path).expect("couldn't read the file!");
    println!("------ file content ------\n{}\n", file_content);

    let mut elements: Vec<i32> = file_content
        .split(",")
        .map(|x| x.parse::<i32>().unwrap()) // parse into i32
        .collect();

    elements.sort(); // sort elements in ascending order
    elements
}

fn main() {
    let cli_args = CliArgs::parse();
    let elements = parse_elements_from_file(cli_args.filepath);

    let mut list = List::new();
    for element in elements {
        list.push(element);
    }

    list.print("------ linked list ------");
    list.remove_at_index(1); // removing 2nd node
    list.print("------ without 2nd node ------");

    println!("------ solana command to transfer token ------");
    // print the command to transfer a token
    for token_value in list.iter() {
        println!("solana transfer --from <KEYPAIR> test_wallet {} --allow-unfunded-recipient --url https://api.devnet.solana.com --fee-payer <KEYPAIR>\n", token_value);
    }
}
