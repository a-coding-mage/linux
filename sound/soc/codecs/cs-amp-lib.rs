// SPDX-License-Identifier: GPL-2.0-only
//
// Common code for Cirrus Logic Smart Amplifiers
//
// Copyright (C) 2024 Cirrus Logic, Inc. and
//               Cirrus Logic International Semiconductor Ltd.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type u8 = u8;
type u16 = u16;
type u32 = u32;
type u64 = u64;
type s8 = i8;
type efi_char16_t = u16;
type efi_status_t = u64;

const ENODEV: c_int = 19;
const ENOENT: c_int = 2;
const EFBIG: c_int = 27;
const EACCES: c_int = 13;
const EIO: c_int = 5;
const ENOMEM: c_int = 12;
const EOVERFLOW: c_int = 75;
const EINVAL: c_int = 22;

const GFP_KERNEL: u32 = 0;
const CONFIG_FW_CS_DSP: bool = false;
const CONFIG_EFI: bool = false;
const CONFIG_SND_SOC_CS_AMP_LIB_TEST_HOOKS: bool = false;
const EFI_RT_SUPPORTED_GET_VARIABLE: u32 = 0;
const EFI_RT_SUPPORTED_SET_VARIABLE: u32 = 0;
const EFI_VARIABLE_NON_VOLATILE: u32 = 0x00000001;
const EFI_VARIABLE_BOOTSERVICE_ACCESS: u32 = 0x00000002;
const EFI_VARIABLE_RUNTIME_ACCESS: u32 = 0x00000004;
const EFI_SUCCESS: efi_status_t = 0;
const EFI_NOT_FOUND: efi_status_t = 14;
const EFI_BUFFER_TOO_SMALL: efi_status_t = 5;
const EFI_WRITE_PROTECTED: efi_status_t = 8;
const EFI_UNSUPPORTED: efi_status_t = 3;
const EFI_ACCESS_DENIED: efi_status_t = 15;
const EFI_SECURITY_VIOLATION: efi_status_t = 26;
const PCI_VENDOR_ID_DELL: c_int = 0x1028;

