/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * crc-itu-t.h - CRC ITU-T V.41 routine
 *
 * Implements the standard CRC ITU-T V.41:
 *   Width 16
 *   Poly  0x1021 (x^16 + x^12 + x^5 + 1)
 *   Init  0
 */

// Dependency intent: u8, u16, and size_t correspond to the supplied C types.

unsafe extern "C" {
    pub static crc_itu_t_table: [u16; 256];

    pub fn crc_itu_t(crc: u16, buffer: *const u8, len: usize) -> u16;
}

pub unsafe fn crc_itu_t_byte(crc: u16, data: u8) -> u16 {
    (crc << 8) ^ crc_itu_t_table[(((crc >> 8) ^ data as u16) & 0xff) as usize]
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
