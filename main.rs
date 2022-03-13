use std::fs;
use std::path::Path;

use std::env;
extern crate dotenv;



fn main() {
    
        dotenv::from_path("./.env").expect("error loading env");
        
        let path_to_images = env::var("PATH_IMAGES").expect("env error");
        let path_to_installers = env::var("PATH_INSTALLERS").expect("env error");
        let path_to_documents =env::var("PATH_DOCUMENTS").expect("env error");

        for entry in fs::read_dir(env::var("FOLDER_TO_ORGANIZE").expect("env error")).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
           
            if path.is_file() {
                if Path::new(&path).extension() != None && Path::new(&path).file_name() != None {
                    
                    let path_string = Path::new(&path).to_str().unwrap();
                    let file_name = Path::new(&path).file_name().unwrap().to_str().unwrap();
                    let file_type= Path::new(&path).extension().unwrap();

                    //IMAGES
                    if file_type == "jpeg" ||file_type == "png" || file_type == "jpg" {
                        
                        let destination_file = format!("{}/{}",path_to_images,file_name);
                        fs::copy(path_string,destination_file).expect("copy error");
                        
                    }
                    //INSTALLERS
                    if file_type == "dmg" || file_type == "app" {
                        let destination_file = format!("{}/{}",path_to_installers,file_name);
                        fs::copy(path_string,destination_file).expect("copy error");
                    }

                    if file_type == "pdf" {
                        let destination_file = format!("{}/{}",path_to_documents,file_name);
                        fs::copy(path_string,destination_file).expect("copy error");
                    }

                    fs::remove_file(path_string).expect("remove error");
                }
            }
        }
    
}