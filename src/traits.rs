pub(crate) trait Converts<A, B> {
    fn convert(&self, a: A) -> B;
}