#[repr(C)]
pub struct efi_guid_t {
    b: [u8; 16],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dentry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cs_dsp {
    dev: *mut device,
    pwr_lock: mutex,
    ctl_list: list_head,
}

#[repr(C)]
pub struct cs_dsp_coeff_ctl {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cirrus_amp_cal_controls {
    ambient: *const c_char,
    calr: *const c_char,
    status: *const c_char,
    checksum: *const c_char,
    mem_region: c_int,
    alg_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cirrus_amp_cal_data {
    calTarget: [u32; 2],
    calTime: [u32; 2],
    calAmbient: s8,
    calStatus: u8,
    calR: u16,
}

#[repr(C)]
pub struct cirrus_amp_efi_data {
    size: u32,
    count: u32,
    data: [cirrus_amp_cal_data; 0],
}

#[repr(C)]
pub struct efi_rt {
    get_variable: unsafe extern "C" fn(
        *mut efi_char16_t,
        *mut efi_guid_t,
        *mut u32,
        *mut c_ulong,
        *mut c_void,
    ) -> efi_status_t,
    set_variable: unsafe extern "C" fn(
        *mut efi_char16_t,
        *mut efi_guid_t,
        u32,
        c_ulong,
        *mut c_void,
    ) -> efi_status_t,
}

#[repr(C)]
pub struct cs_amp_test_hooks {
    get_efi_variable: unsafe extern "C" fn(
        *mut efi_char16_t,
        *mut efi_guid_t,
        *mut u32,
        *mut c_ulong,
        *mut c_void,
    ) -> efi_status_t,
    set_efi_variable: unsafe extern "C" fn(
        *mut efi_char16_t,
        *mut efi_guid_t,
        u32,
        c_ulong,
        *mut c_void,
    ) -> efi_status_t,
    write_cal_coeff: unsafe extern "C" fn(
        *mut cs_dsp,
        *const cirrus_amp_cal_controls,
        *const c_char,
        u32,
    ) -> c_int,
    read_cal_coeff: unsafe extern "C" fn(
        *mut cs_dsp,
        *const cirrus_amp_cal_controls,
        *const c_char,
        *mut u32,
    ) -> c_int,
}

unsafe extern "C" {
    static efi: efi_rt;

    fn ktime_get_real_ns() -> u64;
    fn cs_dsp_get_ctl(
        dsp: *mut cs_dsp,
        name: *const c_char,
        mem_region: c_int,
        alg_id: u32,
    ) -> *mut cs_dsp_coeff_ctl;
    fn cs_dsp_coeff_write_ctrl(
        ctl: *mut cs_dsp_coeff_ctl,
        off: c_uint,
        buf: *mut c_void,
        len: c_ulong,
    ) -> c_int;
    fn cs_dsp_coeff_read_ctrl(
        ctl: *mut cs_dsp_coeff_ctl,
        off: c_uint,
        buf: *mut c_void,
        len: c_ulong,
    ) -> c_int;
    fn efi_rt_services_supported(mask: u32) -> bool;
    fn kzalloc(size: c_ulong, flags: u32) -> *mut c_void;
    fn kmalloc(size: c_ulong, flags: u32) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn devm_kstrdup(dev: *mut device, s: *const c_char, flags: u32) -> *mut c_char;
    fn debugfs_lookup(name: *const c_char, parent: *mut dentry) -> *mut dentry;
    fn debugfs_create_dir(name: *const c_char, parent: *mut dentry) -> *mut dentry;
    fn dev_name(dev: *mut device) -> *const c_char;
    fn dput(dentry: *mut dentry);
    fn strlen(s: *const c_char) -> c_ulong;
    fn sscanf(s: *const c_char, fmt: *const c_char, ...) -> c_int;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: c_ulong) -> *mut c_void;
    fn memset(dst: *mut c_void, c: c_int, n: c_ulong) -> *mut c_void;
    fn list_empty(head: *const list_head) -> bool;
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn cs_amp_cal_target_u64(data: *const cirrus_amp_cal_data) -> u64;
}

type c_uint = u32;

macro_rules! c_str {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

const fn efi_guid(
    a: u32,
    b: u16,
    c: u16,
    d0: u8,
    d1: u8,
    d2: u8,
    d3: u8,
    d4: u8,
    d5: u8,
    d6: u8,
    d7: u8,
) -> efi_guid_t {
    let ab = a.to_le_bytes();
    let bb = b.to_le_bytes();
    let cb = c.to_le_bytes();
    efi_guid_t {
        b: [ab[0], ab[1], ab[2], ab[3], bb[0], bb[1], cb[0], cb[1], d0, d1, d2, d3, d4, d5, d6, d7],
    }
}

static CIRRUS_LOGIC_CALIBRATION_EFI_NAME: [efi_char16_t; 30] = [
    b'C' as u16, b'i' as u16, b'r' as u16, b'r' as u16, b'u' as u16, b's' as u16, b'S' as u16,
    b'm' as u16, b'a' as u16, b'r' as u16, b't' as u16, b'A' as u16, b'm' as u16, b'p' as u16,
    b'C' as u16, b'a' as u16, b'l' as u16, b'i' as u16, b'b' as u16, b'r' as u16, b'a' as u16,
    b't' as u16, b'i' as u16, b'o' as u16, b'n' as u16, b'D' as u16, b'a' as u16, b't' as u16,
    b'a' as u16, 0,
];
static mut CIRRUS_LOGIC_CALIBRATION_EFI_GUID: efi_guid_t =
    efi_guid(0x02f9af02, 0x7734, 0x4233, 0xb4, 0x3d, 0x93, 0xfe, 0x5a, 0xa3, 0x5d, 0xb3);

static LENOVO_SPEAKER_ID_EFI_NAME: [efi_char16_t; 11] = [
    b'S' as u16, b'd' as u16, b'w' as u16, b'S' as u16, b'p' as u16, b'e' as u16, b'a' as u16,
    b'k' as u16, b'e' as u16, b'r' as u16, 0,
];
static mut LENOVO_SPEAKER_ID_EFI_GUID: efi_guid_t =
    efi_guid(0x48df970e, 0xe27f, 0x460a, 0xb5, 0x86, 0x77, 0x19, 0x80, 0x1d, 0x92, 0x82);

static HP_SPEAKER_ID_EFI_NAME: [efi_char16_t; 12] = [
    b'H' as u16, b'P' as u16, b'S' as u16, b'p' as u16, b'e' as u16, b'a' as u16, b'k' as u16,
    b'e' as u16, b'r' as u16, b'I' as u16, b'D' as u16, 0,
];
static mut HP_SPEAKER_ID_EFI_GUID: efi_guid_t =
    efi_guid(0xc49593a4, 0xd099, 0x419b, 0xa2, 0xc3, 0x67, 0xe9, 0x80, 0xe6, 0x1d, 0x1e);

static HP_CALIBRATION_EFI_NAME: [efi_char16_t; 24] = [
    b'S' as u16, b'm' as u16, b'a' as u16, b'r' as u16, b't' as u16, b'A' as u16, b'm' as u16,
    b'p' as u16, b'C' as u16, b'a' as u16, b'l' as u16, b'i' as u16, b'b' as u16, b'r' as u16,
    b'a' as u16, b't' as u16, b'i' as u16, b'o' as u16, b'n' as u16, b'D' as u16, b'a' as u16,
    b't' as u16, b'a' as u16, 0,
];
static mut HP_CALIBRATION_EFI_GUID: efi_guid_t =
    efi_guid(0x53559579, 0x8753, 0x4f5c, 0x91, 0x30, 0xe8, 0x2a, 0xcf, 0xb8, 0xd8, 0x93);

static DELL_SSIDEXV2_EFI_NAME: [efi_char16_t; 13] = [
    b'S' as u16, b'S' as u16, b'I' as u16, b'D' as u16, b'e' as u16, b'x' as u16, b'V' as u16,
    b'2' as u16, b'D' as u16, b'a' as u16, b't' as u16, b'a' as u16, 0,
];
static mut DELL_SSIDEXV2_EFI_GUID: efi_guid_t =
    efi_guid(0x6a5f35df, 0x1432, 0x4656, 0x85, 0x97, 0x31, 0x04, 0xd5, 0xbf, 0x3a, 0xb0);

#[repr(C)]
struct cs_amp_lib_cal_efivar {
    name: *mut efi_char16_t,
    guid: *mut efi_guid_t,
}

static mut cs_amp_lib_cal_efivars: [cs_amp_lib_cal_efivar; 2] = [
    cs_amp_lib_cal_efivar {
        name: HP_CALIBRATION_EFI_NAME.as_ptr() as *mut efi_char16_t,
        guid: unsafe { &raw mut HP_CALIBRATION_EFI_GUID },
    },
    cs_amp_lib_cal_efivar {
        name: CIRRUS_LOGIC_CALIBRATION_EFI_NAME.as_ptr() as *mut efi_char16_t,
        guid: unsafe { &raw mut CIRRUS_LOGIC_CALIBRATION_EFI_GUID },
    },
];

const CS_AMP_CAL_DEFAULT_EFI_ATTR: u32 =
    EFI_VARIABLE_NON_VOLATILE | EFI_VARIABLE_BOOTSERVICE_ACCESS | EFI_VARIABLE_RUNTIME_ACCESS;

/* Offset from Unix time to Windows time (100ns since 1 Jan 1601) */
const UNIX_TIME_TO_WINDOWS_TIME_OFFSET: u64 = 116444736000000000u64;

static mut cs_amp_efi_cal_write_lock: mutex = mutex { _private: [] };

unsafe fn IS_ENABLED(v: bool) -> bool {
    v
}

unsafe fn IS_REACHABLE(v: bool) -> bool {
    v
}

unsafe fn ERR_PTR<T>(err: c_int) -> *mut T {
    err as isize as *mut T
}

unsafe fn PTR_ERR<T>(ptr: *mut T) -> c_int {
    ptr as isize as c_int
}

unsafe fn IS_ERR<T>(ptr: *const T) -> bool {
    (ptr as usize) >= (!4095usize)
}

unsafe fn IS_ERR_OR_NULL<T>(ptr: *const T) -> bool {
    ptr.is_null() || IS_ERR(ptr)
}

unsafe fn PTR_ERR_OR_ZERO<T>(ptr: *mut T) -> c_int {
    if IS_ERR(ptr) {
        PTR_ERR(ptr)
    } else {
        0
    }
}

unsafe fn struct_size_cirrus_amp_efi_data(n: u32) -> c_ulong {
    (size_of::<cirrus_amp_efi_data>() + size_of::<cirrus_amp_cal_data>() * n as usize) as c_ulong
}

unsafe fn cirrus_amp_efi_data_at(data: *mut cirrus_amp_efi_data, i: c_int) -> *mut cirrus_amp_cal_data {
    (data as *mut u8)
        .add(size_of::<cirrus_amp_efi_data>() + size_of::<cirrus_amp_cal_data>() * i as usize)
        as *mut cirrus_amp_cal_data
}

unsafe fn cs_amp_time_now_in_windows_time() -> u64 {
    let time_in_100ns: u64 = ktime_get_real_ns() / 100;

    time_in_100ns.wrapping_add(UNIX_TIME_TO_WINDOWS_TIME_OFFSET)
}

unsafe extern "C" fn cs_amp_write_cal_coeff(
    dsp: *mut cs_dsp,
    controls: *const cirrus_amp_cal_controls,
    ctl_name: *const c_char,
    val: u32,
) -> c_int {
    let cs_ctl: *mut cs_dsp_coeff_ctl;
    let mut beval: u32 = val.to_be();
    let ret: c_int;

    if IS_REACHABLE(CONFIG_FW_CS_DSP) {
        mutex_lock(&raw mut (*dsp).pwr_lock);
        cs_ctl = cs_dsp_get_ctl(dsp, ctl_name, (*controls).mem_region, (*controls).alg_id);
        ret = cs_dsp_coeff_write_ctrl(
            cs_ctl,
            0,
            &mut beval as *mut u32 as *mut c_void,
            size_of::<u32>() as c_ulong,
        );
        mutex_unlock(&raw mut (*dsp).pwr_lock);

        if ret < 0 {
            dev_err((*dsp).dev, c_str!("Failed to write to '%s': %d\n"), ctl_name, ret);
            return ret;
        }

        return 0;
    }

    -ENODEV
}

unsafe extern "C" fn cs_amp_read_cal_coeff(
    dsp: *mut cs_dsp,
    controls: *const cirrus_amp_cal_controls,
    ctl_name: *const c_char,
    val: *mut u32,
) -> c_int {
    let cs_ctl: *mut cs_dsp_coeff_ctl;
    let mut beval: u32 = 0;
    let ret: c_int;

    if !IS_REACHABLE(CONFIG_FW_CS_DSP) {
        return -ENODEV;
    }

    mutex_lock(&raw mut (*dsp).pwr_lock);
    cs_ctl = cs_dsp_get_ctl(dsp, ctl_name, (*controls).mem_region, (*controls).alg_id);
    ret = cs_dsp_coeff_read_ctrl(
        cs_ctl,
        0,
        &mut beval as *mut u32 as *mut c_void,
        size_of::<u32>() as c_ulong,
    );
    mutex_unlock(&raw mut (*dsp).pwr_lock);

    if ret < 0 {
        dev_err((*dsp).dev, c_str!("Failed to read '%s': %d\n"), ctl_name, ret);
        return ret;
    }

    *val = u32::from_be(beval);

    0
}

unsafe fn _cs_amp_write_cal_coeffs(
    dsp: *mut cs_dsp,
    controls: *const cirrus_amp_cal_controls,
    data: *const cirrus_amp_cal_data,
) -> c_int {
    let mut ret: c_int;

    dev_dbg(
        (*dsp).dev,
        c_str!("Calibration: Ambient=%#x, Status=%#x, CalR=%d\n"),
        (*data).calAmbient as c_int,
        (*data).calStatus as c_int,
        (*data).calR as c_int,
    );

    if list_empty(&raw const (*dsp).ctl_list) {
        dev_info((*dsp).dev, c_str!("Calibration disabled due to missing firmware controls\n"));
        return -ENOENT;
    }

    ret = cs_amp_write_cal_coeff(dsp, controls, (*controls).ambient, (*data).calAmbient as u32);
    if ret != 0 {
        return ret;
    }

    ret = cs_amp_write_cal_coeff(dsp, controls, (*controls).calr, (*data).calR as u32);
    if ret != 0 {
        return ret;
    }

    ret = cs_amp_write_cal_coeff(dsp, controls, (*controls).status, (*data).calStatus as u32);
    if ret != 0 {
        return ret;
    }

    ret = cs_amp_write_cal_coeff(
        dsp,
        controls,
        (*controls).checksum,
        ((*data).calR as u32).wrapping_add(1),
    );
    if ret != 0 {
        return ret;
    }

    0
}

unsafe fn _cs_amp_read_cal_coeffs(
    dsp: *mut cs_dsp,
    controls: *const cirrus_amp_cal_controls,
    data: *mut cirrus_amp_cal_data,
) -> c_int {
    let time: u64;
    let mut val: u32 = 0;
    let mut ret: c_int;

    if list_empty(&raw const (*dsp).ctl_list) {
        dev_info((*dsp).dev, c_str!("Calibration disabled due to missing firmware controls\n"));
        return -ENOENT;
    }

    ret = cs_amp_read_cal_coeff(dsp, controls, (*controls).ambient, &mut val);
    if ret != 0 {
        return ret;
    }

    (*data).calAmbient = val as s8;

    ret = cs_amp_read_cal_coeff(dsp, controls, (*controls).calr, &mut val);
    if ret != 0 {
        return ret;
    }

    (*data).calR = val as u16;

    ret = cs_amp_read_cal_coeff(dsp, controls, (*controls).status, &mut val);
    if ret != 0 {
        return ret;
    }

    (*data).calStatus = val as u8;

    /* Fill in timestamp */
    time = cs_amp_time_now_in_windows_time();
    (*data).calTime[0] = time as u32;
    (*data).calTime[1] = (time >> 32) as u32;

    0
}

/**
 * cs_amp_write_cal_coeffs - Write calibration data to firmware controls.
 * @dsp:	Pointer to struct cs_dsp.
 * @controls:	Pointer to definition of firmware controls to be written.
 * @data:	Pointer to calibration data.
 *
 * Returns: 0 on success, else negative error value.
 */
#[no_mangle]
pub unsafe extern "C" fn cs_amp_write_cal_coeffs(
    dsp: *mut cs_dsp,
    controls: *const cirrus_amp_cal_controls,
    data: *const cirrus_amp_cal_data,
) -> c_int {
    if IS_REACHABLE(CONFIG_FW_CS_DSP) || IS_ENABLED(CONFIG_SND_SOC_CS_AMP_LIB_TEST_HOOKS) {
        _cs_amp_write_cal_coeffs(dsp, controls, data)
    } else {
        -ENODEV
    }
}

/**
 * cs_amp_read_cal_coeffs - Read calibration data from firmware controls.
 * @dsp:	Pointer to struct cs_dsp.
 * @controls:	Pointer to definition of firmware controls to be read.
 * @data:	Pointer to calibration data where results will be written.
 *
 * Returns: 0 on success, else negative error value.
 */
#[no_mangle]
pub unsafe extern "C" fn cs_amp_read_cal_coeffs(
    dsp: *mut cs_dsp,
    controls: *const cirrus_amp_cal_controls,
    data: *mut cirrus_amp_cal_data,
) -> c_int {
    if IS_REACHABLE(CONFIG_FW_CS_DSP) || IS_ENABLED(CONFIG_SND_SOC_CS_AMP_LIB_TEST_HOOKS) {
        _cs_amp_read_cal_coeffs(dsp, controls, data)
    } else {
        -ENODEV
    }
}

/**
 * cs_amp_write_ambient_temp - write value to calibration ambient temperature
 * @dsp:	Pointer to struct cs_dsp.
 * @controls:	Pointer to definition of firmware controls to be read.
 * @temp:	Temperature in degrees celcius.
 *
 * Returns: 0 on success, else negative error value.
 */
#[no_mangle]
pub unsafe extern "C" fn cs_amp_write_ambient_temp(
    dsp: *mut cs_dsp,
    controls: *const cirrus_amp_cal_controls,
    temp: u32,
) -> c_int {
    cs_amp_write_cal_coeff(dsp, controls, (*controls).ambient, temp)
}

unsafe extern "C" fn cs_amp_get_efi_variable(
    name: *mut efi_char16_t,
    guid: *mut efi_guid_t,
    mut returned_attr: *mut u32,
    size: *mut c_ulong,
    buf: *mut c_void,
) -> efi_status_t {
    let mut attr: u32 = 0;

    if returned_attr.is_null() {
        returned_attr = &mut attr;
    }

    if efi_rt_services_supported(EFI_RT_SUPPORTED_GET_VARIABLE) {
        return (efi.get_variable)(name, guid, returned_attr, size, buf);
    }

    EFI_NOT_FOUND
}

unsafe extern "C" fn cs_amp_set_efi_variable(
    name: *mut efi_char16_t,
    guid: *mut efi_guid_t,
    attr: u32,
    size: c_ulong,
    buf: *mut c_void,
) -> efi_status_t {
    if !efi_rt_services_supported(EFI_RT_SUPPORTED_SET_VARIABLE) {
        return EFI_NOT_FOUND;
    }

    (efi.set_variable)(name, guid, attr, size, buf)
}

unsafe fn cs_amp_convert_efi_status(status: efi_status_t) -> c_int {
    match status {
        EFI_SUCCESS => 0,
        EFI_NOT_FOUND => -ENOENT,
        EFI_BUFFER_TOO_SMALL => -EFBIG,
        EFI_WRITE_PROTECTED | EFI_UNSUPPORTED | EFI_ACCESS_DENIED | EFI_SECURITY_VIOLATION => -EACCES,
        _ => -EIO,
    }
}

unsafe fn cs_amp_alloc_get_efi_variable(
    name: *mut efi_char16_t,
    guid: *mut efi_guid_t,
    returned_attr: *mut u32,
) -> *mut c_void {
    let mut status: efi_status_t;
    let mut size: c_ulong = 0;

    status = cs_amp_get_efi_variable(name, guid, ptr::null_mut(), &mut size, ptr::null_mut());
    if status != EFI_BUFFER_TOO_SMALL {
        return ERR_PTR(cs_amp_convert_efi_status(status));
    }

    /* Over-alloc to ensure strings are always NUL-terminated */
    let buf = kzalloc(size.wrapping_add(1), GFP_KERNEL);
    if buf.is_null() {
        return ERR_PTR(-ENOMEM);
    }

    status = cs_amp_get_efi_variable(name, guid, returned_attr, &mut size, buf);
    if status != EFI_SUCCESS {
        kfree(buf);
        return ERR_PTR(cs_amp_convert_efi_status(status));
    }

    buf
}

unsafe fn cs_amp_get_cal_efi_buffer(
    dev: *mut device,
    name: *mut *mut efi_char16_t,
    guid: *mut *mut efi_guid_t,
    attr: *mut u32,
) -> *mut cirrus_amp_efi_data {
    let mut efi_data: *mut cirrus_amp_efi_data;
    let mut data_size: c_ulong = 0;
    let data: *mut u8;
    let mut status: efi_status_t = EFI_NOT_FOUND;
    let mut i: c_int = 0;
    let ret: c_int;

    /* Find EFI variable and get size */
    while i < cs_amp_lib_cal_efivars.len() as c_int {
        status = cs_amp_get_efi_variable(
            cs_amp_lib_cal_efivars[i as usize].name,
            cs_amp_lib_cal_efivars[i as usize].guid,
            attr,
            &mut data_size,
            ptr::null_mut(),
        );
        if status == EFI_BUFFER_TOO_SMALL {
            break;
        }
        i += 1;
    }

    if status != EFI_BUFFER_TOO_SMALL {
        return ERR_PTR(-ENOENT);
    }

    if !name.is_null() {
        *name = cs_amp_lib_cal_efivars[i as usize].name;
    }

    if !guid.is_null() {
        *guid = cs_amp_lib_cal_efivars[i as usize].guid;
    }

    if data_size < size_of::<cirrus_amp_efi_data>() as c_ulong {
        dev_err(dev, c_str!("EFI cal variable truncated\n"));
        return ERR_PTR(-EOVERFLOW);
    }

    /* Get variable contents into buffer */
    data = kmalloc(data_size, GFP_KERNEL) as *mut u8;
    if data.is_null() {
        return ERR_PTR(-ENOMEM);
    }

    status = cs_amp_get_efi_variable(
        cs_amp_lib_cal_efivars[i as usize].name,
        cs_amp_lib_cal_efivars[i as usize].guid,
        attr,
        &mut data_size,
        data as *mut c_void,
    );
    if status != EFI_SUCCESS {
        ret = -EINVAL;
        kfree(data as *mut c_void);
        dev_err(dev, c_str!("Failed to read calibration data from EFI: %d\n"), ret);
        return ERR_PTR(ret);
    }

    efi_data = data as *mut cirrus_amp_efi_data;
    dev_dbg(
        dev,
        c_str!("Calibration: Size=%d, Amp Count=%d\n"),
        (*efi_data).size,
        (*efi_data).count,
    );

    if ((*efi_data).count > 128) || (struct_size_cirrus_amp_efi_data((*efi_data).count) > data_size) {
        dev_err(dev, c_str!("EFI cal variable truncated\n"));
        ret = -EOVERFLOW;
        kfree(data as *mut c_void);
        dev_err(dev, c_str!("Failed to read calibration data from EFI: %d\n"), ret);
        return ERR_PTR(ret);
    }

    /* This could be zero-filled space pre-allocated by the BIOS */
    if (*efi_data).size == 0 {
        (*efi_data).size = data_size as u32;
    }

    efi_data
}

unsafe fn cs_amp_set_cal_efi_buffer(
    _dev: *mut device,
    name: *mut efi_char16_t,
    guid: *mut efi_guid_t,
    attr: u32,
    data: *mut cirrus_amp_efi_data,
) -> c_int {
    let status: efi_status_t;

    status = cs_amp_set_efi_variable(
        name,
        guid,
        attr,
        struct_size_cirrus_amp_efi_data((*data).count),
        data as *mut c_void,
    );

    cs_amp_convert_efi_status(status)
}

unsafe fn _cs_amp_get_efi_calibration_data(
    dev: *mut device,
    target_uid: u64,
    amp_index: c_int,
    out_data: *mut cirrus_amp_cal_data,
) -> c_int {
    let efi_data: *mut cirrus_amp_efi_data;
    let mut cal: *mut cirrus_amp_cal_data = ptr::null_mut();
    let mut i: c_int;
    let ret: c_int;

    efi_data = cs_amp_get_cal_efi_buffer(dev, ptr::null_mut(), ptr::null_mut(), ptr::null_mut());
    if IS_ERR(efi_data) {
        return PTR_ERR(efi_data);
    }

    if target_uid != 0 {
        i = 0;
        while i < (*efi_data).count as c_int {
            let entry = cirrus_amp_efi_data_at(efi_data, i);
            let cal_target: u64 = cs_amp_cal_target_u64(entry);

            /* Skip empty entries */
            if (*entry).calTime[0] == 0 && (*entry).calTime[1] == 0 {
                i += 1;
                continue;
            }

            /* Skip entries with unpopulated silicon ID */
            if cal_target == 0 {
                i += 1;
                continue;
            }

            if cal_target == target_uid {
                cal = entry;
                break;
            }
            i += 1;
        }
    }

    if cal.is_null()
        && amp_index >= 0
        && amp_index < (*efi_data).count as c_int
        && ((*cirrus_amp_efi_data_at(efi_data, amp_index)).calTime[0] != 0
            || (*cirrus_amp_efi_data_at(efi_data, amp_index)).calTime[1] != 0)
    {
        let entry = cirrus_amp_efi_data_at(efi_data, amp_index);
        let cal_target: u64 = cs_amp_cal_target_u64(entry);

        /*
         * Treat unpopulated cal_target as a wildcard.
         * If target_uid != 0 we can only get here if cal_target == 0
         * or it didn't match any cal_target value.
         * If target_uid == 0 it is a wildcard.
         */
        if cal_target == 0 || target_uid == 0 {
            cal = entry;
        } else {
            dev_warn(dev, c_str!("Calibration entry %d does not match silicon ID"), amp_index);
        }
    }

    if !cal.is_null() {
        memcpy(
            out_data as *mut c_void,
            cal as *const c_void,
            size_of::<cirrus_amp_cal_data>() as c_ulong,
        );
        ret = 0;
    } else {
        dev_warn(dev, c_str!("No calibration for silicon ID %#llx\n"), target_uid);
        ret = -ENOENT;
    }

    kfree(efi_data as *mut c_void);

    ret
}

unsafe fn _cs_amp_set_efi_calibration_data(
    dev: *mut device,
    mut amp_index: c_int,
    num_amps: c_int,
    in_data: *const cirrus_amp_cal_data,
) -> c_int {
    let cal_target: u64 = cs_amp_cal_target_u64(in_data);
    let num_entries: c_ulong;
    let mut data: *mut cirrus_amp_efi_data;
    let mut name: *mut efi_char16_t = CIRRUS_LOGIC_CALIBRATION_EFI_NAME.as_ptr() as *mut efi_char16_t;
    let mut guid: *mut efi_guid_t = &raw mut CIRRUS_LOGIC_CALIBRATION_EFI_GUID;
    let mut attr: u32 = CS_AMP_CAL_DEFAULT_EFI_ATTR;
    let mut i: c_int;
    let mut ret: c_int;

    if cal_target == 0 {
        return -EINVAL;
    }

    data = cs_amp_get_cal_efi_buffer(dev, &mut name, &mut guid, &mut attr);
    ret = PTR_ERR_OR_ZERO(data);
    if ret == -ENOENT {
        data = ptr::null_mut();
    } else if ret != 0 {
        return ret;
    }

    if !data.is_null() {
        /*
         * If the EFI variable is just zero-filled reserved space the count
         * must be set.
         */
        if (*data).count == 0 {
            (*data).count =
                (((*data).size as usize - size_of::<cirrus_amp_efi_data>()) / size_of::<cirrus_amp_cal_data>())
                    as u32;
        }

        if amp_index < 0 {
            /* Is there already a slot for this target? */
            amp_index = 0;
            while amp_index < (*data).count as c_int {
                if cs_amp_cal_target_u64(cirrus_amp_efi_data_at(data, amp_index)) == cal_target {
                    break;
                }
                amp_index += 1;
            }

            /* Else find an empty slot */
            if amp_index >= (*data).count as c_int {
                amp_index = 0;
                while amp_index < (*data).count as c_int {
                    let entry = cirrus_amp_efi_data_at(data, amp_index);
                    if (*entry).calTime[0] == 0 && (*entry).calTime[1] == 0 {
                        break;
                    }
                    amp_index += 1;
                }
            }
        } else {
            /*
             * If the index is forced there could be another active
             * slot with the same calTarget. So deduplicate.
             */
            i = 0;
            while i < (*data).count as c_int {
                let entry = cirrus_amp_efi_data_at(data, i);
                if i == amp_index {
                    i += 1;
                    continue;
                }

                if (*entry).calTime[0] == 0 && (*entry).calTime[1] == 0 {
                    i += 1;
                    continue;
                }

                if cs_amp_cal_target_u64(entry) == cal_target {
                    memset(
                        (*entry).calTime.as_mut_ptr() as *mut c_void,
                        0,
                        size_of::<[u32; 2]>() as c_ulong,
                    );
                }
                i += 1;
            }
        }
    }

    if amp_index < 0 {
        amp_index = 0;
    }

    num_entries = core::cmp::max(num_amps, amp_index + 1) as c_ulong;
    if data.is_null() || ((*data).count as c_ulong) < num_entries {
        let new_data: *mut cirrus_amp_efi_data;
        let new_data_size: c_ulong = struct_size_cirrus_amp_efi_data(num_entries as u32);

        new_data = kzalloc(new_data_size, GFP_KERNEL) as *mut cirrus_amp_efi_data;
        if new_data.is_null() {
            ret = -ENOMEM;
            kfree(data as *mut c_void);
            return ret;
        }

        if !data.is_null() {
            memcpy(
                new_data as *mut c_void,
                data as *const c_void,
                struct_size_cirrus_amp_efi_data((*data).count),
            );
            kfree(data as *mut c_void);
        }

        data = new_data;
        (*data).count = num_entries as u32;
        (*data).size = new_data_size as u32;
    }

    *cirrus_amp_efi_data_at(data, amp_index) = *in_data;
    ret = cs_amp_set_cal_efi_buffer(dev, name, guid, attr, data);
    if ret != 0 {
        dev_err(dev, c_str!("Failed writing calibration to EFI: %d\n"), ret);
    }
    kfree(data as *mut c_void);

    ret
}

/**
 * cs_amp_get_efi_calibration_data - get an entry from calibration data in EFI.
 * @dev:	struct device of the caller.
 * @target_uid:	UID to match, or zero to ignore UID matching.
 * @amp_index:	Entry index to use, or -1 to prevent lookup by index.
 * @out_data:	struct cirrus_amp_cal_data where the entry will be copied.
 *
 * This function can perform 3 types of lookup:
 *
 * (target_uid > 0, amp_index >= 0)
 *	UID search with fallback to using the array index.
 *	Search the calibration data for a non-zero calTarget that matches
 *	target_uid, and if found return that entry. Else, if the entry at
 *	[amp_index] has calTarget == 0, return that entry. Else fail.
 *
 * (target_uid > 0, amp_index < 0)
 *	UID search only.
 *	Search the calibration data for a non-zero calTarget that matches
 *	target_uid, and if found return that entry. Else fail.
 *
 * (target_uid == 0, amp_index >= 0)
 *	Array index fetch only.
 *	Return the entry at [amp_index].
 *
 * An array lookup will be skipped if amp_index exceeds the number of
 * entries in the calibration array, and in this case the return will
 * be -ENOENT. An out-of-range amp_index does not prevent matching by
 * target_uid - it has the same effect as passing amp_index < 0.
 *
 * If the EFI data is too short to be a valid entry, or the entry count
 * in the EFI data overflows the actual length of the data, this function
 * returns -EOVERFLOW.
 *
 * Return: 0 if the entry was found, -ENOENT if no entry was found,
 *	   -EOVERFLOW if the EFI file is corrupt, else other error value.
 */
#[no_mangle]
pub unsafe extern "C" fn cs_amp_get_efi_calibration_data(
    dev: *mut device,
    target_uid: u64,
    amp_index: c_int,
    out_data: *mut cirrus_amp_cal_data,
) -> c_int {
    if IS_ENABLED(CONFIG_EFI) || IS_ENABLED(CONFIG_SND_SOC_CS_AMP_LIB_TEST_HOOKS) {
        _cs_amp_get_efi_calibration_data(dev, target_uid, amp_index, out_data)
    } else {
        -ENOENT
    }
}

/**
 * cs_amp_set_efi_calibration_data - write a calibration data entry to EFI.
 * @dev:	struct device of the caller.
 * @amp_index:	Entry index to use, or -1 to use any available slot.
 * @num_amps:	Maximum number of amps to reserve slots for, or -1 to ignore.
 * @in_data:	struct cirrus_amp_cal_data entry to be written to EFI.
 *
 * If a Vendor-specific variable exists it will be updated,
 * else if the Cirrus variable exists it will be updated
 * else the Cirrus variable will be created.
 *
 * If amp_index >= 0 the data will be placed in this entry of the calibration
 * data array, overwriting what was in that entry. Any other entries with the
 * same calTarget will be marked empty.
 *
 * If amp_index < 0 and in_data->calTarget matches any existing entry, that
 * entry will be overwritten. Else the first available free entry will be used,
 * extending the size of the EFI variable if there are no free entries.
 *
 * If num_amps > 0 the EFI variable will be sized to contain at least this
 * many calibration entries, with any new entries marked empty.
 *
 * Return: 0 if the write was successful, -EFBIG if space could not be made in
 *	   the EFI file to add the entry, -EACCES if it was not possible to
 *	   read or write the EFI variable.
 */
#[no_mangle]
pub unsafe extern "C" fn cs_amp_set_efi_calibration_data(
    dev: *mut device,
    amp_index: c_int,
    num_amps: c_int,
    in_data: *const cirrus_amp_cal_data,
) -> c_int {
    if IS_ENABLED(CONFIG_EFI) || IS_ENABLED(CONFIG_SND_SOC_CS_AMP_LIB_TEST_HOOKS) {
        let ret: c_int;
        mutex_lock(&raw mut cs_amp_efi_cal_write_lock);
        ret = _cs_amp_set_efi_calibration_data(dev, amp_index, num_amps, in_data);
        mutex_unlock(&raw mut cs_amp_efi_cal_write_lock);
        return ret;
    }

    -ENOENT
}

#[repr(C)]
struct cs_amp_spkid_efi {
    name: *mut efi_char16_t,
    guid: *mut efi_guid_t,
    values: [u8; 2],
}

unsafe fn cs_amp_get_efi_byte_spkid(dev: *mut device, info: *const cs_amp_spkid_efi) -> c_int {
    let status: efi_status_t;
    let mut size: c_ulong;
    let mut spkid: u8 = 0;
    let mut i: c_int;
    let ret: c_int;

    size = size_of::<u8>() as c_ulong;
    status = cs_amp_get_efi_variable(
        (*info).name,
        (*info).guid,
        ptr::null_mut(),
        &mut size,
        &mut spkid as *mut u8 as *mut c_void,
    );
    ret = cs_amp_convert_efi_status(status);
    if ret < 0 {
        return ret;
    }

    if size == 0 {
        return -ENOENT;
    }

    i = 0;
    while i < (*info).values.len() as c_int {
        if (*info).values[i as usize] == spkid {
            return i;
        }
        i += 1;
    }

    dev_err(dev, c_str!("EFI speaker ID bad value %#x\n"), spkid as c_int);

    -EINVAL
}

static mut cs_amp_spkid_byte_types: [cs_amp_spkid_efi; 2] = [
    cs_amp_spkid_efi {
        name: LENOVO_SPEAKER_ID_EFI_NAME.as_ptr() as *mut efi_char16_t,
        guid: unsafe { &raw mut LENOVO_SPEAKER_ID_EFI_GUID },
        values: [0xd0, 0xd1],
    },
    cs_amp_spkid_efi {
        name: HP_SPEAKER_ID_EFI_NAME.as_ptr() as *mut efi_char16_t,
        guid: unsafe { &raw mut HP_SPEAKER_ID_EFI_GUID },
        values: [0x30, 0x31],
    },
];

/**
 * cs_amp_get_vendor_spkid - get a speaker ID from vendor-specific storage
 * @dev:	pointer to struct device
 *
 * Known vendor-specific methods of speaker ID are checked and if one is
 * found its speaker ID value is returned.
 *
 * Return: >=0 is a valid speaker ID. -ENOENT if a vendor-specific method
 *	   was not found. -EACCES if the vendor-specific storage could not
 *	   be read. Other error values indicate that the data from the
 *	   vendor-specific storage was found but could not be understood.
 */
#[no_mangle]
pub unsafe extern "C" fn cs_amp_get_vendor_spkid(dev: *mut device) -> c_int {
    let mut i: c_int;
    let ret: c_int;

    if !efi_rt_services_supported(EFI_RT_SUPPORTED_GET_VARIABLE)
        && !IS_ENABLED(CONFIG_SND_SOC_CS_AMP_LIB_TEST_HOOKS)
    {
        return -ENOENT;
    }

    i = 0;
    while i < cs_amp_spkid_byte_types.len() as c_int {
        ret = cs_amp_get_efi_byte_spkid(dev, &raw const cs_amp_spkid_byte_types[i as usize]);
        if ret != -ENOENT {
            return ret;
        }
        i += 1;
    }

    -ENOENT
}

unsafe fn cs_amp_devm_get_dell_ssidex(
    dev: *mut device,
    _ssid_vendor: c_int,
    _ssid_device: c_int,
) -> *const c_char {
    let mut hex_prefix: c_uint = 0;
    let mut audio_id: [c_char; 4] = [0; 4];
    let mut delim: c_char = 0;
    let p: *mut c_char;
    let ret: c_int;

    if !efi_rt_services_supported(EFI_RT_SUPPORTED_GET_VARIABLE)
        && !IS_ENABLED(CONFIG_SND_SOC_CS_AMP_LIB_TEST_HOOKS)
    {
        return ERR_PTR(-ENOENT);
    }

    let ssidex_buf = cs_amp_alloc_get_efi_variable(
        DELL_SSIDEXV2_EFI_NAME.as_ptr() as *mut efi_char16_t,
        &raw mut DELL_SSIDEXV2_EFI_GUID,
        ptr::null_mut(),
    ) as *mut c_char;
    if IS_ERR(ssidex_buf) {
        return ssidex_buf;
    }

    /*
     * SSIDExV2 string is a series of underscore delimited fields.
     * First field is all or part of the SSID. Second field should be
     * a 2-character audio hardware id, followed by other identifiers.
     * Older models did not have the 2-character audio id, so reject
     * the string if the second field is not 2 characters.
     */
    ret = sscanf(
        ssidex_buf,
        c_str!("%8x_%2s%c"),
        &mut hex_prefix,
        audio_id.as_mut_ptr(),
        &mut delim,
    );
    if ret < 2 {
        kfree(ssidex_buf as *mut c_void);
        return ERR_PTR(-ENOENT);
    }

    if ret == 3 && delim != b'_' as c_char {
        kfree(ssidex_buf as *mut c_void);
        return ERR_PTR(-ENOENT);
    }

    if strlen(audio_id.as_ptr()) != 2 {
        kfree(ssidex_buf as *mut c_void);
        return ERR_PTR(-ENOENT);
    }

    p = devm_kstrdup(dev, audio_id.as_ptr(), GFP_KERNEL);
    kfree(ssidex_buf as *mut c_void);
    if p.is_null() {
        return ERR_PTR(-ENOMEM);
    }

    p
}

/**
 * cs_amp_devm_get_vendor_specific_variant_id - get variant ID string
 * @dev:	 pointer to struct device
 * @ssid_vendor: PCI Subsystem Vendor (-1 if unknown)
 * @ssid_device: PCI Subsystem Device (-1 if unknown)
 *
 * Known vendor-specific hardware identifiers are checked and if one is
 * found its content is returned as a NUL-terminated string. The returned
 * string is devm-managed.
 *
 * The returned string is not guaranteed to be globally unique.
 * Generally it should be combined with some other qualifier, such as
 * PCI SSID, to create a globally unique ID.
 *
 * If the caller has a PCI SSID it should pass it in @ssid_vendor and
 * @ssid_device. If the vendor-spefic ID contains this SSID it will be
 * stripped from the returned string to prevent duplication.
 *
 * If the caller does not have a PCI SSID, pass -1 for @ssid_vendor and
 * @ssid_device.
 *
 * Return:
 * * a pointer to a devm-managed string
 * * ERR_PTR(-ENOENT) if no vendor-specific qualifier
 * * ERR_PTR error value
 */
#[no_mangle]
pub unsafe extern "C" fn cs_amp_devm_get_vendor_specific_variant_id(
    dev: *mut device,
    ssid_vendor: c_int,
    ssid_device: c_int,
) -> *const c_char {
    if ssid_vendor == PCI_VENDOR_ID_DELL || ssid_vendor < 0 {
        return cs_amp_devm_get_dell_ssidex(dev, ssid_vendor, ssid_device);
    }

    ERR_PTR(-ENOENT)
}

/**
 * cs_amp_create_debugfs - create a debugfs directory for a device
 *
 * @dev: pointer to struct device
 *
 * Creates a node under "cirrus_logic" in the root of the debugfs filesystem.
 * This is for Cirrus-specific debugfs functionality to be grouped in a
 * defined way, independently of the debugfs provided by ALSA/ASoC.
 * The general ALSA/ASoC debugfs may not be enabled, and does not necessarily
 * have a stable layout or naming convention.
 *
 * Return: Pointer to the dentry for the created directory, or -ENODEV.
 */
#[no_mangle]
pub unsafe extern "C" fn cs_amp_create_debugfs(dev: *mut device) -> *mut dentry {
    let mut dir: *mut dentry;
    let created: *mut dentry;

    /* debugfs_lookup() can return NULL or ERR_PTR on error */
    dir = debugfs_lookup(c_str!("cirrus_logic"), ptr::null_mut());
    if !IS_ERR_OR_NULL(dir) {
        created = debugfs_create_dir(dev_name(dev), dir);
        dput(dir);

        return created;
    }

    dir = debugfs_create_dir(c_str!("cirrus_logic"), ptr::null_mut());

    debugfs_create_dir(dev_name(dev), dir)
}

static cs_amp_test_hook_ptrs: cs_amp_test_hooks = cs_amp_test_hooks {
    get_efi_variable: cs_amp_get_efi_variable,
    set_efi_variable: cs_amp_set_efi_variable,
    write_cal_coeff: cs_amp_write_cal_coeff,
    read_cal_coeff: cs_amp_read_cal_coeff,
};

#[no_mangle]
pub static cs_amp_test_hooks: *const cs_amp_test_hooks = if CONFIG_SND_SOC_CS_AMP_LIB_TEST_HOOKS {
    &cs_amp_test_hook_ptrs
} else {
    ptr::null()
};

/* MODULE_DESCRIPTION("Cirrus Logic amplifier library"); */
/* MODULE_AUTHOR("Richard Fitzgerald <rf@opensource.cirrus.com>"); */
/* MODULE_LICENSE("GPL"); */
/* MODULE_IMPORT_NS("FW_CS_DSP"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
