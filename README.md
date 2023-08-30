# hapsi

A package that represents a musical concept.

## Example

```rust
use hapsi::prelude::*;

let scale = Diatonic::major(&"C".parse().unwrap());
let scaled = Scaled::new(scale, Twelve);
let keyboard = Keyboard::new(scaled);
let mut tones = keyboard.class_iter();

assert_eq!(tones.next(), Some(&"C".parse().unwrap()));
assert_eq!(tones.next(), Some(&"D".parse().unwrap()));
assert_eq!(tones.next(), Some(&"E".parse().unwrap()));
assert_eq!(tones.next(), Some(&"F".parse().unwrap()));
assert_eq!(tones.next(), Some(&"G".parse().unwrap()));
assert_eq!(tones.next(), Some(&"A".parse().unwrap()));
assert_eq!(tones.next(), Some(&"B".parse().unwrap()));
assert_eq!(tones.next(), None);
```

WIP
