pub trait GraphOperations {
    fn shortest_route(&self, start_node: i64, end_node: i64) -> Option<Vec<i64>>;
}
