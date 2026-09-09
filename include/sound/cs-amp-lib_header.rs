/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2024 Cirrus Logic, Inc. and
 *                    Cirrus Logic International Semiconductor Ltd.
 */

// C dependencies: <linux/efi.h> and <linux/types.h>.

use core::ffi::c_char;

#[repr(C)]
pub struct cs_dsp;

#[repr(C, packed)]
pub struct cirrus_amp_cal_data {
    pub calTarget: [u32; 2],
    pub calTime: [u32; 2],
    pub calAmbient: i8,
    pub calStatus: u8,
    pub calR: u16,
}

#[repr(C, packed)]
pub struct cirrus_amp_efi_data {
    pub size: u32,
    pub count: u32,
    // Flexible array member, counted by `count` in the C declaration.
    pub data: [cirrus_amp_cal_data; 0],
}

/**
 * struct cirrus_amp_cal_controls - definition of firmware calibration controls
 * @alg_id:    ID of algorithm containing the controls.
 * @mem_region: DSP memory region containing the controls.
 * @ambient:   Name of control for calAmbient value.
 * @calr:      Name of control for calR value.
 * @status:    Name of control for calStatus value.
 * @checksum:  Name of control for checksum value.
 */
#[repr(C)]
pub struct cirrus_amp_cal_controls {
    pub alg_id: u32,
    pub mem_region: i32,
    pub ambient: *const c_char,
    pub calr: *const c_char,
    pub status: *const c_char,
    pub checksum: *const c_char,
}

unsafe extern "C" {
    pub fn cs_amp_write_cal_coeffs(
        dsp: *mut cs_dsp,
        controls: *const cirrus_amp_cal_controls,
        data: *const cirrus_amp_cal_data,
    ) -> i32;
    pub fn cs_amp_read_cal_coeffs(
        dsp: *mut cs_dsp,
        controls: *const cirrus_amp_cal_controls,
        data: *mut cirrus_amp_cal_data,
    ) -> i32;
    pub fn cs_amp_write_ambient_temp(
        dsp: *mut cs_dsp,
        controls: *const cirrus_amp_cal_controls,
        temp: u32,
    ) -> i32;
    pub fn cs_amp_get_efi_calibration_data(
        dev: *mut device,
        target_uid: u64,
        amp_index: i32,
        out_data: *mut cirrus_amp_cal_data,
    ) -> i32;
    pub fn cs_amp_set_efi_calibration_data(
        dev: *mut device,
        amp_index: i32,
        num_amps: i32,
        in_data: *const cirrus_amp_cal_data,
    ) -> i32;
    pub fn cs_amp_get_vendor_spkid(dev: *mut device) -> i32;
    pub fn cs_amp_devm_get_vendor_specific_variant_id(
        dev: *mut device,
        ssid_vendor: i32,
        ssid_device: i32,
    ) -> *const c_char;
    pub fn cs_amp_create_debugfs(dev: *mut device) -> *mut dentry;
}

pub unsafe fn cs_amp_cal_target_u64(data: *const cirrus_amp_cal_data) -> u64 {
    ((*data).calTarget[1] as u64).wrapping_shl(32) | (*data).calTarget[0] as u64
}

#[repr(C)]
pub struct cs_amp_test_hooks {
    pub get_efi_variable: Option<unsafe extern "C" fn(
        name: *mut efi_char16_t,
        guid: *mut efi_guid_t,
        returned_attr: *mut u32,
        size: *mut usize,
        buf: *mut core::ffi::c_void,
    ) -> efi_status_t>,
    pub set_efi_variable: Option<unsafe extern "C" fn(
        name: *mut efi_char16_t,
        guid: *mut efi_guid_t,
        attr: u32,
        size: usize,
        buf: *mut core::ffi::c_void,
    ) -> efi_status_t>,
    pub write_cal_coeff: Option<unsafe extern "C" fn(
        dsp: *mut cs_dsp,
        controls: *const cirrus_amp_cal_controls,
        ctl_name: *const c_char,
        val: u32,
    ) -> i32>,
    pub read_cal_coeff: Option<unsafe extern "C" fn(
        dsp: *mut cs_dsp,
        controls: *const cirrus_amp_cal_controls,
        ctl_name: *const c_char,
        val: *mut u32,
    ) -> i32>,
}

pub static mut cs_amp_test_hooks: *const cs_amp_test_hooks;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
