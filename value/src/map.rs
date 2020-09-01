use std::{cmp, collections::btree_map, fmt, iter, ops};

pub type InnerMap<K, V> = btree_map::BTreeMap<K, V>;

#[derive(Eq, PartialEq)]
pub struct Map<K, V> {
    inner: InnerMap<K, V>,
}

impl<K, V> Map<K, V>
where
    K: cmp::Ord,
{
    pub fn new() -> Self {
        Self {
            inner: InnerMap::new(),
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.inner.insert(key, value)
    }
}

impl<K, V> From<InnerMap<K, V>> for Map<K, V> {
    fn from(v: InnerMap<K, V>) -> Self {
        Self { inner: v }
    }
}

impl<K, V> Into<InnerMap<K, V>> for Map<K, V> {
    fn into(self) -> InnerMap<K, V> {
        self.inner
    }
}

impl<K, V> ops::Deref for Map<K, V> {
    type Target = InnerMap<K, V>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<K, V> fmt::Debug for Map<K, V>
where
    K: fmt::Debug,
    V: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl<K, V> iter::FromIterator<(K, V)> for Map<K, V>
where
    K: Ord,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        iter.into_iter().collect::<InnerMap<K, V>>().into()
    }
}

impl<K, V> IntoIterator for Map<K, V> {
    type Item = (K, V);
    type IntoIter = btree_map::IntoIter<K, V>;
    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<'a, K, V> IntoIterator for &'a Map<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = btree_map::Iter<'a, K, V>;
    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

impl<K, V> Default for Map<K, V>
where
    K: cmp::Ord,
{
    fn default() -> Self {
        InnerMap::default().into()
    }
}
