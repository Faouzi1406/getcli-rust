use prettyprint::{PrettyPrint, PrettyPrinter};
use reqwest::blocking::Client;
use clap::Parser;
use save_response::save_to_file::save_response;
mod value_empty_check;
mod jsonstring_to_hashmap;
mod save_response;
#[derive(Parser)]
struct Cli {
    typeofrequest: String,
    url: String,
    body:Option<String>, 
    save_resp:Option<bool>
}


fn make_request(type_reques:&str, url:&str, body_request:Option<String>)->String {
    let mut body:String =  "string".to_owned();

    if  type_reques ==  "get" {
    body  = reqwest::blocking::get(url) 
        .unwrap()
        .text() 
        .unwrap();
    
 

    if body.is_empty() {
        body = "Request response body was empty".to_owned();
    }
    }
    
    
    if type_reques == "post" {
        
        if body_request == None {
            let client  = Client::new();
            let request = client.post(url).send();
           
            let _request = match request{
                Ok(content) => content,
                Err(error) => panic!("Could not make the post request to url: {}, Error: {}", url, error)  
            };

            let req_body= _request.text();

            let _responsebody = match req_body{
                Ok(content) =>  {body = content},
                Err(error) => panic!("There was and error {:?} ", error)
                
            };


        }


        //TODO: Hanlde request body if different from none. 
        //Currently only supports json
        //Check if body_request != none if so we can make a post request with the  body
        else {
            let body_request_clone = body_request.clone();
                    
                let mut json_string_test= jsonstring_to_hashmap::json_hasmap::JsonString{
            json_string: body_request_clone.unwrap()
        };
            
                let value  = json_string_test.parse();
            let client  = Client::new();
            let request = client.post(url)
                .json(&value)
                .send();
            
            let handle_error_request = match request {
                Ok(content) => content,
                Err(error) => panic!("There was and error with making the post request, error: {:?}", error)
                
            };
             
            let body_text  = handle_error_request.text();

            let _body_response_text = match body_text{
                Ok(content) => {body = content},
                Err(error) => panic!("There was and error with making the post request, error: {:?}", error)
            };
        }
    }
    
    if type_reques == "delete" && body_request == None{
        let client = Client::new();
        let request = client.delete(url).send();

        let request_error = match request {
            Ok(content) => content,
            Err(error) => panic!("There was and error making the DELETE request, ERROR: {}", error)
            
        };
        
        let get_body_text = request_error.text();
        
        match get_body_text {
            Ok(content) => {body = content},
            Err(error) => panic!("There was and error with the delete response body, error: {}", error)
        };

    }
        
    
    if type_reques == "delete" && body_request !=None{
        let clone_body_req = body_request.clone();
            let mut jsonstring = jsonstring_to_hashmap::json_hasmap::JsonString{
                json_string:  clone_body_req.unwrap()
            };

            let value = jsonstring.parse();

            let client = Client::new();

            let req = client.delete(url)
                .json(&value)
                .send();

            let handle_error_request = match req {
                Ok(content) => content,
                Err(error) =>  panic!("There was and error {}", error)
            };
            
            let body_content  =  handle_error_request.text();

            match body_content {
                Ok(content) => {body = content},
                Err(error) => panic!("There was and error with the delete repsonse body, error: {}", error)
            };

}


    format!("Body: {}", body)
}


fn main() {
       let args = Cli::parse();

       let body = make_request(&args.typeofrequest,&args.url, args.body);
        
       let save_body = save_response(&body).is_ok();

       if !save_body {
           println!("Couldn't save file");
       }
       let printer = PrettyPrinter::default()
           .language("json")
           .build() 
           .unwrap();

        printer.string(body).expect("Couldn't print the body");
       }       

