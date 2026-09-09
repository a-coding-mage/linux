/* SPDX-License-Identifier: GPL-2.0 */

// Translated from <asm-generic/sections.h> dependency context.

/// External linker-defined start symbol.
pub unsafe extern "C" {
    pub static mut _start: [core::ffi::c_char; 0];
}

/// `asmlinkage void csky_start(unsigned int unused, void *dtb_start);`
///
/// The `asmlinkage` calling-convention annotation is preserved by the C ABI.
pub unsafe extern "C" {
    pub fn csky_start(unused: u32, dtb_start: *mut core::ffi::c_void);

    /// `asmlinkage void csky_start_secondary(void);`
    pub fn csky_start_secondary();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
