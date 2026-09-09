/*
 * f75375s.h - platform data structure for f75375s sensor
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2007, Riku Voipio <riku.voipio@iki.fi>
 */

/* We want to set fans spinning on systems where there is no
 * BIOS to do that for us */
#[repr(C)]
pub struct f75375s_platform_data {
    pub pwm: [u8; 2],
    pub pwm_enable: [u8; 2],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
