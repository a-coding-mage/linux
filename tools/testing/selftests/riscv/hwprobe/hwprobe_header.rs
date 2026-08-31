/* SPDX-License-Identifier: GPL-2.0-only */

// C header dependencies:
// #include <stddef.h>
// #include <asm/hwprobe.h>

// Original C macro:
// #if __BYTE_ORDER == __BIG_ENDIAN
// # define le32_bswap(_x)                         \
//      ((((_x) & 0x000000ffU) << 24) |            \
//       (((_x) & 0x0000ff00U) <<  8) |            \
//       (((_x) & 0x00ff0000U) >>  8) |            \
//       (((_x) & 0xff000000U) >> 24))
// #else
// # define le32_bswap(_x) (_x)
// #endif
#[cfg(target_endian = "big")]
pub const fn le32_bswap(_x: u32) -> u32 {
    ((_x & 0x000000ffU32) << 24)
        | ((_x & 0x0000ff00U32) << 8)
        | ((_x & 0x00ff0000U32) >> 8)
        | ((_x & 0xff000000U32) >> 24)
}

#[cfg(not(target_endian = "big"))]
pub const fn le32_bswap(_x: u32) -> u32 {
    _x
}

/*
 * Rather than relying on having a new enough libc to define this, just do it
 * ourselves.  This way we don't need to be coupled to a new-enough libc to
 * contain the call.
 */
unsafe extern "C" {
    pub fn riscv_hwprobe(
        pairs: *mut riscv_hwprobe,
        pair_count: usize,
        cpusetsize: usize,
        cpus: *mut core::ffi::c_ulong,
        flags: core::ffi::c_uint,
    ) -> core::ffi::c_long;
}
