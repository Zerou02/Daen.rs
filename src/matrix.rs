use std::usize;

use crate::vector2::Vector2;

pub struct Matrix {
    width: i32,
    height: i32,
    matrix: Vec<Vec<f64>>,
}

impl Matrix {
    pub fn new(width: i32, height: i32) -> Matrix {
        let mut vec = vec![];
        for x in 0..height {
            let mut inner = vec![];
            for y in 0..width {
                inner.push(0.0);
            }
            vec.push(inner);
        }
        return Matrix {
            width,
            height,
            matrix: vec,
        };
    }

    pub fn getEntry(&self, i: i32, j: i32) -> f64 {
        return self.matrix[i as usize][j as usize];
    }

    pub fn getRow(&self, index: i32) -> Vec<f64> {
        return self.matrix[index as usize].clone();
    }

    pub fn setRow(&mut self, index: i32, row: Vec<f64>) {
        self.matrix[index as usize] = row;
    }
    pub fn addVec(&mut self, column: i32, vec: Vector2) {
        self.matrix[0][column as usize] = vec.x;
        self.matrix[1][column as usize] = vec.y;
    }

    pub fn setEntry(&mut self, row: i32, column: i32, entry: f64) {
        self.matrix[row as usize][column as usize] = entry;
    }

    pub fn getWidth(&self) -> i32 {
        return self.width;
    }

    pub fn getHeight(&self) -> i32 {
        return return self.height;
    }

    pub fn print(&self) {
        for x in &self.matrix {
            for y in x {
                print!("{} ", y);
            }
            println!();
        }
    }
}
