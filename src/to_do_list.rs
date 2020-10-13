
pub struct ToDoList{
    tasks: Vec<String>,
    complete: Vec<String>,

}

impl ToDoList{
    pub fn add_task(&mut self, new_task: String){
        assert!(!new_task.contains('\n'));
        self.tasks.push(new_task);
    }

    pub fn reprioritize(&mut self, cur_index: usize, new_index: usize){
        //Will need some sort of protection here for if user enters bad index
        let temp = self.tasks.remove(cur_index);
        self.tasks.insert(new_index, temp);
    }

    pub fn delete_task(&mut self, task_index: usize){
        self.tasks.remove(task_index);
    }

    pub fn complete_task(&mut self, task_index: usize){
        //some logic to add task to complete list

        self.complete.push(self.tasks[task_index].to_string());
        self.delete_task(task_index);
    }

    pub fn new() -> Self{
        //The One True Constructor
        Self {
            tasks: Vec::new(),
            complete: Vec::new(),
        }
    }

    pub fn task_iter(&self) -> impl Iterator<Item=&str>{
        self.tasks.iter().map(|s| s.as_str())
    }
}