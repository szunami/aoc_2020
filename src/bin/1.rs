use std::path::Path;
use std::io::BufRead;
use std::{fs::File, io};

fn main() {

    let input = {
        let mut input = vec![];
    
        if let Ok(lines) = read_lines("./data/1/1.txt") {
            for line in lines {
                if let Ok(line) = line {
                    let x: i32 = line.parse().expect("Failed to read line :(");
                    input.push(x);
                }
            }
        }
        input
    };
    
    let mut count  = 0;
 
    for x in &input {
        for y in &input {
            
            count += 1;
            
            if x + y == 2020 {
                println!("{} * {} = {}", x, y, x * y);
            }
        }
        
    }
    
    println!("{}", count);
    
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}