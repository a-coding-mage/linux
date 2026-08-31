// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * AMD Platform Management Framework Test Tool
 *
 * Copyright (c) 2026, Advanced Micro Devices, Inc.
 * All Rights Reserved.
 *
 * Authors: Shyam Sundar S K <Shyam-sundar.S-k@amd.com>
 *          Sanket Goswami <Sanket.Goswami@amd.com>
 */

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::mem::size_of;

type __u32 = u32;

/* From <linux/amd-pmf.h>; provided by the target repository headers/bindings. */
const AMD_PMF_BIOS_PARAMS_MAX: usize = 11;

const DEVICE_NODE: &[u8] = b"/dev/amdpmf_interface\0";

/* Feature flag names */
static FEATURE_NAMES: [&[u8]; 5] = [
    b"Auto Mode\0",
    b"Static Power Slider\0",
    b"Policy Builder (Smart PC)\0",
    b"Dynamic Power Slider AC\0",
    b"Dynamic Power Slider DC\0",
];

static BANNER: &[u8] = b"====================================================\n\
      AMD PMF Metrics info and Feature Status\n\
====================================================\n\n\0";

#[repr(C)]
pub struct amd_pmf_info {
    pub size: usize,
    pub features_supported: u32,
    pub platform_type: u32,
    pub laptop_placement: u32,
    pub lid_state: u32,
    pub user_presence: u32,
    pub slider_position: u32,
    pub skin_temp: i32,
    pub gfx_busy: u32,
    pub ambient_light: i32,
    pub avg_c0_residency: u32,
    pub max_c0_residency: u32,
    pub socket_power: u32,
    pub bios_input: [__u32; AMD_PMF_BIOS_PARAMS_MAX],
    pub bios_output: [__u32; AMD_PMF_BIOS_PARAMS_MAX],
}

