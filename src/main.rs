use std::fs;
use std::fs::File;
use std::io::Write;


fn main() {
    let fname = "./src/list.txt";

    let mut file = File::open(fname).unwrap();

    //read contents of file
    let contents = fs::read_to_string(fname)
        .expect("Something went wrong reading the file");

    //separate contents into separate lines
    let mut items = contents.lines().collect::<Vec<_>>();

  

    //Test adding items to list
    items.push("Replace .txt file with database?");

    let mut file_save = String::new();

    for item in items.iter()
    {
        file_save += item;
        file_save += "\n"
    }

    //iterate through lines and print them
    for (i, item) in items.iter().enumerate()
    {
        println!("Task {}: {}", i+1, item);
    }

    println!("{}",file_save);

    
    //This fragment replaces the file with a file containing "Test write - replace"
    let mut fileNew = File::create(fname).unwrap();
    fileNew.write_all(file_save.as_bytes());

    /*
    
    
    for item in items.iter()
    {
        file.write_all(item.as_bytes());
     
    }
    */

    
    
    

    
  
}
