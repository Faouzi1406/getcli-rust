pub mod json_hasmap {
    use std::collections::HashMap;

    pub struct JsonString{
            pub json_string:String, 
    } 

    
    pub struct Json {
        key:String,
        value: String 
    }



    impl JsonString{
        
        pub fn parse_line(line:&str) -> Json{
            let (key, value) = line.split_once(":").expect("Between a new value there must be a :");
                
            if key.contains("{") || key.contains("}") {
                panic!("Json shouldn't contain brackets");
            }


            Json { key: key.replace("\"", "").replace(" ", "").to_owned(), value: value.replace("\"", "").to_owned()}
        }




        pub fn parse(&mut self) -> HashMap<String, String>{
            let mut  hashed_map:HashMap<String, String>= HashMap::new();
            let string_to_lines = self.json_string.replace(",", "\n");
            
            for line  in string_to_lines.lines(){
                   let value  = JsonString::parse_line(line);

                   hashed_map.insert(value.key, value.value);
            };
            hashed_map
        }
    }
} 
