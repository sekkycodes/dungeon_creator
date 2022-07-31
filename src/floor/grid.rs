/// A dungeon floor consists of a collection of rooms with 3 dimensional coordinates.
/// The FloorGrid helps arrange these rooms into a grid-like format with horizontal and vertical paddings.
/// Example: If you have two rooms in the same column in row 0 and 1 with width 3 and 5 respectively, adding both to the grid will give you:
/// * widths: [0][0] = 3, [0][1] = 5
/// * max_widths [0] = 5
/// * left_pads[0][0] = 1, [0][1] = 0
/// Same logic applies to rows and heights / top paddings as well.
///
/// FloorCells can have negative row and column coordinates.
/// FloorGrid uses the respective offsets to shift them into a positive range.
/// This way the column and row coordinates can be used as a vector index.
pub struct FloorGrid {
    pub row_offset: i32,
    pub col_offset: i32,
    pub heights: Vec<Vec<usize>>,
    pub widths: Vec<Vec<usize>>,
    pub left_pads: Vec<Vec<usize>>,
    pub top_pads: Vec<Vec<usize>>,
    pub max_heights: Vec<usize>,
    pub max_widths: Vec<usize>,
}

impl Default for FloorGrid {
    fn default() -> Self {
        Self {
            row_offset: Default::default(),
            col_offset: Default::default(),
            heights: Default::default(),
            widths: Default::default(),
            left_pads: Default::default(),
            top_pads: Default::default(),
            max_heights: Default::default(),
            max_widths: Default::default(),
        }
    }
}

impl FloorGrid {
    /// Insert a cell into the grid.
    /// Doing this will re-calculate left paddings for rooms in the same column, and top paddings for rooms in the same row.
    /// Adding cells with negative row and col coordinates will only work, if the grid is set up with a positive offset bringing the coordinates into a positive range.
    pub fn insert(&mut self, cell: FloorCell) {
        let row = (cell.row + self.row_offset) as usize;
        let col = (cell.col + self.col_offset) as usize;

        self.heights[row][col] = cell.height;
        self.widths[row][col] = cell.width;

        if self.max_heights[row] < cell.height {
            self.max_heights[row] = cell.height;
            self.set_top_paddings(row);
        }
        self.set_top_padding(row, col, self.max_heights[row]);

        if self.max_widths[col] < cell.width {
            self.max_widths[col] = cell.width;
            self.set_left_paddings(col);
        }
        self.set_left_padding(row, col, self.max_widths[col]);
    }

    fn set_top_paddings(&mut self, row: usize) {
        let height = self.max_heights[row];

        for i in 0..self.heights[row].len() {
            self.set_top_padding(row, i, height);
        }
    }

    fn set_top_padding(&mut self, row: usize, col: usize, max_height: usize) {
        if self.heights[row][col] > 0 {
            self.top_pads[row][col] = (max_height - self.heights[row][col]) / 2;
        } else {
            self.top_pads[row][col] = max_height;
        }
    }

    fn set_left_paddings(&mut self, col: usize) {
        let width = self.max_widths[col];

        for i in 0..self.widths.len() {
            self.set_left_padding(i, col, width);
        }
    }

    fn set_left_padding(&mut self, row: usize, col: usize, max_width: usize) {
        if self.widths[row][col] > 0 {
            self.left_pads[row][col] = (max_width - self.widths[row][col]) / 2;
        } else {
            self.left_pads[row][col] = max_width;
        }
    }

    /// Create a new FloorGrid instance.
    /// Total rows and columns need to be set for the grid.
    /// Trying to add cells with coordinates outside the range will panic.
    /// Set the row and column offset if you need to bring the cells into the index range of the vectors.
    pub fn new(rows: usize, cols: usize) -> FloorGrid {
        FloorGrid {
            heights: vec![vec![0; cols]; rows],
            widths: vec![vec![0; cols]; rows],
            left_pads: vec![vec![0; cols]; rows],
            top_pads: vec![vec![0; cols]; rows],
            max_heights: vec![0; rows],
            max_widths: vec![0; cols],
            ..Default::default()
        }
    }
}

