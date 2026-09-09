/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by generated/vdso-offsets.h in the original header.

// C equivalent:
// #define VDSO_SYMBOL(tsk, name) ((tsk)->mm->context.vdso_base + (vdso_offset_##name))
// The offset corresponding to `name` is supplied as the Rust expression `$offset`.
#[macro_export]
macro_rules! VDSO_SYMBOL {
    ($tsk:expr, $offset:expr) => {
        unsafe { (*$tsk).mm.context.vdso_base + $offset }
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
