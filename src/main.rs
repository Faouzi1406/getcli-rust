use clap::Parser;
mod value_empty_check;

#[derive(Parser)]
struct Cli {
    typeofrequest: String,
    url: String,
}

fn make_request(type_reques:&str, url:&str)->String {
 
    let mut body:String =  "string".to_owned();

    if  type_reques ==  "get" {
    body  = reqwest::blocking::get(url) 
        .unwrap()
        .text() 
        .unwrap();
    
    let check_value_empty = value_empty_check::check_if_value_empty::check_if_value_empty([type_reques,url]);
    
    if check_value_empty != "None" {
        body = "check_value_empty".to_owned();
    }

    if body.is_empty() {
        body = "Request response body was empty".to_owned();
    }
    


    }
    format!("Body: {}", body)
}


fn main() {
       let args = Cli::parse();

       let body = make_request(&args.typeofrequest,&args.url);

       println!("Body: {}" ,body)
}       
