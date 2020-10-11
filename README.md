# rCore 学习记录

## TOC


九月

| Mon  | Tue  | Wed  | Thu                       | Fri  | Sat                      | Sun                       |
| ---- | ---- | ---- | ------------------------- | ---- | ------------------------ | ------------------------- |
|      | 1    | 2    | 3                         | 4    | 5                        | 6                         |
| 7    | 8    | 9    | 10                        | 11   | 12                       | 13                        |
| 14   | 15   | 16   | 17                        | 18   | 19 ([Day 1](#2020.9.19)) | 20 ([Day 2](#2020.9.20)) |
| 21  ([Day 3](#2020.9.21)) | 22  | 23   | 24 ([Day 6](#2020.9.24)) | 25  ([Day 7](#2020.9.25)) | 26     ([Day 8](#2020.9.26))                  | 27   ([Day 9](#2020.9.27))                     |
| 28 ([Day 10](#2020.9.28))  | 29([Day 11](#2020.9.29))   | 30 ([Day 12](#2020.9.30))  |                           |      |                          |                           |

十月

| Mon  | Tue  | Wed  | Thu  | Fri  | Sat  | Sun  |
| ---- | ---- | ---- | ---- | ---- | ---- | ---- |
|      |      |      | 1  ([Day 13](#2020.10.1))  | 2  ([Day 14](#2020.10.2))  | 3    | 4    |
| 5    | 6    | 7    | 8    | 9    | 10   | 11   |
| 12   | 13   | 14   | 15   | 16   | 17   | 18   |
| 19   | 20   | 21   | 22   | 23   | 24   | 25   |
| 26   | 27   | 28   | 29   | 30   | 31   |      |

---

## 2020.9.19

完成Hello World部分的学习

根据这个教程来说，这部分的学习稍微有一点困难。因为我们并不是非常了解rust语言的trait机制，也不太清楚一些接口与实现的部分。给人的感觉是这部分的内容与java，python和c都有相似之处。

有关打印的部分非常类似于python的print方法。有关方法重载的Debug部分非常类似于python的magic function。这部分如果学习过python应该会理解起来更加简单。

复写的rust函数也与python有异曲同工之处，都是具有一个self变量，格式上python也可以这么标记，只不过写法不太一样。

我的话建议在看这个教程之前至少对一门常见的编程语言非常熟悉，比如说C，C++，Python，Java这些。至少需要对程序中出现的大多数东西很熟悉。此外我觉得在看这个教程前对rust的一些基本概念，比如说数据类型和所有权有一个基础的认识，看这个教程的难度应该会下降不少。

## 2020.9.20

完成Primitive部分的学习

很多的数据类型和python很相似。比较难的是理解Array的所有权，但是这一章并没有提这件事，只是说了borrow

Array在编译的时候就确定了大小，不能改变。编译的时候便会动态检查可能出现的访问异常。

## 2020.9.21

> 这一天换了参考资料，主要学习了Tour of Rust。这边的每一章内容都更小，接触核心概念更早，我认为更加适合第一次接触时学习。
>
> Rust By Examples 中很多概念可能会混淆在一起，后面的东西还没有学习到的时候就已经开始使用到了。对于刚开始学习Rust的人来说并不是很友好。

流程控制：

if与python很像，并不需要写括号，但是所有的运算符与C是相同的

循环：

while 与所有语言的while工作原理相同，没有do while语法

loop 相当于while(1)

for 与python的for含义相同，只能遍历迭代器。使用0..4这样的标志表示数字迭代器，左闭右开。如果使用0..=4则两边都是闭。

match：非常关键的关键词，可以将条件进行分发。目前可以完成如同switch的工作，可以判断条件是否落在某个区间，并且可以记录下落在区间上的数值。

返回值：

rust的流程返回值非常有趣，对于任何一个块，只要最后一个语句是一个没有分号的表达式，那么编译器会认为它是一个返回值，否则则不是一个返回值。所有if分支的返回值类型必须相同（或者共享一个基类）（因为rust是强类型语言，一个函数只能返回特定类型的）。这部分如果编译原理课程进行过java实验的话，对于这部分肯定会有印象，其中有一个实验就是要判断block的返回值是否类型相同。

结构体：结构体的声明类似于很多类C语言，需要标记每一个元素的类型，与python不尽相同。比较类似typescript

enum：枚举类更像是C的union，可以拥有很多不同类型但是每次只能选用一个，占用其中最大的空间的数据结构。

模板<T>，与类C语言的模板没有太大区别。手动声明模板类型时需要使用::<T>，

Option<T>: 原生enum类型，只有None与Some<T>两种类型。可以靠item.is_some()或者item.is_none()来判断是否具有内容。不会像javascript那样不确定是否存在某个元素

Result<T,E>：原生enum类型，只有Ok(T)与Err(E)两个选项

？标志：代表一个操作可能成功或失败，失败后不执行后续操作。

所有权：

和C++比较像的一点是rust会在每一个scope结束时按照创建顺序的倒序回收临时变量，除非它的所有权发生了转移。这个释放的过程被称为drop。和C++的类的析构基本上一致，构造的时候从最内层开始构造，析构的时候从最外侧开始析构。

### 所有权的操作

1. move：使用函数传递的时候所有权发生移动。如果该变量没有被返回，则该变量将会在函数的最后释放
2. borrow：使用&在变量前面，与C++的reference一致。你将获得一个类似于所有权的常量指针，可以对内容进行访问，但是不能修改。
3. mutable borrow：使用&mut变量可以获得一个可以修改的所有权，类似于C++的普通reference，如果想要修改原始的数据，原始数据也需要是mutable的（就像C++中你不可以利用reference修改常量）。在mutable borrow时原拥有者不能移动或是修改（可以认为这是一个同步的过程，一次只允许一个所有者对一个数据进行修改，但是允许多个所有者对变量进行读操作。这在异步操作时可以带来较大的安全性）
4. \*：rust没有指针的概念，\*只是用来取值的。利用 \*mutable borrow 来修改reference的值。同时可以使用\*对一个mutable borrow的原始值进行一个值的拷贝，获得一个全新的数据。
5. 两条规则：一、任何一个数据只能同时有一个可变ref或是若干不可变ref并且这两者只能取其一。可以认为rust认为变量应该是一个读写的互斥操作；二、任何ref不能比owner的存活时间长
6. 注：owner和reference不是一个概念。owner是一个值绑定的变量，可以有很多个reference从owner那里获取使用权，但是需要满足读写互斥的规则。owner释放之后，所有的reference都会失效。和操作系统的概念进行比较的话，owner类似于inode，data就是储存的数据，reference是硬链接。不过概念不同的地方是，一旦有了硬链接，原始的inode就不能用于访问。
7. 注：rust的&符号优先级比较低，&foo.x是返回x的reference，但是C++的&foo.x返回的是对应地址位置的x的值。
8. 生命周期标识符\'，在函数中可以使用<\'a>等符号表示变量的生命周期，这个生命周期的标注必须正确
9. static生命周期，所有标记为static的变量据具有`'static`的生命周期并且所有对它的reference必须也是`’static`生命周期

## 2020.9.24

> 了解了一些基础后回归 RBE（Rust By Example）

### Variable Bindings

Rust的变量定义和javascript和python有一定相似度，内存的分配和变量名的定义可能有一定区别。C与C++则是每一个变量名都会直接分配对应的空间和地址。

1. 虽然Rust是强类型语言，但是variable shadowing可以将不同类型的变量绑定到同一个变量上。但是之前的变量便会被丢弃。（使用 let，这个关键词在javascriptES6中被提出，代表一种块作用域。如果学过编译原理的话肯定对这些作用域印象深刻）
2. 变量可以先用let声明之后绑定值，不过用法不常见。在C中还是非常常见的操作。
3. 可以使用名字相同的变量进行freeze操作，即取消它在作用域中的可变特征。不过即使不进行freeze操作，只要作用域不同还是可以使用相同的变量名进行shadowing

```rust
fn main() {
    let _mutable_integer = 7i32;
    println!("{}", _mutable_integer);
    {
        // Shadowing
        let _mutable_integer = 'a';
        println!("{}", _mutable_integer);

        // `_mutable_integer` goes out of scope
    }

    println!("{}",_mutable_integer);
    // 7
    // a
    // 7
}

```

### Types

1. 类型转换：不允许未声明清楚类型的强制类型转换，同时有一些限制。但是除此之外和C应该是一样的，在二进制层面上原理相同。
2. 数字Literal（也就是直接声明的数字）可以直接在后面标注类型`2u8`，大小和描写的相同。数字默认为整型32与浮点64，与64位C++相同，
3. Rust的编译器很强大，有的时候可以可以自动推断一些变量的类型，比如说vec<>
4. 定义的类型必须是UpperCamelCase

### Conversion

大类型之间的转化。From是一个trait，或者说python中的magic function。实现一个泛型的实例就可以完成类型的转化，Into是它的逆向操作。

1. TryFrom TryInto 带有Result和可能错误的转化
2. 与字符串的转化：可以实现ToString函数，不过直接实现fmt::Display会更加方便

### Expression

> 编译原理永远的痛

使用;结尾的句子都是表达式，{}也是表达式。这个在编译原理中应该已经深有体会，完成的任务中如果有Lambda表达式的话则是让人深恶痛绝。

### Flow of Control

流控的不少内容之前也看过了，其中match的用法非常灵巧。记住match对应的default是符号`_`

Rust的reference和pointer和C++非常类似。其中最大的区别相当于是在C++中默认增加了const关键字。

:star:if let，这个关键字在C与C++中还比较常见，是用来赋值并且判断，判断出的结果需要有错误的类型，比如说None（来自Option）

### Functions

rust的函数不需要担心函数的先后顺序。

无返回值的函数不需要标注返回值类型，默认返回()，空tuple是最基本的默认类型。

#### Methods

非常神奇的是，Rust的类的概念似乎比较散。所有的结构体、tuple都是可以被定义成员函数的。

所有没有被传入self的函数都被认为是静态函数

#### 闭包（lambda表达式）

> 又到了编译原理的噩梦了

rust的lambda表达式是我见过lambda表达式中外形最为奇怪的。

```rust
let closure_annotated = |i: i32| -> i32 { i + 1 };
let closure_inferred  = |i     |          i + 1  ;
```

使用`| |`代替小括号进行函数的编写。可以非常智能地进行类型分析。

但是lambda表达式本身的内容是需要考虑很多有关所有权的内容的。

关于lambda表达式的mut标志，实际上和C++的const function有相似之处。对于rust，只要lambda当中需要改变一个mut变量，那么就需要标志为mut类型，来借用变量。（在C++中的变量引用需要手动标注是按值或按ref）

除非明确在lambda表达式前面标记move，否则都是直接借用而不是挪用。也就是默认按照ref进行变量借用。

##### 传入lambda表达式

> 我曾经在Java中对java的lambda表达式深恶痛疾，其中关键之处就是Java对于Lambda表达式有一些非常奇怪的定义，Lambda表达式本质上是某些函数，必须要契合某种Interface



Rust在这方面可能有些相似。传入的闭包必须明确标注一些类型：

* Fn：按不变ref传值
* FnMut：按可变ref传值
* FnOnce: 按值传值，这会导致move 

### Rustlings练习

> 闭包的知识实在是太多了，没有耐心来看了。做一些简单的练习题巩固一下之前的知识

根据建议，下面的做题顺序与`rustlings watch`命令编译的顺序相同。

主要记录一下自己在哪里做错了，或是不太清楚的地方。

#### variables

variables6：const变量命名需要显式指定变量类型

#### if

if1: 经典的二元选择式，别忘了加括号

#### functions

##### functions2

Rust的函数需要显式指定输入变量的类型。如果没有输出则可以不指定输出的类型。【这么做的想法应该是在静态检查的时候让编译器明白你其实是想做些什么】

##### functions5

仔细看，那个函数表达式最后有分号！这种错误简直就是找不同，幸好编译器会给提示

#### primitive_types

##### 3

Rust中的数组概念稍微优点不同，定义与初始化方法要记清楚`[;]`

##### 4

> Beginner’s luck要用完了，语法特点要开始发威了！

slice并不需要声明类型，但是需要使用&a[1..4]进行借用。还要记住，slice标志中所有的都是下标。

##### 5

tuple的展开不需要指定类型

##### 6

tuple不像数组那样用下标，而是用`.2`这样的数字对象进行访问

## 2020.9.25

### Rustlings

#### struct

##### 1

1. struct当中的String类型最好不要使用str抑或是&str，在[rust doc](https://doc.rust-lang.org/stable/book/ch05-02-example-structs.html)中有相关的介绍。其中最关键的是Rust要求struct结构体在编译时可以确认在栈上的大小，或者最多有一个元素不能确定大小。

2. 其中为了得到String需要使用String::from(“string”)函数。

3. 定义结构体中使用`,`分割每一个元素，这和javascript是一样的，但是和C是不同的

4. 为了得到UnitStruct，虽然它的含义确实与()基本相同，但是你需要为其起一个名字

   ```rust
   let unit_struct = ();
   println!("{:?}s are fun!", unit_struct);
   // ()s are fun!
   let unit_struct = {};
   println!("{:?}s are fun!", unit_struct);
   // ()s are fun!
   let unit_struct = UnitStruct{};
   println!("{:?}s are fun!", unit_struct);
   // UnitStructs are fun!
   ```

##### 3

1. 利用panic!(“string”)进行panic

#### Strings

> 实际上，在写C++的时候我就因为std::String与const char*的事情而头疼过。Rust在这里也有类似的概念

##### 2

1. 这道题的正解应该是直接在传入函数的时候加上`&`符号来传入String slice，而不是修改函数的接收者

#### quiz2

非常有趣的quiz，来判断字符串是String还是string_slice

1. format!宏返回的是一个String
2. “string_slice”.trim()返回的是string_slice

#### enums

Rust的enums和其他语言的enum相比非常灵活多变。因为Rust的enums里面的每一个内容都可以视为一个struct，每一个枚举类都可以用于储存值，并且加上Rust的成员函数定义的原理，相当于Rust的enums相当于是其他语言的一个类。

### RBE

> 知识不够用了，必须继续进行学习了

#### 闭包

闭包这部分的定义基本上都在说明Rust语言的函数式特征，包括将函数作为输出，作为返回值等等。

闭包部分比较关键的一点就是传递的函数要求显式地指定函数的泛型类型，包括Fn，FnMut和FnOnce

##### 函数返回值

可以利用闭包进行函数的返回，但是需要符合一下的格式：

```rust
fn create_fn() -> impl Fn() { //使用impl 关键字实现 Trait
    // Fn, FnMut, FnOnce都是Trait
    let text = "Fn".to_owned();

    move || println!("This is a: {}", text) //必须有move关键字
}
```

必须使用move的关键字的原因是在实现trait时如果不移动变量会导致impl结束后变量已经消失，但是返回的闭包中的ref还在。

教程中的Examples in std，其中说明了vec与array的迭代器iter与any，find函数的不同。如果觉得Rust这块的引用理由很奇怪，不妨把它当作C++的指针，考虑一下C++的std::vector的迭代器操作，可能会更加清晰。

Higher Order Functions 不妨称之为高等函数

这些函数比如说map,filter在函数式语言中都非常常见。Option与Iterator中拥有这些函数。具体的实现估计未来还得看文档。

不返回类型：

函数可以使用!说明该函数不会返回任何东西，或者说它根本就不会返回。这不同于返回类型为（）的函数，在Rust语言中返回（）即为其他语言的void类型。有的时候这种不返回类型可以作为expression的一个选项，这样在一些分支的判断中不返回的类型也可以认为是一种任意类型。【就像空集是任何集合的子集一样】

### Modules

> 终于又能看到类似于类的结构了

Modules感觉上和Java的类的概念比较近似，也具有visibility，默认为private，除非特意加上pub关键字标注每一个函数和属性。

Modules里面的pub struct需要额外使用pub声明可见的变量。

Modules的概念不仅是类的概念，还具有路径的概念，即可以用use关键字像是import一样进行模块的调用。super与self关键字也并不是父类和子类的关系，而是作用域的概念。super代表上一层作用域，self为自己这层作用域。

如果要在相同路径下访问某个Module，需要进行declare

### Crates

可以理解为Rust的lib。和module有一定的相似之处，就是都可以引用，不过关键词是 extern crate。使用library的名称来进行引用。这一块和Java有一些类似。

### Cargo

如同Java的Gradle、python的pip类似的包管理工具。附赠了很多实用功能，到时候再看吧。

### Attributes

在文件中的配置信息，有一点点类似C中的一些宏定义。不过有的配置更加高级。

#### macro_rules!

Rust的宏作为抽象语法树展开，可以避免像是C直接根据字符串展开时可能遇到的错误。Rust的宏可以认为是某种函数。

定义方法比C复杂许多，首先先要写一个宏的表达式（字符串表示），其间可以加入变量，而变量需要规定一个称之为Designator的东西，也就是输入的类型。甚至可以使用类似于正则表达式的语法或是match的语法。东西比较多，可能需要之后细看。【实际上宏的原理都差不多，都是某种程度的字符串改造，不过Rust的改造更加安全】

由于Expr表达式的全能性，Rust的宏可以完成很多功能，比如说实现一个新的关键字。

### 新学习内容的Rustlings

#### macro

##### 2

和函数不同，宏的定义与其他的模块是有先后顺序的。需要前面定义后后面的程序才能用到。

##### 3

```rust
// macros3.rs

#[macro_use] //增加这句话，具体原因未知
mod macros {
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!();
}
```

#### move_semantics

##### 3

```rust
// move_semantics3.rs
// Make me compile without adding new lines-- just changing existing lines!
// (no lines with multiple semicolons necessary!)
// Execute `rustlings hint move_semantics3` for hints :)

// I AM NOT DONE

fn main() {
    let vec0 = Vec::new();

    let mut vec1 = fill_vec(vec0);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {//只需要在vec前面加一个mut就可以了
    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
```

## 2020.9.26

### Rustlings 拟合学习法

> 众所周知，考试前最好的复习方法就是直接做题，发现不会的了再去针对性地复习，我的话正在使用这种方法快速过rust的语法部分

#### Error_handling

##### 1

Rust通过自带的Result枚举类进行错误的判断，其包含Ok和Err两种不同的泛型，可以储存指定类型的信息。目前看来和许多语言的try catch的思想并不是非常一致。但是稍微有一点类似于Java的处理方法

##### 2

match表达式整个算是一个表达式，里面的每一个选项使用`,`分割。最后如果不屑分号的话按照返回值处理每一个选项。

##### 3

```rust
// errors3.rs
// This is a program that is trying to use a completed version of the
// `total_cost` function from the previous exercise. It's not working though!
// Why not? What should we do to fix it?
// Execute `rustlings hint errors3` for hints!

use std::num::ParseIntError;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>>{
    let mut tokens = 100;
    let pretend_user_input = "8";

    let cost = total_cost(pretend_user_input)?;

    if cost > tokens {
        println!("You can't afford that many!");
    } else {
        tokens -= cost;
        println!("You now have {} tokens.", tokens);
    }
    Ok(())
}

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}
```

根据文档的提示，第三道题最好的解法应该是这样，使用一个可以描述的全局错误类型Box\<dyn Error\>并且让main函数返回Ok(())。要注意Ok中是要包括内容的。

errorsn

有了error3的经验后这道题就不难了，但是各个地方都有可能出错，要注意在各种地方写出?

#### Option

> * Option能有多难？
> * 好难啊

##### 2

```rust
// option2.rs
// Make me compile! Execute `rustlings hint option2` for hints

// I AM NOT DONE

fn main() {
    if let Some(value) = Some(String::from("rustlings")) {
        println!("the value of optional value is: {}", value);
    } else {
        println!("The optional value doesn't contain anything!");
    }

    let mut optional_values_vec: Vec<Option<i8>> = Vec::new();
    for x in 1..10 {
        optional_values_vec.push(Some(x));
    }

    // TODO: make this a while let statement - remember that vector.pop also adds another layer of Option<T>
    // You can stack `Option<T>`'s into while let and if let
    while let Some(Some(value)) = optional_values_vec.pop() {
        println!("current value: {}", value);
    }
}
```

option2练习题中的嵌套Some的结构模式非常好地展示出了Rust的语法特点。Rust在很多结构体上都可以这样进行结构操作，而不使用unwrap函数。当然，这样做的前提是使用了if let和while let两个可以自动判断是否解构错误的关键词。

### 重新学习时间

> 虽然说如果有英文文档就直接看英文文档学习比较好。但是我今天发现好像看中文的文档学习得更快一些

#### trait

Rust感觉上包括了绝大部分函数式语言的特点，并且目前看上去并不像是一门面向对象的语言，因为李米娜没有非常明显的类的概念。虽然之前具有有一些像类的概念的Module，但是它也只满足面向对象的“封装”的特点，而不满足“继承”与“多态”。

trait主要满足面向对象的”继承“和”多态”的特性。与其说是类，倒不如说是Java的接口的概念。

之前提到过每一个struct或是tuple定义的类型都可以定义自己的成员函数Method，trait可以认为是这类成员函数的一个集合，所以说它的概念有一点像接口，不过trait没有可见性的问题，所有trait中的成员函数均是对外暴露的。这个特征使得其与接口还是有一些区别，更加像是javascript中一个叫做mixin的概念。

trait可以使用#derive[]进行继承，继承的概念基本上就是为这个struct或tuple添加了这些函数的基本实现，如果想要overwrite则可以直接使用Impl进行实现即可进行重写。

trait也可以用于运算符重载，重载Drop,Iterator以及Clone的特征，这部分也和python的magic function比较像，具体情况要具体分析

## 2020.9.27

### Rustling conitnue…

#### arc

Rust提供了自己的同步关键字和锁，不过想要实现还真的不是那么容易。

这个实现模仿了网上介绍文档中的内容，不确定是否为最简单的实现法。

```rust
#![forbid(unused_imports)] // Do not change this, (or the next) line.
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();
    let shared_numbers = Arc::new(Mutex::new(numbers));// TODO
    let mut joinhandles = Vec::new();

    for offset in 0..8 {
        let child_numbers = shared_numbers.clone();
        joinhandles.push(thread::spawn(move || {
            let mut i = offset;
            let mut sum = 0;
            while i < child_numbers.lock().unwrap().len() {
                sum += child_numbers.lock().unwrap()[i];
                i += 5;
            }
            println!("Sum of offset {} is {}", offset, sum);
        }));
    }
    for handle in joinhandles.into_iter() {
        handle.join().unwrap();
    }
}

```



#### iterator

##### 2

需要记住Option和Iterator两个结构具有高阶函数，也就是函数式编程的一些函数。这道题的一些细节之处我也是参考了相关章节的内容才能够完成，不一定是最好的解法。

```rust
pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),// 这一行增加了.to_uppercase()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Step 1.
    // Tests that verify your `capitalize_first` function implementation
    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    // Step 2.
    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        let capitalized_words: Vec<String> = words.iter().map(|x| capitalize_first(x)).collect(); //这行与下一个测试都使用了同样的表达式，但是答案并不相同。原因是前面的变量类型不一样
        assert_eq!(capitalized_words, ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        let capitalized_words: String = words.iter().map(|x| capitalize_first(x)).collect();
        assert_eq!(capitalized_words, "Hello World");
    }
}
```

##### 3,4

iterator3和4的难点都是了解如何使用iterator的高阶函数（Higher Order Functions），分别从collect()和fold()两个函数介绍。要做对这部分需要了解高阶函数的使用方法，属于函数式编程的特点，对于我这种一直学的是OOP的人来说不是很友好。

```rust
// iterators3.rs

#[derive(Debug, PartialEq, Eq)]
pub enum DivisionError {
    NotDivisible(NotDivisibleError),
    DivideByZero,
}

#[derive(Debug, PartialEq, Eq)]
pub struct NotDivisibleError {
    dividend: i32,
    divisor: i32,
}

// This function should calculate `a` divided by `b` if `a` is
// evenly divisible by b.
// Otherwise, it should return a suitable error.
pub fn divide(a: i32, b: i32) -> Result<i32, DivisionError> {
    if b == 0{
        Err(DivisionError::DivideByZero)
    } else if a % b == 0{
        Ok(a / b)
    } else {
        Err(DivisionError::NotDivisible(NotDivisibleError{ dividend: a, divisor: b}))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests that verify your `divide` function implementation
    #[test]
    fn test_success() {
        assert_eq!(divide(81, 9), Ok(9));
    }

    #[test]
    fn test_not_divisible() {
        assert_eq!(
            divide(81, 6),
            Err(DivisionError::NotDivisible(NotDivisibleError {
                dividend: 81,
                divisor: 6
            }))
        );
    }

    #[test]
    fn test_divide_by_0() {
        assert_eq!(divide(81, 0), Err(DivisionError::DivideByZero));
    }

    #[test]
    fn test_divide_0_by_something() {
        assert_eq!(divide(0, 81), Ok(0));
    }

    // Iterator exercises using your `divide` function
    
    #[test]
    fn result_with_list() {
        let numbers = vec![27, 297, 38502, 81];
        let division_results = numbers.into_iter().map(|n| divide(n, 27));
        let x: Result<Vec<i32>, DivisionError> = division_results.collect();//... Fill in here!
        assert_eq!(format!("{:?}", x), "Ok([1, 11, 1426, 3])");
    }

    #[test]
    fn list_of_results() {
        let numbers = vec![27, 297, 38502, 81];
        let division_results = numbers.into_iter().map(|n| divide(n, 27));
        let x : Vec<Result<i32, DivisionError>> = division_results.collect();//... Fill in here!
        assert_eq!(format!("{:?}", x), "[Ok(1), Ok(11), Ok(1426), Ok(3)]");
    }
    
}

```

```rust
// iterators4.rs


pub fn factorial(num: u64) -> u64 {
    // 这是函数式编程的阶乘写法，有的时候fold函数也会成为reduce, inject
    (1..=num).fold(1, |acc, x| acc * x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}

```

#### trait

##### 1

String类型的函数的`+`重载可以与str进行连接，但是不能和String进行连接，这一点挺奇怪的

```rust
trait AppendBar {
    fn append_bar(self) -> Self;
}

impl AppendBar for String {
    //Add your code here
    fn append_bar(self) -> Self {
        self + "Bar"
        // self + String::from("Bar") error
    }
}
```

##### 2

trait2看起来和trait1很像，但是最终要求返回一个Vec类型的Self，此时vec使用过push函数来延长，发生了改变，所以需要在输入的地方定义 mut self。

然而有趣的是trait定义函数的时候并没有规定self的可变属性，但是实现的时候可以将只能读取的借用变成可变借用，不知道这是一个什么原理。

```rust
trait AppendBar {
    fn append_bar(self) -> Self;
}

//TODO: Add your code here

impl AppendBar for Vec<String> {
    fn append_bar(mut self) -> Self{
        self.push(String::from("Bar"));
        return self;
    }
}
```

#### generics

> C++与Java的泛型已经让我非常头疼了，Rust的泛型似乎没有好到哪里去

##### 1

Vec不能储存储存大小在编译期间不能得知的内容

```rust
fn main() {
    // let mut shopping_list: Vec<str> = Vec::new(); // error
    let mut shopping_list: Vec<&str> = Vec::new();
    shopping_list.push("milk");
}
```

##### 2

trait的泛型 impl 需要在 impl 后面加上\<\>

```rust
struct Wrapper<T> {
    value: T
}

impl<T> Wrapper<T> {
    pub fn new(value: T) -> Self {
        Wrapper::<T> { value }
    }
}
```

##### 3

我之前正好不太明白：trait在impl的时候为什么指定的是其他的trait而不是可以多指定几个限定的实现类型？第三题展示出来的就是为什么要求泛型T需要指定trait，因为T只需要在未来拥有相同的行为（函数）就可以认为其可以在泛型中被正确使用。与其声明需要实现的类型，不如说明实现的类型需要满足的最小交集的trait的是什么更加方便。

```rust
pub struct ReportCard<T> {
    pub grade: T,
    pub student_name: String,
    pub student_age: u8,
}

impl<T: std::fmt::Display> ReportCard<T> {
    // 如果泛型编写错误的话编译器会有对应的错误提示
    // T需要满足的最低要求就是可以打印，即拥有Display定义的函数fmt
    pub fn print(&self) -> String {
        format!("{} ({}) - achieved a grade of {}", 
            &self.student_name, &self.student_age, &self.grade)
    }
}
```

#### thead

> 这一个练习就应该早一点出现

学过操作系统的话对于Mutex（互斥量）应该已经丝毫没有任何疑问了。Rust里面似乎是直接提供了一个互斥锁的实现。利用这个互斥锁包裹变量即可进行简单高效的多线程间共享数据的互斥访问。

【Rust中似乎包含了很多这种具有“包裹”性质的结构，需要调用unwrap函数或者别的方法来进行解构。Rust中Vec函数给出返回结果一般也是Option，需要手动展开】

但是问题是Rust的数据有比较复杂的释放机制，如同文件管理一样需要进行引用计数。多线程之间很可能出现计数问题。为了防止这种问题，线程之间共享或者需要在线程之间传递的资源需要一种在多线程状态下也很安全的引用方式，这就是Arc的作用。Arc就是一个线程安全的引用结构，就像Box是一种堆数据的自动指针一样，Arc是一个线程上的自动指针。

```rust
fn main() {
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let status_shared = status.clone();
    thread::spawn(move || {
        for _ in 0..10 {
            thread::sleep(Duration::from_millis(250));
            status_shared.lock().unwrap().jobs_completed += 1;
        }
    });
    while status.lock().unwrap().jobs_completed < 10 {
        println!("waiting... ");
        thread::sleep(Duration::from_millis(500));
    }
}
```

#### conversion

> Rust 作为强类型语言，类型的转换相对于C严格了不止一点

##### using_as

编译器报错很神奇的一点是Rust的除法实际上不支持浮点数除以整数。可能其他语言在实现上也无法做到让浮点数除以整数，只能先把整数隐式转化为浮点数之后再来处理。Rust要求类型转化是显式的。

##### try_from_into

现在的Rust支持使用Range.contains来快速判断一个数是否在Range中。Range就是使用0..2这样的方法构建的结构。可以用这个函数快速进行判断。

##### as_ref

as_ref是另一道可以展示出 trait bound 用处的题目。trait bound 可以展示说明一个类型究竟需要怎样的函数才能使得它在泛型中是通用的，在所有泛型的定义中都可以做到这一点。方法是在定义泛型的时候利用`<T: trait + trait>`的方法进行声明。这有的时候比较像是多继承，不过有一些区别。

### Rustlings 练习完成！

在这几天的时间内，我也算是照着 Rustlings 的练习把 Rust 的一些概念快速地过了一遍。

我的话属于那种需要动手做一做才能记住的人，所以需要一些练习来加强自己的理解。 Rust by Example 虽然有一些在线训练，不过有的部分讲得太快，不够细致，并且前后的顺序什么的也不是非常方便学习，小练习的难度有的时候太低，比如说经常有那种取消注释的题目，也不能说是非常有助于概念的理解。

总而言之，我对 Rustlings 的好感还比较高，题目也不能说是很难，做起来挺有意思并且主要的概念都有所覆盖。如果之前有一些基础，想要速成的话我觉得好好做一做也是好事啊。

## 2020.9.28

今天我花了一整天的时间就为了解决windows的版本问题。

我的windows版本OS内部版本并没有达到安装WSL2的标准，但是电脑不知道为什么并不能继续向上进行功能升级。根据MS的以前的错误处理方案的指导依然没有效果。于是我最后决定重装系统。

在利用MS的工具重装系统之后终于成功地安装了WSL2，但是很多配置文件都没有了，电脑还需要重新配置和适应。

下面介绍一下我遇到的情况以及解决方案：

主要问题：windows10系统在升级版本的时候出现0x8000ffff错误，并且选择`重置`后会显示初始化电脑失败。根据 Microsoft Support 的相关帮助介绍检查了注册表等模块，检测程序没有报错。

解决方案：这个问题的产生很可能是某一次升级windows的时候断电了，导致系统文件除了问题。目前可以解决它的方法是直接重新装一下windows10操作系统。通过下载微软官方的`MediaCreationTool2004.exe`工具选择为其他电脑安装系统并且将一个U盘作为载体，重新安装系统即可。

## 2020.9.29

今天遇到了让人头疼的问题，跟着说明执行命令，在`./configure --target-list=riscv32-softmmu,riscv64-softmmu`时报错`ERROR: "cc" cannot build an executable (is your linker broken?)`，目前不知道究竟是一个什么问题。

根据报错信息和安装过程中的问题，初步推测是linux版本的问题，换用ubuntu18.04后就没有问题了，之前使用的是ubuntu20.04。

可能会缺少一些依赖，这些依赖可以从 https://wiki.qemu.org/Hosts/Linux 中找到，执行`sudo apt-get install git libglib2.0-dev libfdt-dev libpixman-1-dev zlib1g-dev`指令之后就可以正确地按照环境配置的教程完成qemu环境的搭建了。

然而在安装完qemu和rust工具链后依然不能成功编译运行，原因非常简单，是因为rust在安装后还没有合理地配置环境变量。一个简单的方法是打开对应的bash然后在最后一行追加`export export PATH=$PATH:$HOME/.cargo/bin`其中`$HOME/.cargo/bin`要根据当时安装的.cargo目录的位置进行修改。之后运行`source .bashrc`或者重启bash都可以成功配置环境变量。

配置完成后虽然可以正常运行程序了，但是最终还是有错误导致操统不能运行。操统在编译spin的时候报错

```bash
make[1]: Entering directory '/home/simonkorl/rCore-Tutorial/user'
   Compiling proc-macro2 v1.0.23
   Compiling unicode-xid v0.2.1
   Compiling syn v1.0.42
   Compiling rustversion v1.0.3
   Compiling spin v0.5.2
error[E0463]: can't find crate for `core`
  |
  = note: the `riscv64imac-unknown-none-elf` target may not be installed

error: aborting due to previous error

For more information about this error, try `rustc --explain E0463`.
error: could not compile `spin`.

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
Makefile:28: recipe for target 'build' failed
make[1]: *** [build] Error 101
make[1]: Leaving directory '/home/simonkorl/rCore-Tutorial/user'
Makefile:2: recipe for target 'run' failed
make: *** [run] Error 2
```

这个问题也让我花了很多时间进行排查，最后决定先把这个问题放下，去做Lab0。结果没有想到在Lab0中发现了对应的解决方法。

在环境配置部分并没写上两个component的配置：

```shell
# 增加RISC-V三元组
rustup target add riscv64imac-unknown-none-elf
# 增加需要的 cargo-binutils
cargo install cargo-binutils
rustup component add llvm-tools-preview
```

这样就可以让 rCore 成功运行起来了，运行的结果类似如下：

```sh
make[1]: Entering directory '/home/simonkorl/rCore-Tutorial/user'
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
Targets: notebook hello_world
Image resized.
make[1]: Leaving directory '/home/simonkorl/rCore-Tutorial/user'
make[1]: Entering directory '/home/simonkorl/rCore-Tutorial/os'
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

### 环境配置部分文档需要更新的地方

1. 增加Ubuntu版本的警告，20.04版本的依赖可能不满足一些条件，但是18.04可以非常好地契合这个教程。
2. qemu的下载地址下载速度确实非常的慢，目前还没有提供更好的下载地址。
3. 克隆仓库与输出部分仍然是TODO，但是应该并不难填写
4. 需要增加上面的component设置才能成功测试运行。如果一上来不能成功运行标准代码还是挺让人失望的。

### Lab0

从某个角度上来讲，Lab0才是这个教程的开端，它会引导你编写一个最简单的 rust 内核，并且可以保证其可以运行测试。指导书的环境配置部分只需要把 Linux 环境, qemu 与 rust 根据指导安装完毕即可，其余部分更加细节的配置在 Lab0 中会有展现。

**记住，不要太过于较真‘环境配置’一章中可能无法解决的错误，直接开始写Lab会有逐步的引导来解决他们！**

#### Lab0中的问题

执行`rust-objdump target/riscv64imac-unknown-none-elf/debug/os -d --arch-name=riscv64`指令后得到的结果与指导书中不同

```sh
rust-objdump target/riscv64imac-unknown-none-elf/debug/os -d --arch-name=riscv64

target/riscv64imac-unknown-none-elf/debug/os:   file format ELF64-riscv


Disassembly of section .text:

0000000080200000 text_start:
80200000: 09 a0                         j       2
80200002: 01 a0                         j       0
```

但是这并没有影响到最后的程序运行，最后的实验结果依然成功。通过观察汇编代码发现缺少的部分代码恰好是处理栈帧的部分。

## 2020.9.30 

今天在课程上明确了我们最终可以进行展示和说明的内容，并且和组员交流了之后的计划和安排。并且正在编写有关 rCore-Tutorial 部分的 issue

## 2020.10.1

提交了 issue 并且文档得到了修改。其中还有一些细节之处可以改进，不过最主要的问题已经解决了，我认为已经可以将 issue 关闭了，并不需要吹毛求疵。

## 2020.10.2

今天是提交实验一的最后一天，不过我们小组的进展还不足以支撑我们完成实验一的对应要求。我们和老师商量的结果是我们可以把上一个阶段完成的内容作为是实验一进行提交。

我和组员的决定是：先把之前的成果作为实验一提交，之后我们会进行实验一内容的补充完善。

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

### lab-1 可以修改的地方

`context.rs`文件中并不需要引用`riscv::register::scause::Scause`，这个问题间接导致了`handler.rs`中打印 scause 的问题，可以进行修改。

### lab-1 实验碎碎念

#### RISC-V 架构

事实证明，如果不了解指令集架构，在面对基础的汇编以及中断等概念的时候会发生一定的疑惑。这里我稍微记录一些路程上的疑惑之处，以便后人注意。

进行 ucore 实验的时候使用的是 x86-32 架构，只要做过 ucore 实验就知道光为了看明白 ucore 的前两个实验的文档就能掉多少头发，x86-32 架构确实复杂，而且有一些知识迁移到 RISC-V 上是不正确的。

1. 不要试图在 RISC-V 汇编代码中找如同`call`这样的汇编关键词。RISC-V 与 MIPS 都是靠着`jal`命令便可以简单地完成 Jump and Link，跳转后的指令会保存在`x1`寄存器中。在这件事上 RISC-V 与 MIPS 非常相近，但是与 x86 相去甚远。（不过不用再担心让人头痛的栈帧问题）

## 2020.10.9

今天开始完成 Lab-2。

### rCore-Tutorial 碎碎念

在完成 lab-1 后，我发觉自己终于理解 rCore-Tutorial 这个教程设计出来的意义了。

实际上这个教程提供的部分代码并不能让你成功地自己完成一个可以执行的 rCore ，其中省略了不少的内容。虽然这一点在教程的第一章就已经有些提示，不过不到自己动手做的时候还真的没办法亲身感受到这种不爽的感觉。

这种不爽的感觉就是给的代码实际上缺了很多很多需要填补的内容，如同一个到处都有洞的轮胎。不把洞补上就不敢开，但是洞肉眼不可见，只能靠着提示信息自己一点一点去找。

所以这个教程主要是用来做什么的呢？主要就是面向“做实验”的同学，也就是说它和 ucore 的教程的作用比较类似。实现者并不需要实现一个完整的 rCore 内核，只需要根据实验题目回答问题就可以了。同学们只需要知道 rCore 的大概实现思路，如果有一些不清楚的地方再去看代码来解决细节问题。如此看来，这个 rCore 实验还挺简单。

不过，有一件非常让我不爽的事情。就是 ucore 是代码中挖了一些空，我们需要实现那些缺失代码才能运行 ucore ，但是除了那些空之外，我们基本上不需要实现别的东西。做 ucore 的教程的时候留下的记忆基本上就是实现操作系统的相关内容上。但是在做 rCore 的时候，核心代码都已经提供，我自己做的事情大部分时候是在配置一些细枝末节的事情上，然后就是做实验题，这部分和 ucore 感受相同。不知道能不能在之后把 rCore 的实验也一个个分开，挖一些合理的坑。

### lab-2 食用体验

lab-2 中省略的部分不少，在看的时候也不知道如何来实现

#### part1

添加的依赖：

`buddy_system_allocator = "0.4.0" //实际上已经更新到0.5.0版本。目前0.4.0可以正确运行`

需要在 heap.rs 中加入一些use和extern

```rust
use buddy_system_allocator::{LockedHeap};
use super::config::{ KERNEL_HEAP_SIZE };

extern crate alloc;
```

说起来只需要三句话，竟然就不粘贴到文档中，也是有点过分

需要添加 mod.rs。这个似乎没有文档提示，但是通过编译器的报错可以猜出来问题的位置。

```rust
//! 内存模块
//! 
//! 
mod heap;
mod config;

pub fn init() {
    heap::init();
}
```

alloc_error_handler 是一个实验功能，这里会报错

`the '#[alloc_error_handler]' attribute is an experimental feature`

需要在 main.rs 中添加对它的引用，这一点文档已经提示过了

```rust
// ...
//!
//! - `#![feature(panic_info_message)]`  
//!   panic! 时，获取其中的信息并打印
#![feature(panic_info_message)]
//!
//! - `#![feature(alloc_error_handler)]`
//!  内存分配错误时处理
#![feature(alloc_error_handler)]
```

正确运行后可以看到`heap test passed`这样的输出。

小思考题：实际上，用于分配堆的数组位于 .bss 段，因为这个用于堆分配的静态数组是内核代码的一部分，且因为静态未初始化的数组（全都为0就是没有初始化的意思，因为可以通过指定起始地址和大小说明其在 .bss 段的位置。.bss 段是一个会初始化为 0 的数据段。在其中的数据不会直接占用数据空间，而是通过标注的方法说明空间）

#### part2

“物理内存探测”这一章有一些奇怪，虽然 OpenSBI 可以探测物理内存，但是返回的结果并没有被解析，这里使用了非常简单的方法定义的物理内存的区域，总觉得有一点像是在作弊的感觉。ucore 里面探测内存的代码还特别难看懂，光那一块就用了整整一章解释是如何使用汇编代码完成的物理内存探测。相较之下 rCore 真的太简单了。

1. 增加 lazy_static 库，并引用

```toml
lazy_static = { version = "1.4.0", features = ["spin_no_std"] }
```

2. 在 config 中添加 PAGE_SIZE

```rust
pub const PAGE_SIZE: usize = 4096;
```

3. 添加 address.rs（可以直接复制）
4. 在 mod.rs 中引用 address
5. 将 mod.rs 中的所有模块变为 pub（未来可能会引用）

最后会打印出一个地址：`PhysicalAddress(0x80a18808)`，实验即宣告成功。这个物理地址因为实现的细节发生变化，甚至注释的长短都可能会影响到这个数组，这是因为注释的内容也被储存进了代码中。

#### part3

“物理内存管理”这一章需要实现的内容很多（需要复制的内容很多）。这一章在 ucore 中也是非常大的一章，在 rCore-Tutorial 中因为大部分呢分配算法已经被提供，并且没有进行细致的讲解，教程部分长度倒是不长，但是需要实现的内容还是不少。

因为教程中提供的内容已经很少了，绝大部分代码需要从仓库文件中复制得到，代码中的坑也可以合理地规避掉。

一定要记住修改 memory 的 mod.rs ！

```rust
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
```

一定要注意所有的 mod.rs 文件，要好好复制。如果你选择复制完成这个 lab ，那么就不要想着自己去完善 mod.rs ，直接去复制下来。比如说`algorithm/src/allocator/mod.rs`就非常复杂，远远复杂于提供的简介。

algorithm 实际上是一个全新的 package ，需要至少填写`name version edition`三个字段。这个部分直接抄写吧，我也不知道为什么要这么写，尤其是那个 edition ，如果不定义的话编译不过去，书写过后就可以了。如果想要拥有自己的 rCore 的人一定注意！

## 2020.10.10

### 放弃 lab2

lab2 的问题还算是好回答，但是实验题真的是难，暂时决定搁置一下。

### 试试 lab3

我们已经放弃按照教程来书写自己的 rCore 代码了——这个教程并不是很适合这么来做。因为很多具体的实现都被隐藏了起来，教程主要展示的是操作系统实现层面的关键思路。如果想要按照教程实现自己的代码的话就会需要大量的复制粘贴工作——这种工作应该直接交给 diff 工具处理，这在学习操作系统方面没有提供任何帮助，所以我们打算从现在以实验指导书为主要学习材料，利用实验题作为练习来进行 rCore 的实现。

> 不得不说，用 Rust 写这些内容是痛苦的（可能后面一两个章节还会痛苦一段时间），但是为了充分发挥 Rust 的特性，这些挣扎是必要的，一旦我们铺平了这些基础设施，后面的流程会大大简化。

我的感觉和实验三指导书中写的这句话非常类似。如果说第一章的内容还可以靠着汇编以及计算机组成原理的知识试图理解一下，那么实验二和实验三基本上就处于看代码可能都不明白是怎么回事的阶段了。实验指导书虽然有一定的帮助，但是相对于 ucore 来说 Rust 的实现理解起来更难一点（然而 C 的实现也没有简单到哪里去，ucore 的物理内存以及虚拟地址这部分也是能让人头疼好半天，最大的好处就是最复杂的基础配置代码并不需要手写）因为 Rust 本身的语法特点，我敢说如果要手动实现这部分代码的话肯定会非常痛苦。

### 回答问题的心得体会

* 在做这章的实验之前最好把 Sv39 的页表项格式抄写一次，可以有更深的理解.

* 页表这一块初学的话一定会犯错，尤其是 RISC-V 的页表，因为它可能不完全使用全部的地址空间（只用一个最高的9位作为 VPN的索引，剩下的全都是 offset）

* 记住地址是从零开始数的！[38:30]指的是从31位开始的9位bit，也就是抛弃最后的30位（对应1G大小的空间）。其实设计者会尽量设计得简单一点。

### lab3+ ？？？

\#issue 

话说 Lab3+ 一开始好像缺少组件： riscv64-unknown-elf-objcopy。

这个又是一个特别让人头疼的工具链问题：riscv64-unknown-elf-objcopy 是 GNU 的 riscv 交叉编译工具链中的一个组件，但是我们根据教程安装的是 llvm 的工具链，如果需要重新安装工具链的话是个不小的工作量，如果没有指导的话可能会花大量的时间。我在网络上找到了一个还算靠谱的实现[从零开始配置risc-v交叉编译环境与工具链](https://zhuanlan.zhihu.com/p/72862396)，我正在尝试利用这个教程进行配置，希望能够解决这个问题。

**其实不需要配置 GNU 工具链也可以成功运行**，只需要把`user/Makefile`中

`$(foreach bin_file,$(BIN_FILES),riscv64-unknown-elf-objcopy --strip-debug $(bin_file) $(bin_file);)`

一行的内容中的`riscv64-unknown-elf-objcopy`修改为`rust-objcopy`即可：

`$(foreach bin_file,$(BIN_FILES),rust-objcopy --strip-debug $(bin_file) $(bin_file);)`

## 2020.10.11

### lab3 实验中的痛苦时刻

我在实现时钟替换算法的时候遭遇了许多 Rust 语法上的问题，尤其是这个传入 Swapper::push 的`*mut PageTableEntry`参数，可以让人头疼一阵子。

这个传入的 raw pointer 并不能进行安全的转换，需要在`unsafe`的块中进行解引用。这一块还算好理解，但是用来储存它的结构需要好好思考一下。如果使用`(VirtualPageNumber, FrameTracker, *mut PageTableEntry)`的 tuple 进行储存会出如下编译错误：

`error[E0277]: '*mut memory::mapping::page_table_entry::PageTableEntry' cannot be sent between threads safely`

这并不是因为解引用生指针而导致的问题，而是因为不可以在线程之间直接传送储存的生指针，如果被编译器检查出来的话就会出编译问题，不过只要想办法绕过编译器就可以解决。

这里我没有想到很好的办法解决，我的程序中的解决方法参考了[yunwei37](https://github.com/yunwei37/os-summer-of-code-daily) 同学的实现：利用 usize 储存指针，之后再通过 unsafe  转换回指针即可躲过类型检查。不过这个方法应该也没办法完全说保证线程的安全，因为实际上我们还是储存和传递了生指针。

最后为了测试自己写的时钟替换算法，可以将`main.rs`中的下面这段代码解除注释:

```rust
    PROCESSOR.lock().add_thread(create_kernel_thread(
        Process::new_kernel().unwrap(),
        test_page_fault as usize,
        None,
    ));
```

之后注释掉`user_shell`对应的那行代码就可以完成一个简单的测试程序。

如果实现的 Swapper 没有问题，那么最后会有绿色的`test passed`显示。

\# tips

**小提示**：QEMU 的退出(在 Windows10 WSL2 上)是按 Ctrl+A, x 。它的意思其实是，首先按 Ctrl+A ，之后手离开键盘后再单独按 x 键便可退出。

### 重新考虑 lab-2 的实现

在和组员交流之后我终于意识到不能卡死在一道题目上，如果不太会做的话不妨去看看其他人的实现，正好也是我们这个项目的核心：如果我们会卡住的话别人也很可能会卡住，那么我们就需要给他们提供帮助。要想渡人，先要渡己，我们需要自己先明白实验中的内容。

lab-2 对于程序的能力还是有一定考验的，因为线段树属于稍微有一点难度的数据结构，而且结构比较灵活多变，这对这个模块的实现造成了一些困难的。（但是对于程序大佬的话这简直就是白给的题目）。

我的实现参考了[yunwei37](https://github.com/yunwei37/os-summer-of-code-daily)，中间也是遇到了一些问题，主要的问题还是线段树的实现过程中的一些细节（开闭区间啊，实现方法啊什么的）。这些问题想要弄明白的话最好的方法应该是直接看正确的代码（看大佬的代码，别看我的！）。

lab-2 还有一个问题，就是 algorithm crate 中并不能使用 println! 进行调试，需要想一下别的方法来调试。我尝试了使用 if + panic 和 assert 的方法进行调试，也许有更好的方法，希望能够找到大佬的笔记啊。
