

use examples_with_cargo::*;

// Yes we actually need this
use examples_with_cargo::local::*;

fn main() {
    tell_me();

    // You need to explicitly include the module local
    // for this to work
    tell_me_more();
}

