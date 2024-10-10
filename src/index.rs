pub trait Index<K, P> {
    fn insert(&mut self, key: K, point: P);

    fn find(&self, query: &P, n: usize) -> Vec<(K, &P)>;

    fn find_keys(&self, query: &P, n: usize) -> Vec<K>;

    fn len(&self) -> usize;
}
