use std::{
    fmt::Debug,
    io,
    ops::{Index, IndexMut},
    slice::SliceIndex,
};

use aoc_shared::{read_input, AoCTokenizer, Token};

#[derive(Clone, Copy, Debug)]
enum Leaf {
    Node(usize),
    Value(u64),
}

const LEFT: usize = 0;
const RIGHT: usize = 1;

#[derive(Clone, Copy, Default, Debug)]
struct Node {
    parent: Option<usize>,
    children: [Option<Leaf>; 2],
}

impl Node {
    fn kill(&mut self) {
        self.parent = None;
        self.children[LEFT] = None;
        self.children[RIGHT] = None;
    }

    fn is_dead(&self) -> bool {
        self.parent.is_none() && self.children[LEFT].is_none() && self.children[RIGHT].is_none()
    }

    fn is_leaf(&self) -> bool {
        if let Some(Leaf::Value(_)) = self.children[LEFT] {
            if let Some(Leaf::Value(_)) = self.children[RIGHT] {
                return true;
            }
        }
        false
    }
}

#[derive(Clone, Copy, Debug)]
enum Either {
    Left(usize),
    Right(u64),
}

#[derive(Clone, Copy, Debug)]
enum Reduce {
    Explode(usize),
    Split(usize, usize),
}

#[derive(Default, Clone)]
struct Snailnum(Vec<Node>, usize);

impl Snailnum {
    fn push(&mut self, node: Node) {
        self.0.push(node)
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn split(&mut self, node: usize, dir: usize) {
        if let Some(Leaf::Value(v)) = self[node].children[dir] {
            let parent = Some(node);
            let children = [
                Some(Leaf::Value(v >> 1)),             // div by 2, rounds down
                Some(Leaf::Value((v >> 1) + (v & 1))), // div by 2, rounds up
            ];
            self.push(Node { parent, children });
            let new_child_idx = self.len() - 1;
            self[node].children[dir] = Some(Leaf::Node(new_child_idx));
        }
    }

    fn add_sn(&mut self, rhs: &Snailnum) {
        // Update for new root
        let backroot = self.1;
        self.1 = self.len();
        self.0
            .iter_mut()
            .filter(|node| !node.is_dead() && node.parent.is_none())
            .for_each(|node| {
                node.parent = Some(self.1);
            });

        // Add new root. center of tree
        self.push(Node {
            parent: None,
            children: [
                Some(Leaf::Node(backroot)),
                Some(Leaf::Node(self.1 + rhs.1 + 1)),
            ],
        });

        // copy right hand side into our root
        let rstart = self.len();
        let new_rhs_root = rhs.1 + self.1;
        for rnode in &rhs.0 {
            let parent = if let Some(parent) = rnode.parent {
                Some(parent + rstart)
            } else {
                Some(new_rhs_root)
            };
            let mut children = rnode.children.iter().map(|&child| {
                if let Some(l) = child {
                    match l {
                        Leaf::Node(usize) => Some(Leaf::Node(usize + rstart)),
                        Leaf::Value(val) => Some(Leaf::Value(val)),
                    }
                } else {
                    None
                }
            });
            let children = [children.next().unwrap(), children.next().unwrap()];
            self.push(Node { parent, children });
        }
    }

    fn find_reduce(
        &self,
        node_idx: usize,
        depth: usize,
        tenative: Option<Reduce>,
    ) -> Option<Reduce> {
        let node = self[node_idx];
        let mut tenative = tenative;

        if depth > 3 {
            if node.is_leaf() {
                return Some(Reduce::Explode(node_idx));
            } else {
                tenative = Some(Reduce::Explode(node_idx));
            }
        }

        for (idx, &child) in node.children.iter().enumerate() {
            match child {
                Some(Leaf::Node(n)) => {
                    let t = self.find_reduce(n, depth + 1, tenative);
                    if let Some(Reduce::Explode(_)) = t {
                        return t;
                    } else if t.is_some() {
                        tenative = t;
                    }
                }
                Some(Leaf::Value(v)) if v > 9 && tenative.is_none() => {
                    tenative = Some(Reduce::Split(node_idx, idx));
                }
                _ => (),
            }
        }

        tenative
    }

    fn explode(&mut self, node_idx: usize) {
        let mut stack = vec![(node_idx, node_idx, LEFT, None, false)];
        while let Some((nidx, from, direction, value, changed_dir)) = stack.pop() {
            let node = &mut self[nidx];
            if value.is_none() {
                if let Some(parent_idx) = node.parent {
                    for (dir, &child_idx) in node.children.iter().enumerate() {
                        if let Some(Leaf::Value(val)) = child_idx {
                            stack.push((parent_idx, from, dir, Some(val), changed_dir));
                        }
                    }
                }
            } else if let Some(Leaf::Node(child_idx)) = node.children[direction] {
                if let Some(parent_idx) = node.parent {
                    if child_idx == from {
                        stack.push((parent_idx, nidx, direction, value, changed_dir));
                        continue;
                    }
                } else if child_idx == from {
                    continue;
                }
                let (changed_dir, newdir) = if !changed_dir {
                    if direction == LEFT {
                        (!changed_dir, RIGHT)
                    } else {
                        (!changed_dir, LEFT)
                    }
                } else {
                    (changed_dir, direction)
                };
                stack.push((child_idx, nidx, newdir, value, changed_dir));
            } else if let Some(Leaf::Value(old_val)) = node.children[direction] {
                if let Some(value) = value {
                    let new_value = old_val + value;
                    node.children[direction] = Some(Leaf::Value(new_value));
                }
            }
        }
    }

    fn reduce_at(&mut self, reduce_step: Reduce) {
        match reduce_step {
            Reduce::Explode(exploding_idx) => {
                self.explode(exploding_idx);
                // Set exploded child to value zero
                if let Some(parent) = self[exploding_idx].parent {
                    self[parent].children.iter_mut().for_each(|child| {
                        if let Some(Leaf::Node(child_idx)) = child {
                            if *child_idx == exploding_idx {
                                *child = Some(Leaf::Value(0));
                            }
                        }
                    });
                }
                // remove exploded from tree.
                self[exploding_idx].kill();
            }
            Reduce::Split(split_idx, dir) => {
                self.split(split_idx, dir);
            }
        }
    }

    fn recurse_write_snail(&self, node_idx: usize) -> String {
        let mut node_str = "[".to_string();
        match self[node_idx].children[LEFT] {
            Some(Leaf::Node(n)) => {
                node_str += &self.recurse_write_snail(n);
            }
            Some(Leaf::Value(v)) => node_str += &v.to_string(),
            _ => (),
        }
        node_str.push(',');
        match self[node_idx].children[RIGHT] {
            Some(Leaf::Node(n)) => {
                node_str += &self.recurse_write_snail(n);
            }
            Some(Leaf::Value(v)) => node_str += &v.to_string(),
            _ => (),
        }
        node_str.push(']');
        node_str
    }
}

impl Debug for Snailnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.recurse_write_snail(self.1))
    }
}

