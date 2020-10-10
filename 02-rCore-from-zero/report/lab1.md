# lab-1 实验报告

## 原理

### 在 rust_main 函数中，执行 ebreak 命令后至函数结束前，sp 寄存器的值是怎样变化的？

执行 ebreak 命令后触发一个断点中断，CPU 根据 stvec 中储存的处理函数入口位置（设定位置为 `handler.rs`）进入`interrupt.asm`文件所规定的函数。

此后根据该函数的描述，sp 寄存器的值减少`CONTEXT_SIZE * REG_SIZE`以分配用于保存 Context 结构体的空间，并且逐渐为其填充对应的信息。

之后跳转至`handle_interrupt`函数入口，此时`sp`没有变化。在执行`handle_interrupt`函数时，因为局部变量的储存等原因，编译器可能会加入变化`sp`的语句，但是当该函数返回时，`sp`的值与进入函数时是相同的。

`handle_interrupt`函数执行完毕后会执行`__restore`函数，在`__restore`函数最后的`LOAD    x2, 2`语句中`sp`被恢复为进入中断处理程序之前的数值。

## 分析

### 如果去掉 rust_main 后的 panic 会发生什么，为什么？

如果强行修改 rust_main 的返回值类型，那么去掉 panic 后，因为 rust_main 函数并不会返回，所以程序会继续运行 rust_main 之后内存中的代码输出，程序会出现未知结果。也可能发生内存读取位置读取错误。

实际上，如果不改变 rust_main 的返回值类型，rust_main 函数实际上并不允许返回，在编译阶段便会报错：

```rust
pub extern "C" fn rust_main() -> !{
    println!("Hello rCore-Tutorial!");
    // 初始化各种模块
    interrupt::init();

    unsafe {
        llvm_asm!("ebreak"::::"volatile");
    };
    
    //unreachable!();
}
```

结果：

```bash
error[E0308]: mismatched types
  --> src/main.rs:35:34
   |
35 | pub extern "C" fn rust_main() -> !{
   |                   ---------      ^ expected `!`, found `()`
   |                   |
   |                   implicitly returns `()` as its body has no tail or `return` expression
   |
   = note:   expected type `!`
           found unit type `()`

error: aborting due to previous error
```

如果强行修改 rust_main 的返回值类型，那么可能产生类似与下面的结果

```bash
panic: 'Unresolved interrupt: Exception(LoadFault)
Context { x: [80204a70, 802000da, 80216270, 0, 8001de00, 1, 82200000, 82200000, 802048f0, 8000000000006800, 802162c0, 672f6f677261632e, 802048f0, 2f6c726f6b6e6f6d, 0, 802160e4, 4, 1, 1, 8000000000006800, 80200000, 82200000, 0, 0, 2000, 0, 0, 0, 80200000, 0, 0, 0], sstatus: Sstatus { bits: 8000000000006120 }, sepc: 80203444 }
stval: 672f6f6772616340'
```

查看汇编代码可以发现：

```asm risc-v
000000008020127c rust_main:
8020127c: 39 71                         addi    sp, sp, -64
8020127e: 06 fc                         sd      ra, 56(sp)

0000000080201280 .LBB0_4:
80201280: 17 35 00 00                   auipc   a0, 3
80201284: 13 05 05 65                   addi    a0, a0, 1616
80201288: 0c 61                         ld      a1, 0(a0)
```

主函数非常简短，之后便是一些库的代码。在主函数运行结束后无法得知程序继续运行会发生什么结果。

## 实验

### 如果程序访问不存在的地址，会得到 Exception::LoadFault。模仿捕获 ebreak 和时钟中断的方法，捕获 LoadFault（之后 panic 即可）。

```rust
pub fn handle_interrupt(context: &mut Context, scause: Scause, stval: usize){
    // 可以通过 Debug 来查看发生了什么中断
    // println!("in handle_interrupt: {:x?}", scause.cause());
    match scause.cause() {
        // 断点中断(ebreak)
        Trap::Exception(Exception::Breakpoint) => breakpoint(context),
        // 时钟中断
        Trap::Interrupt(Interrupt::SupervisorTimer) => supervisor_timer(context),
        //* 增加一个处理函数
        Trap::Exception(Exception::LoadFault) => load_fault(context), 
        _ => fault(context, scause, stval),

    }
}
```


### 在处理异常的过程中，如果程序想要非法访问的地址是 0x0，则打印 SUCCESS!。

想要访问的地址是 0x0，那么错误地址便储存在 stval 中。可以进行判断：

```rust
fn load_fault(context: & Context, stval: usize){
    if stval == 0x0 {
        println!("SUCCESS!");
    }
    panic!(
        "Catch a LoadFault! {:?}\n",
        context
    )
}
```

## 添加或修改少量代码，使得运行时触发这个异常，并且打印出 SUCCESS!。

> 要求：不允许添加或修改任何 unsafe 代码

本题较难，需要考虑可以修改的位置。因为不能修改 unsafe 代码，所以就算希望跳转到 0x0 地址处也需要找到可以书写跳转代码的位置。有至少两个方法可以完成这个工作

1. 修改 Context 结构体中的 sepc 部分，这会直接影响程序的返回地址，且可以保证完整运行完毕`interrupt.asm`的全部代码，不会导致中断嵌套
2. 想办法令 rust_main 返回，并且在`entry.asm`中调用 rust_main 函数后书写类似与`jal 0x0`的代码进行跳转。这样可以成功初始化中断处理程序。

使用方法 1 可以更好地符合 rust_main 函数的定义，尽量不破坏其不可返回性。以下是一个实现方法。

```rust
// handler.rs
...
fn breakpoint(context: &mut Context) {
    println!("Breakpoint at 0x{:x}", context.sepc);
    context.sepc = 0;
}
```

结果如下

```rust
...
Hello rCore-Tutorial!
mod interrupt initialized
Breakpoint at 0x802012fa
SUCCESS!
panic: 'Catch a LoadFault! ...'
```