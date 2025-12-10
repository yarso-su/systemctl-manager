#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum SearchDirection {
    #[default]
    Forward,
    Backward,
}
