//! 提供线段树实现的分配器 [`SegmentTreeAllocator`]

use super::Allocator;
use alloc::{vec, vec::Vec};

pub struct SegmentTreeAllocator {
    // 结点中储存 bool 类型变量说明该节点是否可用，true 为可用
    tree: Vec<bool>
}

impl Allocator for SegmentTreeAllocator {
    fn new(capacity: usize) -> Self {
        
        let leaf_num = capacity.next_power_of_two();
        let mut tree = vec![true; 2 * leaf_num];
        // 清除多余的叶节点
        for i in capacity..leaf_num {
            tree[leaf_num + i] = false;
        }
        // 初始化树节点
        for i in (1..leaf_num).rev() {
            tree[i] = tree[2 * i] || tree[2 * i + 1];
        }

        Self {
            tree
        }
    }

    fn alloc(&mut self) -> Option<usize> {
        if self.tree[1] == false {
            // 空间已经分配完
            None
        } else {
            let mut tmp = 1;
            while tmp < self.tree.len() / 2 {
                if self.tree[2 * tmp] == true {
                    tmp *= 2;
                } else if self.tree[2 * tmp + 1] == true {
                    tmp = tmp * 2 + 1;
                } else {
                    panic!("Error in SegmentTreeAllocator::alloc(Can't find correct tree node)");
                }
            }
            assert!(self.tree[tmp]);
            self.update_tree(tmp, false);
            return Some(tmp - self.tree.len() / 2);
        }
    }

    fn dealloc(&mut self, index: usize) {
        assert!(!self.tree[self.tree.len() / 2 + index]);
        self.update_tree(self.tree.len() / 2 + index, true);
    }
}

impl SegmentTreeAllocator {
    fn update_tree(&mut self, node: usize, value: bool) {
        assert!(node >= 1 && node < self.tree.len());
        self.tree[node] = value;
        let mut tmp = node;
        while tmp > 1 {
            tmp /= 2;
            self.tree[tmp] = self.tree[tmp * 2] || self.tree[tmp * 2 + 1];
        }
    }
}