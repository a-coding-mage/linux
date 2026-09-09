/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by asm/types.h in the original header.

unsafe extern "C" {
    pub static byte_rev_table: [u8; 256];
}

#[inline(always)]
pub unsafe fn generic___bitrev8(byte: u8) -> u8 {
    // SAFETY: The caller preserves the original C contract for the external table.
    unsafe { byte_rev_table[byte as usize] }
}

#[inline(always)]
pub unsafe fn generic___bitrev16(x: u16) -> u16 {
    ((unsafe { generic___bitrev8((x & 0xff) as u8) } as u16) << 8)
        | (unsafe { generic___bitrev8((x >> 8) as u8) } as u16)
}

#[inline(always)]
pub unsafe fn generic___bitrev32(x: u32) -> u32 {
    ((unsafe { generic___bitrev16((x & 0xffff) as u16) } as u32) << 16)
        | (unsafe { generic___bitrev16((x >> 16) as u16) } as u32)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
