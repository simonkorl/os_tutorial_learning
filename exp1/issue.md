# 文档修改建议：环境部署部分

## 高优先度

### 严格按照“环境部署”章节的引导不能成功运行 rCore-Tutorial/os 代码

实际上，如果作为一个尚未接触过本项目的人，按照这个教程搭建环境后并不能达到“环境部署”章节最后一节描述的使用`make run`指令使得实验完成后的 rCore 运行起来。其问题是没有指令下面的指令：

```bash
# 增加RISC-V三元组
rustup target add riscv64imac-unknown-none-elf
# 增加需要的 cargo-binutils
cargo install cargo-binutils
rustup component add llvm-tools-preview
```

#118 提出了相同的问题，不过文档中令人疑惑的点依然没有得到解决。

这个问题在于为了解决“环境部署”章节的问题，需要在完成 Lab0 的过程中才能找到对应的解决方案。而“环境部署”明显是实验之前的内容，如果环境部署不能成功运行起 rCore 会令人非常沮丧并且浪费同学们大量的时间。在完成本实验的若干名同学（包括我在内）都明确表示遇到过这个问题：

- simonkorl/os_tutorial_learning/log/log.md 2020.9.29
- yunwei37/os-summer-of-code-daily Day6
- leonardodalinky/DailySche Day7

这个问题有几种解决方法，最为简单的一个就是修改“环境部署”章节的最后一节，说明在完成 Lab0 后即可成功运行完整的 rCore。或者是在“环境部署”部分就完成上述指令的执行，并且在 Lab0 进行解释。最后可以考虑将这些依赖作为配置文档进行记录，或是写在 Makefile 中。

另：根据文档说明的配置方法可以成功运行 [rCore](https://github.com/rcore-os/rCore) 的代码，但是 [rCore-Tutorial](https://github.com/rcore-os/rCore-Tutorial) 的代码不能成功运行。

## 中优先度

### 在“安装QEMU”部分提示 Linux 版本

我使用的是 Windows10 WSL 2 进行实验，根据“环境部署”的提示`在微软商店（Microsoft Store）中搜索 Ubuntu，安装第一个（或者你想要的版本）`安装 Ubuntu 后不能成功编译 QEMU-5.0.0 。报错信息为：

```bash
$ ./configure --target-list=riscv32-softmmu,riscv64-softmmu
ERROR: "cc" cannot build an executable (is your linker broken?)
```

在调试一段时间后依然不能成功编译。根据调试信息我发现自己安装的 Ubuntu 版本为 20.04，而 QEMU 依赖库的版本最高为 18.04。在重新从微软商店下载了 Ubuntu18.04 版本后就可以成功地按照本教程的指导进行环境配置了。

希望在 Windows10 WSL 2 小节下注明可以成功运行的 Ubuntu 版本，目前可以确定可以成功配置环境的 Ubuntu 版本为 18.04 。

## 低优先度

### 完善“环境部署”章节的 TODO 部分

“环境部署”一章中还有不少的部分尚未完成，使用 TODO 作为占位符。其中有一些部分实际上比较好填补。

比如“安装完成”标题下的“运行命令”

```bash
# 克隆仓库并编译运行
git clone git@github.com:rcore-os/rCore-Tutorial.git
# 或是 git clone https://github.com/rcore-os/rCore-Tutorial.git
cd rCore-Tutorial
git checkout master

# 编译运行
make run
```

以及“运行输出”

```bash
$ make run

make[1]: Entering directory '/**/rCore-Tutorial/user'
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
Targets: notebook hello_world
Image resized.
make[1]: Leaving directory '/**/rCore-Tutorial/user'
make[1]: Entering directory '/**/rCore-Tutorial/os'
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s

OpenSBI v0.6
   ____                    _____ ____ _____
  / __ \                  / ____|  _ \_   _|
 | |  | |_ __   ___ _ __ | (___ | |_) || |
 | |  | | '_ \ / _ \ '_ \ \___ \|  _ < | |
 | |__| | |_) |  __/ | | |____) | |_) || |_
  \____/| .__/ \___|_| |_|_____/|____/_____|
        | |
        |_|

Platform Name          : QEMU Virt Machine
Platform HART Features : RV64ACDFIMSU
Platform Max HARTs     : 8
Current Hart           : 0
Firmware Base          : 0x80000000
Firmware Size          : 120 KB
Runtime SBI Version    : 0.2

MIDELEG : 0x0000000000000222
MEDELEG : 0x000000000000b109
PMP0    : 0x0000000080000000-0x000000008001ffff (A)
PMP1    : 0x0000000000000000-0xffffffffffffffff (A,R,W,X)
mod memory initialized
mod interrupt initialized
mod driver initialized
.
..
notebook
hello_world
mod fs initialized
hello from kernel thread 7
thread 7 exit
hello from kernel thread 6
thread 6 exit
hello from kernel thread 5
thread 5 exit
hello from kernel thread 4
thread 4 exit
hello from kernel thread 3
thread 3 exit
hello from kernel thread 2
thread 2 exit
hello from kernel thread 1
thread 1 exit
hello from kernel thread 8
thread 8 exit
src/process/processor.rs:87: 'all threads terminated, shutting down'
```

### 提供速度更快的 QEMU-5.0.0 下载来源

“安装 QEMU”一节中提供的下载链接`https://download.qemu.org/qemu-5.0.0.tar.xz`在中国大陆的下载速度确实非常慢，但是注释中描述的“我们提供的地址”依然处于 TODO 状态，如果可以的话希望能提供国内的下载地址。

### 提示 Rust 安装完成后配置环境变量

在“安装 Rust 工具链”后没有提示配置环境变量，可能会导致有一些疑惑。虽然 Rust 安装工具在最后会提示配置环境变量，但是提示并不是很明显。作为教程我认为应该给予一些提示（实际上 Lab0 中便有相关的环境变量配置的提示，可以将其提前至环境部署部分）。
