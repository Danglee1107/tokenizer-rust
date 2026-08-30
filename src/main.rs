use std::collections::HashMap;

mod helpers;

type TokenId = u32;
type Token = String;

struct Vocabulary
{
    table: HashMap<TokenId, Token>,
    size: u32,
}

impl Vocabulary
{
    fn add(&mut self, item: Token)
    {
        self.table.insert(self.size, item);
        self.size += 1
    }
}

struct Merge<'a>
{
    rules: HashMap<u32, Vec<&'a str>>,
}

fn split_into_char(text: &String) -> Vec<String>
{
    let mut chars_vec: Vec<String> = Vec::new();
    for c in text.chars()
    {
        chars_vec.push(c.to_string());
    }
    chars_vec
}

fn hash_freq(chars: &Vec<String>) -> HashMap<String, u32>
{
    let mut hash: HashMap<String, u32> = HashMap::new();
    for c in chars
    {
        *hash.entry(c.clone()).or_insert(0) += 1;
    }
    hash
}


fn update(text: &mut Vec<Token>, vocab: &Vocabulary)
{
    let mut i = 0;
    while i + 1 < text.len()
    {
        let corpus = format!("{}{}", *&text[i],*&text[i+1]);
        if vocab.table.values().any(|v| *v == corpus)
        {
            text[i] = corpus.to_string();
            text.remove(i + 1);
        }
        else
        {
            i += 1;
        }
    }

    for token in text.iter_mut()
    {
        for (k,v) in &vocab.table
        {
            if *token == *v
            {
                *token = format!("<{}>", k);
            }
        }
    }
}

fn visualize_vocab(vocab: &Vocabulary)
{
    let mut keys: Vec<_> = vocab.table.keys().collect();
    keys.sort();
    for key in keys
    {
        println!("{} -> {}",key, vocab.table[key]);
    }

}

fn visualize(text: &Vec<Token>)
{
    let s: String = text.join("");
    println!("{}", s);
}
fn find_most_freq(table: &HashMap<Token, u32>) -> Token
{
    let mut most_freq: Token = String::new();
    let mut _max: u32 = 0;
    for (k,v) in table
    {
        if *v > _max
        {
            _max = *v;
            most_freq = k.clone();

        }
    }
    most_freq
}

fn encode(origin: &String)
{
    let mut text: Vec<String> = split_into_char(&origin);
    let mut vocab = Vocabulary{table: HashMap::new(), size: 0};


    let mut pairs: Vec<String> = Vec::new();
    for _ in 0..10
    {
        let text_size = &text.len();
        for i in 0..*text_size-1
        {
            let pair = &text[i..i+2];
            pairs.push(pair.join(""));

        }

        let table = hash_freq(&pairs);
        let corpus = find_most_freq(&table);
        // println!("{:#?}", table);
        vocab.add(corpus);

        update(&mut text, &vocab);
    }

    visualize_vocab(&vocab);
    visualize(&text);
}

fn main()
{
    // let text = helpers::get_input("enter your text: ");
    let text: String = String::from("low lowest nearst lower nicer");
    // let chars = split_into_char(&text);
    encode(&text);
}
