use std::{collections::{BTreeMap, HashMap}};

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

fn display(tokens: &Vec<ByteToken>)
{
    println!("{:?}", tokens);
}

fn encode(raw: &str, merge: u32)
{
    let bytes: &[u8] = raw.as_bytes();
    let mut tokens : Vec<ByteToken> = bytes.iter().map(|&b| vec![b]).collect();
    let mut vocab: Vocabulary = Vocabulary{id_to_token: HashMap::new(), next_id: 0};
    let mut merge_rules: MergeRules = MergeRules {rules_table: HashMap::new() , size: 0};

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
        merge_rules.add(&most_freq_token);
        pairs.clear();

        apply_vocab(&mut tokens, &vocab);
    }

    display_vocab(&vocab, false);
    display_rules(&merge_rules);

    // display(&tokens);
    // display_as_string(&tokens, &vocab);
}

fn main()
{
    // let text = helpers::get_input("enter your text: ");
    let text: String = String::from("
Proceedings of the 1st Workshop on Stereotypes Across Cultures in Language Technologies (StereACuLT 2026), pages 62–68
July 3, 2026 ©2026 Association for Computational Linguistics
Stereotyped by Silence: How LLMs Erase Northeast Indian
Languages Through Omission and Orthographic Corruption
Badal Nyalang
MWire Labs
Shillong, Meghalaya, India
nyalang@mwirelabs.com
Abstract
Large language models (LLMs) perpetuate cultural stereotypes not only through biased associations but through systematic omission and
orthographic erasure of underrepresented languages. We present empirical evidence of two
compounding failure modes affecting Northeast Indian languages: (1) entity-level invisibility, where state-of-the-art NER systems score
F1 = 0.000 on culturally critical named entities
such as Khasi surnames, Garo festivals, and
tribal names; and (2) orthographic corruption,
where LLM tokenizers corrupt semantically
meaningful diacritics (ï, ñ) and the Garo morpheme boundary marker (U+00B7, ·) at rates
of 18.8–50% across four of five evaluated models. Drawing on NortheastNER (F1 = 0.964, six
entity categories, XLM-RoBERTa-base) and a
systematic tokenization study across Khasi and
Garo, we argue that stereotype-by-omission
constitutes a distinct and measurable harm to
indigenous language communities. We further show that a custom multilingual tokenizer
achieves 26–50% token reduction over five
baseline LLMs, demonstrating that culturally
grounded infrastructure can partially remediate
these failures. Our findings call for cultural
representation audits as a standard component
of multilingual NLP evaluation.
1 Introduction
Stereotype research in NLP has concentrated on
biased associations: models linking gender to occupation, or ethnicity to negative sentiment (Blodgett
et al., 2020; Gallegos et al., 2024). This focus,
while important, leaves a more fundamental problem unaddressed. When a model cannot recognize
the name of a tribal community, cannot preserve
the diacritics that distinguish words in an indigenous language, or has never encountered the name
of a major regional festival, the harm is not an association. It is an absence. The community does
not receive a distorted reflection; it receives none
at all.
Northeast India makes this concrete. The region
comprises eight states and over 220 distinct languages spanning the Austroasiatic, Tibeto-Burman,
and Indo-Aryan families, alongside contact varieties such as Nagamese. Despite tens of millions
of speakers, these languages are almost entirely absent from major multilingual NLP systems (Joshi
et al., 2020). The consequences are not abstract.
NER systems that score F1 = 0.000 on Khasi surnames cannot support legal document processing,
government service delivery, or cultural archiving
in Khasi. Tokenizers that corrupt Garo morpheme
markers at 50% rates cannot serve as reliable infrastructure for any downstream Garo application.
This paper presents evidence of two failure
modes that are facets of a single underlying problem: the systematic exclusion of Northeast Indian languages from multilingual NLP infrastructure. First, we demonstrate entity-level invisibility
through NortheastNER, a domain-specific NER
model for Northeast India. Baseline multilingual
models score F1 = 0.000 on entities such as Lyngdoh (a prominent Khasi surname), Wangala (the
principal Garo harvest festival), and Garo (the
tribal community itself). NortheastNER, fine-tuned
on domain-specific data, achieves F1 = 0.964 on
the same entities. Second, we demonstrate orthographic erasure through a systematic evaluation of
five LLMs on Khasi diacritics (ï, ñ) and the Garo
morpheme boundary marker (U+00B7). Four of
five models corrupt these characters at rates between 18.8% and 50%. A custom multilingual tokenizer achieving 26–50% token reduction across
five languages demonstrates that both failure modes
are addressable through community-grounded infrastructure.
Together, these findings operationalize
stereotype-by-omission as a measurable harm
category, extending existing frameworks for
representation disparity (Joshi et al., 2020;
Gallegos et al., 2024) toward communities absent
62
from model training entirely, and propose cultural
representation audits as a practical response.
2 Background and Related Work
2.1 Bias as Association vs. Bias as Omission
The dominant paradigm in NLP bias research treats
stereotyping as an association problem (Blodgett
et al., 2020). Models encode associations between
demographic groups and attributes, and these associations reflect and amplify societal biases (Gallegos et al., 2024). Hofmann et al. (2024) extend
this to covert discrimination: LLMs make systematically worse decisions about speakers of African
American English based on dialect cues alone (Hu
et al., 2025). Tao et al. (2024) show that LLMs
exhibit strong Western value alignment via World
Values Survey comparisons, reflecting whose values were encoded during training.
These findings assume the target community
is represented in training data. For most Northeast Indian language communities, that assumption
does not hold. The failure is not distortion but erasure. The analysis must start earlier, at the level
of whether the community appears in the model’s
representational world at all.
2.2 Low-Resource Languages and
Tokenization
");
    encode(&text, 10000);

}
