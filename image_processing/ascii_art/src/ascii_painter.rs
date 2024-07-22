fn cast_to_char(value: f64) -> &'static str {
    let ascii_chars = [" ", ".",",","-","~","+","=","@",":",";"];
    let index = (value * (ascii_chars.len() - 1) as f64).round() as usize;
    ascii_chars[index]
}

pub fn to_ascii_image(image: &Vec<Vec<f64>>) -> Vec<Vec<&'static str>> {
    image.iter().map(|row| row.iter().map(|&value| cast_to_char(value)).collect()).collect()
}


pub fn reduce_image_by_sampling(image: &Vec<Vec<f64>>, scale: usize) -> Vec<Vec<f64>> {
    let original_height = image.len();
    let original_width = if !image.is_empty() { image[0].len() } else { 0 };

    // Calculate new dimensions based on the sampling condition
    let new_height = original_height / (scale * 2);
    let new_width = original_width / scale;

    let mut reduced_image = vec![vec![0.0; new_width]; new_height];

    for (new_y, y) in (0..original_height).filter(|&y| y % (scale * 2) == 0).enumerate() {
        for (new_x, x) in (0..original_width).filter(|&x| x % scale == 0).enumerate() {
            if new_y < new_height && new_x < new_width {
                reduced_image[new_y][new_x] = image[y][x];
            }
        }
    }

    reduced_image
}