/* SPDX-License-Identifier: GPL-2.0 */

// Dependency provided by the translated clk-starfive-jh71x0 header.

/* top clocks of ISP/VOUT domain from JH7110 SYSCRG */
#[repr(C)]
pub struct jh7110_top_sysclk {
    pub top_clks: *mut clk_bulk_data,
    pub top_clks_num: core::ffi::c_int,
}

unsafe extern "C" {
    pub fn jh7110_reset_controller_register(
        priv_: *mut jh71x0_clk_priv,
        adev_name: *const core::ffi::c_char,
        adev_id: u32,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
