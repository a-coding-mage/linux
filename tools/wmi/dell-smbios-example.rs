// SPDX-License-Identifier: GPL-2.0-only
/*
 *  Sample application for SMBIOS communication over WMI interface
 *  Performs the following:
 *  - Simple cmd_class/cmd_select lookup for TPM information
 *  - Simple query of known tokens and their values
 *  - Simple activation of a token
 *
 *  Copyright (C) 2017 Dell, Inc.
 */

/*
 * C dependencies translated from:
 *   <errno.h>, <fcntl.h>, <stdio.h>, <stdlib.h>, <sys/ioctl.h>, <unistd.h>
 *   <linux/wmi.h>
 *
 * Constants and structure layout from linux/wmi.h are expected to be supplied by
 * the eventual repository bindings. The field layout below mirrors the fields
 * used by this source file.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};
use core::ptr;

type __u16 = u16;
type __u64 = u64;

const EXIT_SUCCESS: c_int = 0;
const EXIT_FAILURE: c_int = 1;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const O_NONBLOCK: c_int = 0o4000;

/* linux/wmi.h constants; values are provided by the translated dependency. */
extern "C" {
    static DELL_WMI_SMBIOS_CMD: c_ulong;
    static CLASS_TOKEN_READ: __u16;
    static CLASS_TOKEN_WRITE: __u16;
    static CLASS_FLASH_INTERFACE: __u16;
    static SELECT_TOKEN_STD: __u16;
    static SELECT_FLASH_INTERFACE: __u16;
    static CAPSULE_EN_TOKEN: __u16;
    static CAPSULE_DIS_TOKEN: __u16;
}

#[repr(C)]
pub struct dell_wmi_smbios_std_buffer {
    pub cmd_class: __u16,
    pub cmd_select: __u16,
    pub input: [__u32; 4],
    pub output: [__u32; 4],
}

type __u32 = u32;

#[repr(C)]
pub struct dell_wmi_smbios_buffer {
    pub length: __u64,
    pub std: dell_wmi_smbios_std_buffer,
}

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

extern "C" {
    fn printf(format: *const c_char, ...) -> c_int;
    fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int;
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fread(ptr: *mut c_void, size: usize, nmemb: usize, stream: *mut FILE) -> usize;
    fn fclose(stream: *mut FILE) -> c_int;
    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
}

/* It would be better to discover these using udev, but for a simple
 * application they're hardcoded
 */
static ioctl_devfs: *const c_char = b"/dev/wmi/dell-smbios\0".as_ptr() as *const c_char;
static token_sysfs: *const c_char =
    b"/sys/bus/platform/devices/dell-smbios.0/tokens\0".as_ptr() as *const c_char;

unsafe fn show_buffer(buffer: *mut dell_wmi_smbios_buffer) {
    printf(
        b"Call: %x/%x [%x,%x,%x,%x]\nResults: [%8x,%8x,%8x,%8x]\n\0".as_ptr()
            as *const c_char,
        (*buffer).std.cmd_class as c_int,
        (*buffer).std.cmd_select as c_int,
        (*buffer).std.input[0],
        (*buffer).std.input[1],
        (*buffer).std.input[2],
        (*buffer).std.input[3],
        (*buffer).std.output[0],
        (*buffer).std.output[1],
        (*buffer).std.output[2],
        (*buffer).std.output[3],
    );
}

unsafe fn run_wmi_smbios_cmd(buffer: *mut dell_wmi_smbios_buffer) -> c_int {
    let fd: c_int;
    let ret: c_int;

    fd = open(ioctl_devfs, O_NONBLOCK);
    ret = ioctl(fd, DELL_WMI_SMBIOS_CMD, buffer);
    close(fd);
    ret
}

unsafe fn find_token(token: __u16, location: *mut __u16, value: *mut __u16) -> c_int {
    let mut location_sysfs = [0 as c_char; 60];
    let mut value_sysfs = [0 as c_char; 57];
    let mut buf = [0 as c_char; 4096];
    let mut f: *mut FILE;
    let mut ret: c_int;

    ret = sprintf(
        value_sysfs.as_mut_ptr(),
        b"%s/%04x_value\0".as_ptr() as *const c_char,
        token_sysfs,
        token as c_int,
    );
    if ret < 0 {
        printf(b"sprintf value failed\n\0".as_ptr() as *const c_char);
        return 2;
    }
    f = fopen(value_sysfs.as_ptr(), b"rb\0".as_ptr() as *const c_char);
    if f.is_null() {
        printf(
            b"failed to open %s\n\0".as_ptr() as *const c_char,
            value_sysfs.as_ptr(),
        );
        return 2;
    }
    fread(buf.as_mut_ptr() as *mut c_void, 1, 4096, f);
    fclose(f);
    *value = strtol(buf.as_ptr(), ptr::null_mut(), 16) as __u16;

    ret = sprintf(
        location_sysfs.as_mut_ptr(),
        b"%s/%04x_location\0".as_ptr() as *const c_char,
        token_sysfs,
        token as c_int,
    );
    if ret < 0 {
        printf(b"sprintf location failed\n\0".as_ptr() as *const c_char);
        return 1;
    }
    f = fopen(location_sysfs.as_ptr(), b"rb\0".as_ptr() as *const c_char);
    if f.is_null() {
        printf(
            b"failed to open %s\n\0".as_ptr() as *const c_char,
            location_sysfs.as_ptr(),
        );
        return 2;
    }
    fread(buf.as_mut_ptr() as *mut c_void, 1, 4096, f);
    fclose(f);
    *location = strtol(buf.as_ptr(), ptr::null_mut(), 16) as __u16;

    if *location != 0 {
        return 0;
    }
    2
}

