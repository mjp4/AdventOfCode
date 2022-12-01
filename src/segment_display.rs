#![allow(dead_code)]
use itertools::Itertools;

pub struct SegmentDisplay {
    bit_field: usize,
}

impl SegmentDisplay {
    pub fn from_str(input_str: &str) -> SegmentDisplay {
        SegmentDisplay::from_str_with_mapping(input_str, &SegmentMapping::identity_map())
    }

    pub fn from_str_with_mapping(input_str: &str, mapping: &SegmentMapping) -> SegmentDisplay {
        SegmentDisplay {
            bit_field: input_str
                .chars()
                .map(|c| match c {
                    'a' => 1 << mapping.mapping[0],
                    'b' => 1 << mapping.mapping[1],
                    'c' => 1 << mapping.mapping[2],
                    'd' => 1 << mapping.mapping[3],
                    'e' => 1 << mapping.mapping[4],
                    'f' => 1 << mapping.mapping[5],
                    'g' => 1 << mapping.mapping[6],
                    _ => 0,
                })
                .reduce(|field_acc, val| field_acc | val)
                .unwrap_or(0),
        }
    }

    pub fn to_int(&self) -> Option<usize> {
        match self.bit_field {
            0b1110111 => Some(0),
            0b0100100 => Some(1),
            0b1011101 => Some(2),
            0b1101101 => Some(3),
            0b0101110 => Some(4),
            0b1101011 => Some(5),
            0b1111011 => Some(6),
            0b0100101 => Some(7),
            0b1111111 => Some(8),
            0b1101111 => Some(9),
            _ => None,
        }
    }
}

pub struct SegmentMapping {
    mapping: Vec<usize>,
}

impl SegmentMapping {
    pub fn identity_map() -> SegmentMapping {
        SegmentMapping {
            mapping: vec![0, 1, 2, 3, 4, 5, 6],
        }
    }

    pub fn find_valid<'a>(input_strs: &[String]) -> Option<SegmentMapping> {
        SegmentMapping::permutations().find(|sm| sm.is_valid(input_strs))
    }

    pub fn permutations() -> impl Iterator<Item = SegmentMapping> {
        [0, 1, 2, 3, 4, 5, 6]
            .iter()
            .permutations(7)
            .map(|v| SegmentMapping {
                mapping: v.into_iter().map(|&i| i.clone()).collect(),
            })
    }

    pub fn is_valid<'a>(&self, input_strs: &[String]) -> bool {
        input_strs.iter().all(|s| {
            SegmentDisplay::from_str_with_mapping(s, self)
                .to_int()
                .is_some()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_display_values() {
        assert_eq!(SegmentDisplay::from_str("abcefg").to_int().unwrap(), 0);
        assert_eq!(SegmentDisplay::from_str("fc").to_int().unwrap(), 1);
        assert_eq!(SegmentDisplay::from_str("edgca").to_int().unwrap(), 2);
        assert_eq!(SegmentDisplay::from_str("cdafg").to_int().unwrap(), 3);
        assert_eq!(SegmentDisplay::from_str("bcdf").to_int().unwrap(), 4);
        assert_eq!(SegmentDisplay::from_str("abgdf").to_int().unwrap(), 5);
        assert_eq!(SegmentDisplay::from_str("abdefg").to_int().unwrap(), 6);
        assert_eq!(SegmentDisplay::from_str("acf").to_int().unwrap(), 7);
        assert_eq!(SegmentDisplay::from_str("gfbdace").to_int().unwrap(), 8);
        assert_eq!(SegmentDisplay::from_str("abcdfg").to_int().unwrap(), 9);
    }
}
