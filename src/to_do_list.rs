
pub struct ToDoList{
    tasks: Vec<String>,


}

impl ToDoList{
    pub fn add_task(&mut self, new_task: String){
        assert!(!new_task.contains('\n'));
        self.tasks.push(new_task);
    }

    pub fn new() -> Self{
        //The One True Constructor
        Self {
            tasks: Vec::new(),
        }
    }

    pub fn task_iter(&self) -> impl Iterator<Item=&str>{
        self.tasks.iter().map(|s| s.as_str())
    }
}