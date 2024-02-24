
use std::io;

pub fn process_one_line(line: &str) {
    println!("{}", line);
}

pub fn process() -> io::Result<()> {
    let mut buffer = String::new();
    loop {
        io::stdin().read_line(&mut buffer)?;
        process_one_line(&buffer);
    }
}
