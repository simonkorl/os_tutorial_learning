## 2020.10.5

写完了 lab1 但是程序无法运行。运行方法是直接将`main.rs`中的最后一句话注释，并且将返回改为()。

报错为：

```bash
...

Hello rCore-Tutorial!
mod interrupt initialized
in handle_interrupt: Exception(Breakpoint)
Breakpoint at 0x802012ba
in handle_interrupt: Exception(LoadFault)
panic: 'Unresolved interrupt: Exception(LoadFault)
Context { x: [1, 802000da, 80216360, 0, 8001de00, 1, 82200000, 82200000, 80204960, 8000000000006800, 802163b0, 672f6f677261632e, 80204960, 2f6c726f6b6e6f6d, 0, 802161d4, 4, 1, 1, 8000000000006800, 80200000, 82200000, 0, 0, 2000, 0, 0, 0, 80200000, 0, 0, 0], sstatus: Sstatus { bits: 8000000000006120 }, sepc: 802034b6 }
stval: 672f6f6772616340'
```

## 2020.10.6

lab1的`handler.rs`中的注释部分有一点奇怪。利用 Debug 查看中断应该用下面的语句，否则编译无法通过：

```rust
println!("in handle_interrupt: {:x?}", scause.cause());
// println!("{:x?}", context.scause.cause()); // context 没有 scause 成员
```

其实根据测试结果也可以想到为什么昨天写的程序最后会报错，因为现在的 rust_main 函数是不允许进行返回的。在返回后会发生错误（具体原因有待学习），所以会有如此的报错。

如果想要得到实验对应的结果，可以在`main.rs`函数中增加一个`loop`来解决这个问题。

```rust
pub extern "C" fn rust_main() -> !{
    println!("Hello rCore-Tutorial!");
    // 初始化各种模块
    interrupt::init();

    unsafe {
        llvm_asm!("ebreak"::::"volatile");
    };
    
    loop{};
    //unreachable!();
}
```

**注**：不要试图改变 rust_main 的返回类型
