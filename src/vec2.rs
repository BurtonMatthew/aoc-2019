#![allow(dead_code)]
use std::ops::{Index, IndexMut};

pub struct Vec2<T>
{
    data: Vec<T>,
    width: usize
}

impl<T> Index<usize> for Vec2<T>
{
    type Output = [T];

    fn index(&self, row: usize) -> &[T]
    {
        assert!(row < self.height());
        let row_start = row * self.width;
        &self.data[row_start .. row_start + self.width]
    }
}

impl<T> Index<(usize, usize)> for Vec2<T>
{
    type Output = T;

    fn index(&self, pos: (usize, usize)) -> &T
    {
        &self[pos.0][pos.1]
    }
}

impl<T> IndexMut<usize> for Vec2<T>
{
    fn index_mut(&mut self, row: usize) -> &mut [T]
    {
        assert!(row < self.height());
        let row_start = row * self.width;
        &mut self.data[row_start .. row_start + self.width]
    }
}

impl<T> IndexMut<(usize, usize)> for Vec2<T>
{
    fn index_mut(&mut self, pos: (usize, usize)) -> &mut T
    {
        &mut self[pos.0][pos.1]
    }
}

impl<T> Vec2<T>
{
    pub fn new() -> Self
    {
        Self { data: Vec::new(), width: 0 }
    }

    pub fn with_capacity(width: usize, height: usize) -> Self
    {
        Self { data: Vec::with_capacity(width * height), width: width }
    }

    pub fn from_vec(data : Vec<T>, width: usize) -> Self
    {
        assert!(data.len() % width == 0);
        Self { data: data, width: width }
    }

    pub fn width(&self) -> usize
    {
        self.width
    }

    pub fn height(&self) -> usize
    {
        self.data.len() / self.width
    }

    pub fn push_row(&mut self, row: Vec<T>) 
    {
        if self.width > 0 
        {
            assert_eq!(self.width, row.len());
        } 
        else 
        {
            self.width = row.len();
        }
        self.data.extend(row);
    }
}