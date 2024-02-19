use std::io;

fn process() -> io::Result<()> {
    let mut buffer = String::new();
    loop {
        io::stdin().read_line(&mut buffer)?;
        println!("{}", buffer);
    }
}
