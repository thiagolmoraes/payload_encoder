# Payload Encoder

This project allows you to encode an input string into various formats, including URL, Base64, Hexadecimal, Binary, Octal, and HTML Escape.

## How to Use

You can use this program to encode any string into different formats.

Available formats:
- url
- double-url
- base64
- double-base64
- hex
- octal
- html-escape
- binary


### Usage Examples

To encode a simple string:

```bash
payload_encoder "<script>alert(1)</script> --format url" 
```

Direct from cargo run
```bash
cargo run -- "<script>alert(1)</script> --format url" 
```
