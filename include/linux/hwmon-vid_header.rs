/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
    hwmon-vid.h - VID/VRM/VRD voltage conversions

    Originally part of lm_sensors
    Copyright (c) 2002 Mark D. Studebaker <mdsxyz123@yahoo.com>
    With assistance from Trent Piepho <xyzzy@speakeasy.org>

*/

extern "C" {
    pub fn vid_from_reg(val: i32, vrm: u8) -> i32;
    pub fn vid_which_vrm() -> u8;
}

/* vrm is the VRM/VRD document version multiplied by 10.
   val is in mV to avoid floating point in the kernel.
   Returned value is the 4-, 5- or 6-bit VID code.
   Note that only VRM 9.x is supported for now. */
pub(crate) fn vid_to_reg(val: i32, vrm: u8) -> i32 {
    match vrm {
        91 => { /* VRM 9.1 */
            if val >= 1100 && val <= 1850 {
                ((18499 - val * 10) / 25 + 5) / 10
            } else {
                -1
            }
        }
        90 => { /* VRM 9.0 */
            if val >= 1100 && val <= 1850 {
                ((18499 - val * 10) / 25 + 5) / 10
            } else {
                -1
            }
        }
        _ => {
            // EINVAL is supplied by the kernel environment.
            -EINVAL
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
