/* SPDX-License-Identifier: GPL-2.0-only */

// Dependency intent from <linux/dpll.h> and "core.h" is preserved here.

pub struct fwnode_handle;

/**
 * struct zl3073x_pin_props - pin properties
 * @fwnode: pin firmware node
 * @dpll_props: DPLL core pin properties
 * @package_label: pin package label
 * @esync_control: embedded sync support
 */
#[repr(C)]
pub struct zl3073x_pin_props {
    pub fwnode: *mut fwnode_handle,
    pub dpll_props: dpll_pin_properties,
    pub package_label: [core::ffi::c_char; 8],
    pub esync_control: bool,
}

unsafe extern "C" {
    pub fn zl3073x_prop_dpll_type_get(
        zldev: *mut zl3073x_dev,
        index: u8,
    ) -> dpll_type;

    pub fn zl3073x_pin_props_get(
        zldev: *mut zl3073x_dev,
        dir: dpll_pin_direction,
        index: u8,
    ) -> *mut zl3073x_pin_props;

    pub fn zl3073x_pin_props_put(props: *mut zl3073x_pin_props);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
