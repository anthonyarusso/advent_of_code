struct Point {
    x: i32,
    y: i32,
    z: i32,
    active: bool,
}
impl PartialEq for Point {
    // test equality only by coordinate values, not 'active'
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}
impl Point {
    fn new(x: i32, y: i32, z: i32, active: bool) -> Self {
        Point {
            x,
            y,
            z,
            active,
        }
    }
    // All blocks change simultaneously. Therefore checking the status
    // of a block should not mutate the block. Store the states of
    // the blocks and apply them all at a later time.
    fn check(&self, blocks: &Vec<Point>) -> bool {
        let each_direction = &[0, -1, 1];
        let mut active_neighbors = 0;

        for i in each_direction.iter() { // for x
            for j in each_direction.iter() { // for y
                for k in each_direction.iter() {
                    // track the current block
                    let current_block = Point::new(
                        self.x + i,
                        self.y + j,
                        self.z + k,
                        false);

                    if self.active {
                    } else {
                        for block in blocks.iter() {
                            if current_block == block {
                                active_neighbors += 1;
                            }
                        }
                    }
                } // end for k
            } // end for j
        } // end for i
        true
    }
}

fn main() {
    let mut active_blocks = vec![
        Point::new(0,1,0, true),
        Point::new(1,0,0, true),
        Point::new(-1, -1, 0, true),
        Point::new(0, -1, 0, true),
        Point::new(1, -1, 0, true),
    ];
    let p = Point::new(0, 0, 0, true);
    p.check(&active_blocks);
}
