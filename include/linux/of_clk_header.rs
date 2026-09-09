/* SPDX-License-Identifier: GPL-2.0 */
/*
 * OF clock helpers
 */

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct of_device_id {
    _private: [u8; 0],
}

/* CONFIG_COMMON_CLK && CONFIG_OF */
#[cfg(all(CONFIG_COMMON_CLK, CONFIG_OF))]
extern "C" {
    pub fn of_clk_get_parent_count(np: *const device_node) -> u32;
    pub fn of_clk_get_parent_name(
        np: *const device_node,
        index: core::ffi::c_int,
    ) -> *const core::ffi::c_char;
    pub fn of_clk_init(matches: *const of_device_id);
}

/* !CONFIG_COMMON_CLK || !CONFIG_OF */
#[cfg(not(all(CONFIG_COMMON_CLK, CONFIG_OF)))]
#[inline]
pub unsafe fn of_clk_get_parent_count(np: *const device_node) -> u32 {
    0
}

#[cfg(not(all(CONFIG_COMMON_CLK, CONFIG_OF)))]
#[inline]
pub unsafe fn of_clk_get_parent_name(
    np: *const device_node,
    index: core::ffi::c_int,
) -> *const core::ffi::c_char {
    core::ptr::null()
}

#[cfg(not(all(CONFIG_COMMON_CLK, CONFIG_OF)))]
#[inline]
pub unsafe fn of_clk_init(matches: *const of_device_id) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
