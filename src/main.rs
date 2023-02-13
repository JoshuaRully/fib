use std::io;

fn main() {
    println!("Input the nth Fibonacci number you'd like to know.");

    loop {
        let mut nth = String::new();

        io::stdin()
            .read_line(&mut nth)
            .expect("Failed to read line.");

        let nth: f64 = match nth.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a valid number.");
                continue;
            }
        };

        // O(1) space and time complexity but limited to <= 70th Fibonacci number
        // formula & derivation found here: https://r-knott.surrey.ac.uk/Fibonacci/fibFormula.html#:~:text=the%20n%2Dth%20Fibonacci%20number%20is%20the%20sum%20of%20the,task%2C%20even%20with%20a%20calculator!
        let golden_ratio: f64 = (1.0 + f64::sqrt(5.0)) / 2.0;
        let golden_recip: f64 = (1.0 - f64::sqrt(5.0)) / 2.0;

        let nth_num =
            ((golden_ratio.powf(nth) - golden_recip.powf(nth)) / f64::sqrt(5.0)).round() as i64;
        println!("The nth Fibonacci number is: {nth_num}");
        break;
    }
}
