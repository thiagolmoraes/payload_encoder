use urlencoding::encode;
use clap::{Parser, ValueEnum};
use base64::{Engine as _, engine::general_purpose};
use itertools::Itertools; 

#[derive(ValueEnum, Clone, Debug)]
enum Format {
    #[clap(name = "url")]
    UrlEncoding,
    #[clap(name = "double-url")]
    DoubleUrlEncoding,
    #[clap(name = "base64")]
    Base64,
    #[clap(name = "double-base64")]
    DoubleBase64,
    #[clap(name = "hex")]
    Hex,
    #[clap(name = "octal")]
    Octal,
    #[clap(name = "html-escape")]
    HTMLEscape,
    #[clap(name = "binary")]
    Binary,
}

#[derive(Parser, Debug)]
#[command(name = "payload_encoder")]
#[command(about = "Enter with a payload to encoding", long_about = None)]
struct Args {
    input: String,

    #[arg(short, long, value_enum)]
    format: Format,
}

fn url_enc(input: &str) -> String {
    let single_encoding = encode(input).to_string();
    println!("Simple Encoded: {}", single_encoding);
    single_encoding
}

fn double_enc(input: &str) {
    let first_encoding = url_enc(input);
    let double_encoding = encode(&first_encoding);
    println!("Double Encoded: {}", double_encoding);
}

fn hex_enc(input: &str) {
    let hex_enc = hex::encode(input)
        .as_bytes()
        .chunks(2)
        .map(|chunk| std::str::from_utf8(chunk).unwrap())
        .join(" ");
    println!("Hex Encoded: {}", hex_enc);
}

fn base64_enc(input: &str) -> String {
    let mut string_b64 = String::new();
    general_purpose::STANDARD.encode_string(input, &mut string_b64);
    println!("Base64 Encoded: {}", string_b64);
    string_b64
}

fn double_base64_enc(input: &str) {
    let first_b64 = base64_enc(input);
    let mut double_b64 = String::new();
    general_purpose::STANDARD.encode_string(&first_b64, &mut double_b64);
    println!("Double Base64 Encoded: {}", double_b64);
}

fn binary_enc(input: &str) {
    let bin_enc = input
        .as_bytes()
        .iter()
        .map(|byte| format!("{:08b}", byte))
        .collect::<Vec<String>>()
        .join(" ");
    println!("Binary Encoded: {}", bin_enc);
}

fn html_escape(input: &str) {
    let html_escp = input
        .chars()
        .map(|c| match c {
            '<' => "&lt;".to_string(),
            '>' => "&gt;".to_string(),
            '&' => "&amp;".to_string(),
            '"' => "&quot;".to_string(),
            '\'' => "&#39;".to_string(),
            _ => c.to_string(),
        })
        .collect::<Vec<String>>()
        .join("");
    println!("HTML Escape: {}", html_escp);
}

fn octal_enc(input: &str) {
    let octa_enc = input
        .as_bytes()
        .iter()
        .map(|byte| format!("{:03o}", byte))
        .collect::<Vec<String>>()
        .join(" ");
    println!("Octal Encoded: {}", octa_enc);
}

fn main() {
    let args = Args::parse();

    match args.format {
        Format::UrlEncoding => { url_enc(&args.input); },
        Format::DoubleUrlEncoding => { double_enc(&args.input); },
        Format::Base64 => { base64_enc(&args.input); },
        Format::DoubleBase64 => { double_base64_enc(&args.input); },
        Format::Hex => { hex_enc(&args.input); },
        Format::Octal => { octal_enc(&args.input); },
        Format::HTMLEscape => { html_escape(&args.input); },
        Format::Binary => { binary_enc(&args.input); },
    }
}
