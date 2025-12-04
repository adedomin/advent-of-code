use std::{
    fmt::Write as FmtWrite,
    io::Write,
    // iter::FusedIterator,
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

    /// Try and get an index
    pub fn get(&self, (x, y): (usize, usize)) -> Option<&T> {
        if self.in_bounds(x as isize, y as isize) {
            Some(&self[(x, y)])
        } else {
            None
        }
    }

    /// Try and get a mutable index
    pub fn get_mut(&mut self, (x, y): (usize, usize)) -> Option<&mut T> {
        if self.in_bounds(x as isize, y as isize) {
            Some(&mut self[(x, y)])
        } else {
            None
        }
    }

    /// Try and get an index, allowing for user calculations that could be negative.
    pub fn get_isize(&self, (x, y): (isize, isize)) -> Option<&T> {
        if self.in_bounds(x, y) {
            Some(&self[(x as usize, y as usize)])
        } else {
            None
        }
    }

    /// Try and get an index, allowing for user calculations that could be negative.
    pub fn get_isize_mut(&mut self, (x, y): (isize, isize)) -> Option<&mut T> {
        if self.in_bounds(x, y) {
            Some(&mut self[(x as usize, y as usize)])
        } else {
            None
        }
    }

    pub fn get_neigh_iter(&self, (x, y): (usize, usize)) -> impl Iterator<Item = Neighbor<&T>> {
        let x = x as isize;
        let y = y as isize;
        #[rustfmt::skip]
        let move_mat: [(isize, isize); 8] = [
            (x - 1, y - 1), (x, y - 1), (x + 1, y - 1),
            (x - 1, y    ),             (x + 1, y    ),
            (x - 1, y + 1), (x, y + 1), (x + 1, y + 1),
        ];
        self.get_neigh_iter_real(move_mat)
    }

    pub fn get_neigh_card_iter(
        &self,
        (x, y): (usize, usize),
    ) -> impl Iterator<Item = Neighbor<&T>> {
        let x = x as isize;
        let y = y as isize;
        #[rustfmt::skip]
        let move_mat: [(isize, isize); 4] = [
                            (x, y - 1),
            (x - 1, y    ),             (x + 1, y    ),
                            (x, y + 1),
        ];
        self.get_neigh_iter_real(move_mat)
    }

    pub fn get_neigh_iter_real<const N: usize>(
        &self,
        move_mat: [(isize, isize); N],
    ) -> impl Iterator<Item = Neighbor<&T>> {
        move_mat
            .into_iter()
            .filter_map(|(x, y)| self.to_bounds(x, y))
            .map(|(x, y)| Neighbor(&self[(x, y)], x, y))
    }

    pub fn pad_in_bounds(&self, x: usize, y: usize) -> bool {
        (1..self.1 - 1).contains(&x) && (1..self.2 - 1).contains(&y)
    }

    pub fn in_bounds(&self, x: isize, y: isize) -> bool {
        (x > -1 && x < (self.1 as isize)) && (y > -1 && y < (self.2 as isize))
    }

    pub fn to_bounds(&self, x: isize, y: isize) -> Option<(usize, usize)> {
        self.in_bounds(x, y).then_some((x as usize, y as usize))
    }

    pub fn xrange(&self) -> std::ops::Range<usize> {
        0..self.1
    }

    pub fn pad_xrange(&self) -> std::ops::Range<usize> {
        1..self.1 - 1
    }

    pub fn yrange(&self) -> std::ops::Range<usize> {
        0..self.2
    }

    pub fn pad_yrange(&self) -> std::ops::Range<usize> {
        1..self.2 - 1
    }

    pub fn xyrange(&self) -> impl Iterator<Item = (usize, usize)> + use<'_, T> {
        (0..self.2).flat_map(|y| (0..self.1).map(move |x| (x, y)))
    }

    pub fn pad_xyrange(&self) -> impl Iterator<Item = (usize, usize)> + use<'_, T> {
        self.pad_yrange()
            .flat_map(|y| self.pad_xrange().map(move |x| (x, y)))
    }

    pub fn swap(&mut self, (x, y): (usize, usize), (ox, oy): (usize, usize)) {
        self.0
            .swap(flat_coord(x, y, self.1), flat_coord(ox, oy, self.1));
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

// Return a slice of the underlying vector, for a given row.
// This could improve performance over potentially costly indexing operations.
// impl<T> Index<usize> for FlatVec2D<T> {
//     type Output = [T];

//     fn index(&self, y: usize) -> &Self::Output {
//         let off = y * self.1;
//         let end = off + self.1;
//         &self.0[off..end]
//     }
// }

// impl<T> IndexMut<usize> for FlatVec2D<T> {
//     fn index_mut(&mut self, y: usize) -> &mut Self::Output {
//         let off = y * self.1;
//         let end = off + self.1;
//         &mut self.0[off..end]
//     }
// }

impl<T> Index<(usize, usize)> for FlatVec2D<T> {
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.0[flat_coord(x, y, self.1)]
    }
}

