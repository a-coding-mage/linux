/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies corresponding to <linux/phylink.h> and <linux/regmap.h>.

#[repr(C)]
pub struct phylink_pcs {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fwnode_handle {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn mtk_pcs_lynxi_create(
        dev: *mut device,
        fwnode: *mut fwnode_handle,
        regmap: *mut regmap,
        ana_rgc3: u32,
    ) -> *mut phylink_pcs;

    pub fn mtk_pcs_lynxi_destroy(pcs: *mut phylink_pcs);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
