pub type Kernel = Vec<Vec<f64>>;

pub trait KernelConstructors {
    fn identity() -> Self;
    fn blur(blurring_intensity: f64) -> Self;
    fn gaussian(size: usize, sigma: f64) -> Self;
    fn edge_x() -> Self;
    fn edge_y() -> Self;
    fn edge_all() -> Self;
    fn sharpen() -> Self;
    fn emboss() -> Self;
    fn edge_enhance() -> Self;
}

impl KernelConstructors for Kernel {
    fn identity() -> Self {
        vec![
            vec![0.0, 0.0, 0.0],
            vec![0.0, 1.0, 0.0],
            vec![0.0, 0.0, 0.0],
        ]
    }

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

    fn gaussian(size: usize, sigma: f64) -> Self {
        todo!()
    }

    fn edge_x() -> Self {
        todo!()
    }

    fn edge_y() -> Self {
        todo!()
    }

    fn edge_all() -> Self {
        todo!()
    }

    fn sharpen() -> Self {
        todo!()
    }

    fn emboss() -> Self {
        todo!()
    }

    fn edge_enhance() -> Self {
        todo!()
    }
}
