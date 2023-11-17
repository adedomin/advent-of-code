use std::ops::{Index, IndexMut};

pub struct FlatVec2D<T>(pub Vec<T>, pub usize, pub usize);

impl<T> FlatVec2D<T> {
    pub fn new(xdim: usize, ydim: usize) -> Self
    where
        T: Default + Clone,
    {
        FlatVec2D(vec![T::default(); xdim * ydim], xdim, ydim)
    }
}

impl<T> Index<(usize, usize)> for FlatVec2D<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (x, y) = index;
        &self.0[flat_coord(x, y, self.1)]
    }
}

impl<T> IndexMut<(usize, usize)> for FlatVec2D<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (x, y) = index;
        &mut self.0[flat_coord(x, y, self.1)]
    }
}

#[derive(Clone, Copy)]
pub enum Rot2D {
    None,
    Clock90,
    Clock180,
    Clock270,
}

pub fn flat_coord(x: usize, y: usize, dim: usize) -> usize {
    x + y * dim
}

pub fn flat_coord_rot(x: usize, y: usize, xdim: usize, ydim: usize, rot: Rot2D) -> usize {
    match rot {
        Rot2D::None => flat_coord(x, y, xdim),
        Rot2D::Clock90 => flat_coord((ydim - 1) - y, x, ydim),
        Rot2D::Clock180 => flat_coord((xdim - 1) - x, (ydim - 1) - y, xdim),
        Rot2D::Clock270 => flat_coord(y, (xdim - 1) - x, ydim),
    }
}

impl<T> Index<(usize, usize, Rot2D)> for FlatVec2D<T> {
    type Output = T;

    fn index(&self, index: (usize, usize, Rot2D)) -> &Self::Output {
        let (x, y, rot) = index;
        &self.0[flat_coord_rot(x, y, self.1, self.2, rot)]
    }
}

impl<T> IndexMut<(usize, usize, Rot2D)> for FlatVec2D<T> {
    fn index_mut(&mut self, index: (usize, usize, Rot2D)) -> &mut Self::Output {
        let (x, y, rot) = index;
        &mut self.0[flat_coord_rot(x, y, self.1, self.2, rot)]
    }
}

pub fn parse_to_flat2d<T>(input: &[u8]) -> FlatVec2D<T>
where
    T: Default + Clone + From<u8>,
{
    let row_width = input.iter().position(|&chr| chr == b'\n').unwrap();
    let col_len = ((input.len() - 1) / (row_width + 1)) + 1;

    let mut ret = FlatVec2D(vec![T::default(); row_width * col_len], row_width, col_len);

    let mut i = 0;
    let mut j = 0;
    input.iter().for_each(|&el| {
        if el == b'\n' {
            i = 0;
            j += 1;
        } else if el != b'\n' {
            ret[(i, j)] = el.into();
            i += 1;
        }
    });

    ret
}
