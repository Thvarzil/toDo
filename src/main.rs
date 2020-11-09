use std::fs;
use std::fs::File;
use std::io::Write;
mod to_do_list;
use to_do_list::ToDoList;
use std::env;

fn main() {
    let fname = "./src/list.txt";
    let mut list = ToDoList::new();
    let mut is_completed = false;
    //read contents of file
    let contents = fs::read_to_string(fname).expect("Something went wrong reading the file");

    for item in contents.lines() {
        if is_completed {
            list.add_comp(item.to_string());
        } else {
            if !is_completed {
                list.add_task(item.to_string());
            } else if item == "C\n" {
                is_completed = true;
            }
        }
    }
    
    //Test completing item on list
    list.complete_task(0);

    let mut file_save = String::new();

    for item in list.task_iter() {
        file_save += item;
        file_save += "\n"
    }

    for item in list.comp_iter() {
        file_save += item;
        file_save += "\n"
    }
    //Replaces the file with updated list
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
