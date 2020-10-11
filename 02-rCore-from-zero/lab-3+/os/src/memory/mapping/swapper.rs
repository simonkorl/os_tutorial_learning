//! 页面置换算法

use super::*;
use crate::memory::{frame::FrameTracker, *};
use alloc::collections::VecDeque;
use alloc::vec::Vec;

/// 管理一个线程所映射的页面的置换操作
pub trait Swapper {
    /// 新建带有一个分配数量上限的置换器
    fn new(quota: usize) -> Self;

    /// 是否已达到上限
    fn full(&self) -> bool;

    /// 取出一组映射
    fn pop(&mut self) -> Option<(VirtualPageNumber, FrameTracker)>;

    /// 添加一组映射（不会在以达到分配上限时调用）
    fn push(&mut self, vpn: VirtualPageNumber, frame: FrameTracker, entry: *mut PageTableEntry);

    /// 只保留符合某种条件的条目（用于移除一段虚拟地址）
    fn retain(&mut self, predicate: impl Fn(&VirtualPageNumber) -> bool);
}

pub type SwapperImpl = ClockSwapper;

/// 页面置换算法基础实现：FIFO
pub struct FIFOSwapper {
    /// 记录映射和添加的顺序
    queue: VecDeque<(VirtualPageNumber, FrameTracker)>,
    /// 映射数量上限
    quota: usize,
}

impl Swapper for FIFOSwapper {
    fn new(quota: usize) -> Self {
        Self {
            queue: VecDeque::new(),
            quota,
        }
    }
    fn full(&self) -> bool {
        self.queue.len() == self.quota
    }
    fn pop(&mut self) -> Option<(VirtualPageNumber, FrameTracker)> {
        self.queue.pop_front()
    }
    fn push(&mut self, vpn: VirtualPageNumber, frame: FrameTracker, _entry: *mut PageTableEntry) {
        self.queue.push_back((vpn, frame));
    }
    fn retain(&mut self, predicate: impl Fn(&VirtualPageNumber) -> bool) {
        self.queue.retain(|(vpn, _)| predicate(vpn));
    }
}

pub struct ClockSwapper {
    vector: Vec<(VirtualPageNumber, FrameTracker, usize)>,
    // 可以分配的元素的最大个数
    quota: usize,
    // 时钟指针，永远指向一个可以分配的空间的下标，或是空间已满指向刚插入元素的下一个元素
    pointer: usize,
}

impl Swapper for ClockSwapper {
    fn new(quota: usize) -> Self {
        Self {
            vector: Vec::new(),
            quota,
            pointer: 0usize
        }
    }

    fn full(&self) -> bool {
        self.vector.len() == self.quota
    }
    // 这里默认 pop 不会在队列未满的时候调用
    fn pop(&mut self) -> Option<(VirtualPageNumber, FrameTracker)>{
        loop {
            if let Some(cur) = self.vector.get(self.pointer) {
                let (_, _, entry) = *cur;
                unsafe {
                    let mut entry_ref = (entry as *mut PageTableEntry).as_mut().unwrap();
                    entry_ref.set_flags(entry_ref.flags() - Flags::ACCESSED);
                    if entry_ref.flags().contains(Flags::ACCESSED) {
                        entry_ref.set_flags(entry_ref.flags() - Flags::ACCESSED);
                        self.pointer = {
                            if self.pointer + 1 >= self.quota {
                                0
                            } else {
                                self.pointer + 1
                            }
                        };
                    } else {
                        let t = self.vector.remove(self.pointer);
                        return Some((t.0, t.1));
                    }
                }
            } else {
                panic!("Error in ClockSwapper::pop: no current element({}: {})", self.pointer, self.vector.len());
            }
        }
    }
    // 这里默认 push 不会在队列满的时候调用
    fn push(&mut self, vpn: VirtualPageNumber, frame: FrameTracker, entry: *mut PageTableEntry) {
        self.vector.insert(self.pointer, (vpn, frame, entry as usize));

        unsafe {
            let mut entry_ref = *entry;
            let flags = entry_ref.flags();
            entry_ref.set_flags(flags | Flags::ACCESSED);
        }
        
        self.pointer = {
            if self.pointer + 1 >= self.quota {
                0
            } else {
                self.pointer + 1
            }
        };
    }

    fn retain(&mut self, predicate: impl Fn(&VirtualPageNumber) -> bool) {
        self.vector.retain(|(vpn, _, _)| predicate(vpn));
        self.pointer = self.vector.len();
    }
}