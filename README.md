# Ketsugou

## Exmaples

```rust
use std::collections::VecDeque;
let kitten: VecDeque<_> = vec!['k', 'i', 't', 't', 'e', 'n'].into();
let sitting: VecDeque<_> = vec!['s', 'i', 't', 't', 'i', 'n', 'g'].into();

let expected: VecDeque<_> = vec![
    Merged::Replace('k', 's'),
    Merged::Keep('i', 'i'),
    Merged::Keep('t', 't'),
    Merged::Keep('t', 't'),
    Merged::Replace('e', 'i'),
    Merged::Keep('n', 'n'),
    Merged::Append('g'),
]
.into();
let merged = ketsugou::merge(
    kitten,
    sitting,
    |x, y| *x == *y,
    ketsugou::constant_cost2(1.0),
    ketsugou::constant_cost1(1.0),
    ketsugou::constant_cost1(1.0),
);

assert_eq!(expected, merged);
```
