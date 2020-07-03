use std::collections::HashMap;

trait Counter {
    type Item;

    fn counts(&self) -> HashMap<&Self::Item, usize>;
}

impl<T> Counter for [T]
where
    T: Eq + std::hash::Hash,
{
    type Item = T;

    fn counts(&self) -> HashMap<&Self::Item, usize> {
        let mut counts = HashMap::new();
        for k in self {
            *counts.entry(k).or_insert(0) += 1;
        }
        counts
    }
}

#[cfg(test)]
mod tests {
    use crate::Counter;

    #[test]
    fn vec_i32() {
        let l = vec![1, 2, 3, 1, 3, 3];
        let counts = l.counts();

        assert_eq!(counts[&1], 2);
        assert_eq!(counts[&2], 1);
        assert_eq!(counts[&3], 3);
    }

    #[test]
    fn arr_i32() {
        let l = [1, 2, 3, 1, 3, 3];
        let counts = l.counts();

        assert_eq!(counts[&1], 2);
        assert_eq!(counts[&2], 1);
        assert_eq!(counts[&3], 3);
    }

    #[test]
    fn slice_i32() {
        let l = &[1, 2, 3, 1, 3, 3][..];
        let counts = l.counts();

        assert_eq!(counts[&1], 2);
        assert_eq!(counts[&2], 1);
        assert_eq!(counts[&3], 3);
    }

    #[test]
    fn slice_str() {
        let s1 = String::from("two");
        let l = ["one", "two", "two", &s1];
        let counts = l.counts();

        assert_eq!(counts[&"one"], 1);
        assert_eq!(counts[&"two"], 3);
    }
}
