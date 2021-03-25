/// tell the compiler not to optimize away the given
/// argument (which is expected to be the function call
/// you want to benchmark).
///
/// This should use core::hint::bench_black_box
/// but it's not yet stabilized, see
/// https://github.com/rust-lang/rust/issues/64102
///
/// In the meantime, it uses the same implementation
/// than Criterion.
pub fn pretend_used<T>(t: T) -> T {
    unsafe {
        let ret = std::ptr::read_volatile(&t);
        std::mem::forget(t);
        ret
    }
}
