mod editor;
mod terminal;
mod document;
mod row;
mod filetype;
mod highlighting;

use editor::Editor;
pub use editor::Position;
pub use editor::SearchDirection;
pub use terminal::Terminal;
pub use row::Row;
pub use document::Document;
pub use filetype::FileType;
pub use filetype::HighlightingOptions;

fn main() {
    Editor::default().run();
}