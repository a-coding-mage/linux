/* SPDX-License-Identifier: GPL-2.0 */

/*
 * The VDSO symbols are mapped into Linux so we can just use regular symbol
 * addressing to get their offsets in userspace.  The symbols are mapped at
 * an offset of 0, but since the linker must support setting weak undefined
 * symbols to the absolute address 0 it also happens to support other low
 * addresses even when the code model suggests those low addresses would not
 * otherwise be available.
 */

/*
 * Rust cannot concatenate identifiers in stable macro_rules! macros.  The
 * symbol argument therefore names the corresponding __vdso_* linker symbol
 * directly (for example, `__vdso_clock_gettime`).
 */
#[macro_export]
macro_rules! VDSO_SYMBOL {
    ($base:expr, $symbol:ident) => {{
        extern "C" {
            static $symbol: u8;
        }
        (($base as usize).wrapping_add((&$symbol as *const u8) as usize))
            as *mut core::ffi::c_void
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
