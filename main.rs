use std::fs;
use std::path::Path;

fn main() {
        //TODO: Implement dotenv
        for entry in fs::read_dir("/Users/carlos97gr/Downloads").unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
           
            if path.is_file() {
                if Path::new(&path).extension() != None && Path::new(&path).file_name() != None {
                    
                    let path_string = Path::new(&path).to_str().unwrap();
                    let file_name = Path::new(&path).file_name().unwrap().to_str().unwrap();
                    let file_type= Path::new(&path).extension().unwrap();
                  
                    if file_type == "jpeg" ||file_type == "png" || file_type == "jpg" {
                        let path_to = "/Users/carlos97gr/Downloads/images/";
                        let destination_file = format!("{}{}",path_to,file_name);
                        fs::copy(path_string,destination_file).expect("copy error");
                        
                    }

                    if file_type == "dmg" || file_type == "app" {
                        let path_to = "/Users/carlos97gr/Downloads/installers/";
                        let destination_file = format!("{}{}",path_to,file_name);
                        fs::copy(path_string,destination_file).expect("copy error");
                    }

                    if file_type == "pdf" {
                        let path_to = "/Users/carlos97gr/Downloads/documents/";
                        let destination_file = format!("{}{}",path_to,file_name);
                        fs::copy(path_string,destination_file).expect("copy error");
                    }

                    fs::remove_file(path_string).expect("remove error");
                }
            }
        }
    
}