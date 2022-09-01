# Sorting

## `Ord` trait

[docs](https://doc.rust-lang.org/std/cmp/trait.Ord.html)

`Ord` 
- requires `Eq + PartialOrd<Self>` 
- Total order required. It will always have an answer to "is A greater than B"
- `PartialOrd` may have elements that don't have an order. Ex: floating point numbers have only `PartialOrd` because they have the `nan` type where ordering doesn't make sense.


## Sort 

**Stable sort**
: If I have 2 elements that are equal and appear in some order in the starting array they will remain in that order.

Slice has  `sort(&mut self)` and is stable. There is `sort_unstable` for unstable sort. 

[sort wiki](https://en.wikipedia.org/wiki/Sorting_algorithm). Every sort has pros and cons. 