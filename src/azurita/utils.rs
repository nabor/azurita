trait Contains {
    fn contains(&self, char) -> bool;
}

impl Contains for String {
    fn contains(&self, c: char) -> bool {
        for i in self.chars() {
            if i == c { return true }
        }
        return false
    }
}
