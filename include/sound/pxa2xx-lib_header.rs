/* SPDX-License-Identifier: GPL-2.0 */

// Dependency equivalent of <linux/types.h>: u32 is represented by Rust's u32.

/* modem registers, used by touchscreen driver */
extern "C" {
    pub fn pxa2xx_ac97_read_modr() -> u32;
    pub fn pxa2xx_ac97_read_misr() -> u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
