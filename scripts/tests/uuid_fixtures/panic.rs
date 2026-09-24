// SPDX-License-Identifier: GPL-2.0-only
//! Private executable panic termination; genuine native core is linked.
#[panic_handler]
fn panic(_: &core::panic::PanicInfo<'_>) -> ! {
    unsafe extern "C" {
        fn abort() -> !;
    }
    // SAFETY: libc abort terminates this private test process.
    unsafe { abort() }
}
