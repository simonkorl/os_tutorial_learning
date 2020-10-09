//! 内存模块
//! 
//! 
pub mod heap;
pub mod config;
pub mod address;

pub fn init() {
    heap::init();
}