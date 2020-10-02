use std::fs;
use std::fs::File;
use std::io::Write;
mod to_do_list;
use to_do_list::ToDoList;


fn main() {

    let fname = "./src/list.txt";
    let mut list = ToDoList::new();

    //read contents of file
    let contents = fs::read_to_string(fname)
        .expect("Something went wrong reading the file");

    for item in contents.lines() {
        list.add_task(item.to_string());
    }
    
    //Test adding items to list
    list.add_task(String::from("Replace .txt file with database?"));
    

    let mut file_save = String::new();

    for item in list.task_iter()
    {
        file_save += item;
        file_save += "\n"
    }
    
    //Replaces the file with a file containing "Test write - replace"
    let mut file_new = File::create(fname).unwrap();
    file_new.write_all(file_save.as_bytes()).unwrap();  
}

//Testing fragments
 
    /*
    //iterate through lines and print them
    for (i, item) in items.iter().enumerate()
    {
        println!("Task {}: {}", i+1, item);
    }*/