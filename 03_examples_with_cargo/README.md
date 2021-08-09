# Examples with Cargo

I am still learning my ways around cargo's project structure.

Here I would like to leave some notes around the examples facilities in cargo.

## What is this about?
I really like that cargo has facilitites for the usual structure you would expect in a moderm software project:
* src: for your source files.
* test: for unit tests.
* examples: to hold examples on how to use your project.

That's really cool but how do you use it? what goes where?

## The structure

Cargo runs examples via the following command:

```bash
cargo run --example <myexample>
```

Where *myexample* is also the filename of a rs source file in the examples directory.

There are a couple of nice variations to keep in mind:

```bash
# runs example in release mode
cargo run --release --example <myexample>

# runs example with parameters
cargo run --example <myexample> -- <arg1> <arg2>
```

# How to run the example of a library
The question that motivated me to write this entry was, how can I create an example of a library in a cargo project.

There are different ways you can declare code in lib.rs, for example within or without a module. I have made two pieces in the lib.rs to illustrate both cases.

I tried then to use this code in the example file *tellme.rs*. I started trying to do something like:
```rust
// Do not try this at home...
use crate::*;
```

That did not work. My take is that quite rightly, cargo does not consider the example code part of the same crate, that would be silly! This I still need to confirm.

By making this distinction cargo provides some sort of sandbox that allows you to put code together as a user would:

```rust
use examples_with_cargo::local::*;
```

Now, I tried to cheat, and include everything on lib.rs, all functions and all modules. Cargo did not like this. Take a look at the *tellmeall* example to see what I mean.

## Credits
My initial learnings came from [ct's](https://stackoverflow.com/users/1369495/ct) answer in [StackOverflow](https://stackoverflow.com/questions/54469463/how-do-i-run-a-projects-example-using-cargo)