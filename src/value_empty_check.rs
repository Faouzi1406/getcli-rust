pub mod check_if_value_empty{
    pub fn check_if_value_empty(values:[&str; 2]) -> String {
        let mut empty_value_return:String = "None".to_owned();

        for value in values  {
            if value.is_empty() {
                let empty_value = format!("Empty value: {}", value);
                empty_value_return = empty_value;
            }
        }

       empty_value_return 
    }
}


