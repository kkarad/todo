use std::env;

#[derive(Debug)]
struct Todo {
    title: String,
    completed: bool
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let command: String = args[1].clone();
    if command == "add" {
        let new_todo_title = args[2].clone();
        let new_todo = Todo{title: new_todo_title, completed: false};
        println!("{:?}", new_todo)
    }
}
