/* SPDX-License-Identifier: GPL-2.0 */

/* C: extern void __iomem *auxio_register; */
unsafe extern "C" {
    pub static mut auxio_register: *mut core::ffi::c_void;
}

/*
 * C conditional dependency:
 *   #if defined(__sparc__) && defined(__arch64__)
 *   #include <asm/auxio_64.h>
 *   #else
 *   #include <asm/auxio_32.h>
 *   #endif
 *
 * The declarations from the selected architecture-specific header are
 * supplied by the corresponding Rust translation unit.
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
