use std::fs;
use std::fs::File;
use std::io::Write;


fn main() {
    let fname = "list.txt";
    
    //read contents of file
    let contents = fs::read_to_string(fname)
        .expect("Something went wrong reading the file");
    //separate contents into separate lines
    let mut items = contents.lines().collect::<Vec<_>>();

    //iterate through lines and print them
    for (i, item) in items.iter().enumerate()
    {
        println!("Task {}: {}", i+1, item);
    }

    //Test adding items to list
    items.push("Take a nap in R'lyeh");

  

    let mut file = File::open(fname).unwrap();
    

    for item in items.iter()
    {
        file.write_all(item.as_bytes());
     
    }

    file.sync_all();
  
}
