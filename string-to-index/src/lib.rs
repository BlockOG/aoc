use rustc_hash::FxHashMap;

#[derive(Debug)]
pub struct StringToIndex<'a> {
    strings: FxHashMap<&'a str, usize>,
}

impl<'a> StringToIndex<'a> {
    pub fn new() -> Self {
        Self {
            strings: FxHashMap::default(),
        }
    }

    pub fn get(&mut self, s: &str) -> usize {
        let i = self.strings.len();
        *self
            .strings
            .entry(unsafe { &*(s as *const _) })
            .or_insert(i)
    }
}
