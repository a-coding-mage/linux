/* SPDX-License-Identifier: GPL-2.0 */
/*
 * SGI One-Wire (W1) IP
 */

// Original C header guard: PLATFORM_DATA_SGI_W1_H

#[repr(C)]
pub struct sgi_w1_platform_data {
    pub dev_id: [i8; 64],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