unsafe fn token_is_active(
    location: *mut __u16,
    cmpvalue: *mut __u16,
    buffer: *mut dell_wmi_smbios_buffer,
) -> c_int {
    let mut ret: c_int;

    (*buffer).std.cmd_class = CLASS_TOKEN_READ;
    (*buffer).std.cmd_select = SELECT_TOKEN_STD;
    (*buffer).std.input[0] = *location as __u32;
    ret = run_wmi_smbios_cmd(buffer);
    if ret != 0 || (*buffer).std.output[0] != 0 {
        return ret;
    }
    ret = ((*buffer).std.output[1] == *cmpvalue as __u32) as c_int;
    ret
}

unsafe fn query_token(token: __u16, buffer: *mut dell_wmi_smbios_buffer) -> c_int {
    let mut location: __u16 = 0;
    let mut value: __u16 = 0;
    let mut ret: c_int;

    ret = find_token(token, &mut location, &mut value);
    if ret != 0 {
        printf(
            b"unable to find token %04x\n\0".as_ptr() as *const c_char,
            token as c_int,
        );
        return 1;
    }
    token_is_active(&mut location, &mut value, buffer)
}

unsafe fn activate_token(buffer: *mut dell_wmi_smbios_buffer, token: __u16) -> c_int {
    let mut location: __u16 = 0;
    let mut value: __u16 = 0;
    let mut ret: c_int;

    ret = find_token(token, &mut location, &mut value);
    if ret != 0 {
        printf(
            b"unable to find token %04x\n\0".as_ptr() as *const c_char,
            token as c_int,
        );
        return 1;
    }
    (*buffer).std.cmd_class = CLASS_TOKEN_WRITE;
    (*buffer).std.cmd_select = SELECT_TOKEN_STD;
    (*buffer).std.input[0] = location as __u32;
    (*buffer).std.input[1] = 1;
    ret = run_wmi_smbios_cmd(buffer);
    ret
}

unsafe fn query_buffer_size(buffer_size: *mut __u64) -> c_int {
    let f: *mut FILE;

    f = fopen(ioctl_devfs, b"rb\0".as_ptr() as *const c_char);
    if f.is_null() {
        return -EINVAL;
    }
    fread(
        buffer_size as *mut c_void,
        core::mem::size_of::<__u64>(),
        1,
        f,
    );
    fclose(f);
    EXIT_SUCCESS
}

fn main() {
    unsafe {
        let mut buffer: *mut dell_wmi_smbios_buffer = core::mem::MaybeUninit::uninit().assume_init();
        let mut ret: c_int;
        let mut value: __u64 = 0;

        ret = query_buffer_size(&mut value);
        if ret == EXIT_FAILURE || value == 0 {
            printf(b"Unable to read buffer size\n\0".as_ptr() as *const c_char);
            goto_out(&mut ret, buffer);
            std::process::exit(ret);
        }
        printf(
            b"Detected required buffer size %lld\n\0".as_ptr() as *const c_char,
            value,
        );

        buffer = malloc(value as usize) as *mut dell_wmi_smbios_buffer;
        if buffer.is_null() {
            printf(b"failed to alloc memory for ioctl\n\0".as_ptr() as *const c_char);
            ret = -ENOMEM;
            goto_out(&mut ret, buffer);
            std::process::exit(ret);
        }
        (*buffer).length = value;

        /* simple SMBIOS call for looking up TPM info */
        (*buffer).std.cmd_class = CLASS_FLASH_INTERFACE;
        (*buffer).std.cmd_select = SELECT_FLASH_INTERFACE;
        (*buffer).std.input[0] = 2;
        ret = run_wmi_smbios_cmd(buffer);
        if ret != 0 {
            printf(
                b"smbios ioctl failed: %d\n\0".as_ptr() as *const c_char,
                ret,
            );
            ret = EXIT_FAILURE;
            goto_out(&mut ret, buffer);
            std::process::exit(ret);
        }
        show_buffer(buffer);

        /* query some tokens */
        ret = query_token(CAPSULE_EN_TOKEN, buffer);
        printf(
            b"UEFI Capsule enabled token is: %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        ret = query_token(CAPSULE_DIS_TOKEN, buffer);
        printf(
            b"UEFI Capsule disabled token is: %d\n\0".as_ptr() as *const c_char,
            ret,
        );

        /* activate UEFI capsule token if disabled */
        if ret != 0 {
            printf(b"Enabling UEFI capsule token\0".as_ptr() as *const c_char);
            if activate_token(buffer, CAPSULE_EN_TOKEN) != 0 {
                printf(b"activate failed\n\0".as_ptr() as *const c_char);
                ret = -1;
                goto_out(&mut ret, buffer);
                std::process::exit(ret);
            }
        }
        ret = EXIT_SUCCESS;
        goto_out(&mut ret, buffer);
        std::process::exit(ret);
    }
}

unsafe fn goto_out(ret: *mut c_int, buffer: *mut dell_wmi_smbios_buffer) {
    free(buffer as *mut c_void);
    let _ = ret;
}
