// We need to tell rust there is another file we need to read.
mod ut_common_code;

// It does it automatically for the code in the crate, eg in src
// but it does not do it for anything else.
// That's why we do not need a mod line for the following line to work.
use mycode::mycode_core::core_print;

// But we need it for this one.
use ut_common_code::ut_print;

#[test]
fn test1() {
    core_print();
    ut_print();
}
