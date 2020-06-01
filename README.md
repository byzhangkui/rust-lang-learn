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

### Variables VS Constants

const 类型不能使用 mut 变成可变类型

# 变量隐藏（Shadowing)

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

#### 作用：

少定义一些变量，如上个列子，在不支持变量隐藏的语言中，就要写成：

```rust
    let spaces_str = "    "; 
    let spaces_len = spaces.len(); 
```

