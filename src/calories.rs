#[derive(Debug, PartialEq)]
pub struct CalorieCounter {
    third_max: usize,
    second_max: usize,
    pub max: usize,
    current_counter: usize,
}

impl CalorieCounter {
    pub fn new(
        third_max: usize,
        second_max: usize,
        max: usize,
        current_counter: usize,
    ) -> CalorieCounter {
        CalorieCounter {
            third_max,
            second_max,
            max,
            current_counter,
        }
    }

    pub fn reset() -> CalorieCounter {
        CalorieCounter::new(0, 0, 0, 0)
    }

    pub fn with_next(&self, input: &str) -> CalorieCounter {
        if input.is_empty() {
            CalorieCounter::new(self.third_max, self.second_max, self.max, 0)
        } else {
            let input_int: usize = input.parse().unwrap();
            let new_count = self.current_counter + input_int;
            if new_count <= self.third_max {
                CalorieCounter::new(self.third_max, self.second_max, self.max, new_count)
            } else if new_count <= self.second_max {
                CalorieCounter::new(new_count, self.second_max, self.max, new_count)
            } else if new_count <= self.max {
                CalorieCounter::new(self.second_max, new_count, self.max, new_count)
            } else {
                CalorieCounter::new(self.second_max, self.max, new_count, new_count)
            }
        }
    }

    pub fn fold_step(last_counter: CalorieCounter, new_input: String) -> CalorieCounter {
        last_counter.with_next(&new_input)
    }

    pub fn top_three_sum(&self) -> usize {
        self.max + self.second_max + self.third_max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_next() {
        let cc = CalorieCounter::new(3, 4, 12, 3);
        assert_eq!(cc.with_next("4"), CalorieCounter::new(4, 7, 12, 7));
        assert_eq!(cc.with_next(""), CalorieCounter::new(3, 4, 12, 0));
        assert_eq!(cc.with_next("14"), CalorieCounter::new(4, 12, 17, 17));
    }

    #[test]
    fn day_1_2022_example() {
        let input_lines = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"
            .lines();
        assert_eq!(
            input_lines
                .map(|s| s.to_string())
                .fold(CalorieCounter::reset(), CalorieCounter::fold_step)
                .max,
            24000
        )
    }
    #[test]
    fn day_1_2022_bossmc_example() {
        let input_lines = "100

200

50
50
50
50
50
50"
            .lines();
        assert_eq!(
            input_lines
                .map(|s| s.to_string())
                .fold(CalorieCounter::reset(), CalorieCounter::fold_step)
                .top_three_sum(),
            600
        )
    }

}
