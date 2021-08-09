use mycode::mycode_core::core_print;
// This does not work!
//use mycode::ut_common_code::ut_print;

// This does not work!
//use mycode::test::ut_common_code::ut_print;

// This does not work!
//use mycode::tests::ut_common_code::ut_print;

fn main() {
    println!("Hello from the example code");
    core_print();
    //ut_print();
}