impl<Idx> Index<Idx> for Snailnum
where
    Idx: SliceIndex<[Node]>,
{
    type Output = <Idx as SliceIndex<[Node]>>::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.0[index]
    }
}

impl<Idx> IndexMut<Idx> for Snailnum
where
    Idx: SliceIndex<[Node]>,
{
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        self.0.index_mut(index)
    }
}

fn magnitude(snailnum: &Snailnum, root: usize) -> u64 {
    snailnum[root]
        .children
        .iter()
        .enumerate()
        .fold(0u64, |acc, (idx, &child)| match child {
            Some(Leaf::Node(n)) => acc + magnitude(snailnum, n) * (3 - idx as u64),
            Some(Leaf::Value(v)) => acc + v * (3 - idx as u64),
            None => panic!("Invalid snailnum."),
        })
}

fn parse(input: Vec<u8>) -> Vec<Snailnum> {
    let mut nodes = vec![Snailnum::default()];
    let mut stack = vec![];
    let mut i = 0usize;

    // NOTES:  we assume that numbers are single digit and ascii 0-9
    for token in AoCTokenizer::new(&input) {
        match token {
            Token::Something(num) => {
                stack.push(Either::Right((num[0] - b'0') as u64));
            }
            Token::Delimiter(sep) if sep == b'[' => {
                let nidx = nodes[i].len();
                nodes[i].push(Node::default());
                stack.push(Either::Left(nidx));
            }
            Token::Delimiter(sep) if sep == b']' => {
                let right = stack.pop().expect("Malformed Snailnum.");
                let left = stack.pop().expect("Mailformed Snailnum.");
                let root = stack.pop().expect("Mailformed Snailnum.");

                match root {
                    Either::Left(node) => {
                        [left, right]
                            .iter()
                            .enumerate()
                            .for_each(|(idx, &child)| match child {
                                Either::Left(node_c) => {
                                    nodes[i][node].children[idx] = Some(Leaf::Node(node_c));
                                    nodes[i][node_c].parent = Some(node);
                                }
                                Either::Right(value) => {
                                    nodes[i][node].children[idx] = Some(Leaf::Value(value));
                                }
                            });
                        stack.push(Either::Left(node));
                    }
                    _ => panic!("Malformed Snailnum."),
                }
            }
            Token::Newline => {
                stack.clear(); // should just be the root of the snailnum.
                i += 1;
                nodes.push(Snailnum::default());
            }
            Token::End => {
                if nodes
                    .last()
                    .expect("Expected at least one Snailnum.")
                    .is_empty()
                {
                    nodes.pop();
                }
            }
            _ => (),
        }
    }

    nodes
}

fn solve(snails: &[Snailnum]) -> (u64, u64) {
    let mut sum = snails[0].clone();
    for snail in &snails[1..] {
        sum.add_sn(snail);
        while let Some(reduce_step) = sum.find_reduce(sum.1, 0, None) {
            sum.reduce_at(reduce_step);
        }
    }
    let p1 = magnitude(&sum, sum.1);

    let mut max = u64::MIN;
    for (i, snail1) in snails.iter().enumerate() {
        for (j, snail2) in snails.iter().enumerate() {
            if i == j {
                continue;
            }

            for (mut sn1, sn2) in [(snail1.clone(), snail2), (snail2.clone(), snail1)] {
                sn1.add_sn(sn2);
                while let Some(reduce_step) = sn1.find_reduce(sn1.1, 0, None) {
                    sn1.reduce_at(reduce_step);
                }
                let mag = magnitude(&sn1, sn1.1);
                if mag > max {
                    max = mag;
                }
            }
        }
    }
    (p1, max)
}

pub fn main() -> io::Result<()> {
    let input = read_input()?;
    let parsed = parse(input);
    let sum = solve(&parsed);
    println!("Part1 {:?}", sum);
    Ok(())
}
