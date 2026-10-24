use native_dialog::{DialogBuilder, MessageLevel};

use tokio::fs;

pub mod main_window;
pub mod plot_window;

fn alert(level: MessageLevel, message: impl ToString) {
    DialogBuilder::message()
        .set_title("QPoint Measure")
        .set_text(message)
        .set_level(level)
        .alert()
        .show()
        .unwrap();
}

async fn save_file(
    content: String,
    filter: &str,
    extensions: impl AsRef<[&str]>,
) -> Result<(), String> {
    let path = DialogBuilder::file()
        .set_title("QPoint Measure: Save plot image")
        .add_filter(filter, extensions)
        .save_single_file()
        .spawn()
        .await
        .unwrap();

    if let Some(p) = path {
        fs::write(&p, content)
            .await
            .map_err(|e| format!("Failed to write {}: {}", p.display(), e))
    } else {
        Ok(())
    }
}
