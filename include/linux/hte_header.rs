/* SPDX-License-Identifier: GPL-2.0 */

// External Linux types and symbols referenced by this header are supplied by
// the surrounding translation unit.

#[allow(non_camel_case_types)]
pub enum hte_device {}
#[allow(non_camel_case_types)]
pub enum of_phandle_args {}
#[allow(non_camel_case_types)]
pub enum device {}

pub type u8 = ::core::ffi::c_uchar;
pub type u32 = ::core::ffi::c_uint;
pub type u64 = ::core::ffi::c_ulonglong;
pub type clockid_t = ::core::ffi::c_int;

/// HTE line edge flags.
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum hte_edge {
    HTE_EDGE_NO_SETUP = 1 << 0,
    HTE_RISING_EDGE_TS = 1 << 1,
    HTE_FALLING_EDGE_TS = 1 << 2,
}

/// HTE subsystem return values used during callback.
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum hte_return {
    HTE_CB_HANDLED,
    HTE_RUN_SECOND_CB,
}

/// HTE timestamp data.
#[repr(C)]
pub struct hte_ts_data {
    pub tsc: u64,
    pub seq: u64,
    pub raw_level: ::core::ffi::c_int,
}

/// Clock source info that HTE provider uses to timestamp.
#[repr(C)]
pub struct hte_clk_info {
    pub hz: u64,
    pub type_: clockid_t,
}

pub type hte_ts_cb_t = Option<unsafe extern "C" fn(*mut hte_ts_data, *mut ::core::ffi::c_void) -> hte_return>;
pub type hte_ts_sec_cb_t = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> hte_return>;

/// Line attributes.
#[repr(C)]
pub struct hte_line_attr {
    pub line_id: u32,
    pub line_data: *mut ::core::ffi::c_void,
    pub edge_flags: usize,
    pub name: *const ::core::ffi::c_char,
}

/// HTE timestamp descriptor.
#[repr(C)]
pub struct hte_ts_desc {
    pub attr: hte_line_attr,
    pub hte_data: *mut ::core::ffi::c_void,
}

/// HTE operations set by providers.
#[repr(C)]
pub struct hte_ops {
    pub request: Option<unsafe extern "C" fn(*mut hte_chip, *mut hte_ts_desc, u32) -> ::core::ffi::c_int>,
    pub release: Option<unsafe extern "C" fn(*mut hte_chip, *mut hte_ts_desc, u32) -> ::core::ffi::c_int>,
    pub enable: Option<unsafe extern "C" fn(*mut hte_chip, u32) -> ::core::ffi::c_int>,
    pub disable: Option<unsafe extern "C" fn(*mut hte_chip, u32) -> ::core::ffi::c_int>,
    pub get_clk_src_info: Option<unsafe extern "C" fn(*mut hte_chip, *mut hte_clk_info) -> ::core::ffi::c_int>,
}

/// Abstract HTE chip.
#[repr(C)]
pub struct hte_chip {
    pub name: *const ::core::ffi::c_char,
    pub dev: *mut device,
    pub ops: *const hte_ops,
    pub nlines: u32,
    pub xlate_of: Option<unsafe extern "C" fn(*mut hte_chip, *const of_phandle_args, *mut hte_ts_desc, *mut u32) -> ::core::ffi::c_int>,
    pub xlate_plat: Option<unsafe extern "C" fn(*mut hte_chip, *mut hte_ts_desc, *mut u32) -> ::core::ffi::c_int>,
    pub match_from_linedata: Option<unsafe extern "C" fn(*const hte_chip, *const hte_ts_desc) -> bool>,
    pub of_hte_n_cells: u8,
    pub gdev: *mut hte_device,
    pub data: *mut ::core::ffi::c_void,
}

