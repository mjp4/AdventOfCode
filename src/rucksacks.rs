use std::collections::HashSet;
use std::convert::TryInto;
use std::iter::FromIterator;

struct Item(char);

struct Compartment(HashSet<char>);

struct Rucksack {
    one: Compartment,
    two: Compartment
}

impl Rucksack {
    fn from_list_of_contents(contents: &str) -> Rucksack {
        let compartment_length = contents.len() / 2;
        let (one_contents, two_contents) = contents.split_at(compartment_length);

        Rucksack {
            one: Compartment(HashSet::from_iter(one_contents.chars())),
            two: Compartment(HashSet::from_iter(two_contents.chars())),
        }
    }

    fn item_in_both_compartments(&self) -> Item {
        // Intersection of one and two
        self.one.intersection(self.two).first()
    }
}

impl Item {
    fn priority(&self) -> usize {
        (self as &u32).try_into().unwrap()
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority() {
        assert_eq!(Item('a').priority(), 1)
    }

    #[test]
    fn test_example_2022_day3() {
        let input_lines = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
            .lines()
            .map(|s| s.to_string());
        assert_eq!(
            input_lines
                .map(|s| Rucksack::from_list_of_contents(&s).item_in_both_compartments().priority())
                .sum::<usize>(),
            157
        );
    }
}
