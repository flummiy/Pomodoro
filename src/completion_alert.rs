use std::{fs::File, io::BufReader, path::Path, thread};

use notify_rust::Notification;

use crate::timer::TimerPhase;

const NOTIFICATION_SOUND_PATH: &str = "assets/notification.mp3";

pub fn alert_timer_finished(phase: TimerPhase) {
    show_desktop_notification(phase);
    play_notification_sound();
}

fn show_desktop_notification(phase: TimerPhase) {
    let body = match phase {
        TimerPhase::Work => "Time for a break.",
        TimerPhase::ShortBreak | TimerPhase::LongBreak => "Time to get back to work.",
    };

    let _ = Notification::new()
        .appname("Pomodoro")
        .summary(&format!("{} complete", phase.label()))
        .body(body)
        .show();
}

fn play_notification_sound() {
    thread::spawn(|| {
        let _ = play_notification_sound_blocking();
    });
}

fn play_notification_sound_blocking() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let sound_path = Path::new(env!("CARGO_MANIFEST_DIR")).join(NOTIFICATION_SOUND_PATH);
    let sound_file = BufReader::new(File::open(sound_path)?);
    let mut sink = rodio::DeviceSinkBuilder::open_default_sink()?;
    sink.log_on_drop(false);
    let player = rodio::play(sink.mixer(), sound_file)?;

    player.sleep_until_end();

    Ok(())
}
