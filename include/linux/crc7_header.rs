/* SPDX-License-Identifier: GPL-2.0 */

// Dependency equivalent of <linux/types.h>.

unsafe extern "C" {
    pub fn crc7_be(crc: u8, buffer: *const u8, len: usize) -> u8;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
