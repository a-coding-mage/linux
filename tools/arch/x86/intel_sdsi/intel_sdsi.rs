// SPDX-License-Identifier: GPL-2.0
/*
 * sdsi: Intel On Demand (formerly Software Defined Silicon) tool for
 * provisioning certificates and activation payloads on supported cpus.
 *
 * See https://github.com/intel/intel-sdsi/blob/master/os-interface.rst
 * for register descriptions.
 *
 * Copyright (C) 2022 Intel Corporation. All rights reserved.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

type size_t = usize;
type ssize_t = isize;

#[repr(C)]
struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
struct DIR {
    _private: [u8; 0],
}

#[repr(C)]
struct dirent {
    d_ino: c_ulong,
    d_off: c_long,
    d_reclen: u16,
    d_type: u8,
    d_name: [c_char; 256],
}

#[repr(C)]
struct option {
    name: *const c_char,
    has_arg: c_int,
    flag: *mut c_int,
    val: c_int,
}

unsafe extern "C" {
    static mut optarg: *mut c_char;
    static mut errno: c_int;

    fn opendir(name: *const c_char) -> *mut DIR;
    fn readdir(dirp: *mut DIR) -> *mut dirent;
    fn chdir(path: *const c_char) -> c_int;
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fclose(stream: *mut FILE) -> c_int;
    fn fread(ptr: *mut c_void, size: size_t, nmemb: size_t, stream: *mut FILE) -> size_t;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn perror(s: *const c_char);
    fn strlen(s: *const c_char) -> size_t;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn fgets(s: *mut c_char, size: c_int, stream: *mut FILE) -> *mut c_char;
    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn snprintf(s: *mut c_char, n: size_t, format: *const c_char, ...) -> c_int;
    fn getopt_long_only(
        argc: c_int,
        argv: *mut *mut c_char,
        optstring: *const c_char,
        longopts: *const option,
        longindex: *mut c_int,
    ) -> c_int;
    fn access(pathname: *const c_char, mode: c_int) -> c_int;
    fn realpath(path: *const c_char, resolved_path: *mut c_char) -> *mut c_char;

    static mut stderr: *mut FILE;
}

const O_RDONLY: c_int = 0;
const O_WRONLY: c_int = 1;
const F_OK: c_int = 0;
const required_argument: c_int = 1;
const no_argument: c_int = 0;
const PATH_MAX: usize = 4096;

const SDSI_DEV: &[u8] = b"intel_vsec.sdsi\0";
const AUX_DEV_PATH: &[u8] = b"/sys/bus/auxiliary/devices/\0";
const GUID_V1: u32 = 0x6dd191;
const REGS_SIZE_GUID_V1: usize = 72;
const GUID_V2: u32 = 0xF210D9EF;
const REGS_SIZE_GUID_V2: usize = 80;
const STATE_CERT_MAX_SIZE: usize = 4096;
const METER_CERT_MAX_SIZE: usize = 4096;
const STATE_MAX_NUM_LICENSES: u32 = 16;
const STATE_MAX_NUM_IN_BUNDLE: u32 = 8;
const FEAT_LEN: usize = 5; /* 4 plus NUL terminator */

fn round_up(x: usize, y: usize) -> usize {
    (((x) - 1) | ((y) - 1)) + 1
}

#[repr(C)]
#[derive(Copy, Clone)]
struct nvram_content_auth_err_sts {
    bits: u64,
}

