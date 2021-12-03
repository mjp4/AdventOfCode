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
        let new_bitcount: Vec<usize> = self.bitcount.iter()
            .enumerate()
            .map(|(i, bc)| bc + ((bits & (1 << i)) >> i))
            .collect();

        BitAccumulator {
            count: self.count + 1,
            bitcount: new_bitcount,
        }
    }

    fn most_common_bits(&self) -> usize {
        self.bitcount.iter().enumerate()
            .map(|(i, bc)| if 2 * bc > self.count {
                1 << i
            } else {
                0
            })
        .sum()
    }

    fn least_common_bits(&self) -> usize {
        self.bitcount.iter().enumerate()
            .map(|(i, bc)| if 2 * bc < self.count {
                1 << i
            } else {
                0
            })
        .sum()
    }

    pub fn multiply_most_by_least(&self) -> usize {
        self.most_common_bits() * self.least_common_bits()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_accumulate() {
        assert_eq!(
            BitAccumulator::zeroed(5)
                .accumulate(0b00100)
                .accumulate(0b11110)
                .accumulate(0b10110)
                .accumulate(0b10111)
                .accumulate(0b10101)
                .accumulate(0b01111)
                .accumulate(0b00111)
                .accumulate(0b11100)
                .accumulate(0b10000)
                .accumulate(0b11001)
                .accumulate(0b00010)
                .accumulate(0b01010)
                .multiply_most_by_least(),
            198
        )
    }
}
