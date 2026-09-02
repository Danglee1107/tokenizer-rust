use std::collections::{HashMap, BTreeMap};

// mod helpers;

type TokenId = u32;
type Token = String;
type ByteToken = Vec<u8>;

struct Vocabulary
{
    id_to_token: HashMap<TokenId, ByteToken>,
    next_id: u32,
}

impl Vocabulary
{
    fn add(&mut self, token: &ByteToken)
    {
        self.id_to_token.insert(self.next_id, token.to_vec());
        self.next_id += 1
    }
}

fn token_frequency(tokens: &Vec<ByteToken>) -> BTreeMap<ByteToken, u32>
{
    let mut counts: BTreeMap<ByteToken, u32> = BTreeMap::new();
    for token in tokens
    {
        *counts.entry(token.clone()).or_insert(0) += 1;
    }
    counts
}

fn most_frequent_token<'a, I>(frequencies: I) -> ByteToken
where I: IntoIterator<Item = (&'a ByteToken, &'a u32)>,
{
    let mut most_freq_token: ByteToken = Vec::new();
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

fn apply_vocab(tokens: &mut Vec<ByteToken>, vocab: &Vocabulary)
{
    let mut i = 0;
    while i + 1 < tokens.len()
    {
        let mut pair: ByteToken = tokens[i].clone();
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
}

fn display_as_string(byte_tokens: &Vec<ByteToken>, vocab: &Vocabulary)
{
    let mut text: Token = String::new();
    let token_to_id: HashMap<ByteToken, u32> =
        vocab.id_to_token
        .iter()
        .map(|(id, token)| (token.clone(), *id))
        .collect();

    for token in byte_tokens
    {
        if token.len() == 1 
        {
            text.push(token[0] as char);
        }
        else if let Some(id) = token_to_id.get(token)
        {
            let base = format!("<{}>", id);
            text.push_str(&base);
        }
        else
        {
            text.push_str("<?>");
            
        }
    }
    println!("{}", text);
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

fn display(tokens: &Vec<ByteToken>)
{
    println!("{:?}", tokens);
}

fn encode(raw: &str, merge: u32)
{
    // let mut tokens: Vec<Token> = tokenize_chars(&raw);
    let bytes: &[u8] = raw.as_bytes();
    let mut tokens : Vec<ByteToken> = bytes.iter().map(|&b| vec![b]).collect();
    let mut vocab: Vocabulary = Vocabulary{id_to_token: HashMap::new(), next_id: 0};

    let mut pairs: Vec<ByteToken> = Vec::new();
    for _ in 0..merge
    {
        let token_size = tokens.len();
        for i in 0..token_size-1
        {
            let mut pair = tokens[i].clone();
            pair.extend(tokens[i + 1].iter().copied());
            pairs.push(pair);

        }

        let frequencies: BTreeMap<ByteToken, u32> = token_frequency(&pairs);
        let most_freq_token: ByteToken = most_frequent_token(&frequencies);

        if frequencies.get(&most_freq_token) == Some(&1) // words appear one
        {
            break
        }

        vocab.add(&most_freq_token);
        pairs.clear();

        apply_vocab(&mut tokens, &vocab);
    }

    display_vocab(&vocab);
    println!("{}", raw);
    display(&tokens);
    display_as_string(&tokens, &vocab);
}

fn main()
{
    // let text = helpers::get_input("enter your text: ");
    let text: String = String::from("low lowest nearst lower nicer");
    encode(&text, 10);

}
