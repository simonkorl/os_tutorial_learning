//! 内存模块
//! 
//! 
mod heap;
mod config;

pub fn init() {
    heap::init();
}