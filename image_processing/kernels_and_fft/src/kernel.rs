pub type Kernel = Vec<Vec<f64>>;
/// A type alias for a 2D vector representing a kernel matrix.
///
/// This matrix is used in image processing operations such as blurring,
/// sharpening, edge detection, etc. Each element of the matrix represents
/// a weight that affects how a pixel's value is computed from itself and its
/// neighbors.

pub trait KernelConstructors {
    // fn identity() -> Self;
    fn blur(blurring_intensity: f64) -> Self;
    // fn gaussian(size: usize, sigma: f64) -> Self;
    // fn edge_x() -> Self;
    // fn edge_y() -> Self;
    // fn edge_all() -> Self;
    // fn sharpen() -> Self;
    // fn emboss() -> Self;
    // fn edge_enhance() -> Self;
}

impl KernelConstructors for Kernel {
    // fn identity() -> Self {
    //     vec![
    //         vec![0.0, 0.0, 0.0],
    //         vec![0.0, 1.0, 0.0],
    //         vec![0.0, 0.0, 0.0],
    //     ]
    // }

    /// Constructs a blurring kernel with uniform intensity.
    ///
    /// This function creates a 3x3 matrix where each element is the reciprocal
    /// of the specified blurring intensity. This uniform distribution is used
    /// to apply a simple blur effect across an image by averaging the pixels
    /// around a target pixel based on the blurring intensity.
    ///
    /// # Arguments
    ///
    /// * `blurring_intensity` - A `f64` that determines the intensity of the blur.
    ///   Higher values result in less blurring, as each matrix element will be smaller,
    ///   reducing the effect of neighboring pixels on the target pixel.
    ///
    /// # Returns
    ///
    /// A `Kernel` (Vec<Vec<f64>>) representing the blurring matrix.
    ///
    /// # Examples
    ///
    /// ```
    /// let blur_kernel = Kernel::blur(2.0);
    /// ```
    fn blur(blurring_intensity: f64) -> Self {
        vec![
            vec![
                1.0 / blurring_intensity,
                1.0 / blurring_intensity,
                1.0 / blurring_intensity,
            ],
            vec![
                1.0 / blurring_intensity,
                1.0 / blurring_intensity,
                1.0 / blurring_intensity,
            ],
            vec![
                1.0 / blurring_intensity,
                1.0 / blurring_intensity,
                1.0 / blurring_intensity,
            ],
        ]
    }

    // fn gaussian(size: usize, sigma: f64) -> Self {
    //     todo!()
    // }
    //
    // fn edge_x() -> Self {
    //     todo!()
    // }
    //
    // fn edge_y() -> Self {
    //     todo!()
    // }
    //
    // fn edge_all() -> Self {
    //     todo!()
    // }
    //
    // fn sharpen() -> Self {
    //     todo!()
    // }
    //
    // fn emboss() -> Self {
    //     todo!()
    // }
    //
    // fn edge_enhance() -> Self {
    //     todo!()
    // }
}
