use std::{collections::HashSet, path::Path};
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
    
    let mut visited = HashSet::new();
 
    for x in input {
        
        if visited.contains(&(2020 - x)){
            println!("{} * {} = {}", x, 2020 - x, x * (2020 - x));

        }
        
        visited.insert(x);
        
        count += 1;
    }
    
    println!("{}", count);
    
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}