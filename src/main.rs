use reqwest::blocking::Client;
use clap::Parser;
mod value_empty_check;

#[derive(Parser)]
struct Cli {
    typeofrequest: String,
    url: String,
    body:Option<String>,
}


fn make_request(type_reques:&str, url:&str, body_request:Option<String>, )->String {
 
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

    //TODO: Handle post request
    if type_reques == "post" {
        
        if body_request == None {
            let client  = Client::new();
            let request = client.post(url).send();
            
            let _request = match request{
                Ok(content) => content,
                Err(error) => panic!("Could not make the post request to url: {}, Error: {}", url, error)  
            };
            body = "succes".to_owned();
        }


        //Hanlde request body if different from none.
        if body_request != None {
            body = "testing".to_owned()       
            
        }
    }

    format!("Body: {}", body)
}


fn main() {
       let args = Cli::parse();

       let body = make_request(&args.typeofrequest,&args.url, args.body);

       println!("Body: {:?}" ,body)
}       
