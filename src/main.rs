use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
enum Status {
    Done,
    InProgress,
    ToDo,
}
#[derive(Clone, Serialize, Deserialize, Debug)]
struct Task {
    id: i32,
    message: String,
    status: Status,
}
impl Task {
    fn update(&mut self, message: String) {
        self.message = message;
    }
    fn change_status(&mut self, new_status: Status) {
        self.status = new_status;
    }
    fn display(&self) {
        println!(
            "Task id:{} | status: {:?} | {}",
            self.id, self.status, self.message
        );
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct TaskHolder {
    all_tasks: Vec<Task>,
}

impl TaskHolder {
    fn create(&mut self, msg: String) {
        let id = match self.all_tasks.last() {
            None => 1,
            Some(last) => last.id + 1,
        };
        self.all_tasks.push(Task {
            id,
            message: msg,
            status: Status::ToDo,
        });
    }

    fn remove(&mut self, id: i32) {
        match self.all_tasks.get(id as usize - 1) {
            None => {
                panic!("Did not found a Task with that index!")
            }
            Some(_) => self.all_tasks.remove(id as usize - 1),
        };
    }

    fn update(&mut self, id: i32, msg: String) {
        match self.all_tasks.get_mut(id as usize - 1) {
            None => {
                panic!("Did not found a Task with that index!")
            }
            Some(task) => task.update(msg),
        };
    }

    fn mark_done(&mut self, id: i32) {
        match self.all_tasks.get_mut(id as usize - 1) {
            None => {
                panic!("Did not found a Task with that index!")
            }
            Some(task) => task.change_status(Status::Done),
        };
    }

    fn mark_in_progress(&mut self, id: i32) {
        match self.all_tasks.get_mut(id as usize - 1) {
            None => {
                panic!("Did not found a Task with that index!")
            }
            Some(task) => task.change_status(Status::InProgress),
        };
    }
    fn list(&self, status: Option<Status>) {
        let cloned_tasks = self.all_tasks.clone();
        let filtered_task: Vec<Task> = match status {
            None => cloned_tasks.iter().filter(|_| true).cloned().collect(),
            Some(Status::Done) => cloned_tasks
                .iter()
                .filter(|el| el.status == Status::Done)
                .cloned()
                .collect(),
            Some(Status::ToDo) => cloned_tasks
                .iter()
                .filter(|el| el.status == Status::ToDo)
                .cloned()
                .collect(),
            Some(Status::InProgress) => cloned_tasks
                .iter()
                .filter(|el| el.status == Status::InProgress)
                .cloned()
                .collect(),
        };
        for task in filtered_task.iter() {
            println!("------------------------------------------------------------------------");
            task.display();
            println!("------------------------------------------------------------------------");
        }
    }
}

fn panic_error() {
    panic!(
        "\nBad action!\n\nUse following: \ncreate Message\nupdate id newMessage\ndelete id\nmark-in-progress id\nmark-done id\nlist status[done,todo,inprogress](not required)\n\n"
    );
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match std::fs::exists("todos.json").unwrap() {
        true => {}
        false => {
            std::fs::write("todos.json", "").unwrap();
        }
    }
    let json = std::fs::read_to_string("todos.json").unwrap();

    let mut todo_lista: TaskHolder = if !json.is_empty() {
        serde_json::from_str(&json).unwrap()
        // println!("{:#?}", todo_lista);
    } else {
        TaskHolder { all_tasks: vec![] }
    };
    match args.get(1) {
        None => panic_error(),
        Some(action) => match action.as_str() {
            "list" => match args.get(2) {
                None => todo_lista.list(None),
                Some(status) => match status.as_str() {
                    "done" => todo_lista.list(Some(Status::Done)),
                    "todo" => todo_lista.list(Some(Status::ToDo)),
                    "inprogress" => todo_lista.list(Some(Status::InProgress)),

                    _ => panic_error(),
                },
            },
            "create" => match args.get(2) {
                None => panic_error(),
                Some(msg) => todo_lista.create(String::from(msg)),
            },
            "remove" => match args.get(2) {
                None => panic_error(),
                Some(id) => todo_lista.remove(id.parse().unwrap()),
            },
            "update" => match args.get(2) {
                None => panic_error(),
                Some(id) => match args.get(3) {
                    None => panic_error(),
                    Some(msg) => todo_lista.update(id.parse().unwrap(), String::from(msg)),
                },
            },
            "mark-done" => match args.get(2) {
                None => panic_error(),
                Some(id) => {
                    todo_lista.mark_done(id.parse().unwrap());
                }
            },
            "mark-in-progress" => match args.get(2) {
                None => panic_error(),
                Some(id) => {
                    todo_lista.mark_in_progress(id.parse().unwrap());
                }
            },
            _ => panic_error(),
        },
    };

    std::fs::write(
        "todos.json",
        serde_json::to_string_pretty(&todo_lista).unwrap(),
    )
    .unwrap();
    println!();
}
