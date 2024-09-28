use std::thread;
use std::thread::sleep;

fn main() {
    let data = "86967897737416471853297327050364959
    11861322575564723963297542624962850
    70856234701860851907960690014725639
    38397966707106094172783238747669219
    52380795257888236525459303330302837
    58495327135744041048897885734297812
    69920216438980873548808413720956532
    16278424637452589860345374828574668";

    let lines = data.split_whitespace();
    let mut row_digit_sum = vec![];

    for (i, row) in lines.enumerate(){
        row_digit_sum.push(thread::spawn( move || -> u32 {
            sleep(std::time::Duration::from_millis(100));
            let row_sum = row.chars().map(|x|
                x.to_digit(10).expect("Should be a number")).sum();
            println!("Row - {i} sum {row_sum}");
            row_sum
        }))
    }

    let total_sum = row_digit_sum.into_iter()
        .map(|c| c.join().unwrap()).sum::<u32>();
    println!("Total sum result: {}", total_sum);
}
