/* SPDX-License-Identifier: GPL-2.0 */

// `dev_t` is supplied by the surrounding translated dependency set.
extern "C" {
    pub fn md_autodetect_dev(dev: dev_t);
}

#[cfg(feature = "CONFIG_BLK_DEV_MD")]
extern "C" {
    pub fn md_run_setup();
}

#[cfg(not(feature = "CONFIG_BLK_DEV_MD"))]
#[inline]
pub fn md_run_setup() {
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
