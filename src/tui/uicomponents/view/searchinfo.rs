use crate::prelude::*;

pub struct SearchInfo {
    pub prev_location: LineIdx,
    pub prev_scroll_offset: RowIdx,
    pub query: Option<String>,
}
