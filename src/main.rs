use std::collections::{BinaryHeap, HashMap};

use indicatif::{ProgressBar, ProgressStyle};

use indexmap::IndexMap;

// mod helpers;

type TokenId = u32;
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
        self.next_id += 1;
        self.id_to_token.insert(self.next_id, token.clone());
    }
}

struct MergeRules
{
    rules_table: HashMap<ByteToken, u32>,
    size: u32,
}

impl MergeRules
{
    fn add(&mut self, merge: &ByteToken)
    {
        self.size += 1;
        self.rules_table.insert(merge.clone(), self.size);
    }
}

struct MergeInfo
{
    new_index: usize,
    old_left: Option<ByteToken>,
    old_right: Option<ByteToken>,
}


fn record_pair
(
    left: &ByteToken,
    right: &ByteToken,
    frequencies: &mut IndexMap<ByteToken, u32>,
    pair_parts: &mut HashMap<ByteToken, (ByteToken, ByteToken)>,
    heap: &mut BinaryHeap<(u32, ByteToken)>,
)
{
    let pair: ByteToken = [left.as_slice(), right.as_slice()].concat();
    pair_parts.entry(pair.clone()).or_insert((left.clone(), right.clone()));
    let c = frequencies.entry(pair.clone()).or_insert(0);
    *c += 1;
    heap.push((*c, pair));
}

