use std::{fmt::Display, hash::Hash, vec};

use enum_iterator::{all, Sequence};
use int_enum::IntEnum;

#[repr(u8)]
#[derive(Debug, PartialEq, Sequence, Copy, Clone, IntEnum)]
pub enum ResistorColor {
    Black = 0,
    Brown = 1,
    Red = 2,
    Orange = 3,
    Yellow = 4,
    Green = 5,
    Blue = 6,
    Violet = 7,
    Grey = 8,
    White = 9,
}

pub fn color_to_value(_color: ResistorColor) -> usize {
    _color as usize
}

pub fn value_to_color_string(value: usize) -> String {
    match ResistorColor::from_int(value as u8) {
        Ok(color) => format!("{:?}", color),
        Err(_) => "value out of range".to_string(),
    }
}

pub fn colors() -> Vec<ResistorColor> {
    all::<ResistorColor>().collect::<Vec<_>>()
}

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(a: &[T], b: &[T]) -> Comparison {
    use Comparison::*;
    match (a.len(), b.len()) {
        (0, 0) => Equal,
        (0, _) => Sublist,
        (_, 0) => Superlist,
        (m, n) if m > n => {
            if a.windows(n).any(|v| v == b) {
                Superlist
            } else {
                Unequal
            }
        }
        (m, n) if m < n => {
            if a.windows(m).any(|v| v == b) {
                Sublist
            } else {
                Unequal
            }
        }
        (_, _) => {
            if a == b {
                Equal
            } else {
                Unequal
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{sublist, Comparison};
    #[test]
    fn empty_equals_empty() {
        let v: &[u32] = &[];
        assert_eq!(Comparison::Equal, sublist(v, v));
    }
    #[test]
    fn test_empty_is_a_sublist_of_anything() {
        assert_eq!(Comparison::Sublist, sublist(&[], &['a', 's', 'd', 'f']));
    }
    #[test]
    fn test_anything_is_a_superlist_of_empty() {
        assert_eq!(Comparison::Superlist, sublist(&['a', 's', 'd', 'f'], &[]));
    }
    #[test]
    fn test_1_is_not_2() {
        assert_eq!(Comparison::Unequal, sublist(&[1], &[2]));
    }
    #[test]
    fn test_compare_larger_equal_lists() {
        use std::iter::repeat;
        let v: Vec<char> = repeat('x').take(1000).collect();
        assert_eq!(Comparison::Equal, sublist(&v, &v));
    }
    #[test]
    fn test_sublist_at_start() {
        assert_eq!(Comparison::Sublist, sublist(&[1, 2, 3], &[1, 2, 3, 4, 5]));
    }
    #[test]
    fn sublist_in_middle() {
        assert_eq!(Comparison::Sublist, sublist(&[4, 3, 2], &[5, 4, 3, 2, 1]));
    }
    #[test]
    fn sublist_at_end() {
        assert_eq!(Comparison::Sublist, sublist(&[3, 4, 5], &[1, 2, 3, 4, 5]));
    }
    #[test]
    fn partially_matching_sublist_at_start() {
        assert_eq!(Comparison::Sublist, sublist(&[1, 1, 2], &[1, 1, 1, 2]));
    }
    #[test]
    fn sublist_early_in_huge_list() {
        let huge: Vec<u32> = (1..1_000_000).collect();
        assert_eq!(Comparison::Sublist, sublist(&[3, 4, 5], &huge));
    }
    #[test]
    fn huge_sublist_not_in_huge_list() {
        let v1: Vec<u64> = (10..1_000_001).collect();
        let v2: Vec<u64> = (1..1_000_000).collect();
        assert_eq!(Comparison::Unequal, sublist(&v1, &v2));
    }
    #[test]
    fn superlist_at_start() {
        assert_eq!(Comparison::Superlist, sublist(&[1, 2, 3, 4, 5], &[1, 2, 3]));
    }
    #[test]
    fn superlist_in_middle() {
        assert_eq!(Comparison::Superlist, sublist(&[5, 4, 3, 2, 1], &[4, 3, 2]));
    }
    #[test]
    fn superlist_at_end() {
        assert_eq!(Comparison::Superlist, sublist(&[1, 2, 3, 4, 5], &[3, 4, 5]));
    }
    #[test]
    fn second_list_missing_element_from_first_list() {
        assert_eq!(Comparison::Unequal, sublist(&[1, 2, 3], &[1, 3]));
    }
    #[test]
    fn superlist_early_in_huge_list() {
        let huge: Vec<u32> = (1..1_000_000).collect();
        assert_eq!(Comparison::Superlist, sublist(&huge, &[3, 4, 5]));
    }
    #[test]
    fn recurring_values_sublist() {
        assert_eq!(
            Comparison::Sublist,
            sublist(&[1, 2, 1, 2, 3], &[1, 2, 3, 1, 2, 1, 2, 3, 2, 1])
        );
    }
    #[test]
    fn recurring_values_unequal() {
        assert_eq!(
            Comparison::Unequal,
            sublist(&[1, 2, 1, 2, 3], &[1, 2, 3, 1, 2, 3, 2, 3, 2, 1])
        );
    }
}
