
static _INPUT_PATH: &str = "tasks/25/input.txt";

fn main() {

    let input: Vec<Vec<f64>> = t25::load_input(_INPUT_PATH);

    println!("input: {:?}", input);

    let mut tokens: usize = 0;

    for eq_input in input {
        let x = eq_input[0];
        let z = eq_input[1];
        let y = eq_input[2];
        let w = eq_input[3];
        let a = eq_input[4] + 10000000000000.;
        let b = eq_input[5] + 10000000000000.;

        let d = w * x - y * z;
        if d == 0. {
            continue
        }

        let b_button_presses = (b * x - a * z) / d;
        let a_button_presses = (a - b_button_presses * y) / x;

        if a_button_presses.fract() != 0.0 || b_button_presses.fract() != 0.0 {
            continue
        }

        tokens += (a_button_presses * 3. + b_button_presses) as usize;
    }

    println!("total tokens: {tokens}");
}

