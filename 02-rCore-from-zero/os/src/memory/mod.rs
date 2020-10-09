//! 内存模块
//! 
//!

// 因为模块内包含许多基础设施类别，实现了许多以后可能会用到的函数，
// 所以在模块范围内不提示「未使用的函数」等警告
#![allow(dead_code)]

pub mod heap;
pub mod config;
pub mod address;
pub mod frame;
pub mod range;

// 一定要记住写这一行引用！
pub use {address::*, config::*, frame::FRAME_ALLOCATOR, range::Range};

/// 一个缩写，模块中一些函数会使用
pub type MemoryResult<T> = Result<T, &'static str>;

pub fn init() {
    heap::init();

    //* 这一行代码先复制防止之后的问题，之后的 lab 应该有解释
    // 允许内核读写用户态内存
    unsafe { riscv::register::sstatus::set_sum() };

    println!("mod memory initialized"); 
}