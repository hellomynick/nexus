use tokio::process::Command;

pub fn handler_with_command() {
    Command::new("ffmpeg").arg("i");
}
