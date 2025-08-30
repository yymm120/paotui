const COMMANDS: &[&str] = &["fragment", "start_tx_map_with_channel", "clear_task", "clear_channel", "add_task", "watch_location",
"watch_direction", "clear_watch_direction", "clear_watch_location"];

fn main() {
  tauri_plugin::Builder::new(COMMANDS)
    .android_path("android")
    .ios_path("ios")
    .build();
}