fn apply_vocab(tokens: &mut Vec<ByteToken>, left: &ByteToken, right: &ByteToken) -> Vec<MergeInfo>
{
    let new_vocab: ByteToken = [left.as_slice(), right.as_slice()].concat();
    let mut merges: Vec<MergeInfo> = Vec::new();
    let mut i = 0;

    while i + 1 < tokens.len()
    {
        if &tokens[i] == left && &tokens[i + 1] == right
        {
            let old_left = if i >= 1 { Some(tokens[i - 1].clone()) } else { None };
            let old_right = if i + 2 < tokens.len() { Some(tokens[i + 2].clone()) } else { None };

            tokens[i] = new_vocab.clone();
            tokens.remove(i + 1);

            merges.push(MergeInfo { new_index: i, old_left, old_right })
        }
        i += 1;
    }
    merges
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

fn update_frequencies_after_merge
(
    merges: Vec<MergeInfo>,
    left: &ByteToken,
    right: &ByteToken,
    frequencies: &mut IndexMap<ByteToken, u32>,
    pair_parts: &mut HashMap<ByteToken, (ByteToken, ByteToken)>,
    heap: &mut BinaryHeap<(u32, ByteToken)>,
)
{
    let new_vocab: ByteToken = [left.as_slice(), right.as_slice()].concat();

    for m in merges
    {
        if let Some(ref ol) = m.old_left
        {
            let old_pair: ByteToken = [ol.as_slice(), left.as_slice()].concat();
            if let Some(c) = frequencies.get_mut(&old_pair)
            {
                if *c > 0 { *c -= 1; }
                heap.push((*c, old_pair));
            }
        }

        if let Some(ref or_) = m.old_right
        {
            let old_pair: ByteToken = [right.as_slice(), or_.as_slice()].concat();
            if let Some(c) = frequencies.get_mut(&old_pair)
            {
                if *c > 0 { *c -= 1; }
                heap.push((*c, old_pair));
            }
        }

        if let Some(ref ol) = m.old_left
        {
            let new_pair: ByteToken = [ol.as_slice(), new_vocab.as_slice()].concat();
            pair_parts.entry(new_pair.clone()).or_insert((ol.clone(), new_vocab.clone()));
            let c = frequencies.entry(new_pair.clone()).or_insert(0);
            *c += 1;
            heap.push((*c, new_pair));
        }

        if let Some(ref or_) = m.old_right
        {
            let new_pair: ByteToken = [new_vocab.as_slice(), or_.as_slice()].concat();
            pair_parts.entry(new_pair.clone()).or_insert((new_vocab.clone(), or_.clone()));
            let c = frequencies.entry(new_pair.clone()).or_insert(0);
            *c += 1;
            heap.push((*c, new_pair));
        }
    }

    frequencies.insert(new_vocab.clone(), 0);
    heap.push((0, new_vocab));
}

fn display_vocab(vocab: &Vocabulary, bytes: bool)
{
    let mut keys: Vec<_> = vocab.id_to_token.keys().collect();
    keys.sort();
    for key in keys
    {
        let token: ByteToken = vocab.id_to_token[key].clone();
        if !bytes
        {
            if let Ok(t) = std::str::from_utf8(&token)
            {
                println!("{} -> '{}'",key, t);
            }
            else 
            {
                println!("{} -> {:?}", key, token);
            }
            continue;
        }
        println!("{} -> {:?}",key, token);
    }
}

fn display_rules(merge_rules: &MergeRules)
{
    let mut keys: Vec<_> = merge_rules.rules_table.keys().collect();
    keys.sort();
    for key in keys
    {
        println!("{:?} -> {}", key, merge_rules.rules_table[key]);
    }
}

fn encode(raw: &str, merge: u32)
{
    let start_total = std::time::Instant::now();
    
    //initialize
    let bytes: &[u8] = raw.as_bytes();
    let mut tokens : Vec<ByteToken> = bytes.iter().map(|&b| vec![b]).collect();
    let mut vocab: Vocabulary = Vocabulary{id_to_token: HashMap::new(), next_id: 0};
    let mut merge_rules: MergeRules = MergeRules {rules_table: HashMap::new() , size: 0};
    let mut frequencies: IndexMap<ByteToken, u32> = IndexMap::new();
    let mut pair_parts: HashMap<ByteToken, (ByteToken, ByteToken)> = HashMap::new();
    let mut heap: BinaryHeap<(u32, ByteToken)> = BinaryHeap::new();

    // progress bar 
    let pb = ProgressBar::new(merge as u64);
    pb.set_style
    (
        ProgressStyle::with_template(
            "[{elapsed_precise}] [{bar:40.green/red}] {pos}/{len} ({eta})"
        )
        .unwrap()
        .progress_chars("=C•"),
    );

    //timer
    let mut count_time = std::time::Duration::ZERO;
    let mut freq_time = std::time::Duration::ZERO;
    let mut merge_time = std::time::Duration::ZERO;

    let start = std::time::Instant::now();
    let token_size = tokens.len();
    for i in 0..token_size-1
    {
        let left: ByteToken  = tokens[i].clone();
        let right: ByteToken  = tokens[i + 1].clone();
        record_pair(&left, &right, &mut frequencies, &mut pair_parts, &mut heap);
    }

    count_time += start.elapsed();

    for _ in 0..merge
    {

        let start = std::time::Instant::now();
        let mut most_freq_token: Option<ByteToken> = None;
        loop 
        {
            let (heap_count, pair) = match heap.pop()
            {
                Some(t) => t,
                None => break, // nothing left
            };

            let real_count = frequencies.get(&pair).copied().unwrap_or(0);

            if real_count != heap_count
            {
                continue;
            }

            if real_count > 1 
            {
                most_freq_token = Some(pair);
                break;
            }
        }
        freq_time += start.elapsed();

        // end if now pair of words appear twice
        let most_freq_token = match most_freq_token
        {
            Some(t) => t,
            None =>
            {
                pb.finish_with_message("not more merge possible!");
                break;
            },
        };
        
        let (left, right) = pair_parts.get(&most_freq_token).unwrap().clone();

        vocab.add(&most_freq_token);
        merge_rules.add(&most_freq_token);

        let start = std::time::Instant::now();
        let merges = apply_vocab(&mut tokens,&left, &right);
        update_frequencies_after_merge
            (
                merges,
                &left,
                &right,
                &mut frequencies,
                &mut pair_parts,
                &mut heap
            );
        merge_time += start.elapsed();

        // update progress bar
        pb.inc(1);
    }
    pb.finish_with_message("Encoding Complete!");

    // println!("time takes for encoding:");
    println!("Frequency counting : {:?}", count_time);
    println!("Find most frequency: {:?}", freq_time);
    println!("Apply merges       : {:?}", merge_time);
    println!("total              : {:?}", start_total.elapsed());

    //display
    println!("Vocabulary:");
    display_vocab(&vocab, false);
    println!("----------------");
    println!("Rules:");
    display_rules(&merge_rules);
    println!("{:?}", tokens);
    display_as_string(&tokens, &vocab);
}

fn main()
{
    // let text = helpers::get_input("enter your text: ");
    let text1: String = String::from("low lower lowest pretty prettier prettiest cat in the hat");
    println!("time for 1 page: {} chars", text1.len());
    encode(&text1, 10000);

}
