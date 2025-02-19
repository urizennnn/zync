use rfd::FileDialog;

pub fn open_explorer_and_file_select() {
    if let Some(path) = FileDialog::new().pick_file() {
        println!("Selected file: {}", path.display());
    } else {
        println!("No file selected.");
    }
}
