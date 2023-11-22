use std::{
    fmt::Write as FmtWrite,
    io::Write,
    ops::{Index, IndexMut},
};

#[derive(Clone)]
pub struct FlatVec2D<T>(pub Vec<T>, pub usize, pub usize);

// Struct that returns a reference inside a FlatVec2D, with its coordinates.
pub struct Neighbor<T>(pub T, pub usize, pub usize);

impl<T> FlatVec2D<T> {
    pub fn new(xdim: usize, ydim: usize) -> Self
    where
        T: Default + Clone,
    {
        FlatVec2D(vec![T::default(); xdim * ydim], xdim, ydim)
    }

    /// Get all adjacent (including diagonal) neighbors, filtering for those out of bounds.
    pub fn get_neigh(&self, x: usize, y: usize) -> Vec<Neighbor<&T>> {
        let x = x as isize;
        let y = y as isize;
        #[rustfmt::skip]
        let move_mat = vec![
            (x - 1, y - 1), (x, y - 1), (x + 1, y - 1),
            (x - 1, y    ),             (x + 1, y    ),
            (x - 1, y + 1), (x, y + 1), (x + 1, y + 1),
        ];
        self.get_neigh_real(move_mat)
    }

    /// Get all cardinally adjacent neighbors (N, S, E, W), filtering for those out of bounds.
    pub fn get_neigh_cardinal(&self, x: usize, y: usize) -> Vec<Neighbor<&T>> {
        let x = x as isize;
        let y = y as isize;
        #[rustfmt::skip]
        let move_mat = vec![
                        (x, y - 1),
            (x - 1, y),             (x + 1, y),
                        (x, y + 1),
        ];
        self.get_neigh_real(move_mat)
    }

    fn get_neigh_real(&self, mut move_mat: Vec<(isize, isize)>) -> Vec<Neighbor<&T>> {
        move_mat
            .drain(..)
            .filter(|&(x, y)| {
                (x > -1 && x < (self.1 as isize)) && (y > -1 && y < (self.2 as isize))
            })
            .map(|(x, y)| Neighbor(&self[(x as usize, y as usize)], x as usize, y as usize))
            .collect::<Vec<Neighbor<&T>>>()
    }
}

impl FlatVec2D<u8> {
    pub fn write_pgm(&self, writable: &mut impl Write) -> std::io::Result<()> {
        let header = format!("P5\n{} {}\n127\n", self.1, self.2).into_bytes();
        writable.write_all(&header)?;
        writable.write_all(&self.0[..])?;
        Ok(())
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

impl<T: std::fmt::Debug> std::fmt::Debug for FlatVec2D<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.2 {
            for x in 0..self.1 {
                std::fmt::Debug::fmt(&self[(x, y)], f)?;
            }
            f.write_char('\n')?;
        }
        Ok(())
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
