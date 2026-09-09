/* SPDX-License-Identifier: GPL-2.0 */

// <linux/limits.h> — for INT_MAX (dependency supplied externally)
// <linux/string.h>
// <asm/byteorder.h>

pub const INT32_MAX: i32 = S32_MAX;
pub const UINT32_MAX: u32 = U32_MAX;

pub type fdt16_t = __be16;
pub type fdt32_t = __be32;
pub type fdt64_t = __be64;

#[inline]
pub fn fdt32_to_cpu(x: fdt32_t) -> u32 {
    be32_to_cpu(x)
}

#[inline]
pub fn cpu_to_fdt32(x: u32) -> fdt32_t {
    cpu_to_be32(x)
}

#[inline]
pub fn fdt64_to_cpu(x: fdt64_t) -> u64 {
    be64_to_cpu(x)
}

#[inline]
pub fn cpu_to_fdt64(x: u64) -> fdt64_t {
    cpu_to_be64(x)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
