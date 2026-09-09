/* SPDX-License-Identifier: GPL-2.0 */

// Opaque types supplied by other translation units.
#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct icst_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

/**
 * enum icst_control_type - the type of ICST control register
 */
#[repr(C)]
#[derive Copy, Clone, PartialEq, Eq)]
pub enum icst_control_type {
    ICST_VERSATILE, /* The standard type, all control bits available */
    ICST_INTEGRATOR_AP_CM, /* Only 8 bits of VDW available */
    ICST_INTEGRATOR_AP_SYS, /* Only 8 bits of VDW available */
    ICST_INTEGRATOR_AP_PCI, /* Odd bit pattern storage */
    ICST_INTEGRATOR_CP_CM_CORE, /* Only 8 bits of VDW and 3 bits of OD */
    ICST_INTEGRATOR_CP_CM_MEM, /* Only 8 bits of VDW and 3 bits of OD */
    ICST_INTEGRATOR_IM_PD1, /* Like the Versatile, all control bits */
}

/**
 * struct clk_icst_desc - descriptor for the ICST VCO
 * @params: ICST parameters
 * @vco_offset: offset to the ICST VCO from the provided memory base
 * @lock_offset: offset to the ICST VCO locking register from the provided
 *\tmemory base
 */
#[repr(C)]
pub struct clk_icst_desc {
    pub params: *const icst_params,
    pub vco_offset: u32,
    pub lock_offset: u32,
}

extern "C" {
    pub fn icst_clk_register(
        dev: *mut device,
        desc: *const clk_icst_desc,
        name: *const core::ffi::c_char,
        parent_name: *const core::ffi::c_char,
        base: *mut core::ffi::c_void,
    ) -> *mut clk;

    pub fn icst_clk_setup(
        dev: *mut device,
        desc: *const clk_icst_desc,
        name: *const core::ffi::c_char,
        parent_name: *const core::ffi::c_char,
        map: *mut regmap,
        ctype: icst_control_type,
    ) -> *mut clk;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
