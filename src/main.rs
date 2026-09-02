use std::collections::{HashMap, BTreeMap};

// mod helpers;

type TokenId = u32;
type Token = String;
type Bytes_token = u8;

struct Vocabulary
{
    id_to_token: HashMap<TokenId, Vec<Bytes_token>>,
    next_id: u32,
}

impl Vocabulary
{
    fn add(&mut self, token: &Vec<Bytes_token>)
    {
        self.id_to_token.insert(self.next_id, token.to_vec());
        self.next_id += 1
    }
}

fn tokenize_chars(text: &str) -> Vec<Token>
{
    let mut tokens: Vec<Token> = Vec::new();
    for c in text.chars()
    {
        tokens.push(c.to_string());
    }
    tokens
}

fn token_frequency(tokens: &Vec<Vec<Bytes_token>>) -> BTreeMap<Vec<Bytes_token>, u32>
{
    let mut counts: BTreeMap<Vec<Bytes_token>, u32> = BTreeMap::new();
    for token in tokens
    {
        *counts.entry(token.clone()).or_insert(0) += 1;
    }
    counts
}

fn most_frequent_token<'a, I>(frequencies: I) -> Vec<Bytes_token>
where I: IntoIterator<Item = (&'a Vec<Bytes_token>, &'a u32)>,
{
    let mut most_freq_token: Vec<Bytes_token> = Vec::new();
    let mut highest_freq: u32 = 0;
    for (k,v) in frequencies
    {
        if *v > highest_freq
        {
            highest_freq = *v;
            most_freq_token = k.clone();

        }
    }
    most_freq_token
}

fn apply_vocab(tokens: &mut Vec<Vec<Bytes_token>>, vocab: &Vocabulary)
{
    let mut i = 0;
    while i + 1 < tokens.len()
    {
        // let pair: Token = format!("{}{}", *&tokens[i],*&tokens[i+1]);
        let mut pair: Vec<Bytes_token> = tokens[i].clone();
        pair.extend(tokens[i + 1].iter().copied());
        if vocab.id_to_token.values().any(|v| *v == pair)
        {
            tokens[i] = pair;
            tokens.remove(i + 1);
        }
        else
        {
            i += 1;
        }
    }

    // for token in tokens.iter_mut()
    // {
    //     for (k,v) in &vocab.id_to_token
    //     {
    //         if *token == *v
    //         {
    //             *token = format!("<{}>", k);
    //         }
    //     }
    // }
}

fn display_vocab(vocab: &Vocabulary)
{
    let mut keys: Vec<_> = vocab.id_to_token.keys().collect();
    keys.sort();
    for key in keys
    {
        println!("{} -> {:?}",key, vocab.id_to_token[key]);
    }

}

fn display(tokens: &Vec<Token>)
{
    let s: String = tokens.join("");
    println!("{}", s);
}

fn encode(raw: &str, merge: u32)
{
    // let mut tokens: Vec<Token> = tokenize_chars(&raw);
    let bytes: &[Bytes_token] = raw.as_bytes();
    let mut tokens : Vec<Vec<Bytes_token>> = bytes.iter().map(|&b| vec![b]).collect();
    let mut vocab: Vocabulary = Vocabulary{id_to_token: HashMap::new(), next_id: 0};

    let mut pairs: Vec<Vec<Bytes_token>> = Vec::new();
    for _ in 0..merge
    {
        let token_size = tokens.len();
        for i in 0..token_size-1
        {
            let mut pair = tokens[i].clone();
            pair.extend(tokens[i + 1].iter().copied());
            pairs.push(pair);

        }

        let frequencies: BTreeMap<Vec<Bytes_token>, u32> = token_frequency(&pairs);
        let most_freq_token: Vec<Bytes_token> = most_frequent_token(&frequencies);

        if frequencies.get(&most_freq_token) == Some(&1) // words appear one
        {
            break
        }

        vocab.add(&most_freq_token);
        pairs.clear();
        // println!("{:#?}", frequencies);
        // println!("{:#?}", most_freq_token);

        apply_vocab(&mut tokens, &vocab);
    }
    println!("{:#?}", vocab.id_to_token);

    // display_vocab(&vocab);
    // display(&tokens);
}

fn main()
{
    // let text = helpers::get_input("enter your text: ");
    let text: String = String::from("low lowest nearst lower nicer");
    encode(&text, 10);

}
