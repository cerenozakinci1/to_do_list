
#[derive(Clone)]
struct Task {
    id: usize,
    description: String,
    is_completed: bool,
}
fn add_task(tasks: &mut Vec<Task>, description: &str) -> Task {
    let id = tasks.len() as usize + 1;
    let task = Task {
        id,
        description: description.to_string(),
        is_completed: false,
    };
    tasks.push(task.clone()); //we don't want the ownership to be moved to vector
    task
}
fn complete_task(tasks: &mut Vec<Task>, id: usize) -> Option<&Task> {
    if id > 0 && id <= tasks.len() {
        tasks[id - 1].is_completed = true; //id-1 to get the index
        Some(&tasks[id-1])
    } else {
        None
    }
}
fn list_tasks(tasks: &[Task]) {
    for task in tasks {
        println!("ID: {}", task.id);
        println!("Description: {}", task.description);
        println!("Completed: {}\n", task.is_completed);
    }
}

fn main() {
   let mut todo_list: Vec<Task> = Vec::new();
   //Code can be tested here
   /* 
    // Adding tasks
    let _task1 = add_task(&mut todo_list, "Do homework");
    let _task2 = add_task(&mut todo_list, "Clean the kitchen");
    let _task3 = add_task(&mut todo_list, "Cook");

    // Listing tasks
    list_tasks(&todo_list);
    // Completing a task
    complete_task(&mut todo_list, 2);

    println!("{}", "testing complete_task");
    // Listing tasks
    list_tasks(&todo_list);
     */
}