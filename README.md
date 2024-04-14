# Cerpton

A "double" [Caesar cipher](https://en.wikipedia.org/wiki/Caesar_cipher) that contains a mix of different alphabets.
Cerpton takes two values and shifts the "alphabet" starting from two different positions (a long with a few extra
things).

Example:
```rust
use cerpton::{Encoder, libcerpton_encode};

let mut encoder = Encoder::new(2, 1);

// Creates a new ``Alphabet``
encoder.set_alphabet();

// Check if the input setting is ok
if !encoder.setting_good() {
    panic!("Invalid setting");
}

let regular_text = String::from("This is some text!!!");
let encoded_text = encoder.encode(regular_text);

println!("{} -> {}", regular_text, encoded_text);
// "This is some text!!!" -> "Sg$غ $غ غ[ШJ KJwKللل"

// Or using a single function:
let encoded_text = libcerpton_encode([2, 1, 0, 0, 0, 0], regular_text);
```

