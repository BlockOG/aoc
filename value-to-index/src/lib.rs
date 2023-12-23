use std::hash::Hash;

use rustc_hash::FxHashMap;

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

pub struct ValueToIndex<T>
where
    T: Eq + Hash + Copy,
{
    values: FxHashMap<T, usize>,
}

impl<T> ValueToIndex<T>
where
    T: Eq + Hash + Copy,
{
    pub fn new() -> Self {
        Self {
            values: FxHashMap::default(),
        }
    }

    pub fn get(&mut self, s: &T) -> usize {
        let i = self.values.len();
        *self.values.entry(unsafe { *(s as *const T) }).or_insert(i)
    }
}
