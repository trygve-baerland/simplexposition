/// Returns whether all elements in the iterator are equal or not.
/// If the iterator is empty, all elements are vacuously equal.
pub fn all_same<T>(mut elems: impl Iterator<Item = T>) -> bool
where
    T: PartialEq,
{
    let Some(first) = elems.next() else {
        return true;
    };

    return elems.all(|e| e == first);
}

#[cfg(test)]
mod tests {
    use super::*;

    mod all_same {
        use super::*;

        #[test]
        fn nonempty_same() {
            let values = vec![1, 1, 1];

            assert!(all_same(values.into_iter()));
        }

        #[test]
        fn empty() {
            let values: Vec<usize> = vec![];

            assert!(all_same(values.into_iter()));
        }

        #[test]
        fn one_element() {
            let values = vec![1];

            assert!(all_same(values.into_iter()));
        }

        #[test]
        fn not_all_same() {
            let values = vec![1, 1, 2];

            assert!(!all_same(values.into_iter()));
        }
    }
}
