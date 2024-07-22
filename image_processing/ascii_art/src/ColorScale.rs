struct ColorScale {
    r_scale: f64,
    g_scale: f64,
    b_scale: f64,
}

impl ColorScale {
    /// Creates a new `ColorScale` ensuring the sum of its elements equals 1.
    /// If the sum is not 1, it normalizes the components so that their sum will be 1.
    ///
    /// # Arguments
    ///
    /// * `r_scale` - The red scale component.
    /// * `g_scale` - The green scale component.
    /// * `b_scale` - The blue scale component.
    ///
    /// # Returns
    ///
    /// A new instance of `ColorScale`.
    pub fn new(mut r_scale: f64, mut g_scale: f64, mut b_scale: f64) -> Self {

        let sum = r_scale + g_scale + b_scale;
        if sum != 0.0 {
            r_scale /= sum;
            g_scale /= sum;
            b_scale /= sum;
        }
        ColorScale { r_scale, g_scale, b_scale }
    }
}