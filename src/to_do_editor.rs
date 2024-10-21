use crate::{edit_list_commands, load_file, remove_to_do_list, save_file, text_format_manager::*, ToDoList};
use std::io::{stdin, stdout, Write};

pub fn edit_to_do(file_name: &String) -> String {
    let mut to_do_list: ToDoList = load_file(file_name);
    let mut l:bool = true;

    while l {
        let mut input: String = String::new();
        print!("{}", input_line_advanced(&to_do_list.format_setting, &to_do_list.name, "edit"));
        stdout().flush().expect("Error, could not print");
        stdin().read_line(& mut input).expect("An Error occured while reading the Userinput");

        to_do_list = edit_list_commands(&input.trim(), to_do_list);
        if input.trim() == "e" || input.trim() == "exit" {
            l = false;
        }
    }

    return to_do_list.name;
}

pub fn edit_list_name(mut to_do_list: ToDoList) -> ToDoList {
    let mut input: String = String::new();
    print!("{}", input_line_advanced(&to_do_list.format_setting, &to_do_list.name, "edit/new name"));
    stdout().flush().expect("Error, could not print");
    stdin().read_line(& mut input).expect("An Error occured while reading the Userinput");

    remove_to_do_list(&to_do_list);
    to_do_list.name = input.trim().to_string();
    save_file(&to_do_list);

    return to_do_list;
}