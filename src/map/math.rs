use std::{cmp::Ordering, ops::RangeInclusive};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Rect {
    pub row1: i32,
    pub row2: i32,
    pub col1: i32,
    pub col2: i32,
}

impl Rect {
    pub fn new(row1: i32, row2: i32, col1: i32, col2: i32) -> Self {
        Self {
            row1,
            row2,
            col1,
            col2,
        }
    }

    // Returns true if this overlaps with other
    pub fn intersect(&self, other: &Rect) -> bool {
        self.row1 <= other.row2
            && self.row2 >= other.row1
            && self.col1 <= other.col2
            && self.col2 >= other.col1
    }

    // Returns the center of the rectangle
    pub fn center(&self) -> Position {
        Position::new((self.row1 + self.row2) / 2, (self.col1 + self.col2) / 2)
    }

    pub fn rows(&self) -> RangeInclusive<usize> {
        (self.row1 as usize)..=(self.row2 as usize)
    }

    pub fn cols(&self) -> RangeInclusive<usize> {
        (self.col1 as usize)..=(self.col2 as usize)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Position {
    pub row: i32,
    pub col: i32,
}

impl Position {
    pub fn new(row: i32, col: i32) -> Self {
        Self { row, col }
    }
}

impl Default for Position {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.row, self.col).cmp(&(other.row, other.col))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Dimension {
    pub vertical: usize,
    pub horizontal: usize,
}

impl Dimension {
    pub fn new(vertical: usize, horizontal: usize) -> Self {
        Self {
            vertical,
            horizontal,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn rect_intersect_checks_whether_another_rect_overlaps() {
        let rect1 = Rect::new(5, 8, 5, 8);
        let rect2 = Rect::new(5, 8, 5, 8);
        let rect3 = Rect::new(5, 5, 5, 5);
        let rect4 = Rect::new(7, 9, 7, 9);
        let rect5 = Rect::new(9, 9, 10, 10);

        assert!(rect1.intersect(&rect2)); // overlap due to same placement
        assert!(rect1.intersect(&rect3));
        assert!(rect1.intersect(&rect4));
        assert!(!rect1.intersect(&rect5));
    }

    #[test]
    fn rect_rows_returns_range_over_all_row_axis_coordinate_points() {
        let rect = Rect::new(5, 8, 5, 7);
        assert_eq!(5..=8, rect.rows());
    }

    #[test]
    fn rect_cols_returns_range_over_all_col_axis_coordinate_points() {
        let rect = Rect::new(5, 8, 5, 7);
        assert_eq!(5..=7, rect.cols());
    }
}