/* SPDX-License-Identifier: GPL-2.0 */
// _LINUX_CRC_CCITT_H

extern "C" {
    pub static crc_ccitt_table: [u16; 256];

    pub fn crc_ccitt(crc: u16, buffer: *const u8, len: usize) -> u16;
}

#[inline]
pub unsafe fn crc_ccitt_byte(crc: u16, c: u8) -> u16 {
    (crc >> 8) ^ crc_ccitt_table[((crc ^ c) & 0xff) as usize]
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
