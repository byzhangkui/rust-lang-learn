# 变量 Variables

默认情况下，变量是不可变的（by default variables are immutable）。但也可以将变量定义成可变的。

默认不可变的原因是更安全和简单的支持并发。

例子：

```rust
    let x = 5;
    x = 6;
```

编译报错：

```
error[E0384]: cannot assign twice to immutable variable `x`
 --> src/main.rs:3:5
  |
2 |     let x = 5;
  |         -
  |         |
  |         first assignment to `x`
  |         help: make this binding mutable: `mut x`
3 |     x = 6;
  |     ^^^^^ cannot assign twice to immutable variable
```

提示不能给非可变变量赋值两次。

如果希望变量可被改变，在变量名前加 `mut` 关键词

```rust
    let mut x = 5;
    x = 6;
```

不能：

```rust
    let x = 5, y = 6; //一行定义两个变量
    let x = y = 5; //连续定义
```



### Variables VS Constants

const 类型不能使用 mut 变成可变类型

## 变量隐藏（Shadowing)

同一作用域中，允许定义一个与前一个变量名相同的变量，后一个变量“隐藏”前一个变量。

```rust
    let x = 5;
    let x = x + 1;
```

新变量可同前一变量类型不同。

```rust
    let spaces = "    "; // 字符串类型
    let spaces = spaces.len(); // 4 数字类型
```

#### 用处：

少定义一些变量，如上个列子，在不支持变量隐藏的语言中，就要写成：

```rust
    let spaces_str = "    "; 
    let spaces_len = spaces.len(); 
```



# 数据类型([Data Types](https://doc.rust-lang.org/book/ch03-02-data-types.html#data-types))

2种数据类型集合：scalar and compound

Rust是静态类型语言，所有变量的类型必须在编译期确定。

## Scalar Types

包括：integers, floating-point numbers, Booleans, and characters. 

#### [Integer Types](https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-types)

| Length  | Signed  | Unsigned |
| ------- | ------- | -------- |
| 8-bit   | `i8`    | `u8`     |
| 16-bit  | `i16`   | `u16`    |
| 32-bit  | `i32`   | `u32`    |
| 64-bit  | `i64`   | `u64`    |
| 128-bit | `i128`  | `u128`   |
| arch    | `isize` | `usize`  |

i 前缀代表有符号，u 前缀代表无符号

有符号整型表达的范围为 $-(2^{n-1})$ 到 $2^{n-1} - 1$

无符号整型表达的范围为 0 到 $2^n - 1$ 

isize 和 usize 依赖其运行的系统位数，32位系统上为32位，64位系统上为64位。

> C++中对应的类型位 int8_t,int16_t,int32_t,int64_t,uint8_t,uint16_t,uint32_t,uint64_t,size_t。
>
> C++中没有 i128/u128/isize 对应的类型

#### [Floating-Point Types](https://doc.rust-lang.org/book/ch03-02-data-types.html#floating-point-types)

Rust 浮点数有分别为32位和64位的 f32 和 f64。默认类型是 f64。

```rust
    let x = 1.0; // f64
    let y: f32 = 2.0; // f32
```

浮点数符合 IEEE-754 规范。f32 是单精度浮点数，f64 是双精度浮点数。

#### 数字运算

支持 +、-、*、/、% 运算。

```rust
let result = 1 + 2 * 3 / 4 % 5;
```



#### bool 类型

Rust 支持 bool 类型，其值包括 true 和 false 两种。bool 类型占用1个字节。

```rust
    // boolean
    let flag = true;
    let flag : bool = false;
```

#### 字符类型

char 类型，使用单引号定义。Rust 的 char 使用4个字节存储。

```rust
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
```



## [Compound Types](https://doc.rust-lang.org/book/ch03-02-data-types.html#compound-types)

组合类型是用一个类型表达多个值。Rust 有两个主要的组合类型：元组(tuples) 和 数组(arrays)

