pub struct FloorGrid {
    heights: Vec<Vec<usize>>,
    widths: Vec<Vec<usize>>,
    left_pads: Vec<Vec<usize>>,
    top_pads: Vec<Vec<usize>>,
    max_heights: Vec<usize>,
    max_widths: Vec<usize>,
}

impl FloorGrid {
    pub fn insert(&mut self, cell: FloorCell) {
        self.heights[cell.row][cell.col] = cell.height;
        self.widths[cell.row][cell.col] = cell.width;

        if self.max_heights[cell.row] < cell.height {
            self.max_heights[cell.row] = cell.height;
            self.set_top_paddings(cell.row);
        }

        if self.max_widths[cell.col] < cell.width {
            self.max_widths[cell.col] = cell.width;
            self.set_left_paddings(cell.col);
        }
    }

    fn set_top_paddings(&mut self, row: usize) {
        let height = self.max_heights[row];

        for i in 0..self.heights.len() {
            if self.heights[row][i] > 0 {
                self.top_pads[row][i] = (height - self.heights[row][i]) / 2;
            } else {
                self.top_pads[row][i] = height;
            }
        }
    }

    fn set_left_paddings(&mut self, col: usize) {
        let width = self.max_widths[col];

        for i in 0..self.widths.len() {
            if self.widths[i][col] > 0 {
                self.left_pads[i][col] = (width - self.widths[i][col]) / 2;
            } else {
                self.left_pads[i][col] = width;
            }
        }
    }

    pub fn new(rows: usize, cols: usize) -> FloorGrid {
        FloorGrid {
            heights: vec![vec![0; cols]; rows],
            widths: vec![vec![0; cols]; rows],
            left_pads: vec![vec![0; cols]; rows],
            top_pads: vec![vec![0; cols]; rows],
            max_heights: vec![0; rows],
            max_widths: vec![0; cols],
        }
    }
}

pub struct FloorCell {
    col: usize,
    row: usize,
    height: usize,
    width: usize,
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
    pub fn floor_grid_calculates_padding_of_inserted_room() {
        let mut fg = FloorGrid::new(2, 2);

        fg.insert(FloorCell {
            col: 1,
            row: 0,
            height: 3,
            width: 5,
        });

        assert_eq!(0, fg.top_pads[1][0]);
        assert_eq!(0, fg.left_pads[0][0]);
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
}
