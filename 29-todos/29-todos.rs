use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;

struct Todos {
    path: PathBuf,
    items: Vec<String>,
}

impl Todos {
    fn read(filename: impl AsRef<Path>) -> io::Result<Self> {
        let file = File::open(&filename)?;
        let file = BufReader::new(file);
        let mut todos: Vec<String> = Vec::new();

        for line in file.lines() {
            let line = line?;
            if line.is_empty() {
                continue;
            }
            todos.push(line);
        }
        let mut path = PathBuf::new();
        path.push(filename);

        Ok(Self {
            path: path,
            items: todos,
        })
    }

    fn print(&self) {
        for (index, todo) in self.items.iter().enumerate() {
            println!("{}: {}", index, todo);
        }
    }

    fn add(&mut self, text: String) {
        self.items.push(text);
    }

    fn remove(&mut self, index: usize) -> bool {
        if index < self.items.len() {
            let _ = &self.items.remove(index);
            true
        } else {
            false
        }
    }

    fn save(&self) {
        fs::write(&self.path, &self.items.join("\n")).unwrap();
    }
}

enum Command {
    Add,
    Remove,
    Save,
    Quit,
}

impl FromStr for Command {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "add" => Ok(Self::Add),
            "remove" => Ok(Self::Remove),
            "save" => Ok(Self::Save),
            "quit" => Ok(Self::Quit),
            _ => Err(()),
        }
    }
}

fn gets() -> Option<String> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).ok()?;
    Some(buf.trim().to_string())
}

fn get_command() -> Option<Command> {
    println!("请输入指令 1. add 2. remove 3. save，然后按 Enter: ");
    gets().map_or(None, |command| Command::from_str(&command).ok())
}

fn add_todo(todos: &mut Todos) {
    println!("请输入代办事项: ");
    if let Some(text) = gets() {
        todos.add(text);
    }
}

fn remove_todo(todos: &mut Todos) {
    println!("请输入要删除的编号: ");
    if let Some(text) = gets() {
        if let Ok(index) = text.parse::<usize>() {
            if !&todos.remove(index) {
                println!("無法刪除編號{}的項目！", index);
            }
        }
    }
}

fn process_command(todos: &mut Todos, command: Command) {
    match command {
        Command::Add => {
            add_todo(todos);
        }
        Command::Save => {
            todos.save();
        }
        Command::Remove => {
            remove_todo(todos);
        }
        Command::Quit => {
            std::process::exit(0);
        }
    }
}

fn main() -> io::Result<()> {
    let path = env::current_dir()?.parent().unwrap().join("todos.txt");
    let mut todos = Todos::read(&path)?;

    loop {
        todos.print();
        if let Some(command) = get_command() {
            process_command(&mut todos, command);
        }
    }

    Ok(())
}

// while (true)
//   print "请输入指令 1. add 2. remove 3. save，然后按 Enter: "
//   command = gets.chomp

//   if command == "add"
//     print "请输入代办事项: "
//     # ...
//   elsif command == "remove"
//     print "请输入要删除的编号: "
//     # ...
//   elsif command == "save"
//     puts "存盘离开"

//     # ...
//     break;
//   else
//     puts "看不懂，请再输入一次"
//   end
// end
