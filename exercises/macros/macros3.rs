// macros3.rs
// Make me compile, without taking the macro out of the module!
// Execute `rustlings hint macros3` for hints :)

mod macros {
    // 然而，如果这个宏被加上 #[macro_export] 属性，那么它就在 crate 的根作用域里被定义， 而且能直接使用它。


    #[macro_export] macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}


fn main() {
    my_macro!();
}
