use serde::{Deserialize, Serialize};
use serde_json::{from_value, json};
use core::time;
use std::{fs::{self, File, OpenOptions}, io::{stdout, Error, Read, Write}};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};

#[derive(Serialize, Deserialize, Clone)]
pub struct Product {
    id: u32,
    price: u32,
    quantity: u32,
    name: String,
    description: String,
}

const PRODUCTS_LIST:&str = "static/products.json";

pub fn product_handler(user_input: String) -> Result<Product, Error> {

    // example of the user_input: Laptop ; 200 ; 200 ; Very handle and powerful laptop
    let splitted_input: Vec<&str> = user_input.split(";").map(|s| s.trim()).collect();

    println!("{:?}", splitted_input);

    if splitted_input.len() != 4 {
        return Err(Error::new(std::io::ErrorKind::InvalidInput, "Invalid input of the product"));
    }

    let id = if let Ok(highest_id) = get_highest_id() {
        highest_id
    } else {
        return Err(Error::new(
            std::io::ErrorKind::InvalidData,
            "Was not able to assign an ID to your product",
        ));
    };

    let name = splitted_input[0].to_string();
    let price = splitted_input[1].parse().map_err(|_| Error::new(std::io::ErrorKind::InvalidInput, "Price should be an int"));
    let quantity = splitted_input[2].parse().map_err(|_| Error::new(std::io::ErrorKind::InvalidInput, "Quantity should be an int"));
    let description = splitted_input[3].to_string();
    
    let product = Product{
        id,
        name,
        price: price?,
        quantity: quantity?,
        description,
        
    };

    Ok(product)
}

pub fn remove_product(user_input: String) {
    let mut products: Vec<Product> = match read_product_json() {
        Ok(items) => items,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    let id = match user_input.parse::<u32>() {
        Ok(num) => num,
        Err(error) => {
            println!("You have to enter a correct ID as an integer, error: {}", error);
            return;
        }
    };

    let chosen_product = products.iter().find(|p| p.id == id).cloned();
    
    if chosen_product.is_none() {
        println!("We did not find any products with this id, please try again");
        return;
    }

    let chosen_product = chosen_product.unwrap();
    
    print!(
        "Are you sure you want to delete {} with id {} (y/n (Default is n)) ",
        chosen_product.name, chosen_product.id
    );
    stdout().flush().unwrap();

    if user_agreement() {
        if let Some(index) = products.iter().position(|x| x.id == chosen_product.id) {
            products.remove(index);
            let _ = write_product_json(products);
            println!("\nProduct successfully removed.");
        }
    } else {
        println!("Returning back...");
        return;
    }
}

fn user_agreement() -> bool {
    loop {
        if event::poll(time::Duration::from_secs(100)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                if key_event.kind == KeyEventKind::Press {
                    match key_event.code {
                        KeyCode::Char('y') | KeyCode::Char('Y') => return true,
                        KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Enter => return false,
                        _ => {
                            println!("\nPlease press \"y\" or \"n\".");
                            continue;
                        }
                    }
                }
            }
        } else {
            println!("\nTimed out. Defaulting to \"no\".");
            return false;
        }
    }
}

pub fn add_product(product: Product) {
    if let Err(error) = open_json_file() {
        println!("{}", error);
        return;
    }

    let value = json!(product);
    let json_input: Product = from_value(value).expect("Was not able to create a json_input");

    let mut products: Vec<Product> = match read_product_json() {
        Ok(items) => items,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    products.push(json_input);

    let _ = write_product_json(products);

    println!("Product successfully added.");
}

fn get_highest_id() -> Result<u32, Error> {

    match open_json_file() {
        Ok(_) => (),
        Err(error) =>  { 
            print!("{}", error); 
            return Err(error);
        }
    }

    let mut file = File::open(PRODUCTS_LIST)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let products: Vec<Product> = serde_json::from_str(&contents).unwrap_or_else(|_| vec![]);

    let highest_id = products.iter().map(|p| p.id).max().unwrap_or(0);    

    Ok(highest_id + 1)
}

fn open_json_file() -> Result<(), Error> {
    OpenOptions::new()
        .write(true)
        .create(true)
        .open(PRODUCTS_LIST)?;

    Ok(())
}

fn read_product_json() -> Result<Vec<Product>, Error> {
    open_json_file()?;

    let file = File::open(PRODUCTS_LIST);
    let mut contents = String::new();
    let _ = file.expect("Was not able to read a file").read_to_string(&mut contents);

    let products:Vec<Product> = serde_json::from_str(&contents).unwrap_or_else(|_| vec![]);

    Ok(products)

}

fn write_product_json(products: Vec<Product>) -> Result<(), Error> {
    let serialized_products = match serde_json::to_string_pretty(&products) {
        Ok(json) => json,
        Err(e) => {
            println!("Failed to serialize products: {}", e);
            return Ok(());
        }
    };

    if let Err(e) = fs::write(PRODUCTS_LIST, serialized_products) {
        println!("Failed to write to file: {}", e);
    };

    Ok(())
}