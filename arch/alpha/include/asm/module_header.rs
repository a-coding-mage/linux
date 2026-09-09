/* SPDX-License-Identifier: GPL-2.0 */

// Dependency equivalent of: #include <asm-generic/module.h>

#[repr(C)]
pub struct mod_arch_specific {
    pub gotsecindex: core::ffi::c_uint,
}

pub const ARCH_SHF_SMALL: _ = SHF_ALPHA_GPREL;

// When MODULE is defined, the original header emits:
// .section .got,"aws",@progbits; .align 3; .previous

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
