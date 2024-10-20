use crate::{text_format_manager::*, load_file, ToDoList};
use std::io::{stdin, stdout, Write};

pub fn edit_to_do(file_name: &String) {
    let mut to_do_list: ToDoList = load_file(file_name);

    let mut l:bool = true;
    while l {
        let mut input: String = String::new();
        print!("{} ", input_line_advanced(&to_do_list.format_setting, &to_do_list.name, "edit"));
        stdout().flush().expect("Error, could not print");
        stdin().read_line(& mut input).expect("An Error occured while reding the Userinput");

        l = false;
    }
}