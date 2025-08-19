pub fn clear() {
    if cfg!(target_os = "windows") {
        std::process::Command::new("cls").status().unwrap();
    } else {
        std::process::Command::new("clear").status().unwrap();
    }
}
