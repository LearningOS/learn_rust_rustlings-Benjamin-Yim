// macros4.rs
// Make me compile! Execute `rustlings hint macros4` for hints :)

macro_rules! my_macro {
    // 宏可以重载，从而接受不同的参数组合。在这方面，macro_rules! 的作用类似于 匹配（match）代码块：
    () => (
        println!("Check out my macro!");
    );
     //  每个分支都必须以分号结束。
    ($val:expr) => (
        println!("Look at this other macro: {}", $val);
    );
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