impl<T> IndexMut<(usize, usize)> for FlatVec2D<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.0[flat_coord(x, y, self.1)]
    }
}

/// alias around rem_euclid
pub fn wrap(idx: isize, bounds: usize) -> usize {
    idx.rem_euclid(bounds as isize) as usize
}

/// This implements wrapping Indices
impl<T> Index<(isize, isize)> for FlatVec2D<T> {
    type Output = T;

    fn index(&self, (x, y): (isize, isize)) -> &Self::Output {
        let x = wrap(x, self.1);
        let y = wrap(y, self.2);
        &self.0[flat_coord(x, y, self.1)]
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

pub fn inverse_flat_coord(i: usize, dim: usize) -> (usize, usize) {
    let x = i % dim;
    let y = i / dim;
    (x, y)
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

    fn index(&self, (x, y, rot): (usize, usize, Rot2D)) -> &Self::Output {
        &self.0[flat_coord_rot(x, y, self.1, self.2, rot)]
    }
}

impl<T> IndexMut<(usize, usize, Rot2D)> for FlatVec2D<T> {
    fn index_mut(&mut self, (x, y, rot): (usize, usize, Rot2D)) -> &mut Self::Output {
        &mut self.0[flat_coord_rot(x, y, self.1, self.2, rot)]
    }
}

/// More robust way of getting dimensions of a textual grid of data.
/// if the rows are not of equal length, the greatest row length is returned.
///
/// note that trailing newlines are ignored.
fn get_row_col(input: &[u8]) -> (usize, usize) {
    input
        .split(|&b| b == b'\n')
        .rev()
        // remove trailing newlines
        .skip_while(|row| row.is_empty())
        .fold((0, 0), |(r, c), row| (r.max(row.len()), c + 1))
}

pub fn parse_to_flat2d<T>(input: &[u8]) -> FlatVec2D<T>
where
    T: Default + Clone + From<u8>,
{
    let (row_width, col_len) = get_row_col(input);

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

/// This is a padded version of parse_to_flat2d
/// this allows you to solve some puzzles where a junk type would end searching
/// thus you'll never go out of bounds.
pub fn pad_to_flat2d<T>(input: &[u8], pad: T) -> FlatVec2D<T>
where
    T: Clone + From<u8>,
{
    let (row_width, col_len) = get_row_col(input);
    // now pad them
    let row_width = row_width + 2;
    let col_len = col_len + 2;

    let mut ret = FlatVec2D(vec![pad; row_width * col_len], row_width, col_len);

    let mut i = 1;
    let mut j = 1;
    input.iter().for_each(|&el| {
        if el == b'\n' {
            i = 1;
            j += 1;
        } else if el != b'\n' {
            ret[(i, j)] = el.into();
            i += 1;
        }
    });

    ret
}
