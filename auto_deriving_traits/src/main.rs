use derive_more::{Deref,DerefMut};

#[derive(Deref,Debug,Clone)]
struct Numm {
    num: i32,
}

fn borrow_num(num: &Numm) {
    println!("the num is {:?}", *num);
}

#[derive(Deref,DerefMut,Debug,Clone)]
struct MyCollection(Vec<Numm>);

fn populate(col: &mut MyCollection) {
    for i in 1..3 {
        col.push(Numm{num: i});
    }
}

fn main() {
    println!("Hello, world!");
    let mynum = Numm{num: 42};
    borrow_num(&mynum);
    println!("Num is still here {:?}", *mynum);

    let mut mycol = MyCollection(Vec::new());
    populate(&mut mycol);
    println!("First entry{:?}", *mycol);
}
