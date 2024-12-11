

pub mod src {
pub mod alloc_testing;
pub mod compare_int;
pub mod compare_pointer;
pub mod compare_string;
pub mod framework;
pub mod hash_int;
pub mod hash_pointer;
pub mod hash_string;
pub mod set;
}
pub use src::{
    alloc_testing::*,
    compare_int::*,
    compare_pointer::*,
    compare_string::*,
    framework::*,
    hash_int::*,
    hash_pointer::*,
    hash_string::*,
    set::*
};