pub struct FloorCell {
    pub col: i32,
    pub row: i32,
    pub height: usize,
    pub width: usize,
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    pub fn floor_grid_inserts_dimensions() {
        let mut fg = FloorGrid::new(2, 2);

        fg.insert(FloorCell {
            col: 0,
            row: 1,
            height: 3,
            width: 5,
        });

        assert_eq!(3, fg.heights[1][0]);
        assert_eq!(5, fg.widths[1][0]);
    }

    #[test]
    pub fn floor_grid_adjusts_paddings_of_empty_rooms_on_insert() {
        let mut fg = FloorGrid::new(2, 2);

        fg.insert(FloorCell {
            col: 1,
            row: 0,
            height: 3,
            width: 5,
        });

        assert_eq!(3, fg.top_pads[0][0]);
        assert_eq!(5, fg.left_pads[1][1]);
    }

    #[test]
    pub fn floor_grid_calculates_padding_of_inserted_room_same_sized() {
        let mut fg = FloorGrid::new(2, 2);

        fg.insert(FloorCell {
            col: 1,
            row: 0,
            height: 3,
            width: 3,
        });
        fg.insert(FloorCell {
            col: 0,
            row: 0,
            height: 3,
            width: 3,
        });
        fg.insert(FloorCell {
            col: 0,
            row: 1,
            height: 3,
            width: 3,
        });

        // check room cell paddings
        assert_eq!(0, fg.top_pads[0][1]);
        assert_eq!(0, fg.left_pads[0][1]);
        assert_eq!(0, fg.top_pads[0][0]);
        assert_eq!(0, fg.left_pads[0][0]);
        assert_eq!(0, fg.top_pads[1][0]);
        assert_eq!(0, fg.left_pads[1][0]);
        // check empty cell paddings
        assert_eq!(3, fg.top_pads[1][1]);
        assert_eq!(3, fg.left_pads[1][1]);
    }

    #[test]
    pub fn floor_grid_adjusts_padding_of_other_filled_rooms_on_insert() {
        let mut fg = FloorGrid::new(2, 2);

        fg.insert(FloorCell {
            col: 1,
            row: 0,
            height: 3,
            width: 3,
        });
        fg.insert(FloorCell {
            col: 0,
            row: 1,
            height: 5,
            width: 5,
        });
        fg.insert(FloorCell {
            col: 0,
            row: 0,
            height: 7,
            width: 7,
        });

        assert_eq!(2, fg.top_pads[0][1]);
        assert_eq!(1, fg.left_pads[1][0]);
    }

    #[test]
    pub fn floor_grid_recognizes_larger_dimensions() {
        let mut fg = FloorGrid::new(2, 2);

        fg.insert(FloorCell {
            col: 1,
            row: 0,
            height: 3,
            width: 5,
        });

        assert_eq!(3, fg.max_heights[0]);
        assert_eq!(0, fg.max_heights[1]);
        assert_eq!(0, fg.max_widths[0]);
        assert_eq!(5, fg.max_widths[1]);
    }

    #[test]
    pub fn inserts_into_grid_with_positive_row_offset() {
        let mut fg = FloorGrid::new(3, 3);
        fg.row_offset = 2;

        fg.insert(FloorCell {
            col: 1,
            row: -1,
            height: 3,
            width: 3,
        });

        assert_eq!(3, fg.heights[1][1]);
        assert_eq!(3, fg.widths[1][1]);
    }

    #[test]
    pub fn inserts_into_grid_with_negative_col_offset() {
        let mut fg = FloorGrid::new(3, 3);
        fg.col_offset = -2;

        fg.insert(FloorCell {
            col: 3,
            row: 1,
            height: 3,
            width: 3,
        });

        assert_eq!(3, fg.heights[1][1]);
        assert_eq!(3, fg.widths[1][1]);
    }
}
