//! Genuine core panic handler for this freestanding process, with exit control.
unsafe extern "C" { fn proof_abort() -> !; }
#[panic_handler]
fn panic(_: &core::panic::PanicInfo<'_>) -> ! {
    // SAFETY: fixture proof_abort terminates this process, never returns.
    unsafe { proof_abort() }
}
/// Force the real core panic path to prove it was not replaced by a no-op.
#[no_mangle]
pub extern "C" fn panic_control() { panic!("expected private panic control"); }
/// Exercise the genuine build-error fallback retained by kernel atomics at O0.
#[no_mangle]
pub extern "C" fn build_error_control() { build_error::build_error("expected build-error control"); }
