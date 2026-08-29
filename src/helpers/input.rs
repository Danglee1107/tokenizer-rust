use std::io::{self, Write};

pub fn get_input(message: &str) -> String
{
    print!("{}", message);
    io::stdout().flush().unwrap();

    let mut text: String = String::new();
    io::stdin().read_line(&mut text).expect("fail to read line");

    text.trim().to_string()
}

// fn main()
// {
//     println!("{}", "hello world!");
// }
