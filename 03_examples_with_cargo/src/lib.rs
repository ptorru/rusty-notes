
pub fn tell_me() {
    println!("The time is a valuable thing...");
}

pub mod local {
    pub fn tell_me_loc() {
        println!("Tell me something...")
    }

    pub fn tell_me_more() {
        for _i in 1..10 {
            tell_me_loc();
        }
    }
}