use itertools::FoldWhile;
use itertools::FoldWhile::{Continue, Done};

pub fn fold_step(inputs: &[usize], old_cost: usize, new_position: usize) -> FoldWhile<usize> {
    let new_cost = inputs
        .iter()
        .map(|&crab_position| movement_cost(crab_position, new_position))
        .sum();
    if new_cost > old_cost {
        // Divide by two once instead of during each cost calculation
        Done(old_cost / 2)
    } else {
        Continue(new_cost)
    }
}

fn movement_cost(crab_position: usize, aligned_position: usize) -> usize {
    let diff = if crab_position <= aligned_position {
        aligned_position - crab_position
    } else {
        crab_position - aligned_position
    };
    diff * (diff + 1)
}
