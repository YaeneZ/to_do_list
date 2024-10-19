use std::fs::{remove_file, File};
use std::io::{Read, Write};
use crate::{SaveFiles, ToDoList};

pub fn save_file(to_do_list: &ToDoList) {
    let json_string = serde_json::to_string_pretty(to_do_list).expect("Failed to Serialize to Json!");
    let file_path: String = format!("saves/{}.json", to_do_list.name);
    let mut file = File::create(file_path).expect("Error, faled to create new File");
    file.write_all(json_string.as_bytes()).expect("Failed to write to the File");

    check_for_file(&to_do_list.name);
}

pub fn load_file(file_name: &str) -> ToDoList {
    let file_path: String = format!("saves/{}.json", file_name);
    let mut file: File = File::open(file_path)
        .expect("Error, the File does not exist");

    let mut contents: String = String::new();
    file.read_to_string(&mut contents)
        .expect("Error, could not read the File to String");
    
    let to_do_list: ToDoList = serde_json::from_str(&contents)
        .expect("Error, could not deserialize the File into ToDoList");

    return to_do_list;
}

pub fn check_for_file_exist(file_name: &str) -> bool {
    let file_path: String = format!("saves/def/SavesList.json");
    let mut file: File = File::open(&file_path)
        .expect("Error, the File does not exist");

    let mut contents: String = String::new();
    file.read_to_string(&mut contents)
        .expect("Error, could not read the File to String");
    let save_files: SaveFiles = serde_json::from_str(&contents)
        .expect("Error, could not deserialize the File into ToDoList");

    let mut b: bool = false;
    if save_files.files.contains(&file_name.to_string()) {
        b = true;
    }

    return b;
}

pub fn remove_to_do_list(to_do_list: &ToDoList) {
    let path: String = format!("saves/{}.json", to_do_list.name);
    remove_file(path).expect("Error, could not remove File");

    let file_path: String = format!("saves/def/SavesList.json");
    let mut file: File = File::open(&file_path)
        .expect("Error, the File does not exist");

    let mut contents: String = String::new();
    file.read_to_string(&mut contents)
        .expect("Error, could not read the File to String");
    let mut save_files: SaveFiles = serde_json::from_str(&contents)
        .expect("Error, could not deserialize the File into ToDoList");

    save_files.files.retain(|file| *file != to_do_list.name);

    let json_string: String = serde_json::to_string_pretty(&save_files)
        .expect("Error, could not serialize Value");
    let mut file: File = File::create(&file_path)
        .expect("Error, could not create new File");
    file.write_all(json_string.as_bytes())
        .expect("Error, could not write Value to File");
}

fn check_for_file(file_name: &str) {
    let file_path: String = format!("saves/def/SavesList.json");
    let mut file: File = File::open(&file_path)
        .expect("Error, the File does not exist");

    let mut contents: String = String::new();
    file.read_to_string(&mut contents)
        .expect("Error, could not read the File to String");
    let mut save_files: SaveFiles = serde_json::from_str(&contents)
        .expect("Error, could not deserialize the File into ToDoList");

    if !save_files.files.contains(&file_name.to_string()) {
        save_files.files.push(file_name.to_string());

        let json_string: String = serde_json::to_string_pretty(&save_files)
            .expect("Error, could not serialize Value");
        let mut file: File = File::create(&file_path)
            .expect("Error, could not create new File");
        file.write_all(json_string.as_bytes())
            .expect("Error, could not write Value to File");
    }
}

pub fn get_all_files() -> String {
    //let mut save_files: SaveFiles = SaveFiles { files: vec![String::from(String::new())] };

    let file_path: String = format!("saves/def/SavesList.json");
    let mut file: File = File::open(&file_path)
        .expect("Error, the File does not exist");

    let mut contents: String = String::new();
    file.read_to_string(&mut contents)
        .expect("Error, could not read the File to String");
    let save_files: SaveFiles = serde_json::from_str(&contents)
        .expect("Error, could not deserialize the File into ToDoList");

    let mut result:String = String::new();
    for i in save_files.files.iter() {
        let t: ToDoList = load_file(&i);
        let s: String = format!("{}{}{}{}\x1b[0m, ", t.format_setting.attributes, t.format_setting.color, t.format_setting.background, t.name);
        result.push_str(&s);
    }

    return result;
}