//use std::io::BufferedReader;
//use std::io::File;
//use std::from_str::from_str;
use std::fs;

fn main() {
    let fname = "list.txt";
    //let path = Path::new(fname);
    //let mut file = BufferedReader::new(File::open(&path));
    
    //read contents of file
    let contents = fs::read_to_string(fname)
        .expect("Something went wrong reading the file");
    //separate contents into separate lines
    let items = contents.lines().collect::<Vec<_>>();

    //iterate through lines and print them
    for item in items.iter()
    {
        println!("Task: {}", item);
    }

}
