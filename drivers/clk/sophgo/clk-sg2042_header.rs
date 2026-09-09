/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent: <linux/io.h>
// Dependency intent: <linux/clk-provider.h>

/**
 * struct sg2042_clk_data - Common data of clock-controller
 * @iobase: base address of clock-controller
 * @onecell_data: used for adding providers.
 */
#[repr(C)]
pub struct sg2042_clk_data {
    pub iobase: *mut core::ffi::c_void,
    pub onecell_data: clk_hw_onecell_data,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
