#![crate_type = "lib"]

// EMIT_MIR_FOR_EACH_PANIC_STRATEGY
//@ ignore-debug: the debug assertions prevent the inlining we are testing for
//@ compile-flags: -Zmir-opt-level=2 -Zinline-mir -Cdebug-assertions=no

// EMIT_MIR unwrap_unchecked.unwrap_unchecked.Inline.diff
// EMIT_MIR unwrap_unchecked.unwrap_unchecked.PreCodegen.after.mir
pub unsafe fn unwrap_unchecked<T>(slf: Option<T>) -> T {
    // CHECK-LABEL: fn unwrap_unchecked(
    // CHECK: (inlined #[track_caller] Option::<T>::unwrap_unchecked)
    slf.unwrap_unchecked()
}
