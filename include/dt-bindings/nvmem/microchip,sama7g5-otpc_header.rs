/* SPDX-License-Identifier: GPL-2.0 OR BSD-2-Clause */

/*
 * Need to have it as a multiple of 4 as NVMEM memory is registered with
 * stride = 4.
 */
macro_rules! OTP_PKT {
    ($id:expr) => {
        (($id) * 4)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
