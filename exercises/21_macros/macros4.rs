// macros4.rs
//
// Execute `rustlings hint macros4` or use the `hint` watch subcommand for a
// hint.



// suppresion de #[rustfmt::skip] pour que le code ne soit pas reformater
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    }; // Ajout ;
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    }; // Ajout ;
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
