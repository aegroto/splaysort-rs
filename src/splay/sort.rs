pub trait SplaySorter<K: Ord + 'static> {
    fn splay_min(&mut self);
    fn extract_min(&mut self) -> Option<K>;
}