unsafe extern "C" {
    static IOCTL_AMD_PMF_POPULATE_DATA: c_ulong;
    static mut errno: c_int;

    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn printf(format: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;

    static mut stderr: *mut FILE;

    fn amd_pmf_get_platform_type(platform_type: u32) -> *const c_char;
    fn amd_pmf_get_laptop_placement(laptop_placement: u32) -> *const c_char;
    fn amd_pmf_get_slider_position(slider_position: u32) -> *const c_char;
}

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

const O_RDONLY: c_int = 0;

/* Print feature flags */
unsafe fn pmf_print_features(flags: u32) {
    let mut i: usize = 0;

    while i < FEATURE_NAMES.len() {
        printf(
            b"  [%c] %s\n\0".as_ptr() as *const c_char,
            if (flags & (1u32 << i)) != 0 {
                b'x' as c_int
            } else {
                b' ' as c_int
            },
            FEATURE_NAMES[i].as_ptr() as *const c_char,
        );
        i += 1;
    }
}

/* Print BIOS parameters */
unsafe fn pmf_print_bios_params(type_: *const c_char, params: *const __u32) {
    let mut i: c_int = 0;

    while i < AMD_PMF_BIOS_PARAMS_MAX as c_int {
        printf(
            b"  Custom BIOS %s%d: %u\n\0".as_ptr() as *const c_char,
            type_,
            i + 1,
            *params.add(i as usize),
        );
        i += 1;
    }
}

/* Open the PMF device */
unsafe fn pmf_open_device() -> c_int {
    let fd: c_int;

    fd = open(DEVICE_NODE.as_ptr() as *const c_char, O_RDONLY);
    if fd < 0 {
        fprintf(
            stderr,
            b"Error: Cannot open %s: %s\n\0".as_ptr() as *const c_char,
            DEVICE_NODE.as_ptr() as *const c_char,
            strerror(errno),
        );
    }

    fd
}

/* Query PMF info using the single IOCTL */
unsafe fn pmf_get_info(fd: c_int, info: *mut amd_pmf_info) -> c_int {
    let ret: c_int;

    /* Zero-initialize and set size for versioning */
    memset(
        info as *mut c_void,
        0,
        size_of::<amd_pmf_info>(),
    );
    (*info).size = size_of::<amd_pmf_info>();

    ret = ioctl(fd, IOCTL_AMD_PMF_POPULATE_DATA, info);
    if ret < 0 {
        fprintf(
            stderr,
            b"Error: IOCTL_AMD_PMF_POPULATE_DATA failed: %s\n\0".as_ptr() as *const c_char,
            strerror(errno),
        );
        return ret;
    }

    0
}

unsafe fn pmf_print_info(info: *const amd_pmf_info) {
    printf(b"%s\0".as_ptr() as *const c_char, BANNER.as_ptr() as *const c_char);

    /* Feature status */
    printf(b"Feature Status:\n\0".as_ptr() as *const c_char);
    pmf_print_features((*info).features_supported);

    /* Device states */
    printf(b"\nDevice States:\n\0".as_ptr() as *const c_char);
    printf(
        b"  Platform Type:     %s\n\0".as_ptr() as *const c_char,
        amd_pmf_get_platform_type((*info).platform_type),
    );
    printf(
        b"  Laptop Placement:  %s\n\0".as_ptr() as *const c_char,
        amd_pmf_get_laptop_placement((*info).laptop_placement),
    );
    printf(
        b"  Lid State:         %s\n\0".as_ptr() as *const c_char,
        if (*info).lid_state != 0 {
            b"Closed\0".as_ptr() as *const c_char
        } else {
            b"Open\0".as_ptr() as *const c_char
        },
    );
    printf(
        b"  User Presence:     %s\n\0".as_ptr() as *const c_char,
        if (*info).user_presence != 0 {
            b"Present\0".as_ptr() as *const c_char
        } else {
            b"Away\0".as_ptr() as *const c_char
        },
    );
    printf(
        b"  Slider Position:   %s\n\0".as_ptr() as *const c_char,
        amd_pmf_get_slider_position((*info).slider_position),
    );

    /* Thermal and power metrics */
    printf(b"\nThermal/Power Metrics:\n\0".as_ptr() as *const c_char);
    printf(
        b"  Skin Temperature:  %d\n\0".as_ptr() as *const c_char,
        (*info).skin_temp / 100,
    );
    printf(
        b"  GFX Busy:          %u\n\0".as_ptr() as *const c_char,
        (*info).gfx_busy,
    );
    printf(
        b"  Ambient Light:     %d\n\0".as_ptr() as *const c_char,
        (*info).ambient_light,
    );
    printf(
        b"  Avg C0 Residency:  %u\n\0".as_ptr() as *const c_char,
        (*info).avg_c0_residency,
    );
    printf(
        b"  Max C0 Residency:  %u\n\0".as_ptr() as *const c_char,
        (*info).max_c0_residency,
    );
    printf(
        b"  Socket Power:      %u\n\0".as_ptr() as *const c_char,
        (*info).socket_power,
    );

    /* BIOS parameters */
    printf(b"\nCustom BIOS Input Parameters:\n\0".as_ptr() as *const c_char);
    pmf_print_bios_params(
        b"Input\0".as_ptr() as *const c_char,
        (*info).bios_input.as_ptr(),
    );
    printf(b"\nCustom BIOS Output Parameters:\n\0".as_ptr() as *const c_char);
    pmf_print_bios_params(
        b"Output\0".as_ptr() as *const c_char,
        (*info).bios_output.as_ptr(),
    );

    printf(b"\n=================================================\n\0".as_ptr() as *const c_char);
}

fn main() -> c_int {
    unsafe {
        let mut info: amd_pmf_info = core::mem::zeroed();
        let fd: c_int;
        let ret: c_int;

        fd = pmf_open_device();
        if fd < 0 {
            return -1;
        }

        /* Query all info with single IOCTL */
        ret = pmf_get_info(fd, &mut info);
        close(fd);

        if ret < 0 {
            return -1;
        }

        pmf_print_info(&info);

        0
    }
}
