pub mod structs;

/// Re-Export these structs so we can use them like axumbitslib::Student/etc.
pub use structs::student_struct::Student;
pub use structs::state_structs::ArcMutexStateInfo;
pub use structs::state_structs::StateInfo;