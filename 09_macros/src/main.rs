// This is a trait definition
pub trait Summary {
    fn summarize(&self) -> String;
}

// A very simple struct
pub struct Thing;

// Simple implementation of this trait for thing.
impl Summary for Thing {
    fn summarize(&self) -> String {
        "Mything".to_string()
    }
}

// Now let's try to have a macro to define this for us with different names...
macro_rules! summ {
    ($n:ident) => (
        #[derive(Debug)]
        pub struct $n;
        impl Summary for $n {
            fn summarize(&self) -> String {
                "MacroTest".to_string()
            }
        }
    );
}

// Now let's try to have a macro to define this for us with different names...
macro_rules! prn {
    ($n:ident, $t:path) => (
        let $n = $t;
        println!("{}", $n.summarize());
    );
}

summ!(Mtest);
summ!(NoTest);

fn main() {
    let mything = Thing;
    let mytest = Mtest;
    println!("Hello, world!");
    println!("{}", mything.summarize());
    println!("{}", mytest.summarize());
    prn!(mynote, NoTest);
    println!("{:?}", mynote);
}
