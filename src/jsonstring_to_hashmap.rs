pub mod json_hasmap {
    use std::collections::HashMap;
    use substring::Substring;


    pub struct JsonString{
        pub json_string:String, 
    }

    impl JsonString{


        pub fn parse(&mut self) -> HashMap<&str, &str>{
        
            
            let  mut map:HashMap<&str, &str> =  HashMap::new();
            let  mut current_value = 0;
            let  mut value_a:&str = "error";
            let  mut value_b:&str;
            
            let mut value_start = 1;
            
            let mut value_current:&str = "a";
            let mut position = 1;

            for c in self.json_string.chars(){

                if c.to_string() == "\""{
                
               if current_value == 0{
                    value_start= position;
                }


                current_value += 1;

                if current_value == 2 {

                    current_value = 0;
                if value_current == "a"{
                    value_a  = self.json_string.substring(value_start, position -1);
                }

                if value_current == "b"{ 
                    value_b = self.json_string.substring(value_start, position -1);
                    map.insert(value_a, value_b);
                    value_current = "a";
                    current_value = 0;
                }
                }
                }
                 
                if c.to_string() == ":" {
                    value_current = "b";
                }
                position = position +1;
            }

            return map;
        }
    }
} 
