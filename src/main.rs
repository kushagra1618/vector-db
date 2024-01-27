mod storage;
use storage::kd_tree_node::*;

fn main() {
    let points = vec![[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [2.0, 3.0, 4.0]];
    let kd_tree = KDTree::build(points, 0);

    if let Some(nearest) = kd_tree.nearest_neighbor(&[3.0, 3.0, 3.0]) {
        println!("Nearest neighbor: {:?}", nearest);
    }
}
