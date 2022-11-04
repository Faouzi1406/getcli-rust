pub mod save_to_file {
    use std::fs::File;
    use std::io::prelude::*;

    pub fn save_response(file_data:&str) -> std::io::Result<()>{

        let mut created_file = File::create("saved_response.json").expect("couldn't save the response.");
        created_file.write(file_data.as_bytes()).expect("Error saving response to file");

        Ok(())
    }
}
