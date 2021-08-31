# Playing with Cargo Workspaces
So far I really like cargo, but I am feeling a bit un-comfortable with the way dependencies and elements seem to be handled within a project. I was not feeling great about wanting/needing to separate things in different crates, but that are dependencies of each other. How to go about that? Do I need to have a bash script or a Makefile on top of these two crates?

Well, looks like Cargo already handles this via the workspaces mechanism.

## Reading the manual
I followed the following [section 14.3 of the Rust Book](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html)

Everything is there, I will only add a couple of notes

## Gotcha's
Something that misled me a bit is that initially I was adding *package* and *dependencies* sections to the Cargo.toml in the root of the project. This is not required, in there we only need to declare the underlaying crates using the *workspace* keyword.

## Using it
As a final note, after setting it up and creating the two crates you will be able to build all your crates with one command from the root of the directory:

```
cargo build

Compiling child1 v0.1.0 (/home/ptorru/projects/rusty-notes/06_cargo_workspaces/child1)
Compiling child2 v0.1.0 (/home/ptorru/projects/rusty-notes/06_cargo_workspaces/child2)
Finished dev [unoptimized + debuginfo] target(s) in 0.50s
```
