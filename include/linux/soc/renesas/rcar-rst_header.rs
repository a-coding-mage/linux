/* SPDX-License-Identifier: GPL-2.0 */

/*
 * The CONFIG_RST_RCAR conditional is preserved from the C header.  The
 * symbols u32, u64, and ENODEV are supplied by the surrounding kernel
 * translation.
 */

#[cfg(CONFIG_RST_RCAR)]
unsafe extern "C" {
    pub fn rcar_rst_read_mode_pins(mode: *mut u32) -> i32;
    pub fn rcar_rst_set_rproc_boot_addr(boot_addr: u64) -> i32;
}

#[cfg(not(CONFIG_RST_RCAR))]
pub unsafe fn rcar_rst_read_mode_pins(_mode: *mut u32) -> i32 {
    -ENODEV
}

#[cfg(not(CONFIG_RST_RCAR))]
pub unsafe fn rcar_rst_set_rproc_boot_addr(_boot_addr: u64) -> i32 {
    -ENODEV
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
