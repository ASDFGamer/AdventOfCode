use std::fmt;

pub struct Grid {
    pub rows: usize,
    pub columns: usize,
}

pub fn get_dimensions(str_grid: &[&str], validate_grid: bool) -> Grid {
    let rows = str_grid.get(0).unwrap_or(&"").len();
    let grid = Grid {
        rows,
        columns: str_grid.len(),
    };
    if validate_grid {
        for row in str_grid {
            assert_eq!(rows, row.len())
        }
    }
    return grid;
}

#[derive(Clone, Copy)]
pub struct GridLocation {
    pub row: usize,
    pub column: usize,
}

impl fmt::Display for GridLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.row, self.column)
    }
}

impl PartialEq for GridLocation {
    fn eq(&self, other: &GridLocation) -> bool {
        return self.row == other.row && self.column == other.column;
    }
}

pub fn get_neighbors(location: &GridLocation, grid: &Grid) -> Vec<GridLocation> {
    let mut result = Vec::new();
    if location.row > 0 {
        result.push(GridLocation {
            row: location.row - 1,
            column: location.column,
        });
        if location.column > 0 {
            result.push(GridLocation {
                row: location.row - 1,
                column: location.column - 1,
            })
        }
        if location.column < grid.columns {
            result.push(GridLocation {
                row: location.row - 1,
                column: location.column + 1,
            })
        }
    }
    if location.row < grid.rows {
        result.push(GridLocation {
            row: location.row + 1,
            column: location.column,
        });
        if location.column > 0 {
            result.push(GridLocation {
                row: location.row + 1,
                column: location.column - 1,
            })
        }
        if location.column < grid.columns {
            result.push(GridLocation {
                row: location.row + 1,
                column: location.column + 1,
            })
        }
    }
    if location.column > 0 {
        result.push(GridLocation {
            row: location.row,
            column: location.column - 1,
        });
    }
    if location.column < grid.columns {
        result.push(GridLocation {
            row: location.row,
            column: location.column + 1,
        });
    }

    return result;
}
