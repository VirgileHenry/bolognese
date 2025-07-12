use crate::common::{Position, Size};

pub trait GridElement {
    fn size(&self) -> Size;
    fn display_char(&self) -> char;
}

#[derive(Debug)]
pub enum GridError {
    OutOfBound,
    InsertionWouldOverlap,
}

impl core::fmt::Display for GridError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl core::error::Error for GridError {}

impl core::fmt::Display for Position {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

enum GridCell<T: GridElement> {
    Empty,
    Cell(T),
    RefCell(Position),
}

pub struct Grid<T: GridElement> {
    width: usize,
    height: usize,
    cells: Vec<GridCell<T>>,
}

impl<T: GridElement> Grid<T> {
    pub fn new(width: usize, height: usize) -> Self {
        let mut cells = Vec::with_capacity(width * height);
        for _ in 0..width * height {
            cells.push(GridCell::Empty);
        }
        Grid {
            width,
            height,
            cells,
        }
    }

    pub fn from_fn<F: Fn(usize, usize) -> T>(width: usize, height: usize, f: F) -> Self {
        let mut cells = Vec::with_capacity(width * height);
        for y in 0..height {
            for x in 0..width {
                cells.push(GridCell::Cell(f(x, y)));
            }
        }
        Grid {
            width,
            height,
            cells,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn display(&self) {
        let mut result = String::with_capacity(self.width * self.height);
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = Position::new(x, y);
                match self.get(pos) {
                    Some(data) => result.push(data.display_char()),
                    None => result.push(' '),
                }
            }
            result.push('\n');
        }
        println!("{result}");
    }

    pub fn insert(&mut self, top_left: Position, data: T) -> Result<(), GridError> {
        if top_left.x + data.size().width.get() > self.width {
            return Err(GridError::OutOfBound);
        }
        if top_left.y + data.size().height.get() > self.height {
            return Err(GridError::OutOfBound);
        }

        for i in top_left.x..top_left.x + data.size().width.get() {
            for j in top_left.y..top_left.y + data.size().height.get() {
                match &self.cells[j * self.width + i] {
                    GridCell::Empty => {}
                    _ => return Err(GridError::InsertionWouldOverlap),
                }
            }
        }

        for i in top_left.x..top_left.x + data.size().width.get() {
            for j in top_left.y..top_left.y + data.size().height.get() {
                self.cells[j * self.width + i] = GridCell::RefCell(top_left);
            }
        }

        self.cells[top_left.y * self.width + top_left.x] = GridCell::Cell(data);

        Ok(())
    }

    pub fn get(&self, pos: Position) -> Option<&T> {
        match self.cells.get(pos.y * self.width + pos.x)? {
            GridCell::Empty => None,
            GridCell::Cell(data) => Some(data),
            GridCell::RefCell(ref_index) => {
                match self.cells.get(ref_index.y * self.width + pos.y) {
                    Some(GridCell::Cell(data)) => Some(data),
                    _ => panic!(
                        "Invalid grid layout: ref cell at {pos} points to invalid cell at {ref_index}"
                    ),
                }
            }
        }
    }

    pub fn cells(&self) -> impl Iterator<Item = &T> {
        self.cells
            .iter()
            .filter(|cell| match cell {
                GridCell::Cell(_) => true,
                _ => false,
            })
            .map(|cell| match cell {
                GridCell::Cell(cell) => cell,
                _ => unreachable!(),
            })
    }

    pub fn cells_mut(&mut self) -> impl Iterator<Item = (Position, &mut T)> {
        self.cells
            .iter_mut()
            .enumerate()
            .filter(|(_, cell)| match cell {
                GridCell::Cell(_) => true,
                _ => false,
            })
            .map(|(index, cell)| match cell {
                GridCell::Cell(cell) => {
                    (Position::new(index % self.width, index / self.width), cell)
                }
                _ => unreachable!(),
            })
    }
}
