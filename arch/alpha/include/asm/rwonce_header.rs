/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2019 Google LLC.
 */

// The original header is guarded by CONFIG_SMP.  The following item is
// emitted only when the corresponding Rust configuration is enabled.
#[cfg(feature = "CONFIG_SMP")]
mod config_smp {
    // Supplied by the Alpha barrier dependency (asm/barrier.h).
    unsafe extern "C" {
        fn mb();
    }

    /*
     * Alpha is apparently daft enough to reorder address-dependent loads
     * on some CPU implementations. Knock some common sense into it with
     * a memory barrier in READ_ONCE().
     *
     * For the curious, more information about this unusual reordering is
     * available in chapter 15 of the "perfbook":
     *
     *  https://kernel.org/pub/linux/kernel/people/paulmck/perfbook/perfbook.html
     *
     */
    #[allow(non_snake_case)]
    pub unsafe fn __READ_ONCE<T: Copy>(x: *const T) -> T {
        let __x = core::ptr::read_volatile(x);
        mb();
        __x
    }
}

// The original header includes <asm-generic/rwonce.h>; its declarations are
// provided by the surrounding translation unit.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
