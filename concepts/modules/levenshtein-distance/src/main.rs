fn levenshtein_distance(a: &str, b: &str, show: bool) -> usize {
    let m = a.chars().count();
    let n = b.chars().count();

    // Initialize matrix
    let mut d = vec![vec![0; n + 1]; m + 1];

    // Initialize first row and column
    for i in 0..=m {
        d[i][0] = i;
    }
    for j in 0..=n {
        d[0][j] = j;
    }

    // Fill in the rest of the matrix
    for (i, a_char) in a.chars().enumerate() {
        for (j, b_char) in b.chars().enumerate() {
            if a_char == b_char {
                d[i + 1][j + 1] = d[i][j];
            } else {
                d[i + 1][j + 1] = std::cmp::min(
                    std::cmp::min(d[i][j + 1], d[i + 1][j]),
                    d[i][j],
                ) + 1;
            }
        }
    }

    if !show {
        return d[m][n];
    }

    // Print the matrix with strings
    print!("    ");
    for b_char in b.chars() {
        print!(" {} ", b_char);
    }
    println!();
    for (i, row) in d.iter().enumerate() {
        if i > 0 {
            print!("{}", a.chars().nth(i - 1).unwrap());
        } else {
            print!(" ");
        }
        for val in row {
            print!("{:2} ", val);
        }
        println!();
    }

    d[m][n]
}

fn main() {
    let string1 = "zebra stripes";
    let string2 = "chupacabra stripes";
    let string3 = "Pinstripe";
    let string4 = "Leopard Spots";
    let string5 = "Herd of zebras";

    let distance2 = levenshtein_distance(string1, string2, false);
    let distance3 = levenshtein_distance(string1, string3, false);
    let distance4 = levenshtein_distance(string1, string4, false);
    let distance5 = levenshtein_distance(string1, string5, true);


    println!(
        "Levenshtein distance between '{}' and '{}' is: {}",
        string1, string2, distance2
    );

    println!(
        "Levenshtein distance between '{}' and '{}' is: {}",
        string1, string3, distance3
    );
    println!(
        "Levenshtein distance between '{}' and '{}' is: {}",
        string1, string4, distance4
    );
    println!(
        "Levenshtein distance between '{}' and '{}' is: {}",
        string1, string5, distance5
    );
}