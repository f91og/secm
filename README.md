My secret management tool via cli
## Usage
```shell
# make secret
scem make secretName
scem make secretName -l=20
scem make secretName -l=20 -a

scem use secretName
# enter into interactive mode to select a secret
scem use

scem add secretName secretValue
scem rm secretName
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
