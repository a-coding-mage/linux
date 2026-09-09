/* SPDX-License-Identifier: GPL-2.0 */
/*
 *  Copyright 2025 NXP
 */

// C dependencies supplied by other translation units:
// linux/bits.h, linux/clk-provider.h, linux/scmi_protocol.h, linux/types.h

pub const NOT_ATOMIC: bool = false;
pub const ATOMIC: bool = true;

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum scmi_clk_feats {
    SCMI_CLK_ATOMIC_SUPPORTED,
    SCMI_CLK_STATE_CTRL_SUPPORTED,
    SCMI_CLK_RATE_CTRL_SUPPORTED,
    SCMI_CLK_PARENT_CTRL_SUPPORTED,
    SCMI_CLK_DUTY_CYCLE_SUPPORTED,
    SCMI_CLK_EXT_OEM_SSC_SUPPORTED,
    SCMI_CLK_FEATS_COUNT,
}

pub const SCMI_MAX_CLK_OPS: u32 = 1u32 << (SCMI_CLK_FEATS_COUNT as u32);

#[repr(C)]
pub struct scmi_clk {
    pub id: u32,
    pub dev: *mut device,
    pub hw: clk_hw,
    pub info: *const scmi_clock_info,
    pub ph: *const scmi_protocol_handle,
    pub parent_data: *mut clk_parent_data,
}

// Equivalent to: container_of(clk, struct scmi_clk, hw)
#[inline]
pub unsafe fn to_scmi_clk(clk: *mut clk_hw) -> *mut scmi_clk {
    (clk as *mut u8).sub(core::mem::offset_of!(scmi_clk, hw)) as *mut scmi_clk
}

extern "C" {
    pub static scmi_proto_clk_ops: *const scmi_clk_proto_ops;
}

#[repr(C)]
pub struct scmi_clk_oem {
    pub query_ext_oem_feats: Option<
        unsafe extern "C" fn(
            ph: *const scmi_protocol_handle,
            id: u32,
            feats_key: *mut core::ffi::c_uint,
        ) -> core::ffi::c_int,
    >,
    pub set_spread_spectrum: Option<
        unsafe extern "C" fn(
            hw: *mut clk_hw,
            ss_conf: *const clk_spread_spectrum,
        ) -> core::ffi::c_int,
    >,
}

extern "C" {
    pub fn scmi_clk_oem_init(dev: *mut scmi_device) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
