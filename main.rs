use std::fs;
use std::path::Path;

use std::env;
extern crate dotenv;



fn main() {
    
        dotenv::from_path("./.env").expect("error loading env");
        
        let path_to_images = env::var("PATH_IMAGES").expect("env error");
        let path_to_videos = env::var("PATH_VIDEOS").expect("env error");
        let path_to_audios = env::var("PATH_AUDIOS").expect("env error");
        let path_to_documents =env::var("PATH_DOCUMENTS").expect("env error");
        let path_to_installers = env::var("PATH_INSTALLERS").expect("env error");

        for entry in fs::read_dir(env::var("FOLDER_TO_ORGANIZE").expect("env error")).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
           
            if path.is_file() {
                if Path::new(&path).extension() != None && Path::new(&path).file_name() != None {
                    
                    let path_string = Path::new(&path).to_str().unwrap();
                    let file_name = Path::new(&path).file_name().unwrap().to_str().unwrap();
                    let file_type= Path::new(&path).extension().unwrap();

                    //IMAGES
                    if file_type == "jpeg" ||file_type == "png" || file_type == "jpg" || file_type == "heic" {
                        let destination_file = format!("{}/{}",path_to_images,file_name);
                        fs::copy(path_string,destination_file).expect("copy error");
                        
                    }
                    //INSTALLERS
                    if file_type == "dmg" || file_type == "app" {
                        let destination_file = format!("{}/{}",path_to_installers,file_name);
                        fs::copy(path_string,destination_file).expect("copy error");
                    }
                    //DOCUMENTS
                    if file_type == "pdf" || file_type == "doc" || file_type == "docx" || file_type == "ppt" || file_type == "xls"{
                        let destination_file = format!("{}/{}",path_to_documents,file_name);
                        fs::copy(path_string,destination_file).expect("copy error");
                    }
                    //VIDEOS
                    if file_type == "mp4" || file_type == "avi" {
                        let destination_file = format!("{}/{}",path_to_videos,file_name);
                        fs::copy(path_string,destination_file).expect("copy error");
                    }
                    //AUDIOS
                    if file_type == "mp3" {
                        let destination_file = format!("{}/{}",path_to_audios,file_name);
                        fs::copy(path_string,destination_file).expect("copy error");
                    }

                    fs::remove_file(path_string).expect("remove error");
                }
            }
        }
    
}