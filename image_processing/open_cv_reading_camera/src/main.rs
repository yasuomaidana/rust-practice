use opencv::{
    Result,
    prelude::*,
    videoio,
    highgui
};

fn main() -> Result<()> {
    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;
    highgui::named_window("video", highgui::WINDOW_FULLSCREEN)?;
    let mut frame = Mat::default();

    loop {
        cam.read(&mut frame)?;
        highgui::imshow("video", &frame)?;
        let key = highgui::wait_key(10)?;
        match key {
            key if key == 'q' as i32 =>break,
            _ => ()
        }
    }
    Ok(())
}
