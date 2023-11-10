use std::io;

use aoc_shared::read_input;

// fish are only [0, 8]
fn parse(input: Vec<u8>) -> Vec<u8> {
    input.iter().fold(vec![], |mut fish, &digit| match digit {
        b'0'..=b'8' => {
            fish.push(digit - b'0');
            fish
        }
        b',' | b'\n' => fish,
        _ => panic!("invalid input."),
    })
}

const PART_1_CNT: u64 = 80;
const PART_2_CNT: u64 = 256;

// This isn't as cool as below matrix solution.
// fn rotate_cnt(residue: &mut [u128], fishcnt: &mut [u128], cnt: u32) -> u128 {
//     for _ in 0..cnt {
//         residue[0] = fishcnt[1];
//         residue[1] = fishcnt[2];
//         residue[2] = fishcnt[3];
//         residue[3] = fishcnt[4];
//         residue[4] = fishcnt[5];
//         residue[5] = fishcnt[6];
//         residue[6] = fishcnt[0] + fishcnt[7];
//         residue[7] = fishcnt[8];
//         residue[8] = fishcnt[0];
//         residue
//             .iter()
//             .enumerate()
//             .for_each(|(idx, &f)| fishcnt[idx] = f);
//     }
//     fishcnt.iter().sum()
// }

fn m_idx<const RLEN: usize>(x: usize, y: usize) -> usize {
    x * RLEN + y
}

fn setup_resi_mat<const RLEN: usize, const DIM: usize>() -> [u128; DIM] {
    let mut residue = [0u128; DIM];
    for idx in 0..RLEN {
        residue[m_idx::<RLEN>(idx, idx)] = 1;
    }
    residue
}

fn mult_mat<const RLEN: usize, const DIM: usize>(
    lhs: &[u128; DIM],
    rhs: &[u128; DIM],
) -> [u128; DIM] {
    let mut result_set = [0u128; DIM];

    for i in 0..RLEN {
        for j in 0..RLEN {
            for k in 0..RLEN {
                result_set[m_idx::<RLEN>(i, j)] +=
                    lhs[m_idx::<RLEN>(i, k)] * rhs[m_idx::<RLEN>(k, j)];
            }
        }
    }
    result_set
}

fn pow_mat<const RLEN: usize, const DIM: usize>(matrix: &[u128; DIM], exp: u64) -> [u128; DIM] {
    let mut residue = setup_resi_mat::<RLEN, DIM>();
    let mut matrix_clone = matrix.map(|x| x);
    let mut ex = exp;

    while ex - 1 != 0 {
        if ex % 2 != 0 {
            residue = mult_mat::<RLEN, DIM>(&residue, &matrix_clone);
        }
        matrix_clone = mult_mat::<RLEN, DIM>(&matrix_clone, &matrix_clone);
        ex /= 2;
    }
    mult_mat::<RLEN, DIM>(&matrix_clone, &residue)
}

fn solve_sum<const RLEN: usize>(exp_mat: &[u128], fishy: &[u128]) -> u128 {
    let mut sum = 0;
    for i in 0..RLEN {
        for j in 0..RLEN {
            sum += exp_mat[m_idx::<RLEN>(i, j)] * fishy[j];
        }
    }
    sum
}

fn solve(fish: Vec<u8>) -> (u128, u128) {
    //let mut residue = [0u128; FISHLIFESIZ];
    //let mut fishcnt = fish.iter().fold([0u128; 9], |mut acc, &f| {
    //    acc[f as usize] += 1;
    //    acc
    //});

    //let part1 = rotate_cnt(&mut residue, &mut fishcnt, PART_1_CNT);
    //let part2 = rotate_cnt(&mut residue, &mut fishcnt, PART_2_CNT - PART_1_CNT);
    //(part1, part2)

    let fishcnt = fish.iter().fold([0u128; 9], |mut acc, &f| {
        acc[f as usize] += 1;
        acc
    });
    #[rustfmt::skip]
    let recur_mat = [
        0, 1, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 1, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 1, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 1, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 1, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 1, 0, 0,
        1, 0, 0, 0, 0, 0, 0, 1, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 1,
        1, 0, 0, 0, 0, 0, 0, 0, 0
    ];

    let part1_exp_mat = pow_mat::<9, 81>(&recur_mat, PART_1_CNT);
    let part1 = solve_sum::<9>(&part1_exp_mat, &fishcnt);

    let part2_exp_mat = pow_mat::<9, 81>(&recur_mat, PART_2_CNT);
    let part2 = solve_sum::<9>(&part2_exp_mat, &fishcnt);
    (part1, part2)
}

pub fn main() -> io::Result<()> {
    let input = read_input()?;
    let fish = parse(input);
    let (p1, p2) = solve(fish);
    println!("Part1 {}, Part2 {}", p1, p2);
    Ok(())
}
