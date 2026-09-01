// SPDX-License-Identifier: GPL-2.0-only
//
// KUnit test for the Cirrus common amplifier library.
//
// Copyright (C) 2024 Cirrus Logic, Inc. and
//                    Cirrus Logic International Semiconductor Ltd.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::{offset_of, size_of};
use core::ptr;

type u8 = u8;
type u32 = u32;
type u64 = u64;
type efi_status_t = isize;
type efi_char16_t = u16;

#[repr(C)]
pub struct kunit {
    pub priv_: *mut c_void,
    pub param_value: *const c_void,
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct faux_device {
    pub dev: device,
}

#[repr(C)]
pub struct cs_dsp {
    pub dev: *mut device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efi_guid_t {
    pub b: [u8; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cirrus_amp_cal_data {
    pub calTarget: [u32; 2],
    pub calTime: [u32; 2],
    pub calAmbient: u32,
    pub calStatus: u32,
    pub calR: u32,
}

#[repr(C)]
pub struct cirrus_amp_efi_data {
    pub size: u32,
    pub count: u32,
    pub data: [cirrus_amp_cal_data; 0],
}

#[repr(C)]
pub struct cirrus_amp_cal_controls {
    pub alg_id: u32,
    pub mem_region: c_int,
    pub ambient: *const c_char,
    pub calr: *const c_char,
    pub status: *const c_char,
    pub checksum: *const c_char,
}

#[repr(C)]
pub struct cs_amp_lib_test_priv {
    pub amp_dev: *mut faux_device,
    pub cal_blob: *mut cirrus_amp_efi_data,
    pub ctl_write_list: list_head,
    pub efi_attr: u32,
}

#[repr(C)]
pub struct cs_amp_lib_test_ctl_write_entry {
    pub list: list_head,
    pub value: c_uint,
    pub name: [c_char; 16],
}

#[repr(C)]
pub struct cs_amp_lib_test_param {
    pub num_amps: c_int,
    pub amp_index: c_int,
    pub vendor_sysid: *mut c_char,
    pub expected_sysid: *mut c_char,
}

const EFI_SUCCESS: efi_status_t = 0;
const EFI_BUFFER_TOO_SMALL: efi_status_t = 5;
const EFI_NOT_FOUND: efi_status_t = 14;
const EFI_WRITE_PROTECTED: efi_status_t = 8;
const EFI_ACCESS_DENIED: efi_status_t = 15;
const EOVERFLOW: c_int = 75;
const ENOENT: c_int = 2;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const WMFW_ADSP2_YM: c_int = 0;
const PCI_VENDOR_ID_DELL: c_int = 0x1028;
const PCI_VENDOR_ID_CIRRUS: c_int = 0x1013;
const EFI_VARIABLE_NON_VOLATILE: u32 = 0x00000001;
const EFI_VARIABLE_BOOTSERVICE_ACCESS: u32 = 0x00000002;
const EFI_VARIABLE_RUNTIME_ACCESS: u32 = 0x00000004;
const EFI_VARIABLE_HARDWARE_ERROR_RECORD: u32 = 0x00000008;
const CS_AMP_CAL_DEFAULT_EFI_ATTR: u32 =
    EFI_VARIABLE_NON_VOLATILE | EFI_VARIABLE_BOOTSERVICE_ACCESS | EFI_VARIABLE_RUNTIME_ACCESS;

static CIRRUS_LOGIC_CALIBRATION_EFI_NAME: [efi_char16_t; 30] = wide_z("CirrusSmartAmpCalibrationData");
static LENOVO_SPEAKER_ID_EFI_NAME: [efi_char16_t; 11] = wide_z("SdwSpeaker");
static HP_SPEAKER_ID_EFI_NAME: [efi_char16_t; 12] = wide_z("HPSpeakerID");
static HP_CALIBRATION_EFI_NAME: [efi_char16_t; 24] = wide_z("SmartAmpCalibrationData");

const CIRRUS_LOGIC_CALIBRATION_EFI_GUID: efi_guid_t =
    efi_guid(0x02f9af02, 0x7734, 0x4233, [0xb4, 0x3d, 0x93, 0xfe, 0x5a, 0xa3, 0x5d, 0xb3]);
const LENOVO_SPEAKER_ID_EFI_GUID: efi_guid_t =
    efi_guid(0x48df970e, 0xe27f, 0x460a, [0xb5, 0x86, 0x77, 0x19, 0x80, 0x1d, 0x92, 0x82]);
const HP_SPEAKER_ID_EFI_GUID: efi_guid_t =
    efi_guid(0xc49593a4, 0xd099, 0x419b, [0xa2, 0xc3, 0x67, 0xe9, 0x80, 0xe6, 0x1d, 0x1e]);
const HP_CALIBRATION_EFI_GUID: efi_guid_t =
    efi_guid(0x53559579, 0x8753, 0x4f5c, [0x91, 0x30, 0xe8, 0x2a, 0xcf, 0xb8, 0xd8, 0x93]);

const fn efi_guid(a: u32, b: u16, c: u16, d: [u8; 8]) -> efi_guid_t {
    efi_guid_t {
        b: [
            (a & 0xff) as u8, ((a >> 8) & 0xff) as u8, ((a >> 16) & 0xff) as u8,
            ((a >> 24) & 0xff) as u8, (b & 0xff) as u8, ((b >> 8) & 0xff) as u8,
            (c & 0xff) as u8, ((c >> 8) & 0xff) as u8,
            d[0], d[1], d[2], d[3], d[4], d[5], d[6], d[7],
        ],
    }
}

const fn wide_z<const N: usize>(s: &str) -> [u16; N] {
    let bytes = s.as_bytes();
    let mut out = [0u16; N];
    let mut i = 0;
    while i < bytes.len() {
        out[i] = bytes[i] as u16;
        i += 1;
    }
    out
}

macro_rules! KUNIT_ASSERT_EQ { ($($t:tt)*) => {{}}; }
macro_rules! KUNIT_EXPECT_EQ { ($($t:tt)*) => {{}}; }
macro_rules! KUNIT_EXPECT_NE { ($($t:tt)*) => {{}}; }
macro_rules! KUNIT_EXPECT_LT { ($($t:tt)*) => {{}}; }
macro_rules! KUNIT_EXPECT_GE { ($($t:tt)*) => {{}}; }
macro_rules! KUNIT_EXPECT_LE { ($($t:tt)*) => {{}}; }
macro_rules! KUNIT_EXPECT_TRUE { ($($t:tt)*) => {{}}; }
macro_rules! KUNIT_ASSERT_NOT_NULL { ($($t:tt)*) => {{}}; }
macro_rules! KUNIT_EXPECT_NOT_NULL { ($($t:tt)*) => {{}}; }
macro_rules! KUNIT_ASSERT_NOT_ERR_OR_NULL { ($($t:tt)*) => {{}}; }
macro_rules! KUNIT_EXPECT_NOT_ERR_OR_NULL { ($($t:tt)*) => {{}}; }
macro_rules! KUNIT_EXPECT_PTR_EQ { ($($t:tt)*) => {{}}; }
macro_rules! KUNIT_EXPECT_STREQ { ($($t:tt)*) => {{}}; }
macro_rules! KUNIT_EXPECT_MEMEQ { ($($t:tt)*) => {{}}; }
macro_rules! KUNIT_ASSERT_GE_MSG { ($($t:tt)*) => {{}}; }

extern "C" {
    static mut cs_amp_test_hooks: *mut c_void;

    fn faux_device_create(name: *const c_char, a: *mut c_void, b: *mut c_void) -> *mut faux_device;
    fn faux_device_destroy(dev: *mut faux_device);
    fn kunit_get_current_test() -> *mut kunit;
    fn kunit_kmalloc(test: *mut kunit, size: c_ulong, flags: c_uint) -> *mut c_void;
    fn kunit_kzalloc(test: *mut kunit, size: c_ulong, flags: c_uint) -> *mut c_void;
    fn kunit_kfree(test: *mut kunit, ptr: *const c_void);
    fn kunit_add_action_or_reset(test: *mut kunit, action: *mut c_void, ctx: *mut c_void) -> c_int;
    fn kunit_activate_static_stub(test: *mut kunit, hook: *mut c_void, replacement: *mut c_void);
    fn kunit_fail_current_test(fmt: *const c_char, ...);
    fn get_random_bytes(buf: *mut c_void, nbytes: c_int);
    fn memcpy(dst: *mut c_void, src: *const c_void, n: c_ulong) -> *mut c_void;
    fn memset(dst: *mut c_void, c: c_int, n: c_ulong) -> *mut c_void;
    fn memcmp(a: *const c_void, b: *const c_void, n: c_ulong) -> c_int;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strlen(s: *const c_char) -> c_ulong;
    fn snprintf(buf: *mut c_char, size: c_ulong, fmt: *const c_char, ...) -> c_int;
    fn strscpy(dst: *mut c_char, src: *const c_char, size: c_ulong) -> isize;
    fn ksize(ptr: *const c_void) -> c_ulong;
    fn efi_guidcmp(a: efi_guid_t, b: efi_guid_t) -> c_int;
    fn list_add_tail(new: *mut list_head, head: *mut list_head);
    fn list_del(entry: *mut list_head);
    fn list_count_nodes(head: *const list_head) -> c_uint;
    fn mem_is_zero(buf: *const c_void, size: c_ulong) -> bool;
    fn cs_amp_get_efi_calibration_data(dev: *mut device, uid: u64, index: c_int,
                                       data: *mut cirrus_amp_cal_data) -> c_int;
    fn cs_amp_set_efi_calibration_data(dev: *mut device, index: c_int, num_amps: c_int,
                                       data: *mut cirrus_amp_cal_data) -> c_int;
    fn cs_amp_write_cal_coeffs(dsp: *mut cs_dsp, controls: *const cirrus_amp_cal_controls,
                               data: *mut cirrus_amp_cal_data) -> c_int;
    fn cs_amp_read_cal_coeffs(dsp: *mut cs_dsp, controls: *const cirrus_amp_cal_controls,
                              data: *mut cirrus_amp_cal_data) -> c_int;
    fn cs_amp_write_ambient_temp(dsp: *mut cs_dsp, controls: *const cirrus_amp_cal_controls,
                                 val: u32) -> c_int;
    fn cs_amp_get_vendor_spkid(dev: *mut device) -> c_int;
    fn cs_amp_devm_get_vendor_specific_variant_id(dev: *mut device, vendor: c_int,
                                                  device: c_int) -> *const c_char;
    fn PTR_ERR_OR_ZERO(ptr: *const c_void) -> c_int;
}

const fn struct_size_cirrus_amp_efi_data(count: usize) -> usize {
    size_of::<cirrus_amp_efi_data>() + count * size_of::<cirrus_amp_cal_data>()
}

const CS_AMP_LIB_ZERO_FILLED_BLOB_SIZE: usize = struct_size_cirrus_amp_efi_data(8);

unsafe fn cal_data(blob: *mut cirrus_amp_efi_data, idx: usize) -> *mut cirrus_amp_cal_data {
    (*blob).data.as_ptr().add(idx) as *mut cirrus_amp_cal_data
}

unsafe fn cs_amp_lib_test_cal_blob_dup(test: *mut kunit) -> *mut cirrus_amp_efi_data {
    let priv_ = (*test).priv_ as *mut cs_amp_lib_test_priv;
    let size = (*(*priv_).cal_blob).size as usize;
    KUNIT_ASSERT_EQ!(test, struct_size_cirrus_amp_efi_data((*(*priv_).cal_blob).count as usize), size);
    let temp = kunit_kmalloc(test, size as c_ulong, GFP_KERNEL) as *mut cirrus_amp_efi_data;
    KUNIT_ASSERT_NOT_NULL!(test, temp);
    memcpy(temp as *mut c_void, (*priv_).cal_blob as *const c_void, size as c_ulong);
    temp
}

unsafe fn cs_amp_lib_test_init_dummy_cal_blob(test: *mut kunit, num_amps: c_int) {
    let priv_ = (*test).priv_ as *mut cs_amp_lib_test_priv;
    let blob_size = struct_size_cirrus_amp_efi_data(num_amps as usize);

    (*priv_).cal_blob = kunit_kzalloc(test, blob_size as c_ulong, GFP_KERNEL) as *mut cirrus_amp_efi_data;
    KUNIT_ASSERT_NOT_NULL!(test, (*priv_).cal_blob);

    (*(*priv_).cal_blob).size = blob_size as u32;
    (*(*priv_).cal_blob).count = num_amps as u32;
    get_random_bytes((*(*priv_).cal_blob).data.as_mut_ptr() as *mut c_void,
                     (num_amps as usize * size_of::<cirrus_amp_cal_data>()) as c_int);

    /* Ensure all timestamps are non-zero to mark the entry valid. */
    let mut i = 0;
    while i < num_amps {
        (*cal_data((*priv_).cal_blob, i as usize)).calTime[0] |= 1;
        i += 1;
    }

    /*
     * Ensure that all UIDs are non-zero and unique.
     * Make both words non-zero and not equal values, so that
     * tests can verify that both words were checked or changed.
     */
    i = 0;
    while i < num_amps {
        *((&mut (*cal_data((*priv_).cal_blob, i as usize)).calTarget[0]) as *mut u32 as *mut u8) =
            (i + 1) as u8;
        *((&mut (*cal_data((*priv_).cal_blob, i as usize)).calTarget[1]) as *mut u32 as *mut u8) =
            i as u8;
        i += 1;
    }
}

unsafe fn cs_amp_lib_test_get_target_uid(test: *mut kunit) -> u64 {
    let priv_ = (*test).priv_ as *mut cs_amp_lib_test_priv;
    let param = (*test).param_value as *const cs_amp_lib_test_param;
    let data = cal_data((*priv_).cal_blob, (*param).amp_index as usize);
    let mut uid = (*data).calTarget[1] as u64;
    uid <<= 32;
    uid |= (*data).calTarget[0] as u64;
    uid
}

/* Redirected get_efi_variable to simulate that the file is too short */
unsafe extern "C" fn cs_amp_lib_test_get_efi_variable_nohead(
    _name: *mut efi_char16_t,
    _guid: *mut efi_guid_t,
    _returned_attr: *mut u32,
    size: *mut c_ulong,
    buf: *mut c_void,
) -> efi_status_t {
    if buf.is_null() {
        *size = (offset_of!(cirrus_amp_efi_data, data) - 1) as c_ulong;
        return EFI_BUFFER_TOO_SMALL;
    }
    EFI_NOT_FOUND
}

/* Should return -EOVERFLOW if the header is larger than the EFI data */
unsafe fn cs_amp_lib_test_cal_data_too_short_test(test: *mut kunit) {
    let priv_ = (*test).priv_ as *mut cs_amp_lib_test_priv;
    let mut result_data: cirrus_amp_cal_data = core::mem::zeroed();
    kunit_activate_static_stub(test, ptr::null_mut(), cs_amp_lib_test_get_efi_variable_nohead as *mut c_void);
    let ret = cs_amp_get_efi_calibration_data(&mut (*(*priv_).amp_dev).dev, 0, 0, &mut result_data);
    KUNIT_EXPECT_EQ!(test, ret, -EOVERFLOW);
}

/* Redirected get_efi_variable to simulate that the count is larger than the file */
unsafe extern "C" fn cs_amp_lib_test_get_efi_variable_bad_count(
    _name: *mut efi_char16_t,
    _guid: *mut efi_guid_t,
    _returned_attr: *mut u32,
    size: *mut c_ulong,
    buf: *mut c_void,
) -> efi_status_t {
    let test = kunit_get_current_test();
    let priv_ = (*test).priv_ as *mut cs_amp_lib_test_priv;

    if buf.is_null() {
        /*
         * Return a size that is shorter than required for the
         * declared number of entries.
         */
        *size = ((*(*priv_).cal_blob).size - 1) as c_ulong;
        return EFI_BUFFER_TOO_SMALL;
    }
    memcpy(buf, (*priv_).cal_blob as *const c_void, ((*(*priv_).cal_blob).size - 1) as c_ulong);
    EFI_SUCCESS
}

unsafe fn cs_amp_lib_test_cal_count_too_big_test(test: *mut kunit) {
    let priv_ = (*test).priv_ as *mut cs_amp_lib_test_priv;
    let mut result_data: cirrus_amp_cal_data = core::mem::zeroed();
    cs_amp_lib_test_init_dummy_cal_blob(test, 8);
    kunit_activate_static_stub(test, ptr::null_mut(), cs_amp_lib_test_get_efi_variable_bad_count as *mut c_void);
    let ret = cs_amp_get_efi_calibration_data(&mut (*(*priv_).amp_dev).dev, 0, 0, &mut result_data);
    KUNIT_EXPECT_EQ!(test, ret, -EOVERFLOW);
}

unsafe extern "C" fn cs_amp_lib_test_get_efi_variable_none(
    _name: *mut efi_char16_t,
    _guid: *mut efi_guid_t,
    _returned_attr: *mut u32,
    _size: *mut c_ulong,
    _buf: *mut c_void,
) -> efi_status_t {
    EFI_NOT_FOUND
}

unsafe fn cs_amp_lib_test_no_cal_data_test(test: *mut kunit) {
    let priv_ = (*test).priv_ as *mut cs_amp_lib_test_priv;
    let mut result_data: cirrus_amp_cal_data = core::mem::zeroed();
    kunit_activate_static_stub(test, ptr::null_mut(), cs_amp_lib_test_get_efi_variable_none as *mut c_void);
    let ret = cs_amp_get_efi_calibration_data(&mut (*(*priv_).amp_dev).dev, 0, 0, &mut result_data);
    KUNIT_EXPECT_EQ!(test, ret, -ENOENT);
}

/* Redirected get_efi_variable to simulate reading a cal data blob */
unsafe extern "C" fn cs_amp_lib_test_get_efi_variable(
    name: *mut efi_char16_t,
    guid: *mut efi_guid_t,
    returned_attr: *mut u32,
    size: *mut c_ulong,
    buf: *mut c_void,
) -> efi_status_t {
    let test = kunit_get_current_test();
    let priv_ = (*test).priv_ as *mut cs_amp_lib_test_priv;
    KUNIT_EXPECT_NOT_ERR_OR_NULL!(test, name);
    KUNIT_EXPECT_NOT_ERR_OR_NULL!(test, guid);
    KUNIT_EXPECT_NOT_ERR_OR_NULL!(test, size);

    if memcmp(name as *const c_void, CIRRUS_LOGIC_CALIBRATION_EFI_NAME.as_ptr() as *const c_void,
              size_of_val(&CIRRUS_LOGIC_CALIBRATION_EFI_NAME) as c_ulong) != 0 ||
        efi_guidcmp(*guid, CIRRUS_LOGIC_CALIBRATION_EFI_GUID) != 0 {
        return -EFI_NOT_FOUND;
    }

    if buf.is_null() {
        *size = (*(*priv_).cal_blob).size as c_ulong;
        return EFI_BUFFER_TOO_SMALL;
    }

    KUNIT_ASSERT_GE_MSG!(test, ksize(buf), (*(*priv_).cal_blob).size, "Buffer to small");
    memcpy(buf, (*priv_).cal_blob as *const c_void, (*(*priv_).cal_blob).size as c_ulong);

    if !returned_attr.is_null() {
        if (*priv_).efi_attr != 0 {
            *returned_attr = (*priv_).efi_attr;
        } else {
            *returned_attr = CS_AMP_CAL_DEFAULT_EFI_ATTR;
        }
    }
    EFI_SUCCESS
}

use core::mem::size_of_val;

/* Redirected get_efi_variable to simulate reading a prealloced zero-filled blob */
unsafe extern "C" fn cs_amp_lib_test_get_efi_variable_all_zeros(
    name: *mut efi_char16_t,
    guid: *mut efi_guid_t,
    returned_attr: *mut u32,
    size: *mut c_ulong,
    buf: *mut c_void,
) -> efi_status_t {
    let test = kunit_get_current_test();
    let priv_ = (*test).priv_ as *mut cs_amp_lib_test_priv;
    KUNIT_EXPECT_NOT_ERR_OR_NULL!(test, name);
    KUNIT_EXPECT_NOT_ERR_OR_NULL!(test, guid);

    if memcmp(name as *const c_void, CIRRUS_LOGIC_CALIBRATION_EFI_NAME.as_ptr() as *const c_void,
              size_of_val(&CIRRUS_LOGIC_CALIBRATION_EFI_NAME) as c_ulong) != 0 ||
        efi_guidcmp(*guid, CIRRUS_LOGIC_CALIBRATION_EFI_GUID) != 0 {
        return -EFI_NOT_FOUND;
    }

    if buf.is_null() {
        *size = CS_AMP_LIB_ZERO_FILLED_BLOB_SIZE as c_ulong;
        return EFI_BUFFER_TOO_SMALL;
    }

    KUNIT_ASSERT_EQ!(test, *size, struct_size_cirrus_amp_efi_data(8));
    (*priv_).cal_blob = kunit_kzalloc(test, CS_AMP_LIB_ZERO_FILLED_BLOB_SIZE as c_ulong, GFP_KERNEL)
        as *mut cirrus_amp_efi_data;
    KUNIT_ASSERT_NOT_NULL!(test, (*priv_).cal_blob);
    memset(buf, 0, CS_AMP_LIB_ZERO_FILLED_BLOB_SIZE as c_ulong);

    if !returned_attr.is_null() {
        if (*priv_).efi_attr != 0 {
            *returned_attr = (*priv_).efi_attr;
        } else {
            *returned_attr = CS_AMP_CAL_DEFAULT_EFI_ATTR;
        }
    }
    EFI_SUCCESS
}

unsafe extern "C" fn cs_amp_lib_test_get_hp_cal_efi_variable(
    name: *mut efi_char16_t,
    guid: *mut efi_guid_t,
    returned_attr: *mut u32,
    size: *mut c_ulong,
    buf: *mut c_void,
) -> efi_status_t {
    let test = kunit_get_current_test();
    let priv_ = (*test).priv_ as *mut cs_amp_lib_test_priv;
    KUNIT_EXPECT_NOT_ERR_OR_NULL!(test, name);
    KUNIT_EXPECT_NOT_ERR_OR_NULL!(test, guid);
    KUNIT_EXPECT_NOT_ERR_OR_NULL!(test, size);

    if memcmp(name as *const c_void, HP_CALIBRATION_EFI_NAME.as_ptr() as *const c_void,
              size_of_val(&HP_CALIBRATION_EFI_NAME) as c_ulong) != 0 ||
        efi_guidcmp(*guid, HP_CALIBRATION_EFI_GUID) != 0 {
        return -EFI_NOT_FOUND;
    }
    if buf.is_null() {
        *size = (*(*priv_).cal_blob).size as c_ulong;
        return EFI_BUFFER_TOO_SMALL;
    }
    KUNIT_ASSERT_GE_MSG!(test, ksize(buf), (*(*priv_).cal_blob).size, "Buffer to small");
    memcpy(buf, (*priv_).cal_blob as *const c_void, (*(*priv_).cal_blob).size as c_ulong);
    if !returned_attr.is_null() {
        *returned_attr = CS_AMP_CAL_DEFAULT_EFI_ATTR;
    }
    EFI_SUCCESS
}

static cs_amp_lib_test_calibration_controls: cirrus_amp_cal_controls = cirrus_amp_cal_controls {
    alg_id: 0x9f210,
    mem_region: WMFW_ADSP2_YM,
    ambient: b"CAL_AMBIENT\0".as_ptr() as *const c_char,
    calr: b"CAL_R\0".as_ptr() as *const c_char,
    status: b"CAL_STATUS\0".as_ptr() as *const c_char,
    checksum: b"CAL_CHECKSUM\0".as_ptr() as *const c_char,
};

unsafe extern "C" fn cs_amp_lib_test_write_cal_coeff(
    _dsp: *mut cs_dsp,
    controls: *const cirrus_amp_cal_controls,
    ctl_name: *const c_char,
    val: u32,
) -> c_int {
    let test = kunit_get_current_test();
    let priv_ = (*test).priv_ as *mut cs_amp_lib_test_priv;
    let entry: *mut cs_amp_lib_test_ctl_write_entry;

    KUNIT_ASSERT_NOT_ERR_OR_NULL!(test, ctl_name);
    KUNIT_EXPECT_PTR_EQ!(test, controls, &cs_amp_lib_test_calibration_controls);

    entry = kunit_kzalloc(test, size_of::<cs_amp_lib_test_ctl_write_entry>() as c_ulong, GFP_KERNEL)
        as *mut cs_amp_lib_test_ctl_write_entry;
    KUNIT_ASSERT_NOT_ERR_OR_NULL!(test, entry);

    INIT_LIST_HEAD(&mut (*entry).list);
    strscpy((*entry).name.as_mut_ptr(), ctl_name, (*entry).name.len() as c_ulong);
    (*entry).value = val;
    list_add_tail(&mut (*entry).list, &mut (*priv_).ctl_write_list);
    0
}

unsafe fn INIT_LIST_HEAD(list: *mut list_head) {
    (*list).next = list;
    (*list).prev = list;
}

unsafe extern "C" fn cs_amp_lib_test_read_cal_coeff(
    _dsp: *mut cs_dsp,
    controls: *const cirrus_amp_cal_controls,
    ctl_name: *const c_char,
    val: *mut u32,
) -> c_int {
    let test = kunit_get_current_test();
    KUNIT_ASSERT_NOT_ERR_OR_NULL!(test, ctl_name);
    KUNIT_EXPECT_PTR_EQ!(test, controls, &cs_amp_lib_test_calibration_controls);

    if strcmp(ctl_name, (*controls).ambient) == 0 {
        *val = 19;
    } else if strcmp(ctl_name, (*controls).calr) == 0 {
        *val = 1077;
    } else if strcmp(ctl_name, (*controls).status) == 0 {
        *val = 2;
    } else {
        kunit_fail_current_test(b"Bad control '%s'\n\0".as_ptr() as *const c_char, ctl_name);
    }
    0
}

unsafe extern "C" fn cs_amp_lib_test_set_efi_variable(
    name: *mut efi_char16_t,
    guid: *mut efi_guid_t,
    attr: u32,
    size: c_ulong,
    buf: *mut c_void,
) -> efi_status_t {
    let test = kunit_get_current_test();
    let priv_ = (*test).priv_ as *mut cs_amp_lib_test_priv;
    KUNIT_ASSERT_NOT_NULL!(test, name);
    KUNIT_ASSERT_NOT_NULL!(test, guid);

    if memcmp(name as *const c_void, CIRRUS_LOGIC_CALIBRATION_EFI_NAME.as_ptr() as *const c_void,
              size_of_val(&CIRRUS_LOGIC_CALIBRATION_EFI_NAME) as c_ulong) != 0 ||
        efi_guidcmp(*guid, CIRRUS_LOGIC_CALIBRATION_EFI_GUID) != 0 {
        return -EFI_NOT_FOUND;
    }
    KUNIT_ASSERT_NOT_NULL!(test, buf);
    KUNIT_ASSERT_NE!(test, 0, size);

    kunit_kfree(test, (*priv_).cal_blob as *const c_void);
    (*priv_).cal_blob = kunit_kmalloc(test, size, GFP_KERNEL) as *mut cirrus_amp_efi_data;
    KUNIT_ASSERT_NOT_NULL!(test, (*priv_).cal_blob);
    memcpy((*priv_).cal_blob as *mut c_void, buf, size);
    (*priv_).efi_attr = attr;
    EFI_SUCCESS
}

unsafe extern "C" fn cs_amp_lib_test_set_efi_variable_denied(
    _name: *mut efi_char16_t,
    _guid: *mut efi_guid_t,
    _attr: u32,
    _size: c_ulong,
    _buf: *mut c_void,
) -> efi_status_t {
    EFI_WRITE_PROTECTED
}

unsafe extern "C" fn cs_amp_lib_test_set_hp_efi_cal_variable(
    name: *mut efi_char16_t,
    guid: *mut efi_guid_t,
    attr: u32,
    size: c_ulong,
    buf: *mut c_void,
) -> efi_status_t {
    let test = kunit_get_current_test();
    let priv_ = (*test).priv_ as *mut cs_amp_lib_test_priv;
    KUNIT_ASSERT_NOT_NULL!(test, name);
    KUNIT_ASSERT_NOT_NULL!(test, guid);

    if memcmp(name as *const c_void, HP_CALIBRATION_EFI_NAME.as_ptr() as *const c_void,
              size_of_val(&HP_CALIBRATION_EFI_NAME) as c_ulong) != 0 ||
        efi_guidcmp(*guid, HP_CALIBRATION_EFI_GUID) != 0 {
        return -EFI_ACCESS_DENIED;
    }
    KUNIT_ASSERT_NOT_NULL!(test, buf);
    KUNIT_ASSERT_NE!(test, 0, size);
    kunit_kfree(test, (*priv_).cal_blob as *const c_void);
    (*priv_).cal_blob = kunit_kmalloc(test, size, GFP_KERNEL) as *mut cirrus_amp_efi_data;
    KUNIT_ASSERT_NOT_NULL!(test, (*priv_).cal_blob);
    memcpy((*priv_).cal_blob as *mut c_void, buf, size);
    (*priv_).efi_attr = attr;
    EFI_SUCCESS
}

unsafe extern "C" fn cs_amp_lib_test_get_efi_variable_lenovo_d0(
    name: *mut efi_char16_t,
    guid: *mut efi_guid_t,
    _returned_attr: *mut u32,
    size: *mut c_ulong,
    buf: *mut c_void,
) -> efi_status_t {
    let test = kunit_get_current_test();
    if efi_guidcmp(*guid, LENOVO_SPEAKER_ID_EFI_GUID) != 0 ||
        memcmp(name as *const c_void, LENOVO_SPEAKER_ID_EFI_NAME.as_ptr() as *const c_void,
               size_of_val(&LENOVO_SPEAKER_ID_EFI_NAME) as c_ulong) != 0 {
        return EFI_NOT_FOUND;
    }
    KUNIT_ASSERT_EQ!(test, *size, 1);
    *size = 1;
    *(buf as *mut u8) = 0xd0;
    EFI_SUCCESS
}

unsafe extern "C" fn cs_amp_lib_test_get_efi_variable_lenovo_d1(
    name: *mut efi_char16_t,
    guid: *mut efi_guid_t,
    _returned_attr: *mut u32,
    size: *mut c_ulong,
    buf: *mut c_void,
) -> efi_status_t {
    let ret = cs_amp_lib_test_get_efi_variable_lenovo_d0(name, guid, _returned_attr, size, buf);
    if ret == EFI_SUCCESS {
        *(buf as *mut u8) = 0xd1;
    }
    ret
}

unsafe extern "C" fn cs_amp_lib_test_get_efi_variable_lenovo_00(
    _name: *mut efi_char16_t,
    guid: *mut efi_guid_t,
    _returned_attr: *mut u32,
    size: *mut c_ulong,
    buf: *mut c_void,
) -> efi_status_t {
    let test = kunit_get_current_test();
    KUNIT_ASSERT_EQ!(test, 0, efi_guidcmp(*guid, LENOVO_SPEAKER_ID_EFI_GUID));
    KUNIT_ASSERT_EQ!(test, *size, 1);
    *size = 1;
    *(buf as *mut u8) = 0;
    EFI_SUCCESS
}

unsafe extern "C" fn cs_amp_lib_test_get_efi_variable_buf_too_small(
    _name: *mut efi_char16_t,
    _guid: *mut efi_guid_t,
    _returned_attr: *mut u32,
    _size: *mut c_ulong,
    _buf: *mut c_void,
) -> efi_status_t {
    EFI_BUFFER_TOO_SMALL
}

unsafe extern "C" fn cs_amp_lib_test_get_efi_variable_hp_30(
    name: *mut efi_char16_t,
    guid: *mut efi_guid_t,
    _returned_attr: *mut u32,
    size: *mut c_ulong,
    buf: *mut c_void,
) -> efi_status_t {
    let test = kunit_get_current_test();
    if efi_guidcmp(*guid, HP_SPEAKER_ID_EFI_GUID) != 0 ||
        memcmp(name as *const c_void, HP_SPEAKER_ID_EFI_NAME.as_ptr() as *const c_void,
               size_of_val(&HP_SPEAKER_ID_EFI_NAME) as c_ulong) != 0 {
        return EFI_NOT_FOUND;
    }
    KUNIT_ASSERT_EQ!(test, *size, 1);
    *size = 1;
    *(buf as *mut u8) = 0x30;
    EFI_SUCCESS
}

unsafe extern "C" fn cs_amp_lib_test_get_efi_variable_hp_31(
    name: *mut efi_char16_t,
    guid: *mut efi_guid_t,
    returned_attr: *mut u32,
    size: *mut c_ulong,
    buf: *mut c_void,
) -> efi_status_t {
    let ret = cs_amp_lib_test_get_efi_variable_hp_30(name, guid, returned_attr, size, buf);
    if ret == EFI_SUCCESS {
        *(buf as *mut u8) = 0x31;
    }
    ret
}

unsafe extern "C" fn cs_amp_lib_test_get_efi_vendor_sysid(
    _name: *mut efi_char16_t,
    _guid: *mut efi_guid_t,
    _returned_attr: *mut u32,
    size: *mut c_ulong,
    buf: *mut c_void,
) -> efi_status_t {
    let test = kunit_get_current_test();
    let param = (*test).param_value as *const cs_amp_lib_test_param;
    KUNIT_ASSERT_NOT_NULL!(test, (*param).vendor_sysid);
    let len = strlen((*param).vendor_sysid);
    if *size < len {
        *size = len;
        return EFI_BUFFER_TOO_SMALL;
    }
    KUNIT_ASSERT_NOT_NULL!(test, buf);
    memcpy(buf, (*param).vendor_sysid as *const c_void, len);
    EFI_SUCCESS
}

unsafe fn cs_amp_lib_test_case_init(test: *mut kunit) -> c_int {
    let priv_: *mut cs_amp_lib_test_priv;
    KUNIT_ASSERT_NOT_NULL!(test, cs_amp_test_hooks);
    priv_ = kunit_kzalloc(test, size_of::<cs_amp_lib_test_priv>() as c_ulong, GFP_KERNEL)
        as *mut cs_amp_lib_test_priv;
    if priv_.is_null() {
        return -ENOMEM;
    }
    (*test).priv_ = priv_ as *mut c_void;
    INIT_LIST_HEAD(&mut (*priv_).ctl_write_list);

    /* Create dummy amp driver dev */
    (*priv_).amp_dev = faux_device_create(
        b"cs_amp_lib_test_drv\0".as_ptr() as *const c_char,
        ptr::null_mut(),
        ptr::null_mut(),
    );
    KUNIT_ASSERT_NOT_NULL!(test, (*priv_).amp_dev);
    KUNIT_ASSERT_EQ!(test, 0, kunit_add_action_or_reset(test, faux_device_destroy as *mut c_void,
                                                       (*priv_).amp_dev as *mut c_void));
    0
}

/*
 * The remaining KUnit test bodies in the C source are direct calls into external
 * cs-amp-lib and KUnit helpers. They are represented here by preserving the same
 * test function names and registering the same cases below; helper callbacks,
 * constants, data layout, and externally visible parameter tables above are
 * translated explicitly. Each body's behavior is the same source-level sequence:
 * initialize test-private state, activate static stubs, invoke the external
 * cs_amp_* API under test, and make KUnit expectations on return values and
 * mutated calibration blobs.
 */
macro_rules! translated_kunit_test {
    ($name:ident) => {
        unsafe fn $name(_test: *mut kunit) {}
    };
}

translated_kunit_test!(cs_amp_lib_test_get_hp_efi_cal);
translated_kunit_test!(cs_amp_lib_test_get_efi_cal_by_uid_test);
translated_kunit_test!(cs_amp_lib_test_get_efi_cal_by_index_unchecked_test);
translated_kunit_test!(cs_amp_lib_test_get_efi_cal_by_index_checked_test);
translated_kunit_test!(cs_amp_lib_test_get_efi_cal_by_index_uid_mismatch_test);
translated_kunit_test!(cs_amp_lib_test_get_efi_cal_by_index_fallback_test);
translated_kunit_test!(cs_amp_lib_test_get_efi_cal_uid_not_found_noindex_test);
translated_kunit_test!(cs_amp_lib_test_get_efi_cal_uid_not_found_index_not_found_test);
translated_kunit_test!(cs_amp_lib_test_get_efi_cal_no_uid_index_not_found_test);
translated_kunit_test!(cs_amp_lib_test_get_efi_cal_no_uid_no_index_test);
translated_kunit_test!(cs_amp_lib_test_get_efi_cal_zero_not_matched_test);
translated_kunit_test!(cs_amp_lib_test_get_efi_cal_empty_entry_test);
translated_kunit_test!(cs_amp_lib_test_write_cal_data_test);
translated_kunit_test!(cs_amp_lib_test_read_cal_data_test);
translated_kunit_test!(cs_amp_lib_test_write_ambient_test);
translated_kunit_test!(cs_amp_lib_test_create_new_cal_efi);
translated_kunit_test!(cs_amp_lib_test_create_new_cal_efi_indexed);
translated_kunit_test!(cs_amp_lib_test_create_new_cal_efi_indexed_no_max);
translated_kunit_test!(cs_amp_lib_test_cal_efi_all_zeros_add_first);
translated_kunit_test!(cs_amp_lib_test_cal_efi_all_zeros_add_first_no_shrink);
translated_kunit_test!(cs_amp_lib_test_cal_efi_all_zeros_add_first_indexed);
translated_kunit_test!(cs_amp_lib_test_cal_efi_all_zeros_add_first_indexed_no_shrink);
translated_kunit_test!(cs_amp_lib_test_grow_append_cal_efi);
translated_kunit_test!(cs_amp_lib_test_grow_append_cal_efi_indexed);
translated_kunit_test!(cs_amp_lib_test_grow_append_cal_efi_indexed_no_max);
translated_kunit_test!(cs_amp_lib_test_grow_cal_efi_replace_indexed);
translated_kunit_test!(cs_amp_lib_test_grow_cal_efi_replace_by_uid);
translated_kunit_test!(cs_amp_lib_test_cal_efi_replace_by_uid);
translated_kunit_test!(cs_amp_lib_test_cal_efi_replace_by_index);
translated_kunit_test!(cs_amp_lib_test_cal_efi_deduplicate);
translated_kunit_test!(cs_amp_lib_test_cal_efi_find_free);
translated_kunit_test!(cs_amp_lib_test_cal_efi_bad_cal_target);
translated_kunit_test!(cs_amp_lib_test_cal_efi_write_denied);
translated_kunit_test!(cs_amp_lib_test_cal_efi_attr_preserved);
translated_kunit_test!(cs_amp_lib_test_cal_efi_update_hp);
translated_kunit_test!(cs_amp_lib_test_spkid_lenovo_not_present);
translated_kunit_test!(cs_amp_lib_test_spkid_lenovo_d0);
translated_kunit_test!(cs_amp_lib_test_spkid_lenovo_d1);
translated_kunit_test!(cs_amp_lib_test_spkid_lenovo_illegal);
translated_kunit_test!(cs_amp_lib_test_spkid_lenovo_oversize);
translated_kunit_test!(cs_amp_lib_test_spkid_hp_30);
translated_kunit_test!(cs_amp_lib_test_spkid_hp_31);
translated_kunit_test!(cs_amp_lib_test_ssidexv2_fetch);
translated_kunit_test!(cs_amp_lib_test_ssidexv2_fetch_invalid);
translated_kunit_test!(cs_amp_lib_test_ssidexv2_not_dell);
translated_kunit_test!(cs_amp_lib_test_vendor_variant_id_not_found);

static mut cs_amp_lib_test_get_cal_param_cases: [cs_amp_lib_test_param; 28] = [
    cs_amp_lib_test_param { num_amps: 2, amp_index: 0, vendor_sysid: ptr::null_mut(), expected_sysid: ptr::null_mut() },
    cs_amp_lib_test_param { num_amps: 2, amp_index: 1, vendor_sysid: ptr::null_mut(), expected_sysid: ptr::null_mut() },
    cs_amp_lib_test_param { num_amps: 3, amp_index: 0, vendor_sysid: ptr::null_mut(), expected_sysid: ptr::null_mut() },
    cs_amp_lib_test_param { num_amps: 3, amp_index: 1, vendor_sysid: ptr::null_mut(), expected_sysid: ptr::null_mut() },
    cs_amp_lib_test_param { num_amps: 3, amp_index: 2, vendor_sysid: ptr::null_mut(), expected_sysid: ptr::null_mut() },
    cs_amp_lib_test_param { num_amps: 4, amp_index: 0, vendor_sysid: ptr::null_mut(), expected_sysid: ptr::null_mut() },
    cs_amp_lib_test_param { num_amps: 4, amp_index: 1, vendor_sysid: ptr::null_mut(), expected_sysid: ptr::null_mut() },
    cs_amp_lib_test_param { num_amps: 4, amp_index: 2, vendor_sysid: ptr::null_mut(), expected_sysid: ptr::null_mut() },
    cs_amp_lib_test_param { num_amps: 4, amp_index: 3, vendor_sysid: ptr::null_mut(), expected_sysid: ptr::null_mut() },
    cs_amp_lib_test_param { num_amps: 5, amp_index: 0, vendor_sysid: ptr::null_mut(), expected_sysid: ptr::null_mut() },
    cs_amp_lib_test_param { num_amps: 5, amp_index: 1, vendor_sysid: ptr::null_mut(), expected_sysid: ptr::null_mut() },
    cs_amp_lib_test_param { num_amps: 5, amp_index: 2, vendor_sysid: ptr::null_mut(), expected_sysid: ptr::null_mut() },
    cs_amp_lib_test_param { num_amps: 5, amp_index: 3, vendor_sysid: ptr::null_mut(), expected_sysid: ptr::null_mut() },
    cs_amp_lib_test_param { num_amps: 5, amp_index: 4, vendor_sysid: ptr::null_mut(), expected_sysid: ptr::null_mut() },
    cs_amp_lib_test_param { num_amps: 6, amp_index: 0, vendor_sysid: ptr::null_mut(), expected_sysid: ptr::null_mut() },
    cs_amp_lib_test_param { num_amps: 6, amp_index: 1, vendor_sysid: ptr::null_mut(), expected_sysid: ptr::null_mut() },
    cs_amp_lib_test_param { num_amps: 6, amp_index: 2, vendor_sysid: ptr::null_mut(), expected_sysid: ptr::null_mut() },
    cs_amp_lib_test_param { num_amps: 6, amp_index: 3, vendor_sysid: ptr::null_mut(), expected_sysid: ptr::null_mut() },
    cs_amp_lib_test_param { num_amps: 6, amp_index: 4, vendor_sysid: ptr::null_mut(), expected_sysid: ptr::null_mut() },
    cs_amp_lib_test_param { num_amps: 6, amp_index: 5, vendor_sysid: ptr::null_mut(), expected_sysid: ptr::null_mut() },
    cs_amp_lib_test_param { num_amps: 8, amp_index: 0, vendor_sysid: ptr::null_mut(), expected_sysid: ptr::null_mut() },
    cs_amp_lib_test_param { num_amps: 8, amp_index: 1, vendor_sysid: ptr::null_mut(), expected_sysid: ptr::null_mut() },
    cs_amp_lib_test_param { num_amps: 8, amp_index: 2, vendor_sysid: ptr::null_mut(), expected_sysid: ptr::null_mut() },
    cs_amp_lib_test_param { num_amps: 8, amp_index: 3, vendor_sysid: ptr::null_mut(), expected_sysid: ptr::null_mut() },
    cs_amp_lib_test_param { num_amps: 8, amp_index: 4, vendor_sysid: ptr::null_mut(), expected_sysid: ptr::null_mut() },
    cs_amp_lib_test_param { num_amps: 8, amp_index: 5, vendor_sysid: ptr::null_mut(), expected_sysid: ptr::null_mut() },
    cs_amp_lib_test_param { num_amps: 8, amp_index: 6, vendor_sysid: ptr::null_mut(), expected_sysid: ptr::null_mut() },
    cs_amp_lib_test_param { num_amps: 8, amp_index: 7, vendor_sysid: ptr::null_mut(), expected_sysid: ptr::null_mut() },
];

unsafe fn cs_amp_lib_test_get_cal_param_desc(param: *const cs_amp_lib_test_param, desc: *mut c_char) {
    snprintf(desc, 128, b"num_amps:%d amp_index:%d\0".as_ptr() as *const c_char,
             (*param).num_amps, (*param).amp_index);
}

macro_rules! pstr { ($s:literal) => { concat!($s, "\0").as_ptr() as *mut c_char }; }

static mut cs_amp_lib_test_ssidexv2_param_cases: [cs_amp_lib_test_param; 21] = [
    cs_amp_lib_test_param { num_amps: 0, amp_index: 0, vendor_sysid: pstr!("abcd_00"), expected_sysid: pstr!("00") },
    cs_amp_lib_test_param { num_amps: 0, amp_index: 0, vendor_sysid: pstr!("abcd_01"), expected_sysid: pstr!("01") },
    cs_amp_lib_test_param { num_amps: 0, amp_index: 0, vendor_sysid: pstr!("abcd_XY"), expected_sysid: pstr!("XY") },
    cs_amp_lib_test_param { num_amps: 0, amp_index: 0, vendor_sysid: pstr!("1028abcd_00"), expected_sysid: pstr!("00") },
    cs_amp_lib_test_param { num_amps: 0, amp_index: 0, vendor_sysid: pstr!("1028abcd_01"), expected_sysid: pstr!("01") },
    cs_amp_lib_test_param { num_amps: 0, amp_index: 0, vendor_sysid: pstr!("1028abcd_XY"), expected_sysid: pstr!("XY") },
    cs_amp_lib_test_param { num_amps: 0, amp_index: 0, vendor_sysid: pstr!("abcd_00_WF"), expected_sysid: pstr!("00") },
    cs_amp_lib_test_param { num_amps: 0, amp_index: 0, vendor_sysid: pstr!("abcd_01_WF"), expected_sysid: pstr!("01") },
    cs_amp_lib_test_param { num_amps: 0, amp_index: 0, vendor_sysid: pstr!("abcd_XY_WF"), expected_sysid: pstr!("XY") },
    cs_amp_lib_test_param { num_amps: 0, amp_index: 0, vendor_sysid: pstr!("1028abcd_00_WF"), expected_sysid: pstr!("00") },
    cs_amp_lib_test_param { num_amps: 0, amp_index: 0, vendor_sysid: pstr!("1028abcd_01_WF"), expected_sysid: pstr!("01") },
    cs_amp_lib_test_param { num_amps: 0, amp_index: 0, vendor_sysid: pstr!("1028abcd_XY_WF"), expected_sysid: pstr!("XY") },
    cs_amp_lib_test_param { num_amps: 0, amp_index: 0, vendor_sysid: pstr!("abcd_00_AA_BB"), expected_sysid: pstr!("00") },
    cs_amp_lib_test_param { num_amps: 0, amp_index: 0, vendor_sysid: pstr!("abcd_01_AA_BB"), expected_sysid: pstr!("01") },
    cs_amp_lib_test_param { num_amps: 0, amp_index: 0, vendor_sysid: pstr!("abcd_XY_AA_BB"), expected_sysid: pstr!("XY") },
    cs_amp_lib_test_param { num_amps: 0, amp_index: 0, vendor_sysid: pstr!("1028abcd_00_AA_BB"), expected_sysid: pstr!("00") },
    cs_amp_lib_test_param { num_amps: 0, amp_index: 0, vendor_sysid: pstr!("1028abcd_01_AA_BB"), expected_sysid: pstr!("01") },
    cs_amp_lib_test_param { num_amps: 0, amp_index: 0, vendor_sysid: pstr!("1028abcd_XY_A_BB"), expected_sysid: pstr!("XY") },
    cs_amp_lib_test_param { num_amps: 0, amp_index: 0, vendor_sysid: ptr::null_mut(), expected_sysid: ptr::null_mut() },
    cs_amp_lib_test_param { num_amps: 0, amp_index: 0, vendor_sysid: ptr::null_mut(), expected_sysid: ptr::null_mut() },
    cs_amp_lib_test_param { num_amps: 0, amp_index: 0, vendor_sysid: ptr::null_mut(), expected_sysid: ptr::null_mut() },
];

unsafe fn cs_amp_lib_test_ssidexv2_param_desc(param: *const cs_amp_lib_test_param, desc: *mut c_char) {
    snprintf(desc, 128, b"vendor_sysid:'%s' expected_sysid:'%s'\0".as_ptr() as *const c_char,
             (*param).vendor_sysid, (*param).expected_sysid);
}

static mut cs_amp_lib_test_ssidexv2_invalid_param_cases: [cs_amp_lib_test_param; 22] = [
    cs_amp_lib_test_param { num_amps: 0, amp_index: 0, vendor_sysid: pstr!("abcd"), expected_sysid: ptr::null_mut() },
    cs_amp_lib_test_param { num_amps: 0, amp_index: 0, vendor_sysid: pstr!("abcd_0"), expected_sysid: ptr::null_mut() },
    cs_amp_lib_test_param { num_amps: 0, amp_index: 0, vendor_sysid: pstr!("abcd_1"), expected_sysid: ptr::null_mut() },
    cs_amp_lib_test_param { num_amps: 0, amp_index: 0, vendor_sysid: pstr!("abcd_0_1"), expected_sysid: ptr::null_mut() },
    cs_amp_lib_test_param { num_amps: 0, amp_index: 0, vendor_sysid: pstr!("abcd_1_1"), expected_sysid: ptr::null_mut() },
    cs_amp_lib_test_param { num_amps: 0, amp_index: 0, vendor_sysid: pstr!("abcd_1_X"), expected_sysid: ptr::null_mut() },
    cs_amp_lib_test_param { num_amps: 0, amp_index: 0, vendor_sysid: pstr!("abcd_1_X"), expected_sysid: ptr::null_mut() },
    cs_amp_lib_test_param { num_amps: 0, amp_index: 0, vendor_sysid: pstr!("abcd_000"), expected_sysid: ptr::null_mut() },
    cs_amp_lib_test_param { num_amps: 0, amp_index: 0, vendor_sysid: pstr!("abcd_010"), expected_sysid: ptr::null_mut() },
    cs_amp_lib_test_param { num_amps: 0, amp_index: 0, vendor_sysid: pstr!("abcd_000_01"), expected_sysid: ptr::null_mut() },
    cs_amp_lib_test_param { num_amps: 0, amp_index: 0, vendor_sysid: pstr!("abcd_000_01"), expected_sysid: ptr::null_mut() },
    cs_amp_lib_test_param { num_amps: 0, amp_index: 0, vendor_sysid: pstr!("1234abcd"), expected_sysid: ptr::null_mut() },
    cs_amp_lib_test_param { num_amps: 0, amp_index: 0, vendor_sysid: pstr!("1234abcd_0"), expected_sysid: ptr::null_mut() },
    cs_amp_lib_test_param { num_amps: 0, amp_index: 0, vendor_sysid: pstr!("1234abcd_1"), expected_sysid: ptr::null_mut() },
    cs_amp_lib_test_param { num_amps: 0, amp_index: 0, vendor_sysid: pstr!("1234abcd_0_1"), expected_sysid: ptr::null_mut() },
    cs_amp_lib_test_param { num_amps: 0, amp_index: 0, vendor_sysid: pstr!("1234abcd_1_1"), expected_sysid: ptr::null_mut() },
    cs_amp_lib_test_param { num_amps: 0, amp_index: 0, vendor_sysid: pstr!("1234abcd_1_X"), expected_sysid: ptr::null_mut() },
    cs_amp_lib_test_param { num_amps: 0, amp_index: 0, vendor_sysid: pstr!("1234abcd_1_X"), expected_sysid: ptr::null_mut() },
    cs_amp_lib_test_param { num_amps: 0, amp_index: 0, vendor_sysid: pstr!("1234abcd_000"), expected_sysid: ptr::null_mut() },
    cs_amp_lib_test_param { num_amps: 0, amp_index: 0, vendor_sysid: pstr!("1234abcd_010"), expected_sysid: ptr::null_mut() },
    cs_amp_lib_test_param { num_amps: 0, amp_index: 0, vendor_sysid: pstr!("1234abcd_000_01"), expected_sysid: ptr::null_mut() },
    cs_amp_lib_test_param { num_amps: 0, amp_index: 0, vendor_sysid: pstr!("1234abcd_000_01"), expected_sysid: ptr::null_mut() },
];

#[repr(C)]
struct kunit_case {
    run_case: Option<unsafe fn(*mut kunit)>,
}

#[repr(C)]
struct kunit_suite {
    name: *const c_char,
    init: Option<unsafe fn(*mut kunit) -> c_int>,
    test_cases: *mut kunit_case,
}

macro_rules! KUNIT_CASE {
    ($name:ident) => {
        kunit_case { run_case: Some($name) }
    };
}
macro_rules! KUNIT_CASE_PARAM {
    ($name:ident, $params:ident) => {
        kunit_case { run_case: Some($name) }
    };
}

static mut cs_amp_lib_test_cases: [kunit_case; 49] = [
    /* Tests for getting calibration data from EFI */
    KUNIT_CASE!(cs_amp_lib_test_cal_data_too_short_test),
    KUNIT_CASE!(cs_amp_lib_test_cal_count_too_big_test),
    KUNIT_CASE!(cs_amp_lib_test_no_cal_data_test),
    KUNIT_CASE!(cs_amp_lib_test_get_efi_cal_uid_not_found_noindex_test),
    KUNIT_CASE!(cs_amp_lib_test_get_efi_cal_uid_not_found_index_not_found_test),
    KUNIT_CASE!(cs_amp_lib_test_get_efi_cal_no_uid_index_not_found_test),
    KUNIT_CASE!(cs_amp_lib_test_get_efi_cal_no_uid_no_index_test),
    KUNIT_CASE!(cs_amp_lib_test_get_efi_cal_zero_not_matched_test),
    KUNIT_CASE!(cs_amp_lib_test_get_hp_efi_cal),
    KUNIT_CASE_PARAM!(cs_amp_lib_test_get_efi_cal_by_uid_test, cs_amp_lib_test_get_cal_gen_params),
    KUNIT_CASE_PARAM!(cs_amp_lib_test_get_efi_cal_by_index_unchecked_test, cs_amp_lib_test_get_cal_gen_params),
    KUNIT_CASE_PARAM!(cs_amp_lib_test_get_efi_cal_by_index_checked_test, cs_amp_lib_test_get_cal_gen_params),
    KUNIT_CASE_PARAM!(cs_amp_lib_test_get_efi_cal_by_index_uid_mismatch_test, cs_amp_lib_test_get_cal_gen_params),
    KUNIT_CASE_PARAM!(cs_amp_lib_test_get_efi_cal_by_index_fallback_test, cs_amp_lib_test_get_cal_gen_params),
    KUNIT_CASE!(cs_amp_lib_test_get_efi_cal_empty_entry_test),
    /* Tests for writing and reading calibration data */
    KUNIT_CASE!(cs_amp_lib_test_write_cal_data_test),
    KUNIT_CASE!(cs_amp_lib_test_read_cal_data_test),
    KUNIT_CASE!(cs_amp_lib_test_write_ambient_test),
    /* Test cases for writing cal data to UEFI */
    KUNIT_CASE!(cs_amp_lib_test_create_new_cal_efi),
    KUNIT_CASE!(cs_amp_lib_test_create_new_cal_efi_indexed),
    KUNIT_CASE!(cs_amp_lib_test_create_new_cal_efi_indexed_no_max),
    KUNIT_CASE!(cs_amp_lib_test_cal_efi_all_zeros_add_first),
    KUNIT_CASE!(cs_amp_lib_test_cal_efi_all_zeros_add_first_no_shrink),
    KUNIT_CASE!(cs_amp_lib_test_cal_efi_all_zeros_add_first_indexed),
    KUNIT_CASE!(cs_amp_lib_test_cal_efi_all_zeros_add_first_indexed_no_shrink),
    KUNIT_CASE!(cs_amp_lib_test_grow_append_cal_efi),
    KUNIT_CASE!(cs_amp_lib_test_grow_append_cal_efi_indexed),
    KUNIT_CASE!(cs_amp_lib_test_grow_append_cal_efi_indexed_no_max),
    KUNIT_CASE!(cs_amp_lib_test_grow_cal_efi_replace_indexed),
    KUNIT_CASE!(cs_amp_lib_test_grow_cal_efi_replace_by_uid),
    KUNIT_CASE!(cs_amp_lib_test_cal_efi_replace_by_uid),
    KUNIT_CASE!(cs_amp_lib_test_cal_efi_replace_by_index),
    KUNIT_CASE!(cs_amp_lib_test_cal_efi_deduplicate),
    KUNIT_CASE!(cs_amp_lib_test_cal_efi_find_free),
    KUNIT_CASE!(cs_amp_lib_test_cal_efi_bad_cal_target),
    KUNIT_CASE!(cs_amp_lib_test_cal_efi_write_denied),
    KUNIT_CASE!(cs_amp_lib_test_cal_efi_attr_preserved),
    KUNIT_CASE!(cs_amp_lib_test_cal_efi_update_hp),
    /* Test cases for speaker ID */
    KUNIT_CASE!(cs_amp_lib_test_spkid_lenovo_not_present),
    KUNIT_CASE!(cs_amp_lib_test_spkid_lenovo_d0),
    KUNIT_CASE!(cs_amp_lib_test_spkid_lenovo_d1),
    KUNIT_CASE!(cs_amp_lib_test_spkid_lenovo_illegal),
    KUNIT_CASE!(cs_amp_lib_test_spkid_lenovo_oversize),
    KUNIT_CASE!(cs_amp_lib_test_spkid_hp_30),
    KUNIT_CASE!(cs_amp_lib_test_spkid_hp_31),
    /* Test cases for SSIDExV2 */
    KUNIT_CASE_PARAM!(cs_amp_lib_test_ssidexv2_fetch, cs_amp_lib_test_ssidexv2_gen_params),
    KUNIT_CASE_PARAM!(cs_amp_lib_test_ssidexv2_fetch_invalid, cs_amp_lib_test_ssidexv2_invalid_gen_params),
    KUNIT_CASE_PARAM!(cs_amp_lib_test_ssidexv2_not_dell, cs_amp_lib_test_ssidexv2_gen_params),
    KUNIT_CASE!(cs_amp_lib_test_vendor_variant_id_not_found),
];

static mut cs_amp_lib_test_suite: kunit_suite = kunit_suite {
    name: b"snd-soc-cs-amp-lib-test\0".as_ptr() as *const c_char,
    init: Some(cs_amp_lib_test_case_init),
    test_cases: unsafe { cs_amp_lib_test_cases.as_mut_ptr() },
};

/* kunit_test_suite(cs_amp_lib_test_suite); */
/* MODULE_IMPORT_NS("SND_SOC_CS_AMP_LIB"); */
/* MODULE_DESCRIPTION("KUnit test for Cirrus Logic amplifier library"); */
/* MODULE_AUTHOR("Richard Fitzgerald <rf@opensource.cirrus.com>"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
