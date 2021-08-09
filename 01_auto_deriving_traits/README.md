# Auto Deriving Traits
Very important feature in rust are traits.

Traits allow to re-use code accross different data types, which I think is very powerful.

## What is Deref?
Deref is a trait that includes the deref function. *deref* gets invoked when using the "*" operator.

By implementing this function one can have complex data types with custom behaviour.

## What if we could get rust to do some coding for us?
That is what the *derive* macro does. It asks rust to try and derive the Deref trait implementation for our custom data structure Num:

```rust
// Example from: https://jeltef.github.io/derive_more/derive_more/deref.html
use derive_more::{Deref};

// This will derive the Deref method for our custom type.
#[derive(Deref)]
struct Num {
    num: i32,
}
```

## Why would we want this?
This is handy when you have a lot of custom data types and do not want to implement this manually. Lets say, when building an Abstract Syntaxt Tree, we may have a number of custom data structures that may just be wrappers for an existing type. In which case we would want something to derive basic facilities for us, like the Deref methods:

```rust
#[derive(Deref,DerefMut,Debug,Clone)]
struct MyCollection(Vec<Numm>);
```

In the associated examples we test this. We get all the facilities to de-reference our custom structure MyCollection, without having to implement any of the functions manually!
