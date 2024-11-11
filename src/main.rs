mod day1;

fn main() {
    let path = "src/day1.txt";
    let day1_sol_pt1 = day1::compute_calibration_sum(path, false);
    let day1_sol_pt2 = day1::compute_calibration_sum(path, true);
    println!("Calibration sum for part 1 is: {}", day1_sol_pt1);
    println!("Calibration sum for part 2 is: {}", day1_sol_pt2);
}
