use opencv::{Result, prelude::*, videoio, highgui, imgproc};
use opencv::imgproc::COLOR_BGR2HSV;

fn main() -> Result<()> {
    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;
    highgui::named_window("Blue filtered", highgui::WINDOW_AUTOSIZE)?;
    let mut frame = Mat::default();
    let mut hsv = Mat::default();
    let mut mask = Mat::default();

    let lower_blue = opencv::core::Scalar::new(75.0, 25.0, 100.0, 0.0);
    let upper_blue = opencv::core::Scalar::new(140.0, 255.0, 255.0, 0.0);


    loop {
        // Read frame from camera
        cam.read(&mut frame)?;
        // Convert frame to HSV
        imgproc::cvt_color(&frame, &mut hsv, COLOR_BGR2HSV, 0)?;
        // Filter out blue color
        opencv::core::in_range(&hsv, &lower_blue, &upper_blue, &mut mask)?;

        // Apply mask to frame
        let mut blue_only_frame = Mat::default();
        frame.copy_to_masked(&mut blue_only_frame, &mask)?;

        highgui::imshow("Blue filtered", &blue_only_frame)?;

        let key = highgui::wait_key(10)?;
        match key {
            key if key == 'q' as i32 => break,
            _ => ()
        }
    }
    Ok(())
}
