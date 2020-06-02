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

