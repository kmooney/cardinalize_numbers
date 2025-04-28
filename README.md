# Cardinalize Numbers

A Rust library that converts numbers into their cardinal English text representation.

## Features

- Converts integers to English text (e.g., 123 → "one hundred twenty three")
- Supports numbers up to the trillions
- US English spelling and formatting

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
cardinalize_numbers = "0.1.0"
```

Then use it in your code:

```rust
use cardinalize_numbers::cardinalize_us_en;

fn main() {
    let number = 123456;
    let text = cardinalize_us_en(number);
    println!("{}: {}", number, text); // Outputs: 123456: one hundred twenty three thousand four hundred fifty six
}
```

## Example Output

```
1 → "one"
100 → "one hundred"
575 → "five hundred seventy five"
8575 → "eight thousand five hundred seventy five"
1398575 → "one million three hundred ninety eight thousand five hundred seventy five"
```

## Testing

Run the tests with:

```
cargo test
```

## License

This project is available as open source under the terms of the MIT License.