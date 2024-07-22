pub fn reflection_pad(input: &Vec<Vec<f64>>, size: usize) -> Vec<Vec<f64>> {
    let (input_rows, input_cols) = (input.len(), input[0].len());

    // calculate the amount of padding needed for each side
    let (pad_top, pad_bottom) = ((size - 1) / 2, size / 2);
    let (pad_left, pad_right) = ((size - 1) / 2, size / 2);

    // create the output matrix with the correct size
    let mut output =
        vec![vec![0.0; input_cols + pad_left + pad_right]; input_rows + pad_top + pad_bottom];

    // fill in the original input values
    for i in 0..input_rows {
        for j in 0..input_cols {
            output[i + pad_top][j + pad_left] = input[i][j];
        }
    }

    // reflect the top and bottom rows
    for i in 0..pad_top {
        let top_row = input[pad_top - i].clone();
        let bottom_row = input[input_rows - 1 - (pad_bottom - i)].clone();
        for j in 0..input_cols {
            output[i][j + pad_left] = top_row[j];
            let l = output.len();
            output[l - 1 - i][j + pad_left] = bottom_row[j];
        }
    }

    // reflect the left and right columns
    for j in 0..pad_left {
        let left_col = output
            .iter()
            .map(|row| row[pad_left - j + 1])
            .collect::<Vec<f64>>();
        let right_col = output
            .iter()
            .map(|row| row[input_cols - 1 + pad_left - (pad_right - j)])
            .collect::<Vec<f64>>();
        for i in 0..output.len() {
            output[i][j] = left_col[i];
            let l = output[0].len();
            output[i][l - 1 - j] = right_col[i];
        }
    }

    output
}