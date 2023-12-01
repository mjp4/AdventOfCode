#![allow(dead_code, unused_imports)]
struct Valuemap {
    map: Vec<Vec<usize>>,
    height: usize,
    width: usize,
}

impl Valuemap {
    fn empty() -> Valuemap {
        Valuemap {
            map: Vec::new(),
            height: 0,
            width: 0,
        }
    }

    fn with_row_from_digits(self, new_row: &str) -> Valuemap {
        let new_values: Vec<usize> = new_row
            .chars()
            .filter_map(|c| c.to_digit(10).map(|i| i as usize))
            .collect();
        if self.width == 0 || self.width == new_values.len() {
            let mut map = self.map;
            map.push(new_values);
            Valuemap {
                map,
                height: self.height + 1,
                width: self.width,
            }
        } else {
            self
        }
    }

    fn get(&self, (row, column): (usize, usize)) -> Option<usize> {
        if row < self.height && column < self.width {
            Some(self.map[row][column])
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_value_map_creation() {
        /* This test was not completed
        assert_eq!(
            Valuemap::empty().with_row_from_digits("123").with_row_from_digits("324").get((1, 2)),
            Some(4)
            )
        */
    }
}
