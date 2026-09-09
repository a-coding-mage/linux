/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent: <linux/types.h> supplies the C-compatible u16 and u8 types.

unsafe extern "C" {
    pub fn crc_t10dif_update(crc: u16, p: *const u8, len: usize) -> u16;
}

#[inline]
pub unsafe fn crc_t10dif(p: *const u8, len: usize) -> u16 {
    unsafe { crc_t10dif_update(0, p, len) }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
