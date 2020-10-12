
pub struct ToDoList{
    tasks: Vec<String>,


}

impl ToDoList{
    pub fn add_task(&mut self, new_task: String){
        assert!(!new_task.contains('\n'));
        self.tasks.push(new_task);
    }

    pub fn reprioritize(&mut self, cur_index: int, new_index: int){
        if(cur_index>new_index){
            //some logic
        }
        else if(cur_index<new_index){
            //some other logic
        }
    }

    pub fn delete_task(&mut self, task_index: int){
        //some logic
    }

    pub fn complete_task(&mut self, task_index:int){
        //some logic to add task to complete list

        self.delete_task(task_index);
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