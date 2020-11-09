mod debug;
mod map_loading;

pub use debug::{setup_ui, text_update_system};
pub use map_loading::{process_map_change, process_map_loading};
