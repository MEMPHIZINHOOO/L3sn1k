use rfd::{self, FileHandle};


pub async fn choose_file() -> impl Future<Output = Option<FileHandle>> {
    rfd::AsyncFileDialog::new()
        .add_filter( "Json file", &["json"],).pick_file()
} 
