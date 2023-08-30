# hapsi

A package that represents a musical concept.

## Example

```rust
use hapsi::prelude::*;

let scale = Diatonic::major(&"C".parse().unwrap());
let mut tones = scale.classes();

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
