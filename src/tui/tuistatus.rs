#[derive(Default, Eq, PartialEq, Debug)]
pub struct TuiStatus {
    pub total_lines: usize,
    pub current_line_idx: usize,
}

impl TuiStatus {
    pub fn position_indicator_to_string(&self) -> String {
        format!(
            "{}/{}",
            self.current_line_idx.saturating_add(1),
            self.total_lines
        )
    }
}
