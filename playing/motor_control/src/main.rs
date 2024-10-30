use nalgebra::{DMatrix, DVector};

fn multivariate_regression(x: &DMatrix<f64>, y: &DVector<f64>) -> Result<DVector<f64>, String> {
    // Add a column of ones to the design matrix for the intercept term
    let x = x.clone().insert_column(0, 1.0);

    // Calculate the coefficients using the normal equation
    let xtx = x.transpose() * &x;
    if let Some(xtx_inv) = xtx.try_inverse() {
        let coefficients = xtx_inv * x.transpose() * y;
        Ok(coefficients)
    } else {
        Err("Matrix is not invertible".to_string())
    }
}

fn main() {
    // Example usage:
    let x = DMatrix::from_row_slice(5, 2, &[
        1.0, 2.0,
        2.0, 3.0,
        3.0, 1.0,
        4.0, 2.0,
        3.0, 2.3
    ]);
    let y = DVector::from_row_slice(&[3.0, 5.0, 4.0, 6.0, 8.0]);

    match multivariate_regression(&x, &y) {
        Ok(coefficients) => println!("Coefficients: {}", coefficients),
        Err(e) => println!("Error: {}", e),
    }
}