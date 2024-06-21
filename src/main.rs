use urlencoding::encode;
use clap::Parser;
use base64::{Engine as _, engine::general_purpose};
use itertools::Itertools; 

#[derive(Parser, Debug)]
#[command(name="Input")]
#[command(about="Enter with a payload to encoding", long_about=None)]
struct Args{
    input: String
}

fn url_enc(input: &str) -> String{
    encode(input).to_string()
}

fn double_enc(input: &str) -> String{
    let first_encoding = url_enc(input);
    encode(&first_encoding).to_string()
}

fn hex_enc(input: &str) -> String{
    hex::encode(input)
    .as_bytes()
    .chunks(2)
    .map(|chunk| std::str::from_utf8(chunk).unwrap())
    .join(" ")

}

fn base64_enc(input: &str) -> String{
    let mut string_b64 = String::new();
    general_purpose::STANDARD.encode_string(input, &mut string_b64);
    string_b64
}



fn binary_enc(input: &str) -> String{
    input
        .as_bytes()
        .iter()
        .map(|byte| format!("{:08b}",byte))
        .collect::<Vec<String>>()
        .join(" ")
        
}

fn html_escape(input: &str) -> String {
    input
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
        .join("")
}

fn octal_enc(input: &str) -> String{
    input
        .as_bytes()
        .iter()
        .map(|byte| format!("{:03o}",byte))
        .collect::<Vec<String>>()
        .join(" ")
        
}

fn main() {

    let args = Args::parse();
    
    println!("Simple Encoded: {}", url_enc(&args.input));
    println!("Double Encoded: {}", double_enc(&args.input));
    println!("HTML Escape: {}", html_escape(&args.input));
    println!("Hex Encoded: {}", hex_enc(&args.input));
    println!("Base64 Encoded: {}", base64_enc(&args.input));
    println!("Binary Encoded: {}", binary_enc(&args.input));
    println!("Octal Encoded: {}", octal_enc(&args.input));
       

}
