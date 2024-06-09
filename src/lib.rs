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
        assert!(k < self.data.len());

        let mut i = k;
        while i < self.data.len() {
            self.data[i] += v;
            i += lowbit(i);
        }
    }

    pub fn append(&mut self, v: i32) {
        let n = self.data.len();
        let mut value = v;
        
        let mut i = 1;
        while i < lowbit(n) {
            value += self.data[n - i];
            i = i << 1;
        }

        self.data.push(value);
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
        assert!(l > 0);
        assert!(r > 0);
        assert!(l <= r);
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

    #[test]
    fn test_append() {
        let mut tree = BinaryIndexTree::from(&vec![]);
        tree.append(1);
        assert_eq!(tree.prefix_sum(1), 1);
        tree.append(2);
        assert_eq!(tree.prefix_sum(2), 3);
        tree.append(3);
        assert_eq!(tree.prefix_sum(3), 6);
        tree.append(4);
        assert_eq!(tree.prefix_sum(4), 10);
        tree.append(5);
        assert_eq!(tree.prefix_sum(5), 15);
        tree.append(6);
        assert_eq!(tree.prefix_sum(6), 21);
        tree.append(7);
        assert_eq!(tree.prefix_sum(7), 28);
        tree.append(8);
        assert_eq!(tree.prefix_sum(8), 36);
        tree.append(9);
        assert_eq!(tree.prefix_sum(9), 45);
        tree.append(10);
        assert_eq!(tree.prefix_sum(10), 55);
    }
}
