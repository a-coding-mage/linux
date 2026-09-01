/* SPDX-License-Identifier: GPL-2.0 */

pub const AVC_PLUG_INFO_BUF_BYTES: usize = 4;

#[repr(C)]
pub struct fw_unit {
    _private: [u8; 0],
}

/*
 * AV/C Digital Interface Command Set General Specification 4.2
 * (Sep 2004, 1394TA)
 */
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum avc_general_plug_dir {
    AVC_GENERAL_PLUG_DIR_IN = 0,
    AVC_GENERAL_PLUG_DIR_OUT = 1,
    AVC_GENERAL_PLUG_DIR_COUNT = 2,
}

unsafe extern "C" {
    pub fn avc_general_set_sig_fmt(
        unit: *mut fw_unit,
        rate: ::core::ffi::c_uint,
        dir: avc_general_plug_dir,
        plug: ::core::ffi::c_ushort,
    ) -> ::core::ffi::c_int;

    pub fn avc_general_get_sig_fmt(
        unit: *mut fw_unit,
        rate: *mut ::core::ffi::c_uint,
        dir: avc_general_plug_dir,
        plug: ::core::ffi::c_ushort,
    ) -> ::core::ffi::c_int;

    pub fn avc_general_get_plug_info(
        unit: *mut fw_unit,
        subunit_type: ::core::ffi::c_uint,
        subunit_id: ::core::ffi::c_uint,
        subfunction: ::core::ffi::c_uint,
        info: *mut u8,
    ) -> ::core::ffi::c_int;

    pub fn fcp_avc_transaction(
        unit: *mut fw_unit,
        command: *const ::core::ffi::c_void,
        command_size: ::core::ffi::c_uint,
        response: *mut ::core::ffi::c_void,
        response_size: ::core::ffi::c_uint,
        response_match_bytes: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;

    pub fn fcp_bus_reset(unit: *mut fw_unit);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
