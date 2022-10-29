use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    url: String,
}

fn make_request(url:&str)->String {
    let body  = reqwest::blocking::get(url) 
        .unwrap()
        .text()
        .unwrap();

    format!("Body: {}", body)
}

fn main() {
       let args = Cli::parse();

       let body = make_request(&args.url);

    
       println!("Body: {}" ,body)
}       
