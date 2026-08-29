use std::collections::HashMap;

mod helpers;

struct Vocabulary
{
    table: HashMap<String, u32>,
}

struct Merge<'a>
{
    rules: Vec<&'a str>,
}

fn main()
{
    let text = helpers::get_input("enter your text: ");
    println!("{}", text);
}
