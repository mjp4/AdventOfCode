use itertools::Itertools;
use std::cmp;
use std::collections::HashMap;

#[derive(PartialEq, Debug)]
pub struct Coords {
    x: isize,
    y: isize,
}

impl Coords {
    pub fn xy(x: isize, y: isize) -> Coords {
        Coords { x, y }
    }

    #[allow(dead_code)]
    pub fn origin() -> Coords {
        Coords::xy(0, 0)
    }

    pub fn as_cartesian_tuple(&self) -> (isize, isize) {
        (self.x, self.y)
    }

    pub fn from_cartesian_tuple(xy: (isize, isize)) -> Coords {
        Coords::xy(xy.0, xy.1)
    }

    pub fn from_str(input_str: &str) -> Result<Coords, &str> {
        let tuple = input_str
            .split(',')
            .filter_map(|s| s.parse::<isize>().ok())
            .collect_tuple()
            .ok_or("Can't convert str to coords")?;
        Ok(Coords::from_cartesian_tuple(tuple))
    }
}

#[derive(Debug)]
pub struct LineSegment {
    end_1: Coords,
    end_2: Coords,
}

impl LineSegment {
    pub fn from_coords(c1: Coords, c2: Coords) -> LineSegment {
        LineSegment {
            end_1: c1,
            end_2: c2,
        }
    }

    pub fn from_str(input_str: &str) -> Result<LineSegment, &str> {
        let tuple: (Coords, Coords) = input_str
            .split(" -> ")
            .filter_map(|c| Coords::from_str(c).ok())
            .collect_tuple()
            .ok_or("Can't parse string into LineSegment")?;
        Ok(LineSegment::from_coords(tuple.0, tuple.1))
    }

    pub fn is_horiz(&self) -> bool {
        self.end_1.y == self.end_2.y
    }

    pub fn is_vert(&self) -> bool {
        self.end_1.x == self.end_2.x
    }

    pub fn is_45deg(&self) -> bool {
        let x_diff = self.end_1.x - self.end_2.x;
        let y_diff = self.end_1.y - self.end_2.y;
        x_diff * x_diff == y_diff * y_diff
    }

    pub fn coords(&self) -> Vec<Coords> {
        if self.is_horiz() {
            let min_x = cmp::min(self.end_1.x, self.end_2.x);
            let max_x = cmp::max(self.end_1.x, self.end_2.x);
            (min_x..=max_x)
                .map(|x| Coords::xy(x, self.end_1.y))
                .collect()
        } else if self.is_vert() {
            let min_y = cmp::min(self.end_1.y, self.end_2.y);
            let max_y = cmp::max(self.end_1.y, self.end_2.y);
            (min_y..=max_y)
                .map(|y| Coords::xy(self.end_1.x, y))
                .collect()
        } else if self.is_45deg() {
            let min_x = cmp::min(self.end_1.x, self.end_2.x);
            let max_x = cmp::max(self.end_1.x, self.end_2.x);
            let min_y = cmp::min(self.end_1.y, self.end_2.y);
            let max_y = cmp::max(self.end_1.y, self.end_2.y);

            let (y_start, y_step) = if min_x == self.end_1.x {
                // Left to Right from end_1 to end_2
                (
                    self.end_1.y,
                    (self.end_2.y - self.end_1.y) / (max_y - min_y),
                )
            } else {
                // Left to Right from end_2 to end_1
                (
                    self.end_2.y,
                    (self.end_1.y - self.end_2.y) / (max_y - min_y),
                )
            };
            (min_x..=max_x)
                .map(|x| Coords::xy(x, y_start + (x - min_x) * y_step))
                .collect()
        } else {
            println!("Ignoring {:?}", self);
            vec![]
        }
    }
}

impl PartialEq for LineSegment {
    fn eq(&self, other: &Self) -> bool {
        (self.end_1 == other.end_1 && self.end_2 == other.end_2)
            || (self.end_1 == other.end_2 && self.end_2 == other.end_1)
    }
}

pub struct GridCounter {
    grid: HashMap<(isize, isize), usize>,
}

impl GridCounter {
    pub fn new() -> GridCounter {
        GridCounter {
            grid: HashMap::new(),
        }
    }

    pub fn add_coords(self, coords: &Coords) -> GridCounter {
        let mut grid = self.grid;
        *grid.entry(coords.as_cartesian_tuple()).or_default() += 1;
        GridCounter { grid }
    }

    #[allow(dead_code)]
    pub fn get(&self, coords: &Coords) -> usize {
        *self.grid.get(&coords.as_cartesian_tuple()).unwrap_or(&0)
    }

    pub fn into_values(self) -> impl Iterator<Item = usize> {
        self.grid.into_values()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_str_parsing() {
        assert_eq!(
            Coords::from_str("2,3").unwrap().as_cartesian_tuple(),
            (2, 3)
        );
        assert_eq!(
            Coords::from_str("-12,0").unwrap().as_cartesian_tuple(),
            (-12, 0)
        );
    }

    #[test]
    fn check_line_segment_equality() {
        assert_eq!(
            LineSegment::from_coords(Coords::xy(2, 4), Coords::xy(5, 6)),
            LineSegment::from_coords(Coords::xy(2, 4), Coords::xy(5, 6)),
        );
        assert_eq!(
            LineSegment::from_coords(Coords::xy(2, 4), Coords::xy(5, 6)),
            LineSegment::from_coords(Coords::xy(5, 6), Coords::xy(2, 4)),
        );
        assert_ne!(
            LineSegment::from_coords(Coords::xy(2, 4), Coords::xy(5, 7)),
            LineSegment::from_coords(Coords::xy(5, 6), Coords::xy(2, 4)),
        );
    }

    #[test]
    fn check_iter_coords() {
        assert_eq!(
            LineSegment::from_str("1,1 -> 1,3")
                .unwrap()
                .coords()
                .iter()
                .map(|c| c.as_cartesian_tuple())
                .collect_vec(),
            vec![(1, 1), (1, 2), (1, 3)]
        );
    }

    #[test]
    fn check_grid_counter() {
        let grid_counter = GridCounter::new()
            .add_coords(&Coords::xy(5, 3))
            .add_coords(&Coords::xy(2, 1))
            .add_coords(&Coords::xy(5, 3))
            .add_coords(&Coords::xy(2, 1))
            .add_coords(&Coords::xy(2, 1))
            .add_coords(&Coords::xy(4, 7));
        assert_eq!(grid_counter.get(&Coords::xy(5, 3)), 2);
        assert_eq!(grid_counter.get(&Coords::xy(1, 1)), 0);
        assert_eq!(grid_counter.get(&Coords::xy(2, 1)), 3);
        assert_eq!(grid_counter.get(&Coords::xy(4, 7)), 1);
    }

    #[test]
    fn check_test_input() {
        assert_eq!(
            vec![
                "0,9 -> 5,9",
                "8,0 -> 0,8",
                "9,4 -> 3,4",
                "2,2 -> 2,1",
                "7,0 -> 7,4",
                "6,4 -> 2,0",
                "0,9 -> 2,9",
                "3,4 -> 1,4",
                "0,0 -> 8,8",
                "5,5 -> 8,2"
            ]
            .iter()
            .map(|seg_str| LineSegment::from_str(&seg_str).unwrap().coords())
            .flatten()
            .fold(GridCounter::new(), |gc, coords| gc.add_coords(&coords))
            .into_values()
            .filter(|&v| v > 1)
            .count(),
            12
        )
    }
}
