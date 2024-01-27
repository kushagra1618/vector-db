use super::vector_db::{Vector, VectorDB};

enum KDTreeNode {
    Leaf(Vector),
    Internal {
        left: Box<KDTreeNode>,
        right: Box<KDTreeNode>,
        split_value: f32,
        split_dimension: usize,
    },
}
pub struct KDTree {
    root: KDTreeNode,
}
impl KDTree {
    pub fn build(points: Vec<Vector>, depth: usize) -> Self {
        let nodes = Self::_build(points, depth);
        Self { root: nodes }
    }

    fn _build(points: Vec<Vector>, depth: usize) -> KDTreeNode {
        if points.len() == 1 {
            return KDTreeNode::Leaf(points[0]);
        }
        let dim = depth % 3;
        let mut sorted_points = points;
        sorted_points.sort_by(|a, b| a[dim].partial_cmp(&b[dim]).unwrap());
        let median_idx = sorted_points.len() / 2;
        let median_value = sorted_points[median_idx][dim];
        KDTreeNode::Internal {
            left: Box::new(Self::_build(
                sorted_points[..median_idx].to_vec(),
                depth + 1,
            )),
            right: Box::new(Self::_build(
                sorted_points[median_idx..].to_vec(),
                depth + 1,
            )),
            split_value: median_value,
            split_dimension: dim,
        }
    }
}

impl KDTree {
    pub fn nearest_neighbor(&self, query: &Vector) -> Option<&Vector> {
        self.nearest(query, &self.root, None, f32::MAX)
    }

    fn nearest<'a>(
        &'a self,
        query: &Vector,
        node: &'a KDTreeNode,
        best: Option<&'a Vector>,
        best_dist: f32,
    ) -> Option<&'a Vector> {
        match node {
            KDTreeNode::Leaf(point) => {
                let dist = VectorDB::euclidean_dist(query, point);

                if dist < best_dist {
                    Some(point)
                } else {
                    best
                }
            }

            KDTreeNode::Internal {
                left,
                right,
                split_value,
                split_dimension,
            } => {
                let next_node = if query[*split_dimension] < *split_value {
                    left
                } else {
                    right
                };
                let other_node = if query[*split_dimension] < *split_value {
                    right
                } else {
                    left
                };
                let updated_best = self.nearest(query, next_node, best, best_dist);
                let updated_best_dist =
                    updated_best.map_or(best_dist, |v| VectorDB::euclidean_dist(query, v));
                if (query[*split_dimension] - split_value).abs() < updated_best_dist {
                    self.nearest(query, other_node, updated_best, updated_best_dist)
                } else {
                    updated_best
                }
            }
        }
    }
}
