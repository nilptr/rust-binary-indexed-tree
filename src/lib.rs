pub struct BinaryIndexTree {
    data: Vec<i32>,
}

fn lowbit(x: usize) -> usize {
    return x & (!x + 1);
}

impl BinaryIndexTree {
    pub fn from(arr: &Vec<i32>) -> Self {
        // O(n) build
        let mut data = vec![0; arr.len() + 1];
        for (i, &v) in arr.iter().enumerate() {
            let i = i + 1;
            data[i] += v;
            let j = i + lowbit(i);
            if j < data.len() {
                data[j] += data[i];
            }
        }
        return BinaryIndexTree { data };
    }

    pub fn add(&mut self, k: usize, v: i32) {
        let mut i = k;
        while i < self.data.len() {
            self.data[i] += v;
            i += lowbit(i);
        }
    }

    pub fn prefix_sum(&self, k: usize) -> i32 {
        assert!(k < self.data.len());

        let mut ans = 0;
        let mut i = k;
        while i > 0 {
            ans += self.data[i];
            i -= lowbit(i);
        }
        return ans;
    }

    pub fn range_sum(&self, l: usize, r: usize) -> i32 {
        return self.prefix_sum(r) - self.prefix_sum(l - 1);
    }
}

#[cfg(test)]
mod tests {
    use crate::BinaryIndexTree;

    #[test]
    fn test_from() {
        let tree = BinaryIndexTree::from(&vec![1,2,3,4,5,6,7,8,9,10]);
        assert_eq!(tree.prefix_sum(1), 1);
        assert_eq!(tree.prefix_sum(2), 3);
        assert_eq!(tree.prefix_sum(3), 6);
        assert_eq!(tree.prefix_sum(4), 10);
        assert_eq!(tree.prefix_sum(5), 15);
        assert_eq!(tree.prefix_sum(6), 21);
        assert_eq!(tree.prefix_sum(7), 28);
        assert_eq!(tree.prefix_sum(8), 36);
        assert_eq!(tree.prefix_sum(9), 45);
        assert_eq!(tree.prefix_sum(10), 55);
    }
}
