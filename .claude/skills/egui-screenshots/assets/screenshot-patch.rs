// A temporary patch (egui-screenshots skill): paste this method into `impl eframe::App for App`
// in `src/app.rs`, take the screenshot, then restore the file. Never committed.
//
// With `SHOT_TO=<file.ppm>` set, the application waits `SHOT_AFTER_MS` milliseconds (1500 by
// default, time for the fonts and the network), asks egui for a screenshot of its own window,
// writes it as a PPM (no image crate needed) and exits. Without it, nothing changes.
fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    use std::sync::{Once, OnceLock};
    use std::time::{Duration, Instant};

    // `ctx.input(|i| i.time)` stays at 0 inside `logic` (eframe 0.37): measure with an Instant.
    static START: OnceLock<Instant> = OnceLock::new();
    static SENT: Once = Once::new();

    let Ok(path) = std::env::var("SHOT_TO") else {
        return;
    };
    ctx.request_repaint();
    let wait = std::env::var("SHOT_AFTER_MS")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(1500);
    if START.get_or_init(Instant::now).elapsed() < Duration::from_millis(wait) {
        return;
    }
    // Put here what brings the window to the state to show, once (a view opened, sample data).
    SENT.call_once(|| {
        let callback = egui::ScreenshotCallback::new(move |image| {
            let [width, height] = image.size;
            let mut ppm = format!("P6 {width} {height} 255\n").into_bytes();
            for pixel in &image.pixels {
                ppm.extend_from_slice(&[pixel.r(), pixel.g(), pixel.b()]);
            }
            std::fs::write(&path, ppm).expect("the screenshot is written");
            std::process::exit(0);
        });
        ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(callback));
    });
}
