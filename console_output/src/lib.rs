use std::{fs::File, io::Read, process};
use products;

pub fn args_handler(args: String){
    let splitted_args: Vec<_> = args.split(" ").map(|s| s.trim()).collect();
    match splitted_args[0].to_lowercase().as_str() {
        
        "version" => println!("Current version is 1.0"),
        "help" => help_output(),
        "stop" | "exit" => {
            process::exit(0)
        },
        
        "product" => {

            if !splitted_args.contains(&":")
            {
                println!("There should be a \":\" sign after either \"add\" or \"delete\"");
                help_output();
                return;
            }

            let index = splitted_args.iter().position(|x| *x == ":").unwrap();

            if index == 0 {
                println!("You need to add more info about your product");
                help_output();
                return;
            }

            let args = splitted_args[index+1..].to_vec();
            let args = args.join(" ");

            if splitted_args[1].to_lowercase() == "add" {
                let _ = match products::product_handler(args) {
                    Ok(product) => products::add_product(product),
                    Err(error) => { 
                        println!("{}", error);
                        help_output();
                        return;
                    },
                };

            }else if splitted_args[1].to_lowercase() == "delete" || splitted_args[1].to_lowercase() == "remove" || splitted_args[1].to_lowercase() == "del" {
                products::remove_product(args);
            }
        },
        _ => {
            println!("{} is not an option, check help menu", splitted_args[0]);
            help_output();
        },
    }
}

fn help_output() {
    let mut file = File::open("console_output/help_output.txt").expect("Was not able to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Was not able to read to string a file");

    println!("{}", contents);
}