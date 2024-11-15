pub trait MatchesTrait<T> {
    fn matches(&self, value: &Self) -> Result<T, &'static str>;
}

impl MatchesTrait<bool> for usize {
    fn matches(&self, value: &Self) -> Result<bool, &'static str> {
        Ok(self == value)
    }
}
impl MatchesTrait<bool> for String {
    fn matches(&self, value: &Self) -> Result<bool, &'static str> {
        Ok(self == value)
    }
}
