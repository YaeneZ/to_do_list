use crate::{check_for_file_exist, get_all_files, load_file, remove_to_do_list, save_file, text_format_manager::*, to_do_editor::{edit_list_name, edit_to_do}, AppData, FormatSetting, ToDo, ToDoList};
use std::{io::{stdin, stdout, Write}, ptr::null};

pub fn commands (prompt: &str, mut app_data: AppData) -> AppData {
    match prompt {
        "help" | "h" => println!("{}", help()),
        "ll" => println!("{}", list_lists()),
        "new" => println!("{}", create_list()),
        "rm" => println!("{}", remove_list()),
        "edit" => println!("{}", edit_list()),
        "view" => println!("{}", view_list()),

        "exit" | "e" => app_data.active = false,
        _ => println!("Unknown Command, type \"help\" or \"h\" to find help")
    }

    return app_data;
}

fn help() -> String {
    return format!("
----------------------------------------------------------------
This is a \x1b[92mlist\x1b[0m of all the prompts for this Program\n
    help    \x1b[92m->\x1b[0m Lists this menu
    ll      \x1b[92m->\x1b[0m Lists Lists all the lists
    new     \x1b[92m->\x1b[0m Create new empty List
    rm      \x1b[92m->\x1b[0m Remove List
    edit    \x1b[92m->\x1b[0m Enter edit Mode
    view    \x1b[92m->\x1b[0m Enter view Mode

    exit    \x1b[92m->\x1b[0m Exit this Program

----------------------------------------------------------------
        ").to_string();
}

fn list_lists() -> String {
    return get_all_files();
}

fn create_list() -> String {
    //let mut name: String = String::new();
    let mut format_setting: FormatSetting = FormatSetting::new(String::new(), String::new(), String::new());
    let mut list: Vec<ToDo> = Vec::new();

    let mut input: String = String::new();
    print!("{}", input_line("new/name"));
    stdout().flush().expect("Error, could not print");
    stdin().read_line(& mut input).expect("An Error occured while reding the Userinput");
    let name: String = input.trim().to_string();

    let text_formats: TextFormats = TextFormats::default();
    let mut l: bool = true;
    let mut i: usize = 0;
    while l {
        let mut input: String = String::new();
        print!("{}{}{} \x1b[0m", 
        input_line("new/attribute"), 
        text_formats.attributes[i % text_formats.attributes.len()], 
        name.trim());

        stdout().flush().expect("Error, could not print");
        stdin().read_line(& mut input).expect("An Error occured while reding the Userinput");
    
        if input.trim() == "y".to_string() || input.trim() == "yes".to_string() {
            format_setting.attributes = text_formats.attributes[i % text_formats.attributes.len()].to_string();
            l = false;
        }

        i += 1;
    }

    l = true;
    i = 0;
    while l {
        let mut input: String = String::new();
        print!("{}{}{}{} \x1b[0m", input_line("new/attribute"), 
        format_setting.attributes, 
        text_formats.colors[i % text_formats.colors.len()], 
        name.trim());

        stdout().flush().expect("Error, could not print");
        stdin().read_line(& mut input).expect("An Error occured while reding the Userinput");
    
        if input.trim() == "y".to_string() || input.trim() == "yes".to_string() {
            format_setting.color = text_formats.colors[i % text_formats.colors.len()].to_string();
            l = false;
        }

        i += 1;
    }

    l = true;
    i = 0;
    while l {
        let mut input: String = String::new();
        print!("{}{}{}{}{} \x1b[0m", input_line("new/attribute"), 
        format_setting.attributes, 
        format_setting.color,
        text_formats.backgrounds[i % text_formats.backgrounds.len()], 
        name.trim());

        stdout().flush().expect("Error, could not print");
        stdin().read_line(& mut input).expect("An Error occured while reding the Userinput");
    
        if input.trim() == "y".to_string() || input.trim() == "yes".to_string() {
            format_setting.background = text_formats.backgrounds[i % text_formats.backgrounds.len()].to_string();
            l = false;
        }

        i += 1;
    }

    list.push(ToDo::new(0, 0, "Hello".to_string(), 0));

    let to_do_list: ToDoList = ToDoList::new(name, format_setting, list);
    save_file(&to_do_list);
    print!("Successfuly created: {}{}\x1b[0m", text_formater(&to_do_list.format_setting), to_do_list.name);

    return String::new();
}

fn remove_list() -> String {
    let mut input: String = String::new();
    print!("{}", input_line("rm/name"));
    stdout().flush().expect("Error, could not print");
    stdin().read_line(& mut input).expect("An Error occured while reding the Userinput");
    
    if check_for_file_exist(&input.trim()) {
        let to_do_list: ToDoList = load_file(&input.trim());
        let mut confirmation: String = String::new();
        print!("{} Do you realy wnat to delete {}{}\x1b[0m? ", input_line("rm"), text_formater(&to_do_list.format_setting), to_do_list.name);
        stdout().flush().expect("Error, could not print");
        stdin().read_line(& mut confirmation).expect("An Error occured while reding the Userinput");

        if confirmation.trim() == "y" || confirmation.trim() == "yes" {
            println!("removing...");
            remove_to_do_list(&to_do_list);
        }
    }
    else {
        println!("{}\x1b[0m does not exist.", input.trim());
    }

    return String::new();
}

fn edit_list() -> String {
    let mut input: String = String::new();
    print!("{}", input_line("edit/pick name"));
    stdout().flush().expect("Error, could not print");
    stdin().read_line(& mut input).expect("An Error occured while reding the Userinput");

    if check_for_file_exist(&input.trim()) {
        input = edit_to_do(&input.trim().to_string());
    }

    let to_do_list: ToDoList = load_file(&input.trim());
    return format!("edited {}{}\x1b[0m", text_formater(&to_do_list.format_setting), to_do_list.name);
}

fn view_list() -> String {
    return  String::new();
}

pub fn edit_list_commands (prompt: &str, mut to_do_list: ToDoList) -> ToDoList {
    match prompt {
        "help" | "h" => println!("{}", e_help()),
        "name" => to_do_list = edit_list_name(to_do_list),

        "save" => save_file(&to_do_list),

        "exit" | "e" => save_file(&to_do_list),
        _ => println!("Unknown Command, type \"help\" or \"h\" to find help")
    }

    return to_do_list;
}

fn e_help() -> String {
    return format!("
----------------------------------------------------------------
This is a \x1b[92mlist\x1b[0m of all the prompts for this Program\n
    help    \x1b[92m->\x1b[0m Lists this menu
    name    \x1b[92m->\x1b[0m Change the name of the List
    in pr   \x1b[92m->\x1b[0m 
    in pr   \x1b[92m->\x1b[0m 
    in pr   \x1b[92m->\x1b[0m 
    save    \x1b[92m->\x1b[0m 

    exit    \x1b[92m->\x1b[0m Exit this Program

----------------------------------------------------------------
        ").to_string();
}