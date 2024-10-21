mod unix_time_stamp_convertor;
mod serializable_objects;
mod persistance_manager;
mod command_definer;
mod text_format_manager;
mod to_do_editor;

use serializable_objects::*;
use persistance_manager::*;
use command_definer::*;
use std::io::{stdin, stdout, Write};
use text_format_manager::*;

fn main() {
    let mut app_data: AppData = AppData::new(true);

    while app_data.active {
        let mut prompt: String = String::new();
        print!("{}", input_line(""));
        //print!("{}:{}", "User".green(), "~ ".blue());
        stdout().flush().expect("Error, could not print");
        stdin().read_line(& mut prompt).expect("An Error occured while reding the Userinput");
        app_data = commands(prompt.trim(), app_data);
    }

    //let (year, month, day, hour, minute, second) = unix_time_stamp_convertor::unix_convert(1728316507);
    //println!("\nDate: {:04}-{:02}-{:02} {:02}:{:02}:{:02}", year, month, day, hour, minute, second);

    //let mut to_do_list: ToDoList = ToDoList::new("To Do List".to_string(), FormatSetting::default(), Vec::new());
    //let to_do: ToDo = ToDo::new(0, 0, "To Do".to_string(), 0);
    //to_do_list.list.push(to_do);
    //println!("{}", to_do_list.list[0].value);

    //let to_do_list_new = load_file("Todolist");
    //println!("{}", to_do_list_new.list[0].value);

    //let _save: std::io::Result<()> = save_file(&to_do_list, "Todolist");
}

pub struct AppData {
    pub active: bool
}

impl AppData {
    pub fn new (active: bool) -> AppData {
        return  AppData {
            active
        };
    }
}
