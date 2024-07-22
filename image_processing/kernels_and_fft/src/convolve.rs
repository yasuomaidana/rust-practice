use crate::padding;

/// Performs a 2D convolution on a given input matrix with a specified kernel.
///
/// This function applies a 2D convolution operation to an input matrix using a given kernel
/// matrix. The convolution process involves sliding the kernel over the input matrix, computing
/// the sum of element-wise multiplications between the kernel and the portion of the input it
/// covers at each position, and producing an output matrix of these sums. To handle edge cases,
/// the input matrix is first padded using reflection padding to ensure the kernel can be applied
/// at the borders of the input.
///
/// # Arguments
///
/// * `input` - A reference to a 2D vector of `f64` representing the input matrix.
/// * `kernel` - A reference to a 2D vector of `f64` representing the convolution kernel.
///
/// # Returns
///
/// A 2D vector of `f64` representing the output matrix after applying the convolution.
pub fn conv_2d(input: &Vec<Vec<f64>>, kernel: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    // create zero padded version of input list to account for
    // kernel size
    let padded = padding::reflection_pad(input, kernel.len());

    let mut out = vec![vec![0.0; input[0].len()]; input.len()];

    // loop over the range of pixels calculate the matrix
    // product using the kernel
    for i in 0..input.len() {
        for j in 0..input[0].len() {
            let mut sum = 0.0;
            // calculate sum of sub-list and kernel
            for n in 0..kernel.len() {
                for m in 0..kernel[0].len() {
                    sum += padded[i + n][j + m] * kernel[n][m];
                }
            }
            out[i][j] = sum;
        }
    }

    return out;
}