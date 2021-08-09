
use derive_more::{Deref, DerefMut};
use serde::Serialize;

#[derive(Debug)]
pub enum PropertyValue {
    Integer(i32),
    String(String),
}

#[derive(Debug)]
pub struct Property {
    pub name: String,
    pub property: PropertyValue,
}

#[derive(Debug, Deref, DerefMut, Serialize)]
pub struct PropertyList(pub Vec<Property>);



fn main() {
    println!("Hello, world!");





}
