use native_dialog::{DialogBuilder, MessageLevel};

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
