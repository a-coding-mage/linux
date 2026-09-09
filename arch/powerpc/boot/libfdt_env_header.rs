/* SPDX-License-Identifier: GPL-2.0 */

// Original C header guard: _ARCH_POWERPC_BOOT_LIBFDT_ENV_H
// Dependencies supplied by the surrounding translation unit:
// types.h, string.h, and of.h.

pub const INT_MAX: i32 = ((!0u32 >> 1) as i32);
pub const UINT32_MAX: u32 = !0u32;
pub const INT32_MAX: i32 = (UINT32_MAX >> 1) as i32;

pub type uintptr_t = usize;

pub type fdt16_t = __be16;
pub type fdt32_t = __be32;
pub type fdt64_t = __be64;

macro_rules! fdt16_to_cpu {
    ($x:expr) => {
        be16_to_cpu($x)
    };
}

macro_rules! cpu_to_fdt16 {
    ($x:expr) => {
        cpu_to_be16($x)
    };
}

macro_rules! fdt32_to_cpu {
    ($x:expr) => {
        be32_to_cpu($x)
    };
}

macro_rules! cpu_to_fdt32 {
    ($x:expr) => {
        cpu_to_be32($x)
    };
}

macro_rules! fdt64_to_cpu {
    ($x:expr) => {
        be64_to_cpu($x)
    };
}

macro_rules! cpu_to_fdt64 {
    ($x:expr) => {
        cpu_to_be64($x)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
