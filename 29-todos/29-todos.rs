use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;

type Todos = Vec<String>;

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

fn read_todos(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    let file = File::open(filename)?;
    let file = BufReader::new(file);
    let mut todos: Todos = Vec::new();

    for line in file.lines() {
        let line = line?;
        if line.is_empty() {
            continue;
        }
        todos.push(line);
    }

    Ok(todos)
}

fn print_todos(todos: &Todos) {
    for (index, todo) in todos.iter().enumerate() {
        println!("{}: {}", index, todo);
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
        todos.push(text);
    }
}

fn process_command(todos: &mut Todos, command: Command) {
    match command {
        Command::Add => {
            add_todo(todos);
        }
        Command::Quit => {
            std::process::exit(0);
        }
        _ => (),
    }
}

fn main() -> io::Result<()> {
    let path = env::current_dir()?.parent().unwrap().join("todos.txt");
    let mut todos = read_todos(&path)?;
    print_todos(&todos);

    loop {
        if let Some(command) = get_command() {
            process_command(&mut todos, command);
        }
    }

    Ok(())
}

// 简易 Todo 代办事项应用

// text = File.read("todos.txt")

// todos = []
// text.each_line do |line|
//   todos << line.chomp
// end

// todos.each_with_index do |todo, index|
//   puts "#{index}: #{todo}"
// end

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
