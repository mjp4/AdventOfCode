pub struct LanternFish {
    birth_timer: usize,
}

impl LanternFish {
    fn new(birth_timer: usize) -> LanternFish {
        LanternFish { birth_timer }
    }

    fn next_day(self) -> Vec<LanternFish> {
        if self.birth_timer > 0 {
            vec![LanternFish::new(self.birth_timer - 1)]
        } else {
            vec![LanternFish::new(6), LanternFish::new(8)]
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
                .map(|i| LanternFish::new(i))
                .collect(),
        }
    }

    pub fn next_day(self) -> LanternShoal {
        LanternShoal {
            fish: self.fish.into_iter().flat_map(|f| f.next_day()).collect(),
        }
    }

    pub fn count(&self) -> usize {
        self.fish.iter().count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_lantern_shoal() {
        assert_eq!(
            (0..18)
                .fold(LanternShoal::from_str("3,4,3,1,2"), |ls, day| ls.next_day())
                .count(),
            26
        );
        assert_eq!(
            (0..80)
                .fold(LanternShoal::from_str("3,4,3,1,2"), |ls, day| ls.next_day())
                .count(),
            5934
        );
    }
}
