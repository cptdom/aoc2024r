
static _INPUT_PATH: &str = "tasks/25/input.txt";

fn main() {
    // each vec is
    // e.g.
    // Button A: X+18, Y+59
    // Button B: X+49, Y+12
    // Prize: X=620, Y=7885
    // vvv
    // [18, 59, 49, 12, 620, 7885]
    //
    let input: Vec<Vec<f64>> = t25::load_input(_INPUT_PATH);

    let mut tokens: usize = 0;
    // I have to admit, I had to google how to solve these equations
    // other than manually, turns out its matrices
    // uni flashbacks O.O
    // s/o https://www.youtube.com/watch?v=95zVTo2nu3Q
    for eq_input in input {

        let x = eq_input[0]; // A's X movement
        let z = eq_input[1]; // A's Y movement
        let y = eq_input[2]; // B's x movement
        let w = eq_input[3]; // B's Y movement
        let a = eq_input[4]; // prize X
        let b = eq_input[5]; // prize Y

        // equations
        // a_button_presses * x + b_button_presses * z = a
        // a_button_presses * y + b_button_presses * w = b
        // can be written as
        // |x z| * |a_button_presses| = |a|
        // |y w|   |b_button_presses|   |b|

        // use inverse matrix, determinant and formulas to find solution

         // determinant, if zero -> no solution
         let d = w * x - y * z;
         if d == 0. {
             continue
         }

        let b_button_presses = (b * x - a * z) / d;
        let a_button_presses = (a - b_button_presses * y) / x;

        // no fractals
        if a_button_presses.fract() != 0.0 || b_button_presses.fract() != 0.0 {
            continue
        }

        tokens += (a_button_presses * 3. + b_button_presses) as usize;
    }

    println!("total tokens: {tokens}");
}