### 元组(tuple)

元组可存储一组类型不同的值。

特点：元组是固定长度，一旦定义后就不能改变大小。

#### 元组定义

```rust
    let tup : (i32, f64, u8) = (500, 6.4, 1);
or
    let tup = (500, 6.4, 1);
```

#### 元素访问

##### 解构到变量

```rust
    let (x, y, z) = tup;
```

x, y, z 分别为 500，6.4，1

##### 使用下标访问

```rust
    let first = tup.0;
```

### 数组(Array)

数组存储**相同类型**的数据集合，Rust 的数据是**固定长度**。

##### 数组的定义

```rust
    let arr = [1, 2, 3, 4, 5];
    let arr : [i32 : 5] = [1, 2, 3, 4, 5];//数组元素类型为i32，size 为 5
    let arr = [3 : 5]; // 包含 5 个元素，元素值为 3
```

##### 数组访问

通过下标访问

```rust
    let first = arr[0];
```

##### 数据访问越界

###### 如果是常量下标越界：

```rust
    let first = arr[10];
```

编译期会报错：

```
13 |     let first = arr[10];
   |                 ^^^^^^^ index out of bounds: the len is 5 but the index is 10
```

###### 如果是变量下标越界：

```rust
    let index = 10;
    let first = arr[index];
```

会产生运行时错误：

```
thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 10', src/main.rs:13:17
```

# 函数(Function)

## 函数定义：

```rust
fn 函数名(参数) -> 返回值类型 {函数体}
```



```rust
fn main() {
    show(5);
}

fn show(x : i32) {
    println!("x is: {}", x);
}
```

说明：函数的形参需要声明类型



## 语句和表达式

这是Rust 特有的概念，函数体由一系列的语句(statement)和结尾的表达式(expression)组成。

语句：执行操作，但不返回值。意味着不能将语句复制给一个变量。

表达式：计算出一个结果值。表达式不以分号结尾，如果在表达式后加上分号，其变成一个语句。

详情见下一节。



## 带返回值的函数

```rust
fn main() {
    let ret = add(2);
    println!("result is: {}", ret);
}

fn add(x : i32) -> i32 {
    x + 1
}
```

注意，add 中的 x + 1 不带分号，如果加上分号，会得到一个编译错误:

```
6 | fn add(x : i32) -> i32 {
  |    ---             ^^^ expected `i32`, found `()`
  |    |
  |    implicitly returns `()` as its body has no tail or `return` expression
7 |     x + 1;
  |          - help: consider removing this semicolon
```

可以理解为，x + 1 表达式不加分号等价于 return x + 1;

x + 1 替换为 return x + 1 或者 return x + 1; 均可正常编译执行。

# 注释

Rust 支持使用 // 的行注释

# 控制流(Control Flow)

## if 表达式

```rust
    let number = 12;

    if number > 100 {
        println!("Too Big");
    } else if number < 10 {
        println!("Too Small");
    } else {
        println!("Ok");
    }
```

### 注意：

if 表达式的条件必须是 bool 类型，如果非 bool 类型会报错。Rust 不会将非 bool 类型转换为 bool 类型。例如：

```rust
    let number = 12;

    if number {
        println!("Big than zero");
    } 
```

编译时会报错：

```
4 |     if number {
  |        ^^^^^^ expected `bool`, found integer
```

### 在 let 中使用 if 表达式

可以在let 语句中使用 if 

```rust
    let flag = true;
    let result = if flag { 1 } else { 0 };
    println!("The value is: {}", result);
```

Tips：

- 1/0 后不加分号是表达式，参考：语句和表达式一节内容
- If else 分支中返回的值类型要一致，否则会出错

```rust
    let flag = true;
    let result = if flag { 1 } else { "0" };
    println!("The value is: {}", result);
```

```
3 |     let result = if flag { 1 } else { "0" };
  |                            -          ^^^ expected integer, found `&str`
  |                            |
  |                            expected because of this
```

