use std::path::PathBuf;


pub fn save_file_location() -> PathBuf {
    dirs::data_dir().unwrap().join("tanki").join("collection.json")
} 
