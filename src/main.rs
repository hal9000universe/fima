use chrono::NaiveDate;
use std::fs::File;
use std::io::{Read, Write};
use std::str::Split;

#[derive(Debug)]
enum ProductType {
    Food,
    Culture,
    Technology,
    Education,
    Travel,
    Presents,
    Style,
    Other,
}

impl ProductType {
    fn from_string(product_type: &str) -> ProductType {
        //! Convert a string to a ProductType
        match product_type {
            "food" => ProductType::Food,
            "culture" => ProductType::Culture,
            "technology" => ProductType::Technology,
            "education" => ProductType::Education,
            "travel" => ProductType::Travel,
            "presents" => ProductType::Presents,
            "style" => ProductType::Style,
            _ => ProductType::Other,
        }
    }

    fn to_string(&self) -> String {
        //! Convert a ProductType to a string
        match self {
            ProductType::Food => String::from("food"),
            ProductType::Culture => String::from("culture"),
            ProductType::Technology => String::from("technology"),
            ProductType::Education => String::from("education"),
            ProductType::Travel => String::from("travel"),
            ProductType::Presents => String::from("presents"),
            ProductType::Style => String::from("style"),
            ProductType::Other => String::from("other"),
        }
    }
}

#[derive(Debug)]
struct Product {
    name: String,
    price: f32,
    product_type: ProductType,
}

impl Product {
    fn new(name: String, price: f32, product_type: ProductType) -> Product {
        //! Create a new product
        //! by passing in a name, price and product_type
        // parse name (remove spaces, convert to lowercase)
        let name = name.trim().to_lowercase();
        Product {
            name,
            price,
            product_type,
        }
    }
}

#[derive(Debug)]
struct Purchase {
    product: Product,
    quantity: u32,
    date: NaiveDate,
}

impl Purchase {
    fn new(product: Product, quantity: u32, date: NaiveDate) -> Purchase {
        //! Create a new purchase
        //! by passing in a product, quantity and date
        Purchase {
            product,
            quantity,
            date,
        }
    }

    fn value(&self) -> f32 {
        //! Calculate the value of a purchase
        //! by multiplying the product price by the quantity
        self.product.price * self.quantity as f32
    }
}

fn write_to_file(purchase: &Purchase, file_name: &str) {
    //! Write purchase to file
    //! by appending to the file

    // open file if it exists, otherwise create it
    let mut file = File::open(file_name).unwrap_or_else(|_| File::create(file_name).unwrap());

    // write purchase to file if file is empty
    if file.metadata().unwrap().len() == 0 {
        let purchase_string: String = format!(
            "{}, {}, {}, {}, {}",
            purchase.product.name,
            purchase.product.price,
            purchase.product.product_type.to_string(),
            purchase.quantity,
            purchase.date
        );
        file.write_all(purchase_string.as_bytes()).unwrap();
    } else {
        // add purchase to file if file is not empty
        let mut contents: String = String::new();
        file.read_to_string(&mut contents).unwrap();
        let purchase_string = format!(
            "\n{}, {}, {}, {}, {}",
            purchase.product.name,
            purchase.product.price,
            purchase.product.product_type.to_string(),
            purchase.quantity,
            purchase.date
        );
        contents.push_str(&purchase_string);
        let mut file: File = File::create(file_name).unwrap();
        file.write_all(contents.as_bytes()).unwrap();
    }
}

fn cli_update() {
    //! Add a purchase from the command line
    //! by asking the user for the product name, price, quantity and date
    //! and writing the purchase to a file
    loop {
        println!("Add a purchase");
        // create product
        println!("Enter product name:");
        let mut name: String = String::new();
        std::io::stdin().read_line(&mut name).unwrap();
        name = name.trim().to_lowercase();
        // check price is a float
        let price = loop {
            println!("Enter price:");
            let mut price: String = String::new();
            std::io::stdin().read_line(&mut price).unwrap();
            match price.trim().parse::<f32>() {
                Ok(price) => break price,
                Err(_) => {
                    println!("Price must be a float");
                    continue;
                }
            }
        };
        // create product type
        println!("Enter product type:");
        let mut product_type: String = String::new();
        std::io::stdin().read_line(&mut product_type).unwrap();
        let product_type: ProductType = ProductType::from_string(&product_type.trim());
        // check quantity is an integer
        let quantity: u32 = loop {
            println!("Enter quantity:");
            let mut quantity: String = String::new();
            std::io::stdin().read_line(&mut quantity).unwrap();
            match quantity.trim().parse::<u32>() {
                Ok(quantity) => break quantity,
                Err(_) => {
                    println!("Quantity must be an integer");
                    continue;
                }
            }
        };
        // check date can be parsed
        let date: NaiveDate = loop {
            println!("Enter date (yyyy-mm-dd):");
            let mut date: String = String::new();
            std::io::stdin().read_line(&mut date).unwrap();
            match NaiveDate::parse_from_str(&date, "%Y-%m-%d") {
                Ok(date) => break date,
                Err(_) => {
                    println!("Date must be in the format yyyy-mm-dd");
                    continue;
                }
            }
        };
        let product = Product::new(name, price, product_type);
        let purchase = Purchase::new(product, quantity, date);
        write_to_file(&purchase, "purchase.txt");
        println!("Purchase added");
        println!("Add another purchase? (y/n)");
        let mut answer = String::new();
        std::io::stdin().read_line(&mut answer).unwrap();
        if answer.trim() != "y" {
            break;
        }
    }
}

