use itertools::Itertools;

pub struct LanternFish {
    birth_timer: usize,
    count: usize,
}

impl LanternFish {
    fn new(birth_timer: usize, count: usize) -> LanternFish {
        LanternFish { birth_timer, count }
    }

    fn next_day(self) -> Vec<LanternFish> {
        if self.birth_timer > 0 {
            vec![LanternFish::new(self.birth_timer - 1, self.count)]
        } else {
            vec![
                LanternFish::new(6, self.count),
                LanternFish::new(8, self.count),
            ]
        }
    }
}

pub struct LanternShoal {
    fish: Vec<LanternFish>,
}

impl LanternShoal {
    pub fn from_str(input_str: &str) -> LanternShoal {
        LanternShoal {
            fish: input_str
                .split(',')
                .flat_map(|s| s.parse::<usize>().ok())
                .map(|i| LanternFish::new(i, 1))
                .collect(),
        }
    }

    pub fn proceed_n_days(self, n: usize) -> LanternShoal {
        (0..n).fold(self, |ls, _| ls.next_day())
    }

    pub fn next_day(self) -> LanternShoal {
        let mut next_fish: Vec<LanternFish> =
            self.fish.into_iter().flat_map(|f| f.next_day()).collect();
        next_fish.sort_by(|lf1, lf2| lf1.birth_timer.cmp(&lf2.birth_timer));

        LanternShoal {
            fish: next_fish
                .into_iter()
                .coalesce(|lf1, lf2| {
                    if lf1.birth_timer == lf2.birth_timer {
                        Ok(LanternFish::new(lf1.birth_timer, lf1.count + lf2.count))
                    } else {
                        Err((lf1, lf2))
                    }
                })
                .collect(),
        }
    }

    pub fn count(&self) -> usize {
        self.fish.iter().map(|lf| lf.count).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_lantern_shoal() {
        assert_eq!(
            (0..18)
                .fold(LanternShoal::from_str("3,4,3,1,2"), |ls, _| ls.next_day())
                .count(),
            26
        );
        assert_eq!(
            (0..80)
                .fold(LanternShoal::from_str("3,4,3,1,2"), |ls, _| ls.next_day())
                .count(),
            5934
        );
        assert_eq!(
            (0..256)
                .fold(LanternShoal::from_str("3,4,3,1,2"), |ls, _| ls.next_day())
                .count(),
            26984457539
        );
    }
}
