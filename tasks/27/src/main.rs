static _INPUT_PATH: &str = "tasks/27/input.txt";

// boundary - pairs of X and Y coordinates to define quadrants
#[derive(Debug)]
struct Area {
    x_range: (isize, isize),
    y_range: (isize, isize),
}

impl Area {
    fn belongs(&self, x: isize, y: isize) -> bool {
        x >= self.x_range.0 &&
        x <= self.x_range.1 &&
        y >= self.y_range.0 &&
        y <= self.y_range.1
    }

    fn new(x_range: (isize, isize), y_range: (isize, isize)) -> Self {
        Area{
            x_range,
            y_range,
        }
    }
}

const SECONDS: isize = 100;

fn main() {
    let input = t27::load_input(_INPUT_PATH);

    // let's define quadrant boundaries
    // we have a grid of width 101 and height 103
    // so we need to define the quadrant boundaries
    let w: isize = 101;
    let h: isize = 103;

    // quadrant width = w // 2
    // quadrant height = h // 2
    // quadrant boundaries are defined by a pair of coordinates
    // Q1 | Q2
    // -- | --
    // Q3 | Q4
    let q1 = Area::new((0, w/2 - 1), (0, h/2 -1)); // (0,49)(0,51)
    let q2 = Area::new((w/2 +1, w - 1), (0, h/2 -1)); // (51, 101)(0,51)
    let q3 = Area::new((0, w/2 - 1), (h/2+1, h - 1)); // (0,49)(52,103)
    let q4 = Area::new((w/2 +1, w - 1), (h/2+1, h- 1)); // (51,101)(52,103)

    // for each robot input
    // we calculate its position after N seconds
    // using formula
    // and then check if it belongs to any quadrant
    // if it does, we increment that quadrant's count
    let mut q1_cnt: usize = 0;
    let mut q2_cnt: usize = 0;
    let mut q3_cnt: usize = 0;
    let mut q4_cnt: usize = 0;


    for robot in input {
        let final_x = ((robot[0] + SECONDS * robot[2]) % w + w) % w;
        let final_y = ((robot[1] + SECONDS * robot[3]) % h + h) % h;

        // check quadrant

        if q1.belongs(final_x, final_y) {
            q1_cnt += 1;
        }
        if q2.belongs(final_x, final_y) {
            q2_cnt += 1;
        }
        if q3.belongs(final_x, final_y) {
            q3_cnt += 1;
        }
        if q4.belongs(final_x, final_y) {
            q4_cnt += 1;
        }
    }

    let result = q1_cnt * q2_cnt * q3_cnt * q4_cnt;

    println!("final result after {SECONDS} seconds is {result}");
}