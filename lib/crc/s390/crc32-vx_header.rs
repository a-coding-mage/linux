/* SPDX-License-Identifier: GPL-2.0 */

// Translation of the C header's externally defined functions.
extern "C" {
    pub fn crc32_be_vgfm_16(crc: u32, buf: *const u8, size: usize) -> u32;
    pub fn crc32_le_vgfm_16(crc: u32, buf: *const u8, size: usize) -> u32;
    pub fn crc32c_le_vgfm_16(crc: u32, buf: *const u8, size: usize) -> u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
