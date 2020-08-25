# luhn-rs

[![Build Status](https://travis-ci.org/andrew-d/luhn-rs.svg?branch=master)](https://travis-ci.org/andrew-d/luhn-rs) [![Coverage Status](https://coveralls.io/repos/andrew-d/luhn-rs/badge.svg?branch=master&service=github)](https://coveralls.io/github/andrew-d/luhn-rs?branch=master) [![Docs](https://img.shields.io/badge/docs-latest-blue.svg)](https://andrew-d.github.io/luhn-rs/luhn/index.html)

This project allows generating and verifying Luhn check digits and using
arbitrary alphabets.

# Example

Add this to your `Cargo.toml`:

```
[dependencies]
luhn-rs = "0.0.1"
```

Then, in your crate:

```rust
extern crate luhn;

use luhn::Luhn;
```

Generating a check digit:

```rust
// The alphabet given dictates what input characters are allowed.
let l = Luhn::new("abcdef").expect("invalid alphabet given");

let ch = match l.generate("abcdef") {
    Ok(ch) => ch,
    Err(e) => panic!("unexpected generate error: {:?}", e),
};

println!("the luhn check digit is: {}", ch);
```

Verifying a check digit (this uses the last character in the string as the
check digit):

```rust
let l = Luhn::new("abcdef").expect("invalid alphabet given");

println!("validating 'abcdefe': {}", l.validate("abcdefe").unwrap());
println!("validating 'abcdefa': {}", l.validate("abcdefe").unwrap());
```

# License

MIT
