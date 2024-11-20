#[derive(Default)]
pub struct DisjointSetUnion {
    father: Vec<usize>,
}

impl DisjointSetUnion {
    pub fn with_capacity(capacity: usize) -> Self {
        let father = (0..capacity).collect();
        Self { father }
    }

    pub fn new_element(&mut self) -> usize {
        let id = self.len();
        self.father.push(id);
        id
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.father[x] != x {
            self.father[x] = self.find(self.father[x]);
        }
        self.father[x]
    }

    pub fn union(&mut self, x: usize, y: usize) {
        let fx = self.find(x);
        let fy = self.find(y);
        self.father[fx] = fy;
    }

    pub fn count_disjoint(&mut self) -> usize {
        (0..self.len()).filter(|&i| self.find(i) == i).count()
    }

    pub fn len(&self) -> usize {
        self.father.len()
    }

    pub fn is_empty(&self) -> bool {
        self.father.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dsu() {
        let mut dsu = DisjointSetUnion::with_capacity(5);
        assert_eq!(dsu.count_disjoint(), 5);

        dsu.union(0, 1);
        assert_eq!(dsu.count_disjoint(), 4);

        dsu.union(2, 3);
        assert_eq!(dsu.count_disjoint(), 3);

        dsu.union(0, 2);
        assert_eq!(dsu.count_disjoint(), 2);

        let element = dsu.new_element();
        assert_eq!(dsu.count_disjoint(), 3);

        dsu.union(4, element);
        assert_eq!(dsu.count_disjoint(), 2);
    }
}