> let 中使用 if 表达式类似 C++ 中的 ? : 三元表达式作用

## 循环

Rust 可以用 loop, while, for 来实现循环

### loop(无限循环)

```rust
    loop {
        println!("again!");
    }
```

可用 break 打破循环

> 类似 C++ 中的 while(true)

#### 从loop中返回值

```rust
    let mut retry_count = 0;
    
    let result = loop {
        retry_count += 1;

        if retry_count > 3 {
            break false;
        }
    };
    println!("The result is {}", result);
```

结果：

```
The result is false
```



### while 

```rust
    let mut retry_count = 0;
    while retry_count < 3 {
        println!("The result is {}", retry_count);
        retry_count += 1;
    }
```

结果：

```
The result is 0
The result is 1
The result is 2
```

### for

```rust
    let array = [10, 20, 30, 40, 50];

    for item in array.iter() {
        println!("The value is : {}", item);
    }
```

# 所有权（Ownership）

Rust 为什么有所有权的概念？主要用来管理堆内存上分配的对象。

## 所有权规则

1. Rust 中的每一个值都有一个被称为其**所有者**的变量。
2. 值有且只有一个所有者。
3. 当所有者（变量）离开作用域，这个值将被抛弃。

## 变量作用域

```rust
    {                      
        let s = "hello";   
        // do stuff with s
    }   
```

变量 `s` 在大括号限定的作用域内有效。

### String 类型

```rust
        let mut s = String::from("hello");
        s.push_str(", world!"); // push_str() appends a literal to a String
        println!("{}", s); // This will print `hello, world!`
```

String 类型支持 mut ，可动态增删字符，其分配在堆上。

## 内存管理

```rust
    {                      
        let s = "hello";   
        // do stuff with s
    }   
```

在这个例子中，s 离开作用域时，Rust 会自动调用 drop 释放内存。

> 使用RAII管理资源声明周期，类似 C++ 中的 std::string。

### Move

```rust
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}, world!", s1);
```

会的到一个编译错误：

```
18 |         let s1 = String::from("hello");
   |             -- move occurs because `s1` has type `std::string::String`, which does not implement the `Copy` trait
19 |         let s2 = s1;
   |                  -- value moved here
20 |         println!("{}, world!", s1);
   |                                ^^ value borrowed here after move
```

s1 已经无法访问，从 `let s2 = s1;   -- value moved here`这里提示可以推测出 `let s2 = s1;` 这里转移了所有权。这里既不是浅拷贝，也不是深拷贝，而是所有权的转移。

