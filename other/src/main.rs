#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use tutorials::rust_async;

use crate::tutorials::enums;
use crate::tutorials::collections;
use crate::tutorials::error_handling;
use crate::tutorials::types;
use crate::tutorials::functional_features;
use crate::tutorials::smart_pointers;
use crate::tutorials::fearless_concurrency;

mod tutorials;

fn main() {
    // tutorials::guessing_game::guessing_game();

    // tutorials::concepts::variables();

    // tutorials::concepts::data_types();

    // tutorials::concepts::loops();

    // tutorials::ownership::ownership();

    // tutorials::structs::main();

    // enums::main();

    // collections::main();

    // error_handling::main();

    // types::main();

    //functional_features::main();

    // smart_pointers::main();

    // fearless_concurrency::main();

    rust_async::main();
}
