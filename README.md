# Ketsugou

## Exmaples

### a Basic Usage

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

### with Different Types

```rust
use std::collections::VecDeque;
let kitten: VecDeque<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0].into();
let sitting: VecDeque<i64> = vec![2, 2, 4, 4, 6, 6].into();

let expected: VecDeque<Merged<f64, i64>> = vec![
    Merged::Replace(1.0, 2),
    Merged::Keep(2.0, 2),
    Merged::Replace(3.0, 4),
    Merged::Keep(4.0, 4),
    Merged::Replace(5.0, 6),
    Merged::Keep(6.0, 6),
]
.into();
let merged = super::merge(
    kitten,
    sitting,
    |x, y| *x == *y as f64,
    super::constant_cost2(1.0),
    super::constant_cost1(1.0),
    super::constant_cost1(1.0),
);

assert_eq!(expected, merged);
```
