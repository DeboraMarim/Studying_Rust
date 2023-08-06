use std::io;

struct TodoItem {
    title: String,
    completed: bool,
}

impl TodoItem {
    fn new(title: String) -> TodoItem {
        TodoItem {
            title,
            completed: false,
        }
    }
}

struct TodoList {
    items: Vec<TodoItem>,
}

impl TodoList {
    fn new() -> TodoList {
        TodoList { items: Vec::new() }
    }

    fn add_item(&mut self, title: String) {
        let new_item = TodoItem::new(title);
        self.items.push(new_item);
    }

    fn complete_item(&mut self, index: usize) {
        if let Some(item) = self.items.get_mut(index) {
            item.completed = true;
        }
    }

    fn remove_item(&mut self, index: usize) {
        if index < self.items.len() {
            self.items.remove(index);
        }
    }

    fn print_list(&self) {
        for (index, item) in self.items.iter().enumerate() {
            let status = if item.completed { "✔" } else { " " };
            println!("{} [{}] {}", index + 1, status, item.title);
        }
    }
}

fn main() {
    let mut todo_list = TodoList::new();

    loop {
        println!("--- To-Do List ---");
        todo_list.print_list();
        println!("------------------");
        println!("1. Add Item");
        println!("2. Complete Item");
        println!("3. Remove Item");
        println!("4. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)
            .expect("Failed to read line");

        match choice.trim().parse() {
            Ok(1) => {
                println!("Enter the task description:");
                let mut title = String::new();
                io::stdin().read_line(&mut title)
                    .expect("Failed to read line");
                todo_list.add_item(title.trim().to_string());
            }
            Ok(2) => {
                println!("Enter the index of the task to mark as completed:");
                let mut index_str = String::new();
                io::stdin().read_line(&mut index_str)
                    .expect("Failed to read line");
                let index: usize = index_str.trim().parse()
                    .expect("Please enter a valid number");
                todo_list.complete_item(index - 1);
            }
            Ok(3) => {
                println!("Enter the index of the task to remove:");
                let mut index_str = String::new();
                io::stdin().read_line(&mut index_str)
                    .expect("Failed to read line");
                let index: usize = index_str.trim().parse()
                    .expect("Please enter a valid number");
                todo_list.remove_item(index - 1);
            }
            Ok(4) => break,
            _ => println!("Invalid choice!"),
        }
    }

    println!("Goodbye!");
}