![s1 moved to s2](https://doc.rust-lang.org/book/img/trpl04-04.svg)

> 类比 C++ ，可这么理解上述代码的实现：
>
> std::unique_ptr<String> s1 = std::make_unique<String>("hello");
>
> auto s2 = std::move(s1);
>
> 只不过在 Rust 中，RAII 和 move 语义是默认行为。

### Clone

如果需要实现深拷贝，可用 clone() 方法实现。

```rust
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
```

### Copy（Stack-Only Data)

```rust
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);
```

对于基本类型，其大小固定，分配在栈上。赋值时，采用的是 copy 策略。

采用 copy 策略的数据类型：

- 整型，如 `u32`。
- 布尔类型。
- 浮点数，如 `f64`。
- 字符，char`.
- Tuples 里包含的类型都是 Copy 类型的。如 `(i32, i32)` 是 `Copy`类型, 但 `(i32, String)` 不是.

### Ownership and Functions

作为函数的参数传值时，和赋值类似，要么是copy要么是move。举例：

```rust
fn main() {
    let s = String::from("hello");  // s comes into scope
    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here
    println!("{}", s);              //got build error here

    let x = 5;                      // x comes into scope
    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

```

> 类比 C++，
>
> void takes_ownership(std::unique_ptr<String> some_string) {
>
> // do something
>
> }
>
> std::unique_ptr<String> s = std::make_unique<String>("hello");
>
> takes_ownership(std::move(s));
>
> 可以看出，Rust 堆对象定义时，使用类 std::unique_ptr 的智能指针管理内存，赋值和参数传值时，默认是 std::move 语义。



### Return Value 返回值

返回值同样会转移所有权

```rust
fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
  // moved, so nothing happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("hello"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}

```

> 类比 C++ ，Rust 返回值默认通过右值引用绑定。
>
> 如 String&& s1 = gives_ownership(); 

Rust 函数参数传值这种转移所有权的实现，如果调用者不想转移所有权，仍想继续使用变量怎么办？

### 引用 Reference

函数参数传值时，如果不想转移所有权，可以以引用的方式传值：

```rust
fn main() {
    let s1 = String::from("world");
    let len = get_length(&s1);
    println!("The length of {} is {}", s1, len)
}

fn get_length(s : &String) -> usize {
    s.len() // s does not have ownership of what it refer to.
}
```

> 注意：相对于 C++，Rust 的函数参数为引用类型时，调用时需在参数前加 & 明示其类型。如上例子中 get_length(`&s1`)

Rust 中使用引用作为函数参数的情况，称为借用(borrowing)。作为引用参数，不持有其引用对象的所有权，所以在引用生命周期结束时，不会调用 `drop` 释放资源。如在 get_length 中，s 并没有其引用对象的所有权，s 在作用域结束后不会释放其资源。

那么，引用参数可以修改其引用对象的值吗？

```rust
fn main() {
    let s = String::from("hello");
    changes(&s);
}

fn changes(s : &String) {
    s.push_str(", world!");
}
```

这样做，会得到编译错误：

```
6 | fn changes(s : &String) {
  |                ------- help: consider changing this to be a mutable reference: `&mut std::string::String`
7 |     s.push_str(", world!");
  |     ^ `s` is a `&` reference, so the data it refers to cannot be borrowed as mutable
```

Rust 默认都为 immutable ，如果想修改其引用对象，需要用到 mutable reference

### Mutable Reference

如果想在函数中修改其引用的对象，需要这么做：

```rust
fn main() {
    let mut s = String::from("hello");
    changes(&mut s);
    println!("{}", s);
}

fn changes(s : &mut String) {
    s.push_str(", world!");
}
```

把 s 声明为 mut，把 changes 形参类型声明为 mut，调用时也需指明为mut。

> 类比C++，Rust 参数默认的行为是 const&。形参类型声明为 mut 后，其行为等同于 C++ 的 &。

可见，Rust 默认以最安全的方式进行，如果要修改其值，需要将各个涉及的地方都声明 mut。

#### Mutable Reference 的限制

在同一个作用域中，只能有一个 mutable reference 指向一个对象。如下使用会失败：

```rust
    let mut s = String::from("hello");
    let r1 = &mut s;
    let r2 = &mut s;
    println!("{}, {}", r1, r2);
```

```
3 |     let r1 = &mut s;
  |              ------ first mutable borrow occurs here
4 |     let r2 = &mut s;
  |              ^^^^^^ second mutable borrow occurs here
5 |     println!("{}, {}", r1, r2);
  |                        -- first borrow later used here
```

Rust 这么限制的原因是避免并发时的竞争，在上述例子中，r1,r2 作为参数同时传递给函数时，如果函数内没有做好数据保护，就会出现竞争的情况。Rust 在编译期避免了该情况。

> 以下三种行为同时出现会导致数据竞争：
>
> - 2个或多个指针同一时刻访问同一个数据
> - 只是有一个指针试图写数据
> - 没有进行数据保护的机制

可见，如果在不同的作用域，引用同一个对象，因为不同作用域中不能同时访问，不会出现竞争的问题，如：

```rust
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
    }
    let r2 = &mut s;//no problem
```

#### 引用对象生命周期

引用对象生命周期从其定义时开始，终止于其最后一次调用。

什么意思呢？见如下例子：

```rust
    let mut s = String::from("hello");
    let r1 = &mut s;
    println!("{}", r1);// r1 no longer used after this point
    let r2 = &mut s;// no problems
    println!("{}", r2);
```

r1 在 println!("{}", r1) 后就没有再调用，其生命周期结束了，接下来定义 r2 就完全没有问题。如果，r2 定义后仍有代码使用 r1，那么意味着 r1 生命周期没有结束，依然会产生编译错误。

mutable 和 immutable 混用时，也会出现错误，如：

```rust
    let r1 = &s;//no problem
    let r2 = &s;//no problem
    let r3 = &mut s;//BIG PROBLEM
    println!("{}, {}, {}", r1, r2, r3);
```

解决思路从作用域和生命周期两个角度处理，如下一种解决方案:

```rust
    let r1 = &s;//no problem
    let r2 = &s;//no problem
    println!("{}, {}", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut s;//no problem
    println!("{}", r3);
```

### [Dangling References 悬挂指针](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#dangling-references)

在一些语言中，比较容易出现悬挂指针，即其指向的内存已经被其他指针释放，且已经进行了重新分配。Rust 中编译器保证不会出现悬挂引用。

在 Rust 中尝试创建一个悬挂引用：

```rust
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!
```

会得到一个编译错误：

```
5 | fn dangle() -> &String { // dangle returns a reference to a String
  |                ^ help: consider giving it a 'static lifetime: `&'static`
```

解决方法，直接返回 `String` 的值：

```rust
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
```

`Return Value` 一节中已经说明，返回值会进行所有权的转移，所以没有必要想通过返回引用来优化。

### The Slice Type

Slice 是一个很有意思的特性。Slice 可以引用一个集合中的连续子序列。

#### String Slice

```rust
    let s = String::from("hello world!");

    let hello = &s[0..5];// hello
    let world = &s[6..11];// world
```

使用 `[starting_index..ending_index]` 来表达子序列的范围。内部实现为指向起始位置的指针及长度，长度为 ending_index - starting_index。

下面是 .. 范围符号使用示例：

```rust
    let s = String::from("hello");
    let slice = &s[0..2];
    let slice = &s[..2]; // if start with zero, can drop zero

    let slice = &s[3..s.len()];
    let slice = &s[3..]; // if includes the last byte of String, can drop the trailing number.

    let slice = &s[0..s.len()];
    let slice = &s[..]; // drop both values, take a slice of the entire string
```

一个例子：

```rust
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```



这里会存在一个问题，如果 slice 所引用的字符串无效，会出现什么情况？实际上，Rust 编译期保证了其引用的字符串是始终有效的。

```rust
        let mut s = String::from("hello");
        let slice = &s[0..2];
        s.clear();
        println!("sclie: {}", slice);
```

在清空字符串 s 后，使用 slice，编译器会报错误：

```
25 |         let slice = &s[0..2];
   |                      - immutable borrow occurs here
26 |         s.clear();
   |         ^^^^^^^^^ mutable borrow occurs here
27 |         println!("sclie: {}", slice);
   |                               ----- immutable borrow later used here
```

#### String Literals Are Slices

```rust
let s = "Hello, world!";
```

s 的类型是 &str。string 字面量是不可变的；&str 是不可变引用。

#### [String Slices as Parameters](https://doc.rust-lang.org/book/ch04-03-slices.html#string-slices-as-parameters)

```rust
fn first_word(s: &str) -> &str {
```

用 &str 代替 &String 作为函数参数类型会更通用，因为 &String 类型可以很方便的转换成 &str 类型，比如上例子中的 &s[..]。

#### Other Slices

在数组中也可以使用 slice type，如：

```rust
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];
```

slice 类型为 &[i32]。



# Rust 优势

## 资源管理

RAII

## 性能

赋值和参数传递默认move语义

## 安全性

默认immutable