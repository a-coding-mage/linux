/* SPDX-License-Identifier: GPL-2.0 */

// C dependency: <linux/types.h>

extern "C" {
    pub fn crc4(c: u8, x: u64, bits: i32) -> u8;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
