use crate::core:: {
    errors::TermError,
    geometry::Size
};

pub trait Term {
    fn restore(&self) -> Result<(), TermError>;
    fn window_size(&self) -> Result<Size, TermError>;
}