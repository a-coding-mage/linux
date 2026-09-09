/* SPDX-License-Identifier: GPL-2.0 */
/*
 * This is a direct copy of the ev96100.h file, with a global
 * search and replace. The numbers are the same.
 *
 * The reason I'm duplicating this is so that the 64120/96100
 * defines won't be confusing in the source code.
 */

pub const MIPS_GT_BASE: usize = 0x1be00000;

extern "C" {
    pub static mut _pcictrl_gt64120: core::ffi::c_ulong;
}

/*
 * GT64120 config space base address
 */
#[macro_export]
macro_rules! GT64120_BASE {
    () => {
        unsafe { $crate::_pcictrl_gt64120 }
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
