pub trait TryStrip {
    fn try_strip(&self, prefix: &str) -> &str;
}

impl TryStrip for str {
    fn try_strip(&self, prefix: &str) -> &str {
        if let Some(i) = self.strip_prefix(prefix) {
            return i;
        }

        self
    }
}
