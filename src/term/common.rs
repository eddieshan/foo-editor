use crate::core:: {
    errors::TermError,
    geometry::Size
};

pub struct TermInfo {
    pub buffer_size: Size,
    pub screen_size: Size
}

pub trait Term {
    fn restore(&self) -> Result<(), TermError>;
    fn info(&self) -> Result<TermInfo, TermError>;
}