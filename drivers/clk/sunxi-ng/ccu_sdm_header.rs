/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2017 Chen-Yu Tsai. All rights reserved.
 */

// C dependencies supplied by other translation units:
// linux/clk-provider.h and ccu_common.h

#[repr(C)]
pub struct ccu_sdm_setting {
    pub rate: ::std::os::raw::c_ulong,

    /*
     * XXX We don't know what the step and bottom register fields
     * mean. Just copy the whole register value from the vendor
     * kernel for now.
     */
    pub pattern: u32,

    /*
     * M and N factors here should be the values used in
     * calculation, not the raw values written to registers
     */
    pub m: u32,
    pub n: u32,
}

#[repr(C)]
pub struct ccu_sdm_internal {
    pub table: *mut ccu_sdm_setting,
    pub table_size: u32,
    /* early SoCs don't have the SDM enable bit in the PLL register */
    pub enable: u32,
    /* second enable bit in tuning register */
    pub tuning_enable: u32,
    pub tuning_reg: u16,
}

#[macro_export]
macro_rules! _SUNXI_CCU_SDM {
    ($table:expr, $enable:expr, $reg:expr, $reg_enable:expr) => {
        ccu_sdm_internal {
            table: $table.as_mut_ptr(),
            table_size: $table.len() as u32,
            enable: $enable,
            tuning_enable: $reg_enable,
            tuning_reg: $reg,
        }
    };
}

extern "C" {
    pub fn ccu_sdm_helper_is_enabled(
        common: *mut ccu_common,
        sdm: *mut ccu_sdm_internal,
    ) -> bool;

    pub fn ccu_sdm_helper_enable(
        common: *mut ccu_common,
        sdm: *mut ccu_sdm_internal,
        rate: ::std::os::raw::c_ulong,
    );

    pub fn ccu_sdm_helper_disable(
        common: *mut ccu_common,
        sdm: *mut ccu_sdm_internal,
    );

    pub fn ccu_sdm_helper_has_rate(
        common: *mut ccu_common,
        sdm: *mut ccu_sdm_internal,
        rate: ::std::os::raw::c_ulong,
    ) -> bool;

    pub fn ccu_sdm_helper_read_rate(
        common: *mut ccu_common,
        sdm: *mut ccu_sdm_internal,
        m: u32,
        n: u32,
    ) -> ::std::os::raw::c_ulong;

    pub fn ccu_sdm_helper_get_factors(
        common: *mut ccu_common,
        sdm: *mut ccu_sdm_internal,
        rate: ::std::os::raw::c_ulong,
        m: *mut ::std::os::raw::c_ulong,
        n: *mut ::std::os::raw::c_ulong,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
