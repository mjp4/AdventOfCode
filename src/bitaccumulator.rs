pub struct DiagsReport {
    digit_count: usize,
    values: Vec<usize>,
}

impl DiagsReport {
    pub fn new(digit_count: usize, values: Vec<usize>) -> DiagsReport {
        DiagsReport {
            digit_count: digit_count,
            values: values,
        }
    }

    pub fn from_iter(digit_count: usize, iter: impl Iterator<Item = usize>) -> DiagsReport {
        DiagsReport::new(digit_count, iter.collect())
    }

    pub fn cloned(&self) -> DiagsReport {
        DiagsReport::from_iter(self.digit_count, self.iter_values().cloned())
    }

    fn accumulate_bits(&self) -> BitAccumulator {
        self.values
            .iter()
            .fold(BitAccumulator::zeroed(self.digit_count), |ba, bits| {
                ba.accumulate(*bits)
            })
    }

    fn iter_values(&self) -> impl Iterator<Item = &usize> {
        self.values.iter()
    }

    pub fn gamma_rate(&self) -> usize {
        self.accumulate_bits().most_common_bits()
    }

    pub fn epsilon_rate(&self) -> usize {
        self.accumulate_bits().least_common_bits()
    }

    pub fn oxygen_rate(&self) -> usize {
        (0..self.digit_count)
            .rev()
            .fold::<DiagsReport, _>(self.cloned(), |filtered_report, index| {
                let ba = filtered_report.accumulate_bits();
                if filtered_report.values.len() == 1 {
                    DiagsReport::new(index, filtered_report.values)
                } else {
                    DiagsReport::from_iter(
                        index,
                        filtered_report
                            .iter_values()
                            .filter(|v| bit_match_at_index(index, ba.most_common_bits(), **v))
                            .cloned(),
                    )
                }
            })
            .values[0]
    }

    pub fn co2_scrub_rate(&self) -> usize {
        (0..self.digit_count)
            .rev()
            .fold::<DiagsReport, _>(self.cloned(), |filtered_report, index| {
                let ba = filtered_report.accumulate_bits();
                if filtered_report.values.len() == 1 {
                    DiagsReport::new(index, filtered_report.values)
                } else {
                    DiagsReport::from_iter(
                        index,
                        filtered_report
                            .iter_values()
                            .filter(|v| bit_match_at_index(index, ba.least_common_bits(), **v))
                            .cloned(),
                    )
                }
            })
            .values[0]
    }
}

#[derive(PartialEq, Debug)]
pub struct BitAccumulator {
    count: usize,
    bitcount: Vec<usize>,
}

impl BitAccumulator {
    pub fn zeroed(len: usize) -> BitAccumulator {
        BitAccumulator {
            count: 0,
            bitcount: vec![0; len],
        }
    }

    pub fn accumulate(&self, bits: usize) -> BitAccumulator {
        let new_bitcount: Vec<usize> = self
            .bitcount
            .iter()
            .enumerate()
            .map(|(i, bc)| bc + ((bits & (1 << i)) >> i))
            .collect();

        BitAccumulator {
            count: self.count + 1,
            bitcount: new_bitcount,
        }
    }

    pub fn most_common_bits(&self) -> usize {
        self.bitcount
            .iter()
            .enumerate()
            .map(|(i, bc)| if 2 * bc >= self.count { 1 << i } else { 0 })
            .sum()
    }

    pub fn least_common_bits(&self) -> usize {
        self.bitcount
            .iter()
            .enumerate()
            .map(|(i, bc)| if 2 * bc >= self.count { 0 } else { 1 << i })
            .sum()
    }
}

fn bit_match_at_index(index: usize, value_one: usize, value_two: usize) -> bool {
    let bit_mask: usize = 1 << index;
    (value_one & bit_mask) == (value_two & bit_mask)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_diags_rates() {
        let dg = DiagsReport::new(
            5,
            vec![
                0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000,
                0b11001, 0b00010, 0b01010,
            ],
        );
        assert_eq!(dg.gamma_rate(), 22);
        assert_eq!(dg.epsilon_rate(), 9);
        assert_eq!(dg.oxygen_rate(), 23);
        assert_eq!(dg.co2_scrub_rate(), 10);
    }
}
