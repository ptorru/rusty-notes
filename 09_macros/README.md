# Macros

In this entry I am trying to learn how to use Macros in Rust.

Looks like rust (at the time of writing) has two main ways of doing macros:
* Declarative Macros
* Procedural Macros
  * Custom Derive
  * Attribute-like
  * Function-like

The reason to use macros is to be able to generate code. More specifically, within the context of traits, we want to be able to generate functions programmatically, so that when we create a Trait it can be used on many different types.

You may think on Object Programming, and think well, what about Inheritance? I guess is similar? but not quite the same?

My goal in this entry is to declare a trait, create a macro that defines a function for that trait, and then been able to use it with different types.

# Macros like rust-match, wait what?
In the rust book one of the first things it says is the following:
> At their core, declarative macros allow you to write something similar to a Rust match expression.

I did not understand what they meant by this. Now I do. When using declarative Macros, the infrastructure allows you to effectively declare many macros within a single `macro_rules!`block. They way this is done is by specifying patterns, just like the `match`construct does. If a pattern is matched then the following macro is used for the text generation.

# What I achieved
For this very simple example declarative macros were enough, but I am curious about the other types.

Current documentation does not seem to be very supportive of regular users using these.

I would like to explore the other types too.

So, in this example, what I achieved was to be able to declare a couple of macros, one of them will create a new struct type, this definition does include a call to another macro to derive its debug string generator. The second macro does a trait implementation for that new type.

# Conclusion
This is a very powerful feature! Of course very powerful to avoid code repetition, but I can see how this is critical. In a language that is so strongly typed, you certainly need facilities to be able to generate code with generic types, this is how Rust becomes more productive and it does not force you to write everything unlike other languages.


# References
* [The Rust book, chapter 19](https://doc.rust-lang.org/stable/book/ch19-06-macros.html)
* [The Rust book, version 1.3.0](https://doc.rust-lang.org/1.30.0/book/first-edition/macros.html)
* [Rust by Example, about macros](https://doc.rust-lang.org/stable/rust-by-example/macros/syntax.html)
