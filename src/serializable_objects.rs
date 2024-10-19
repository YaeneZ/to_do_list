use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ToDoList {
    pub name: String,
    pub format_setting: FormatSetting,
    pub list: Vec<ToDo>
}

impl ToDoList {
    pub fn new(name: String, format_setting: FormatSetting, list: Vec<ToDo>) -> ToDoList {
        return ToDoList {
            name,
            format_setting,
            list
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ToDo {
    pub status: i32,
    //pub format
    pub priority: i32,
    pub value: String,
    pub due: i64
}

impl ToDo {
    pub fn new(status: i32, priority: i32, value: String, due: i64) -> ToDo {
        return ToDo {
            status,
            priority,
            value,
            due,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct FormatSetting {
    pub attributes: String,
    pub color: String,
    pub background: String
}

impl FormatSetting {
    pub fn new(attributes: String, color: String, background: String) -> FormatSetting {
        return FormatSetting {
            attributes,
            color,
            background
        };
    }

    pub fn default() -> FormatSetting {
        let attributes: String = "\x1b[0m".to_string();
        let color: String = "\x1b[39m".to_string();
        let background: String = "\x1b[49m".to_string();

        return FormatSetting {
            attributes,
            color,
            background
        };
    }
}

#[derive(Serialize, Deserialize)]
pub struct SaveFiles {
    pub files: Vec<String>
}