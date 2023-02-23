// macros4.rs
// 执行 `rustlings hint macros4` 或在观察模式下使用 `hint` 子命令来获取提示。

macro_rules! my_macro {
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    };
    () => {
        println!("Check out my macro!"); 
    }
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
