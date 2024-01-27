pub type Vector = [f32; 3]; // case for 3d vectors
pub struct VectorDB {
    vectors: Vec<Vector>,
}

impl VectorDB {
    pub fn new() -> Self {
        VectorDB {
            vectors: Vec::new(),
        }
    }
    pub fn add(&mut self, vector: Vector) {
        self.vectors.push(vector);
    }
    pub fn get(&self, index: usize) -> Option<&Vector> {
        self.vectors.get(index)
    }
}

impl VectorDB {
    pub fn find_closest(&self, query: Vector) -> Option<&Vector> {
        self.vectors.iter().min_by(|&a, &b| {
            let dist_a = VectorDB::euclidean_dist(&query, a);
            let dist_b = VectorDB::euclidean_dist(&query, b);
            dist_a.partial_cmp(&dist_b).unwrap()
        })
    }

    /// Used to calculate euclidean distance between 2 vectors
    pub fn euclidean_dist(a: &Vector, b: &Vector) -> f32 {
        a.iter()
            .zip(b.iter())
            .map(|(x, y)| (x - y).powi(2))
            .sum::<f32>()
            .sqrt()
    }
}
