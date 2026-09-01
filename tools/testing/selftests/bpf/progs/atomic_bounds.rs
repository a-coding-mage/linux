// SPDX-License-Identifier: GPL-2.0

// Dependencies from the original C source:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_tracing.h>
// #include <stdbool.h>

#[cfg(feature = "enable_atomics_tests")]
#[unsafe(link_section = ".data")]
pub static mut skip_tests: bool = false;

#[cfg(not(feature = "enable_atomics_tests"))]
pub static mut skip_tests: bool = true;

#[unsafe(link_section = "fentry/bpf_fentry_test1")]
pub unsafe extern "C" fn sub(x: core::ffi::c_int) -> core::ffi::c_int {
    let _ = x;

    // Original C condition: #ifdef ENABLE_ATOMICS_TESTS
    #[cfg(feature = "enable_atomics_tests")]
    {
        use core::sync::atomic::{AtomicI32, Ordering};

        let a = AtomicI32::new(0);
        let b = a.fetch_add(1, Ordering::SeqCst);
        /* b is certainly 0 here. Can the verifier tell? */
        while b != 0 {
            continue;
        }
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