impl nvram_content_auth_err_sts {
    unsafe fn sdsi_content_auth_err(&self) -> u64 {
        (self.bits >> 3) & 0x1
    }
    unsafe fn sdsi_metering_auth_err(&self) -> u64 {
        (self.bits >> 5) & 0x1
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
struct enabled_features {
    bits: u64,
}

impl enabled_features {
    unsafe fn sdsi(&self) -> u64 {
        (self.bits >> 3) & 0x1
    }
    unsafe fn attestation(&self) -> u64 {
        (self.bits >> 12) & 0x1
    }
    unsafe fn metering(&self) -> u64 {
        (self.bits >> 26) & 0x1
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
struct key_provision_status {
    bits: u64,
}

impl key_provision_status {
    unsafe fn license_key_provisioned(&self) -> u64 {
        (self.bits >> 1) & 0x1
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
struct auth_fail_count {
    bits: u64,
}

impl auth_fail_count {
    unsafe fn key_failure_count(&self) -> u64 {
        self.bits & 0x7
    }
    unsafe fn key_failure_threshold(&self) -> u64 {
        (self.bits >> 3) & 0x7
    }
    unsafe fn auth_failure_count(&self) -> u64 {
        (self.bits >> 6) & 0x7
    }
    unsafe fn auth_failure_threshold(&self) -> u64 {
        (self.bits >> 9) & 0x7
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
struct availability {
    bits: u64,
}

impl availability {
    unsafe fn available(&self) -> u64 {
        (self.bits >> 48) & 0x7
    }
    unsafe fn threshold(&self) -> u64 {
        (self.bits >> 51) & 0x7
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
struct nvram_update_limit {
    bits: u64,
}

impl nvram_update_limit {
    unsafe fn sdsi_50_pct(&self) -> u64 {
        (self.bits >> 12) & 0x1
    }
    unsafe fn sdsi_75_pct(&self) -> u64 {
        (self.bits >> 13) & 0x1
    }
    unsafe fn sdsi_90_pct(&self) -> u64 {
        (self.bits >> 14) & 0x1
    }
}

#[repr(C)]
union sdsi_regs_extra {
    v1: sdsi_regs_extra_v1,
    v2: sdsi_regs_extra_v2,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct sdsi_regs_extra_v1 {
    socket_id: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct sdsi_regs_extra_v2 {
    reserved: u64,
    socket_id: u64,
    reserved2: u64,
}

#[repr(C)]
struct sdsi_regs {
    ppin: u64,
    auth_err_sts: nvram_content_auth_err_sts,
    en_features: enabled_features,
    key_prov_sts: key_provision_status,
    auth_fail_count: auth_fail_count,
    prov_avail: availability,
    limits: nvram_update_limit,
    pcu_cr3_capid_cfg: u64,
    extra: sdsi_regs_extra,
}

const CONTENT_TYPE_LK_ENC: u32 = 0xD;
const CONTENT_TYPE_LK_BLOB_ENC: u32 = 0xE;

#[repr(C)]
struct state_certificate {
    content_type: u32,
    region_rev_id: u32,
    header_size: u32,
    total_size: u32,
    key_size: u32,
    num_licenses: u32,
}

#[repr(C, packed)]
struct license_key_info {
    key_rev_id: u32,
    key_image_content: [u64; 6],
}

fn LICENSE_BLOB_SIZE(l: u32) -> u32 {
    ((l) & 0x7fffffff).wrapping_mul(4)
}

fn LICENSE_VALID(l: u32) -> bool {
    ((l) & 0x80000000) != 0
}

// License Group Types
const LBT_ONE_TIME_UPGRADE: u32 = 1;
const LBT_METERED_UPGRADE: u32 = 2;

#[repr(C, packed)]
struct license_blob_content {
    type_: u32,
    id: u64,
    ppin: u64,
    previous_ppin: u64,
    rev_id: u32,
    num_bundles: u32,
}

#[repr(C)]
struct bundle_encoding {
    encoding: u32,
    encoding_rsvd: [u32; 7],
}

#[repr(C)]
struct meter_certificate {
    signature: u32,
    version: u32,
    ppin: u64,
    counter_unit: u32,
    bundle_length: u32,
    reserved: u64,
    mmrc_encoding: u32,
    mmrc_counter: u32,
}

#[repr(C)]
struct bundle_encoding_counter {
    encoding: u32,
    counter: u32,
}

const METER_BUNDLE_SIZE: usize = size_of::<bundle_encoding_counter>();
const METER_MAX_NUM_BUNDLES: usize =
    (METER_CERT_MAX_SIZE - size_of::<meter_certificate>()) / size_of::<bundle_encoding_counter>();

fn BUNDLE_COUNT(length: u32) -> usize {
    (length as usize) / METER_BUNDLE_SIZE
}

#[repr(C)]
struct sdsi_dev {
    regs: sdsi_regs,
    sc: state_certificate,
    dev_name: *mut c_char,
    dev_path: *mut c_char,
    guid: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum command {
    CMD_SOCKET_INFO,
    CMD_METER_CERT,
    CMD_METER_CURRENT_CERT,
    CMD_STATE_CERT,
    CMD_PROV_AKC,
    CMD_PROV_CAP,
}

unsafe fn sdsi_list_devices() {
    let mut entry: *mut dirent;
    let aux_dir: *mut DIR;
    let mut found: bool = false;

    aux_dir = opendir(AUX_DEV_PATH.as_ptr() as *const c_char);
    if aux_dir.is_null() {
        fprintf(stderr, b"Cannot open directory %s\n\0".as_ptr() as *const c_char, AUX_DEV_PATH.as_ptr());
        return;
    }

    loop {
        entry = readdir(aux_dir);
        if entry.is_null() {
            break;
        }
        if strncmp(
            SDSI_DEV.as_ptr() as *const c_char,
            (*entry).d_name.as_ptr(),
            strlen(SDSI_DEV.as_ptr() as *const c_char),
        ) == 0
        {
            found = true;
            printf(b"%s\n\0".as_ptr() as *const c_char, (*entry).d_name.as_ptr());
        }
    }

    if !found {
        fprintf(stderr, b"No On Demand devices found.\n\0".as_ptr() as *const c_char);
    }
}

unsafe fn sdsi_update_registers(s: *mut sdsi_dev) -> c_int {
    let regs_ptr: *mut FILE;
    let mut ret: c_int;

    ptr::write_bytes(&mut (*s).regs as *mut sdsi_regs as *mut u8, 0, size_of::<sdsi_regs>());

    /* Open the registers file */
    ret = chdir((*s).dev_path);
    if ret == -1 {
        perror(b"chdir\0".as_ptr() as *const c_char);
        return ret;
    }

    regs_ptr = fopen(b"registers\0".as_ptr() as *const c_char, b"r\0".as_ptr() as *const c_char);
    if regs_ptr.is_null() {
        perror(b"Could not open 'registers' file\0".as_ptr() as *const c_char);
        return -1;
    }

    if (*s).guid != GUID_V1 && (*s).guid != GUID_V2 {
        fprintf(stderr, b"Unrecognized guid, 0x%x\n\0".as_ptr() as *const c_char, (*s).guid);
        fclose(regs_ptr);
        return -1;
    }

    /* Update register info for this guid */
    ret = fread(
        &mut (*s).regs as *mut sdsi_regs as *mut c_void,
        size_of::<u8>(),
        size_of::<sdsi_regs>(),
        regs_ptr,
    ) as c_int;
    if ((*s).guid == GUID_V1 && ret != REGS_SIZE_GUID_V1 as c_int)
        || ((*s).guid == GUID_V2 && ret != REGS_SIZE_GUID_V2 as c_int)
    {
        fprintf(stderr, b"Could not read 'registers' file\n\0".as_ptr() as *const c_char);
        fclose(regs_ptr);
        return -1;
    }

    fclose(regs_ptr);

    0
}

unsafe fn sdsi_read_reg(s: *mut sdsi_dev) -> c_int {
    let mut ret: c_int;

    ret = sdsi_update_registers(s);
    if ret != 0 {
        return ret;
    }

    /* Print register info for this guid */
    printf(b"\n\0".as_ptr() as *const c_char);
    printf(b"Socket information for device %s\n\0".as_ptr() as *const c_char, (*s).dev_name);
    printf(b"\n\0".as_ptr() as *const c_char);
    printf(b"PPIN:                           0x%lx\n\0".as_ptr() as *const c_char, (*s).regs.ppin);
    printf(b"NVRAM Content Authorization Error Status\n\0".as_ptr() as *const c_char);
    printf(
        b"    SDSi Auth Err Sts:          %s\n\0".as_ptr() as *const c_char,
        if (*s).regs.auth_err_sts.sdsi_content_auth_err() != 0 { b"Error\0".as_ptr() } else { b"Okay\0".as_ptr() },
    );

    if (*s).regs.en_features.metering() != 0 {
        printf(
            b"    Metering Auth Err Sts:      %s\n\0".as_ptr() as *const c_char,
            if (*s).regs.auth_err_sts.sdsi_metering_auth_err() != 0 { b"Error\0".as_ptr() } else { b"Okay\0".as_ptr() },
        );
    }

    printf(b"Enabled Features\n\0".as_ptr() as *const c_char);
    printf(b"    On Demand:                  %s\n\0".as_ptr() as *const c_char, if (*s).regs.en_features.sdsi() != 0 { b"Enabled\0".as_ptr() } else { b"Disabled\0".as_ptr() });
    printf(b"    Attestation:                %s\n\0".as_ptr() as *const c_char, if (*s).regs.en_features.attestation() != 0 { b"Enabled\0".as_ptr() } else { b"Disabled\0".as_ptr() });
    printf(b"    On Demand:                  %s\n\0".as_ptr() as *const c_char, if (*s).regs.en_features.sdsi() != 0 { b"Enabled\0".as_ptr() } else { b"Disabled\0".as_ptr() });
    printf(b"    Metering:                   %s\n\0".as_ptr() as *const c_char, if (*s).regs.en_features.metering() != 0 { b"Enabled\0".as_ptr() } else { b"Disabled\0".as_ptr() });
    printf(b"License Key (AKC) Provisioned:  %s\n\0".as_ptr() as *const c_char, if (*s).regs.key_prov_sts.license_key_provisioned() != 0 { b"Yes\0".as_ptr() } else { b"No\0".as_ptr() });
    printf(b"Authorization Failure Count\n\0".as_ptr() as *const c_char);
    printf(b"    AKC Failure Count:          %d\n\0".as_ptr() as *const c_char, (*s).regs.auth_fail_count.key_failure_count() as c_int);
    printf(b"    AKC Failure Threshold:      %d\n\0".as_ptr() as *const c_char, (*s).regs.auth_fail_count.key_failure_threshold() as c_int);
    printf(b"    CAP Failure Count:          %d\n\0".as_ptr() as *const c_char, (*s).regs.auth_fail_count.auth_failure_count() as c_int);
    printf(b"    CAP Failure Threshold:      %d\n\0".as_ptr() as *const c_char, (*s).regs.auth_fail_count.auth_failure_threshold() as c_int);
    printf(b"Provisioning Availability\n\0".as_ptr() as *const c_char);
    printf(b"    Updates Available:          %d\n\0".as_ptr() as *const c_char, (*s).regs.prov_avail.available() as c_int);
    printf(b"    Updates Threshold:          %d\n\0".as_ptr() as *const c_char, (*s).regs.prov_avail.threshold() as c_int);
    printf(b"NVRAM Udate Limit\n\0".as_ptr() as *const c_char);
    printf(b"    50%% Limit Reached:          %s\n\0".as_ptr() as *const c_char, if (*s).regs.limits.sdsi_50_pct() != 0 { b"Yes\0".as_ptr() } else { b"No\0".as_ptr() });
    printf(b"    75%% Limit Reached:          %s\n\0".as_ptr() as *const c_char, if (*s).regs.limits.sdsi_75_pct() != 0 { b"Yes\0".as_ptr() } else { b"No\0".as_ptr() });
    printf(b"    90%% Limit Reached:          %s\n\0".as_ptr() as *const c_char, if (*s).regs.limits.sdsi_90_pct() != 0 { b"Yes\0".as_ptr() } else { b"No\0".as_ptr() });
    if (*s).guid == GUID_V1 {
        printf(b"Socket ID:                      %ld\n\0".as_ptr() as *const c_char, ((*s).regs.extra.v1.socket_id & 0xF) as c_long);
    } else {
        printf(b"Socket ID:                      %ld\n\0".as_ptr() as *const c_char, ((*s).regs.extra.v2.socket_id & 0xF) as c_long);
    }

    0
}

unsafe fn license_blob_type(type_: u32) -> *const c_char {
    match type_ {
        LBT_ONE_TIME_UPGRADE => b"One time upgrade\0".as_ptr() as *const c_char,
        LBT_METERED_UPGRADE => b"Metered upgrade\0".as_ptr() as *const c_char,
        _ => b"Unknown license blob type\0".as_ptr() as *const c_char,
    }
}

unsafe fn content_type(type_: u32) -> *const c_char {
    match type_ {
        CONTENT_TYPE_LK_ENC => b"Licencse key encoding\0".as_ptr() as *const c_char,
        CONTENT_TYPE_LK_BLOB_ENC => b"License key + Blob encoding\0".as_ptr() as *const c_char,
        _ => b"Unknown content type\0".as_ptr() as *const c_char,
    }
}

unsafe fn get_feature(encoding: u32, feature: *mut c_char) {
    let name = &encoding as *const u32 as *const c_char;

    *feature.add(4) = 0;
    *feature.add(3) = *name.add(0);
    *feature.add(2) = *name.add(1);
    *feature.add(1) = *name.add(2);
    *feature.add(0) = *name.add(3);
}

unsafe fn sdsi_meter_cert_show(s: *mut sdsi_dev, show_current: bool) -> c_int {
    let mut buf = [0 as c_char; METER_CERT_MAX_SIZE];
    let mut bec: *mut bundle_encoding_counter;
    let mc: *mut meter_certificate;
    let mut count: u32 = 0;
    let cert_ptr: *mut FILE;
    let cert_fname: *const c_char;
    let mut ret: c_int;
    let size: c_int;
    let mut name = [0 as c_char; FEAT_LEN];

    ret = sdsi_update_registers(s);
    if ret != 0 {
        return ret;
    }

    if (*s).regs.en_features.sdsi() == 0 {
        fprintf(stderr, b"SDSi feature is present but not enabled.\n\0".as_ptr() as *const c_char);
        return -1;
    }

    if (*s).regs.en_features.metering() == 0 {
        fprintf(stderr, b"Metering not supporting on this socket.\n\0".as_ptr() as *const c_char);
        return -1;
    }

    ret = chdir((*s).dev_path);
    if ret == -1 {
        perror(b"chdir\0".as_ptr() as *const c_char);
        return ret;
    }

    cert_fname = if show_current { b"meter_current\0".as_ptr() } else { b"meter_certificate\0".as_ptr() } as *const c_char;
    cert_ptr = fopen(cert_fname, b"r\0".as_ptr() as *const c_char);

    if cert_ptr.is_null() {
        fprintf(stderr, b"Could not open '%s' file: %s\0".as_ptr() as *const c_char, cert_fname, strerror(errno));
        return -1;
    }

    size = fread(buf.as_mut_ptr() as *mut c_void, 1, buf.len(), cert_ptr) as c_int;
    if size == 0 {
        fprintf(stderr, b"Could not read '%s' file\n\0".as_ptr() as *const c_char, cert_fname);
        fclose(cert_ptr);
        return -1;
    }
    fclose(cert_ptr);

    mc = buf.as_mut_ptr() as *mut meter_certificate;

    printf(b"\n\0".as_ptr() as *const c_char);
    printf(b"Meter certificate for device %s\n\0".as_ptr() as *const c_char, (*s).dev_name);
    printf(b"\n\0".as_ptr() as *const c_char);

    get_feature((*mc).signature, name.as_mut_ptr());
    printf(b"Signature:                    %s\n\0".as_ptr() as *const c_char, name.as_ptr());

    printf(b"Version:                      %d\n\0".as_ptr() as *const c_char, (*mc).version);
    printf(b"Count Unit:                   %dms\n\0".as_ptr() as *const c_char, (*mc).counter_unit);
    printf(b"PPIN:                         0x%lx\n\0".as_ptr() as *const c_char, (*mc).ppin);
    printf(b"Feature Bundle Length:        %d\n\0".as_ptr() as *const c_char, (*mc).bundle_length);

    get_feature((*mc).mmrc_encoding, name.as_mut_ptr());
    printf(b"MMRC encoding:                %s\n\0".as_ptr() as *const c_char, name.as_ptr());

    printf(b"MMRC counter:                 %d\n\0".as_ptr() as *const c_char, (*mc).mmrc_counter);
    if ((*mc).bundle_length as usize) % METER_BUNDLE_SIZE != 0 {
        fprintf(stderr, b"Invalid bundle length\n\0".as_ptr() as *const c_char);
        return -1;
    }

    if ((*mc).bundle_length as usize) > METER_MAX_NUM_BUNDLES * METER_BUNDLE_SIZE {
        fprintf(
            stderr,
            b"More than %ld bundles: actual %ld\n\0".as_ptr() as *const c_char,
            METER_MAX_NUM_BUNDLES as c_long,
            BUNDLE_COUNT((*mc).bundle_length) as c_long,
        );
        return -1;
    }

    bec = mc.add(1) as *mut bundle_encoding_counter;

    printf(b"Number of Feature Counters:   %ld\n\0".as_ptr() as *const c_char, BUNDLE_COUNT((*mc).bundle_length) as c_long);
    while (count as usize) < BUNDLE_COUNT((*mc).bundle_length) {
        let mut feature = [0 as c_char; FEAT_LEN];

        get_feature((*bec.add(count as usize)).encoding, feature.as_mut_ptr());
        printf(b"    %s:          %d\n\0".as_ptr() as *const c_char, feature.as_ptr(), (*bec.add(count as usize)).counter);
        count = count.wrapping_add(1);
    }

    0
}

unsafe fn sdsi_state_cert_show(s: *mut sdsi_dev) -> c_int {
    let mut buf = [0 as c_char; STATE_CERT_MAX_SIZE];
    let sc: *mut state_certificate;
    let lki: *mut license_key_info;
    let mut offset: u32 = 0;
    let mut count: u32 = 0;
    let cert_ptr: *mut FILE;
    let mut ret: c_int;
    let size: c_int;

    ret = sdsi_update_registers(s);
    if ret != 0 {
        return ret;
    }

    if (*s).regs.en_features.sdsi() == 0 {
        fprintf(stderr, b"On Demand feature is present but not enabled.\0".as_ptr() as *const c_char);
        fprintf(stderr, b" Unable to read state certificate\0".as_ptr() as *const c_char);
        return -1;
    }

    ret = chdir((*s).dev_path);
    if ret == -1 {
        perror(b"chdir\0".as_ptr() as *const c_char);
        return ret;
    }

    cert_ptr = fopen(b"state_certificate\0".as_ptr() as *const c_char, b"r\0".as_ptr() as *const c_char);
    if cert_ptr.is_null() {
        perror(b"Could not open 'state_certificate' file\0".as_ptr() as *const c_char);
        return -1;
    }

    size = fread(buf.as_mut_ptr() as *mut c_void, 1, buf.len(), cert_ptr) as c_int;
    if size == 0 {
        fprintf(stderr, b"Could not read 'state_certificate' file\n\0".as_ptr() as *const c_char);
        fclose(cert_ptr);
        return -1;
    }
    fclose(cert_ptr);

    sc = buf.as_mut_ptr() as *mut state_certificate;

    /* Print register info for this guid */
    printf(b"\n\0".as_ptr() as *const c_char);
    printf(b"State certificate for device %s\n\0".as_ptr() as *const c_char, (*s).dev_name);
    printf(b"\n\0".as_ptr() as *const c_char);
    printf(b"Content Type:          %s\n\0".as_ptr() as *const c_char, content_type((*sc).content_type));
    printf(b"Region Revision ID:    %d\n\0".as_ptr() as *const c_char, (*sc).region_rev_id);
    printf(b"Header Size:           %d\n\0".as_ptr() as *const c_char, (*sc).header_size.wrapping_mul(4));
    printf(b"Total Size:            %d\n\0".as_ptr() as *const c_char, (*sc).total_size);
    printf(b"OEM Key Size:          %d\n\0".as_ptr() as *const c_char, (*sc).key_size.wrapping_mul(4));
    printf(b"Number of Licenses:    %d\n\0".as_ptr() as *const c_char, (*sc).num_licenses);

    /* Skip over the license sizes 4 bytes per license) to get the license key info */
    lki = (sc as *mut c_void as *mut u8)
        .add(size_of::<state_certificate>() + (4 * (*sc).num_licenses as usize))
        as *mut license_key_info;

    printf(b"License blob Info:\n\0".as_ptr() as *const c_char);
    printf(b"    License Key Revision ID:    0x%x\n\0".as_ptr() as *const c_char, ptr::addr_of!((*lki).key_rev_id).read_unaligned());
    printf(
        b"    License Key Image Content:  0x%lx%lx%lx%lx%lx%lx\n\0".as_ptr() as *const c_char,
        ptr::addr_of!((*lki).key_image_content[5]).read_unaligned(),
        ptr::addr_of!((*lki).key_image_content[4]).read_unaligned(),
        ptr::addr_of!((*lki).key_image_content[3]).read_unaligned(),
        ptr::addr_of!((*lki).key_image_content[2]).read_unaligned(),
        ptr::addr_of!((*lki).key_image_content[1]).read_unaligned(),
        ptr::addr_of!((*lki).key_image_content[0]).read_unaligned(),
    );

    while {
        count = count.wrapping_add(1);
        count < (*sc).num_licenses
    } {
        let blob_size_field = *(buf.as_ptr().add(0x14 + count as usize * 4) as *const u32);
        let blob_size = LICENSE_BLOB_SIZE(blob_size_field);
        let license_valid = LICENSE_VALID(blob_size_field);
        let lbc = (sc as *mut c_void as *mut u8)
            .add(size_of::<state_certificate>())
            .add(4 * (*sc).num_licenses as usize)
            .add(size_of::<license_key_info>())
            .add(offset as usize) as *mut license_blob_content;
        let bundle = (lbc as *mut c_void as *mut u8).add(size_of::<license_blob_content>()) as *mut bundle_encoding;
        let mut feature = [0 as c_char; FEAT_LEN];
        let mut i: u32;

        printf(b"     Blob %d:\n\0".as_ptr() as *const c_char, count.wrapping_sub(1));
        printf(b"        License blob size:          %u\n\0".as_ptr() as *const c_char, blob_size);
        printf(b"        License is valid:           %s\n\0".as_ptr() as *const c_char, if license_valid { b"Yes\0".as_ptr() } else { b"No\0".as_ptr() });
        printf(b"        License blob type:          %s\n\0".as_ptr() as *const c_char, license_blob_type(ptr::addr_of!((*lbc).type_).read_unaligned()));
        printf(b"        License blob ID:            0x%lx\n\0".as_ptr() as *const c_char, ptr::addr_of!((*lbc).id).read_unaligned());
        printf(b"        PPIN:                       0x%lx\n\0".as_ptr() as *const c_char, ptr::addr_of!((*lbc).ppin).read_unaligned());
        printf(b"        Previous PPIN:              0x%lx\n\0".as_ptr() as *const c_char, ptr::addr_of!((*lbc).previous_ppin).read_unaligned());
        printf(b"        Blob revision ID:           %u\n\0".as_ptr() as *const c_char, ptr::addr_of!((*lbc).rev_id).read_unaligned());
        printf(b"        Number of Features:         %u\n\0".as_ptr() as *const c_char, ptr::addr_of!((*lbc).num_bundles).read_unaligned());

        i = 0;
        while i < core::cmp::min(ptr::addr_of!((*lbc).num_bundles).read_unaligned(), STATE_MAX_NUM_IN_BUNDLE) {
            get_feature((*bundle.add(i as usize)).encoding, feature.as_mut_ptr());
            printf(b"                 Feature %d:         %s\n\0".as_ptr() as *const c_char, i, feature.as_ptr());
            i = i.wrapping_add(1);
        }

        if ptr::addr_of!((*lbc).num_bundles).read_unaligned() > STATE_MAX_NUM_IN_BUNDLE {
            fprintf(
                stderr,
                b"        Warning: %d > %d licenses in bundle reported.\n\0".as_ptr() as *const c_char,
                ptr::addr_of!((*lbc).num_bundles).read_unaligned(),
                STATE_MAX_NUM_IN_BUNDLE,
            );
        }

        offset = offset.wrapping_add(blob_size);
    }

    0
}

unsafe fn sdsi_provision(s: *mut sdsi_dev, bin_file: *mut c_char, command: command) -> c_int {
    let bin_fd: c_int;
    let prov_fd: c_int;
    let size: ssize_t;
    let mut ret: c_int;
    let mut buf = [0 as c_char; STATE_CERT_MAX_SIZE];
    let mut cap = *b"provision_cap\0";
    let mut akc = *b"provision_akc\0";
    let prov_file: *mut c_char;

    if bin_file.is_null() {
        fprintf(stderr, b"No binary file provided\n\0".as_ptr() as *const c_char);
        return -1;
    }

    /* Open the binary */
    bin_fd = open(bin_file, O_RDONLY);
    if bin_fd == -1 {
        fprintf(stderr, b"Could not open file %s: %s\n\0".as_ptr() as *const c_char, bin_file, strerror(errno));
        return bin_fd;
    }

    prov_file = if command == command::CMD_PROV_AKC { akc.as_mut_ptr() as *mut c_char } else { cap.as_mut_ptr() as *mut c_char };

    ret = chdir((*s).dev_path);
    if ret == -1 {
        perror(b"chdir\0".as_ptr() as *const c_char);
        close(bin_fd);
        return ret;
    }

    /* Open the provision file */
    prov_fd = open(prov_file, O_WRONLY);
    if prov_fd == -1 {
        fprintf(stderr, b"Could not open file %s: %s\n\0".as_ptr() as *const c_char, prov_file, strerror(errno));
        close(bin_fd);
        return prov_fd;
    }

    /* Read the binary file into the buffer */
    size = read(bin_fd, buf.as_mut_ptr() as *mut c_void, STATE_CERT_MAX_SIZE);
    if size == -1 {
        close(bin_fd);
        close(prov_fd);
        return -1;
    }

    ret = write(prov_fd, buf.as_ptr() as *const c_void, size as size_t) as c_int;
    if ret == -1 {
        close(bin_fd);
        close(prov_fd);
        perror(b"Provisioning failed\0".as_ptr() as *const c_char);
        return ret;
    }

    printf(b"Provisioned %s file %s successfully\n\0".as_ptr() as *const c_char, prov_file, bin_file);

    close(bin_fd);
    close(prov_fd);

    0
}

unsafe fn sdsi_provision_akc(s: *mut sdsi_dev, bin_file: *mut c_char) -> c_int {
    let mut ret: c_int;

    ret = sdsi_update_registers(s);
    if ret != 0 {
        return ret;
    }

    if (*s).regs.en_features.sdsi() == 0 {
        fprintf(stderr, b"On Demand feature is present but not enabled. Unable to provision\0".as_ptr() as *const c_char);
        return -1;
    }

    if (*s).regs.prov_avail.available() == 0 {
        fprintf(stderr, b"Maximum number of updates (%d) has been reached.\n\0".as_ptr() as *const c_char, (*s).regs.prov_avail.threshold() as c_int);
        return -1;
    }

    if (*s).regs.auth_fail_count.key_failure_count() == (*s).regs.auth_fail_count.key_failure_threshold() {
        fprintf(stderr, b"Maximum number of AKC provision failures (%d) has been reached.\n\0".as_ptr() as *const c_char, (*s).regs.auth_fail_count.key_failure_threshold() as c_int);
        fprintf(stderr, b"Power cycle the system to reset the counter\n\0".as_ptr() as *const c_char);
        return -1;
    }

    sdsi_provision(s, bin_file, command::CMD_PROV_AKC)
}

unsafe fn sdsi_provision_cap(s: *mut sdsi_dev, bin_file: *mut c_char) -> c_int {
    let mut ret: c_int;

    ret = sdsi_update_registers(s);
    if ret != 0 {
        return ret;
    }

    if (*s).regs.en_features.sdsi() == 0 {
        fprintf(stderr, b"On Demand feature is present but not enabled. Unable to provision\0".as_ptr() as *const c_char);
        return -1;
    }

    if (*s).regs.prov_avail.available() == 0 {
        fprintf(stderr, b"Maximum number of updates (%d) has been reached.\n\0".as_ptr() as *const c_char, (*s).regs.prov_avail.threshold() as c_int);
        return -1;
    }

    if (*s).regs.auth_fail_count.auth_failure_count() == (*s).regs.auth_fail_count.auth_failure_threshold() {
        fprintf(stderr, b"Maximum number of CAP provision failures (%d) has been reached.\n\0".as_ptr() as *const c_char, (*s).regs.auth_fail_count.auth_failure_threshold() as c_int);
        fprintf(stderr, b"Power cycle the system to reset the counter\n\0".as_ptr() as *const c_char);
        return -1;
    }

    sdsi_provision(s, bin_file, command::CMD_PROV_CAP)
}

unsafe fn read_sysfs_data(file: *const c_char, value: *mut c_int) -> c_int {
    let mut buff = [0 as c_char; 16];
    let fp: *mut FILE;

    fp = fopen(file, b"r\0".as_ptr() as *const c_char);
    if fp.is_null() {
        perror(file);
        return -1;
    }

    if fgets(buff.as_mut_ptr(), 16, fp).is_null() {
        fprintf(stderr, b"Failed to read file '%s'\0".as_ptr() as *const c_char, file);
        fclose(fp);
        return -1;
    }

    fclose(fp);
    *value = strtol(buff.as_ptr(), ptr::null_mut(), 0) as c_int;

    0
}

unsafe fn sdsi_create_dev(dev_no: *mut c_char) -> *mut sdsi_dev {
    let dev_name_len = size_of_val_cstr(SDSI_DEV.as_ptr() as *const c_char) + strlen(dev_no) + 1;
    let s: *mut sdsi_dev;
    let mut guid: c_int = 0;
    let dir: *mut DIR;

    s = malloc(size_of::<sdsi_dev>()) as *mut sdsi_dev;
    if s.is_null() {
        perror(b"malloc\0".as_ptr() as *const c_char);
        return ptr::null_mut();
    }

    (*s).dev_name = malloc(size_of_val_cstr(SDSI_DEV.as_ptr() as *const c_char) + strlen(dev_no) + 1) as *mut c_char;
    if (*s).dev_name.is_null() {
        perror(b"malloc\0".as_ptr() as *const c_char);
        free(s as *mut c_void);
        return ptr::null_mut();
    }

    snprintf((*s).dev_name, dev_name_len, b"%s.%s\0".as_ptr() as *const c_char, SDSI_DEV.as_ptr(), dev_no);

    (*s).dev_path = malloc(size_of_val_cstr(AUX_DEV_PATH.as_ptr() as *const c_char) + dev_name_len) as *mut c_char;
    if (*s).dev_path.is_null() {
        perror(b"malloc\0".as_ptr() as *const c_char);
        free((*s).dev_name as *mut c_void);
        free(s as *mut c_void);
        return ptr::null_mut();
    }

    snprintf((*s).dev_path, size_of_val_cstr(AUX_DEV_PATH.as_ptr() as *const c_char) + dev_name_len, b"%s%s\0".as_ptr() as *const c_char, AUX_DEV_PATH.as_ptr(), (*s).dev_name);
    dir = opendir((*s).dev_path);
    if dir.is_null() {
        fprintf(stderr, b"Could not open directory '%s': %s\n\0".as_ptr() as *const c_char, (*s).dev_path, strerror(errno));
        free((*s).dev_path as *mut c_void);
        free((*s).dev_name as *mut c_void);
        free(s as *mut c_void);
        return ptr::null_mut();
    }

    if chdir((*s).dev_path) == -1 {
        perror(b"chdir\0".as_ptr() as *const c_char);
        free((*s).dev_path as *mut c_void);
        free((*s).dev_name as *mut c_void);
        free(s as *mut c_void);
        return ptr::null_mut();
    }

    if read_sysfs_data(b"guid\0".as_ptr() as *const c_char, &mut guid) != 0 {
        free((*s).dev_path as *mut c_void);
        free((*s).dev_name as *mut c_void);
        free(s as *mut c_void);
        return ptr::null_mut();
    }

    (*s).guid = guid as u32;

    s
}

unsafe fn size_of_val_cstr(s: *const c_char) -> usize {
    strlen(s) + 1
}

unsafe fn sdsi_free_dev(s: *mut sdsi_dev) {
    free((*s).dev_path as *mut c_void);
    free((*s).dev_name as *mut c_void);
    free(s as *mut c_void);
}

unsafe fn usage(prog: *mut c_char) {
    printf(b"Usage: %s [-l] [-d DEVNO [-i] [-s] [-m | -C] [-a FILE] [-c FILE]\n\0".as_ptr() as *const c_char, prog);
}

unsafe fn show_help() {
    printf(b"Commands:\n\0".as_ptr() as *const c_char);
    printf(b"  %-18s\t%s\n\0".as_ptr() as *const c_char, b"-l, --list\0".as_ptr(), b"list available On Demand devices\0".as_ptr());
    printf(b"  %-18s\t%s\n\0".as_ptr() as *const c_char, b"-d, --devno DEVNO\0".as_ptr(), b"On Demand device number\0".as_ptr());
    printf(b"  %-18s\t%s\n\0".as_ptr() as *const c_char, b"-i, --info\0".as_ptr(), b"show socket information\0".as_ptr());
    printf(b"  %-18s\t%s\n\0".as_ptr() as *const c_char, b"-s, --state\0".as_ptr(), b"show state certificate data\0".as_ptr());
    printf(b"  %-18s\t%s\n\0".as_ptr() as *const c_char, b"-m, --meter\0".as_ptr(), b"show meter certificate data\0".as_ptr());
    printf(b"  %-18s\t%s\n\0".as_ptr() as *const c_char, b"-C, --meter_current\0".as_ptr(), b"show live unattested meter data\0".as_ptr());
    printf(b"  %-18s\t%s\n\0".as_ptr() as *const c_char, b"-a, --akc FILE\0".as_ptr(), b"provision socket with AKC FILE\0".as_ptr());
    printf(b"  %-18s\t%s\n\0".as_ptr() as *const c_char, b"-c, --cap FILE>\0".as_ptr(), b"provision socket with CAP FILE\0".as_ptr());
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut bin_file = [0 as c_char; PATH_MAX];
    let mut dev_no: *mut c_char = ptr::null_mut();
    let mut device_selected: bool = false;
    let progname: *mut c_char;
    let mut command_value: c_int = -1;
    let s: *mut sdsi_dev;
    let mut ret: c_int = 0;
    let mut opt: c_int;
    let mut option_index: c_int = 0;

    let long_options = [
        option { name: b"akc\0".as_ptr() as *const c_char, has_arg: required_argument, flag: ptr::null_mut(), val: b'a' as c_int },
        option { name: b"cap\0".as_ptr() as *const c_char, has_arg: required_argument, flag: ptr::null_mut(), val: b'c' as c_int },
        option { name: b"devno\0".as_ptr() as *const c_char, has_arg: required_argument, flag: ptr::null_mut(), val: b'd' as c_int },
        option { name: b"help\0".as_ptr() as *const c_char, has_arg: no_argument, flag: ptr::null_mut(), val: b'h' as c_int },
        option { name: b"info\0".as_ptr() as *const c_char, has_arg: no_argument, flag: ptr::null_mut(), val: b'i' as c_int },
        option { name: b"list\0".as_ptr() as *const c_char, has_arg: no_argument, flag: ptr::null_mut(), val: b'l' as c_int },
        option { name: b"meter\0".as_ptr() as *const c_char, has_arg: no_argument, flag: ptr::null_mut(), val: b'm' as c_int },
        option { name: b"meter_current\0".as_ptr() as *const c_char, has_arg: no_argument, flag: ptr::null_mut(), val: b'C' as c_int },
        option { name: b"state\0".as_ptr() as *const c_char, has_arg: no_argument, flag: ptr::null_mut(), val: b's' as c_int },
        option { name: ptr::null(), has_arg: 0, flag: ptr::null_mut(), val: 0 },
    ];

    progname = *argv.add(0);

    loop {
        opt = getopt_long_only(
            argc,
            argv,
            b"+a:c:d:hilmCs\0".as_ptr() as *const c_char,
            long_options.as_ptr(),
            &mut option_index,
        );
        if opt == -1 {
            break;
        }
        match opt {
            x if x == b'd' as c_int => {
                dev_no = optarg;
                device_selected = true;
            }
            x if x == b'l' as c_int => {
                sdsi_list_devices();
                return 0;
            }
            x if x == b'i' as c_int => {
                command_value = command::CMD_SOCKET_INFO as c_int;
            }
            x if x == b'm' as c_int => {
                command_value = command::CMD_METER_CERT as c_int;
            }
            x if x == b'C' as c_int => {
                command_value = command::CMD_METER_CURRENT_CERT as c_int;
            }
            x if x == b's' as c_int => {
                command_value = command::CMD_STATE_CERT as c_int;
            }
            x if x == b'a' as c_int || x == b'c' as c_int => {
                if !(access(optarg, F_OK) == 0) {
                    fprintf(stderr, b"Could not open file '%s': %s\n\0".as_ptr() as *const c_char, optarg, strerror(errno));
                    return -1;
                }

                if realpath(optarg, bin_file.as_mut_ptr()).is_null() {
                    perror(b"realpath\0".as_ptr() as *const c_char);
                    return -1;
                }

                command_value = if opt == b'a' as c_int { command::CMD_PROV_AKC as c_int } else { command::CMD_PROV_CAP as c_int };
            }
            x if x == b'h' as c_int => {
                usage(progname);
                show_help();
                return 0;
            }
            _ => {
                usage(progname);
                return -1;
            }
        }
    }

    if device_selected {
        s = sdsi_create_dev(dev_no);
        if s.is_null() {
            return -1;
        }

        if command_value == command::CMD_SOCKET_INFO as c_int {
            ret = sdsi_read_reg(s);
        } else if command_value == command::CMD_METER_CERT as c_int {
            ret = sdsi_meter_cert_show(s, false);
        } else if command_value == command::CMD_METER_CURRENT_CERT as c_int {
            ret = sdsi_meter_cert_show(s, true);
        } else if command_value == command::CMD_STATE_CERT as c_int {
            ret = sdsi_state_cert_show(s);
        } else if command_value == command::CMD_PROV_AKC as c_int {
            ret = sdsi_provision_akc(s, bin_file.as_mut_ptr());
        } else if command_value == command::CMD_PROV_CAP as c_int {
            ret = sdsi_provision_cap(s, bin_file.as_mut_ptr());
        } else {
            fprintf(stderr, b"No command specified\n\0".as_ptr() as *const c_char);
            return -1;
        }

        sdsi_free_dev(s);
    } else {
        fprintf(stderr, b"No device specified\n\0".as_ptr() as *const c_char);
        return -1;
    }

    ret
}
