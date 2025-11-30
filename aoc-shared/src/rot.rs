type Point = (isize, isize);

/// North-bound movement vector
pub const NORTH: Point = (0, -1);
/// East-bound movement vector
pub const EAST: Point = rot_right(NORTH);
/// South-bound movement vector
pub const SOUTH: Point = rot_180(NORTH);
/// West-bound movement vector
pub const WEST: Point = rot_left(NORTH);

/// Array of all the, 4-point compass, cardinal movement vectors; order: N, E, S, W.
pub const CARDINALS: [Point; 4] = [NORTH, EAST, SOUTH, WEST];
/// Index into `CARDINALS` repr North
pub const N: usize = 0;
/// Index into `CARDINALS` repr East
pub const E: usize = 1;
/// Index into `CARDINALS` repr South
pub const S: usize = 2;
/// Index into `CARDINALS` repr West
pub const W: usize = 3;

/// Rotate rhs around lhs, clockwise
pub const fn rot_right_around((origin_x, origin_y): Point, (x, y): Point) -> Point {
    // translate to origin
    (-(y - origin_y) + origin_x, (x - origin_x) + origin_y)
}

/// Rotate rhs around lhs, by 180 degrees.
pub const fn rot_180_around(origin: Point, xy: Point) -> Point {
    rot_right_around(origin, rot_right_around(origin, xy))
}

/// Rotate rhs around lhs, counter-clockwise
pub const fn rot_left_around(origin: Point, xy: Point) -> Point {
    rot_right_around(
        origin,
        rot_right_around(origin, rot_right_around(origin, xy)),
    )
}

/// Rotate point around origin, clockwise.
pub const fn rot_right(xy: Point) -> Point {
    rot_right_around((0, 0), xy)
}

/// Rotate point around origin, by 180 degrees.
pub const fn rot_180(xy: Point) -> Point {
    rot_right(rot_right(xy))
}

/// Rotate point around origin, clockwise.
pub const fn rot_left(xy: Point) -> Point {
    rot_right(rot_right(rot_right(xy)))
}
