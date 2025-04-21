struct StaticGrid<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

trait ToStaticGrid<T> {
    fn to_grid(&self) -> StaticGrid<T>;
}