fn read_from_file(file_name: &str) -> Vec<Purchase> {
    //! Read purchases from file
    //! by converting each line to a Purchase
    //! and returning a vector of Purchase
    let mut file: File = File::open(file_name).unwrap();
    let mut contents: String = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut purchases: Vec<Purchase> = Vec::new();
    for line in contents.lines() {
        let mut fields: Split<&str> = line.split(", ");
        // assert fields have correct types
        if let (Some(name), Some(price), Some(product_type), Some(quantity), Some(date)) = (
            fields.next(),
            fields.next(),
            fields.next(),
            fields.next(),
            fields.next(),
        ) {
            // create product and purchase
            let product = Product {
                name: String::from(name),
                price: price.parse::<f32>().expect("price is not a float"),
                product_type: ProductType::from_string(product_type),
            };
            let purchase = Purchase {
                product,
                quantity: quantity.parse::<u32>().expect("quantity is not an integer"),
                date: date.parse::<NaiveDate>().expect("date cannot be parsed"),
            };
            // add purchase to purchases
            purchases.push(purchase);
        }
    }
    return purchases;
}

struct Bucket<'a> {
    product_type: ProductType,
    purchases: Vec<&'a Purchase>,
}

impl<'a> Bucket<'a> {
    fn new(product_type: ProductType, purchases: Vec<&Purchase>) -> Bucket {
        //! Create a new bucket
        //! by passing in a product_type and purchases
        Bucket {
            product_type,
            purchases,
        }
    }
}

fn sort_type_buckets(purchases: &Vec<Purchase>) -> Vec<Bucket> {
    //! Sort purchases into buckets by product type
    //! by iterating over purchases and adding each purchase to the correct bucket
    //! and returning a vector of buckets
    let mut food_bucket: Vec<&Purchase> = Vec::new();
    let mut culture_bucket: Vec<&Purchase> = Vec::new();
    let mut technology_bucket: Vec<&Purchase> = Vec::new();
    let mut education_bucket: Vec<&Purchase> = Vec::new();
    let mut travel_bucket: Vec<&Purchase> = Vec::new();
    let mut presents_bucket: Vec<&Purchase> = Vec::new();
    let mut style_bucket: Vec<&Purchase> = Vec::new();
    let mut other_bucket: Vec<&Purchase> = Vec::new();
    for purchase in purchases {
        match purchase.product.product_type {
            ProductType::Food => food_bucket.push(purchase),
            ProductType::Culture => culture_bucket.push(purchase),
            ProductType::Technology => technology_bucket.push(purchase),
            ProductType::Education => education_bucket.push(purchase),
            ProductType::Travel => travel_bucket.push(purchase),
            ProductType::Presents => presents_bucket.push(purchase),
            ProductType::Style => style_bucket.push(purchase),
            ProductType::Other => other_bucket.push(purchase),
        }
    }
    return vec![
        Bucket::new(ProductType::Food, food_bucket),
        Bucket::new(ProductType::Culture, culture_bucket),
        Bucket::new(ProductType::Technology, technology_bucket),
        Bucket::new(ProductType::Education, education_bucket),
        Bucket::new(ProductType::Travel, travel_bucket),
        Bucket::new(ProductType::Presents, presents_bucket),
        Bucket::new(ProductType::Style, style_bucket),
        Bucket::new(ProductType::Other, other_bucket),
    ];
}

fn eval_bucket_value(bucket: &Bucket) -> f32 {
    //! Calculate the value of a bucket
    //! by iterating over purchases in the bucket
    //! and adding the value of each purchase
    return bucket
        .purchases
        .iter()
        .fold(0.0, |acc, purchase| acc + purchase.value());
}

fn compare_buckets(buckets: Vec<Bucket>) {
    //! Compare buckets
    //! by iterating over buckets and printing the value of each bucket
    // sort buckets by value
    let mut buckets = buckets;
    buckets.sort_by(|a, b| {
        eval_bucket_value(b)
            .partial_cmp(&eval_bucket_value(a))
            .unwrap()
    });
    // print buckets
    for bucket in buckets {
        println!(
            "{}: {}",
            bucket.product_type.to_string(),
            eval_bucket_value(&bucket)
        );
    }
}

fn exec_bucket_comparison() {
    let purchases: Vec<Purchase> = read_from_file("purchase.txt");
    let buckets: Vec<Bucket> = sort_type_buckets(&purchases);
    compare_buckets(buckets);
}

fn main() {
    cli_update();
}
