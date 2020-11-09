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

    //running parse_input to test user input functionality
    let args: Vec<String> = env::args().collect();
    let input = parse_input(&args);
    println!("{}", input.to_string());

    /*
    *   
    *
    *   There are a couple of ways I can handle the user input - but unless I can figure out 
    *   a way to prompt the user for input rather than have it in the same line as executing the program, 
    *   there isn't a good way to tell the user what to expect. 
    *
    *   I can have the user enter a number which is associated with a particular function of the struct, 
    *   followed by the arguments required by the function - this seems like the best option for now.
    *
    *   The other thing that is lacking here, is that the program updates the file too frequently, on a one 
    *   update per command basis, requiring  the user to run the program again to do another command. Something
    *   that will need ot come with the 'prompt user for input' functionality is the ability to maintain the 
    *   program in a running state while executing any number of changes to the list, and saving the updated 
    *   contents once the program is told to end. 
    *
    *
    */

    //iterates through lines for .txt file, adds them as tasks in the list object
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

    //prints contents of list, for testing purposes.
    for item in list.task_iter() {
        println!("{}", item);
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

fn parse_input(args: &[String]) -> String {

    let mut input_string = String::new();

    //for loop to compile arguments into single string
    for arg in args{
        //if statement to ignore default first argument of library name
        if args.iter().position(|r| r == arg).unwrap()!=0{
            input_string.push_str(arg);
        }
    }

    return input_string;

}

//Testing fragments

/*
//iterate through lines and print them
for (i, item) in items.iter().enumerate()
{
    println!("Task {}: {}", i+1, item);
}*/
