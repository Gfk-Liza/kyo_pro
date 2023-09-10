
use std::vec;


type union_find_index = i32;
struct UnionFind {
    pub parents: Vec<union_find_index>,
}
impl UnionFind {
    pub fn new() -> Self {
        Self { parents: Vec::default() }
    }

    pub fn new_by_size(size: union_find_index) -> Self {
        Self { parents: vec![-1; size as usize] }
    }

    pub fn find(&mut self, index: union_find_index) -> union_find_index {
        let tmp = self.parents[index as usize];
        if tmp < 0 {
            return index;
        }
        self.parents[index as usize] = self.find(tmp);

        self.parents[index as usize]
    }

    pub fn merge(&mut self, a: union_find_index, b: union_find_index) {
        let mut a = self.find(a);
        let mut b = self.find(b);

        if a != b {
            if self.parents[a as usize] > self.parents[b as usize] {
                let tmp = b;
                b = a;
                a = tmp;
            }

            self.parents[a as usize] += self.parents[b as usize];
            self.parents[b as usize] = a;
        }
    }

    pub fn is_connected(&mut self, a: union_find_index, b: union_find_index) -> bool {
        self.find(a) == self.find(b)
    }

    pub fn get_size(&mut self, index: union_find_index) -> union_find_index {
        let tmp_index = self.find(index) as usize;
        -self.parents[tmp_index]
    }
}
