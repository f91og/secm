My secret management tool via cli
## Usage
```shell
# enter tui
scem
```

## Memo
**关于mod, use, crate, pub**
pub 用于声明可以被文件外的地方使用
mod 用于声明模块，用在main.rs或其他文件中lib.rs中，类似于一个注册功能
crate 是mod的组织，从main.rs或者lib.rs开始，根据main和lib.rs中的mod注册，形成一个mod树
use 可以在main.rs和lib.rs中和之外的文件中使用，用于导入crate中注册的mod，总而言之就是先在main.js或者lib.rs中利用mod来注册项目里的其他代码文件，然后在其他文件里就可以使用
eg:

constants.rs
```rust
pub const MAX_POINTS: u32 = 100_000;
```

main.rs
```rust
mod constants;
....
println!("max points: {}", constants::MAX_POINTS);
```

handler.rs
```rust
use crate::constants; // 这里需要是use，因为只能在main.rs和lib.rs中注册mod到crate里
...
println!("max points: {}", constants::MAX_POINTS);
```

**关于Result<>, Ok()和Value()**
```rust
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Err("Division by zero is not allowed.".to_string());
    }
    Ok(a / b)
}

fn main() {
    let result = divide(8, 2);
    match result {
        Ok(value) => {
            println!("Result: {}", value);
        },
        Err(error) => {
            println!("Error: {}", error);
        }
    }
}
```

**关于cargo**
```shell
# run project
cargo run

# install current project to ~/.cargo/bin
cargo install --path .
```

**关于数据类型**
vec!和 Vec<&str>的使用区别

**关于所有权**
从编译器的提示可以看到函数的传参是需要引用还是有所有权的
一般来说只读的话则不需要所有权，否则需要所有权，因为所有权的设计其出发点就是避免对某个变量的同时写
所以如果把对变量的修改放到所有权里，则不许需要各种变化，比如改变结构体成员的值，可以是实现在结构体的方法里，而不是在外部操作结构体来改变其中的成员
一些所有权存在的规则：
1. 一个作用域内对一个变量的不可变引用能存在多个，但是可变引用只能有一个，且可变引用不能和不可变引用在一个作用域里
   1. 为了防止对一个变量的同时修改
   2. 为了防止读变量时的不确定（变量修改前后读出的结果不一样）
2. 编译器肯定是一开始想判断为引用为不可变引用的，只有在作用域内出现了修改的情况将其判定为可变引用
3. 结构体掌握了其所有成员的所有权,可以在其方法内修改成员,而外部只能通过不可变引用来访问,避免了外部同时修改的可能