// When CONFIG_HTE is enabled, these functions are provided by the HTE subsystem.
#[cfg(feature = "CONFIG_HTE")]
extern "C" {
    pub fn devm_hte_register_chip(chip: *mut hte_chip) -> ::core::ffi::c_int;
    pub fn hte_push_ts_ns(chip: *const hte_chip, xlated_id: u32, data: *mut hte_ts_data) -> ::core::ffi::c_int;
    pub fn hte_init_line_attr(desc: *mut hte_ts_desc, line_id: u32, edge_flags: usize, name: *const ::core::ffi::c_char, data: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    pub fn hte_ts_get(dev: *mut device, desc: *mut hte_ts_desc, index: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn hte_ts_put(desc: *mut hte_ts_desc) -> ::core::ffi::c_int;
    pub fn hte_request_ts_ns(desc: *mut hte_ts_desc, cb: hte_ts_cb_t, tcb: hte_ts_sec_cb_t, data: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    pub fn devm_hte_request_ts_ns(dev: *mut device, desc: *mut hte_ts_desc, cb: hte_ts_cb_t, tcb: hte_ts_sec_cb_t, data: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    pub fn of_hte_req_count(dev: *mut device) -> ::core::ffi::c_int;
    pub fn hte_enable_ts(desc: *mut hte_ts_desc) -> ::core::ffi::c_int;
    pub fn hte_disable_ts(desc: *mut hte_ts_desc) -> ::core::ffi::c_int;
    pub fn hte_get_clk_src_info(desc: *const hte_ts_desc, ci: *mut hte_clk_info) -> ::core::ffi::c_int;
}

// !CONFIG_HTE stubs return -EOPNOTSUPP, supplied by the surrounding dependency.
#[cfg(not(feature = "CONFIG_HTE"))]
const EOPNOTSUPP: ::core::ffi::c_int = 95;

#[cfg(not(feature = "CONFIG_HTE"))]
pub unsafe fn devm_hte_register_chip(_: *mut hte_chip) -> ::core::ffi::c_int { -EOPNOTSUPP }
#[cfg(not(feature = "CONFIG_HTE"))]
pub unsafe fn hte_push_ts_ns(_: *const hte_chip, _: u32, _: *const hte_ts_data) -> ::core::ffi::c_int { -EOPNOTSUPP }
#[cfg(not(feature = "CONFIG_HTE"))]
pub unsafe fn hte_init_line_attr(_: *mut hte_ts_desc, _: u32, _: usize, _: *const ::core::ffi::c_char, _: *mut ::core::ffi::c_void) -> ::core::ffi::c_int { -EOPNOTSUPP }
#[cfg(not(feature = "CONFIG_HTE"))]
pub unsafe fn hte_ts_get(_: *mut device, _: *mut hte_ts_desc, _: ::core::ffi::c_int) -> ::core::ffi::c_int { -EOPNOTSUPP }
#[cfg(not(feature = "CONFIG_HTE"))]
pub unsafe fn hte_ts_put(_: *mut hte_ts_desc) -> ::core::ffi::c_int { -EOPNOTSUPP }
#[cfg(not(feature = "CONFIG_HTE"))]
pub unsafe fn hte_request_ts_ns(_: *mut hte_ts_desc, _: hte_ts_cb_t, _: hte_ts_sec_cb_t, _: *mut ::core::ffi::c_void) -> ::core::ffi::c_int { -EOPNOTSUPP }
#[cfg(not(feature = "CONFIG_HTE"))]
pub unsafe fn devm_hte_request_ts_ns(_: *mut device, _: *mut hte_ts_desc, _: hte_ts_cb_t, _: hte_ts_sec_cb_t, _: *mut ::core::ffi::c_void) -> ::core::ffi::c_int { -EOPNOTSUPP }
#[cfg(not(feature = "CONFIG_HTE"))]
pub unsafe fn of_hte_req_count(_: *mut device) -> ::core::ffi::c_int { -EOPNOTSUPP }
#[cfg(not(feature = "CONFIG_HTE"))]
pub unsafe fn hte_enable_ts(_: *mut hte_ts_desc) -> ::core::ffi::c_int { -EOPNOTSUPP }
#[cfg(not(feature = "CONFIG_HTE"))]
pub unsafe fn hte_disable_ts(_: *mut hte_ts_desc) -> ::core::ffi::c_int { -EOPNOTSUPP }
#[cfg(not(feature = "CONFIG_HTE"))]
pub unsafe fn hte_get_clk_src_info(_: *const hte_ts_desc, _: *mut hte_clk_info) -> ::core::ffi::c_int { -EOPNOTSUPP }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
