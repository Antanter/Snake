use rand::seq::SliceRandom;
use std::collections::HashSet;
use ggez::graphics::{Color, DrawMode, DrawParam, Mesh, Canvas, Rect};
use ggez::{Context, GameResult};

#[derive(Clone, Copy, PartialEq)]
pub enum Cell {
    Empty,
    Snake,
    Food,
    Wall,
}

pub struct Map {
    width: usize,
    height: usize,
    cells: Vec<Vec<Cell>>,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        let cells = vec![vec![Cell::Empty; width + 1]; height + 1];
        Map { width, height, cells }
    }

    pub fn get_size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn set_cell(&mut self, x: usize, y: usize, cell: Cell) {
        self.cells[y][x] = cell;
    }

    pub fn rewrite_snake(&mut self, positions: &[(usize, usize)]) {
        for row in &mut self.cells { for cell in row { if *cell == Cell::Snake { *cell = Cell::Empty; } } }
        for &(x, y) in positions { self.set_cell(x, y, Cell::Snake); }
    }

    
    pub fn place_food(&mut self, forbidden_positions: &[(usize, usize)]) {
        let mut rng = rand::thread_rng();
        let mut available_positions: Vec<(usize, usize)> = Vec::new();
        let forbidden: HashSet<(usize, usize)> = forbidden_positions.iter().cloned().collect();

        for x in 0..self.width {
            for y in 0..self.height {
                if !forbidden.contains(&(x, y)) {
                    available_positions.push((x, y));
                }
            }
        }

        if let Some(&(x, y)) = available_positions.choose(&mut rng) {
            self.set_cell(x, y, Cell::Food);
        } 
    }

    pub fn render_graphics(&self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        let cell_size = 20.0;

        for (y, row) in self.cells.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let color = match cell {
                    Cell::Empty => Color::new(0.1, 0.1, 0.1, 1.0),
                    Cell::Snake => Color::GREEN,
                    Cell::Food => Color::RED,
                    Cell::Wall => Color::BLUE,
                };

                let rectangle = Rect::new(
                    x as f32 * cell_size,
                    y as f32 * cell_size,
                    cell_size,
                    cell_size,
                );

                let mesh = Mesh::new_rectangle(ctx, DrawMode::fill(), rectangle, color)?;
                canvas.draw(&mesh, DrawParam::default());
            }
        }

        Ok(())
    }
}