pub trait Matches<T> {
    fn matches(&self, value: &Self) -> Result<T, &'static str>;
}

impl Matches<bool> for usize {
    fn matches(&self, value: &Self) -> Result<bool, &'static str> {
        Ok(self == value)
    }
}
impl Matches<bool> for String {
    fn matches(&self, value: &Self) -> Result<bool, &'static str> {
        Ok(self == value)
    }
}