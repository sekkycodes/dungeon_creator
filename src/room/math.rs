use std::{cmp::Ordering, ops::RangeInclusive};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct URect {
    pub row1: usize,
    pub row2: usize,
    pub col1: usize,
    pub col2: usize,
}

impl URect {
    pub fn new(row1: usize, row2: usize, col1: usize, col2: usize) -> Self {
        Self {
            row1,
            row2,
            col1,
            col2,
        }
    }

    // Returns true if this overlaps with other
    pub fn intersect(&self, other: &URect) -> bool {
        self.row1 <= other.row2
            && self.row2 >= other.row1
            && self.col1 <= other.col2
            && self.col2 >= other.col1
    }

    // Returns the center of the rectangle
    pub fn center(&self) -> UPosition {
        UPosition::new((self.row1 + self.row2) / 2, (self.col1 + self.col2) / 2)
    }

    pub fn rows(&self) -> RangeInclusive<usize> {
        (self.row1 as usize)..=(self.row2 as usize)
    }

    pub fn cols(&self) -> RangeInclusive<usize> {
        (self.col1 as usize)..=(self.col2 as usize)
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Rect {
    pub row: i32,
    pub height: usize,
    pub col: i32,
    pub width: usize,
}

impl Rect {
    pub fn new(row: i32, col: i32, height: usize, width: usize) -> Rect {
        Rect {
            row,
            height,
            col,
            width,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct UPosition {
    pub row: usize,
    pub col: usize,
}

impl UPosition {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

impl Default for UPosition {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

impl PartialOrd for UPosition {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for UPosition {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.row, self.col).cmp(&(other.row, other.col))
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
        let rect1 = URect::new(5, 8, 5, 8);
        let rect2 = URect::new(5, 8, 5, 8);
        let rect3 = URect::new(5, 5, 5, 5);
        let rect4 = URect::new(7, 9, 7, 9);
        let rect5 = URect::new(9, 9, 10, 10);

        assert!(rect1.intersect(&rect2)); // overlap due to same placement
        assert!(rect1.intersect(&rect3));
        assert!(rect1.intersect(&rect4));
        assert!(!rect1.intersect(&rect5));
    }

    #[test]
    fn rect_rows_returns_range_over_all_row_axis_coordinate_points() {
        let rect = URect::new(5, 8, 5, 7);
        assert_eq!(5..=8, rect.rows());
    }

    #[test]
    fn rect_cols_returns_range_over_all_col_axis_coordinate_points() {
        let rect = URect::new(5, 8, 5, 7);
        assert_eq!(5..=7, rect.cols());
    }

    #[test]
    fn rect_calculates_its_center_position() {
        let rect = URect::new(4, 6, 4, 6);
        assert_eq!(UPosition::new(5, 5), rect.center());
    }

    #[test]
    fn rect_rounds_down_center_position() {
        let rect = URect::new(4, 5, 4, 5);
        assert_eq!(UPosition::new(4, 4), rect.center());
    }
}
