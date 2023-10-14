** 关于mod, use, crate, pub **
pub 用于声明可以被文件外的地方使用
mod 用于声明模块，用在main.rs或其他文件中lib.rs中，类似于一个注册功能
crate 是mod的组织，从main.rs或者lib.rs开始，根据main和lib.rs中的mod注册，形成一个mod树
use 可以在main.rs和lib.rs中和之外的文件中使用，用于导入crate中注册的mod，eg:

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
        