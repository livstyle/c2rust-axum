pub mod src {
    pub mod alloc_testing;
    pub mod compare_int;
    pub mod compare_pointer;
    pub mod compare_string;
    pub mod framework;
}
pub use src::alloc_testing::*;
pub use src::compare_int::*;
pub use src::compare_pointer::*;
pub use src::compare_string::*;
pub use src::framework::*;
