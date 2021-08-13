pub fn flatten<I>(iter: I) -> Flatten<I>
where
    I: Iterator,
    I::Item: IntoIterator, // O::Item implements IntoIterator (so we can iterate over them)
{
    Flatten::new(iter)
}

pub struct Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator, // O::Item implements IntoIterator (so we can iterate over them)
{
    outer: O,
    inner: Option<<O::Item as IntoIterator>::IntoIter>,
}

impl<O> Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator, // O::Item implements IntoIterator (so we can iterate over them)
{
    fn new(iter: O) -> Self {
        Flatten {
            outer: iter,
            inner: None,
        }
    }
}
impl<O> Iterator for Flatten<O>
where
    O: Iterator,           // O implements Iterator
    O::Item: IntoIterator, // O::Item implements IntoIterator (so we can iterate over them)
{
    type Item = <O::Item as IntoIterator>::Item; // Item is an elem of the O::Item
    fn next(&mut self) -> Option<Self::Item> {
        //self.outer.next().and_then(|inner| inner.into_iter().next())

        loop {
            // Get inner iterator if it's not None
            if let Some(ref mut inner_iter) = self.inner {
                // Get next item `i` from the `inner_iter`
                if let Some(i) = inner_iter.next() {
                    return Some(i);
                }
                // If `i` is none set self.inner to None
                self.inner = None;
            }
            // If `self.inner` is None get next iterator
            let next_inner_item = self.outer.next()?.into_iter(); // Some or None
            self.inner = Some(next_inner_item);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn empty() {
        assert_eq!(flatten(std::iter::empty::<Vec<()>>()).count(), 0);
    }

    #[test]
    fn one() {
        assert_eq!(flatten(std::iter::once(vec!["a"])).count(), 1);
    }

    #[test]
    fn two() {
        assert_eq!(flatten(std::iter::once(vec!["a", "b"])).count(), 2);
    }

    #[test]
    fn two_wide() {
        assert_eq!(flatten(vec![vec!["a"], vec!["b"]].into_iter()).count(), 2);
    }
}
