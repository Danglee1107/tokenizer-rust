use std::collections::HashMap;

// mod helpers;

type TokenId = u32;
type Token = String;

struct Vocabulary
{
    id_to_token: HashMap<TokenId, Token>,
    next_id: u32,
}

impl Vocabulary
{
    fn add(&mut self, token: Token)
    {
        self.id_to_token.insert(self.next_id, token);
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

fn token_frequency(tokens: &Vec<Token>) -> HashMap<Token, u32>
{
    let mut counts: HashMap<Token, u32> = HashMap::new();
    for token in tokens
    {
        *counts.entry(token.clone()).or_insert(0) += 1;
    }
    counts
}

fn apply_vocab(tokens: &mut Vec<Token>, vocab: &Vocabulary)
{
    let mut i = 0;
    while i + 1 < tokens.len()
    {
        let pair: Token = format!("{}{}", *&tokens[i],*&tokens[i+1]);
        if vocab.id_to_token.values().any(|v| *v == pair)
        {
            tokens[i] = pair.to_string();
            tokens.remove(i + 1);
        }
        else
        {
            i += 1;
        }
    }

    for token in tokens.iter_mut()
    {
        for (k,v) in &vocab.id_to_token
        {
            if *token == *v
            {
                *token = format!("<{}>", k);
            }
        }
    }
}

fn display_vocab(vocab: &Vocabulary)
{
    let mut keys: Vec<_> = vocab.id_to_token.keys().collect();
    keys.sort();
    for key in keys
    {
        println!("{} -> {}",key, vocab.id_to_token[key]);
    }

}

fn display(tokens: &Vec<Token>)
{
    let s: String = tokens.join("");
    println!("{}", s);
}

fn most_frequent_token(table: &HashMap<Token, u32>) -> Token
{
    let mut most_freq_token: Token = String::new();
    let mut highest_freq: u32 = 0;
    for (k,v) in table
    {
        if *v > highest_freq
        {
            highest_freq = *v;
            most_freq_token = k.clone();

        }
    }
    most_freq_token
}

fn encode(raw: &str)
{
    let mut tokens: Vec<Token> = tokenize_chars(&raw);
    let mut vocab: Vocabulary = Vocabulary{id_to_token: HashMap::new(), next_id: 0};

    let mut pairs: Vec<Token> = Vec::new();
    for _ in 0..10
    {
        let token_size = tokens.len();
        for i in 0..token_size-1
        {
            let pair = &tokens[i..i+2];
            pairs.push(pair.join(""));

        }

        let frequencies: HashMap<Token, u32> = token_frequency(&pairs);
        let most_freq_token: Token = most_frequent_token(&frequencies);

        if frequencies.get(&most_freq_token) == Some(&1) 
        {
            break
        }

        vocab.add(most_freq_token);
        pairs.clear();

        apply_vocab(&mut tokens, &vocab);
    }

    display_vocab(&vocab);
    display(&tokens);
}

fn main()
{
    // let text = helpers::get_input("enter your text: ");
    let text: String = String::from("low lowest nearst lower nicer");
    encode(&text);
}
