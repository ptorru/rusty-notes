// This is a trait definition
pub trait Summary {
    fn summarize(&self) -> String;
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
    ($n:ident, $t:path) => (
        let $n = $t;
        println!("{}", $n.summarize());
    );
}

summ!(Thing);
summ!(Mtest);
summ!(NoTest);

fn main() {
    summ!(mything, Thing);
    summ!(mytest, Mtest)
    summ!(mynote, NoTest);
    println!("{:?}", mything);
    println!("{:?}", mytest);
    println!("{:?}", mynote);
}
