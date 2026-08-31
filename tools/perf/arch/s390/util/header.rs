// SPDX-License-Identifier: GPL-2.0-only
/*
 * Implementation of get_cpuid().
 *
 * Copyright IBM Corp. 2014, 2018
 * Author(s): Alexander Yarygin <yarygin@linux.vnet.ibm.com>
 *	      Thomas Richter <tmricht@linux.vnet.ibm.com>
 */

use core::ffi::c_void;
use std::os::raw::{c_char, c_int, c_ulong};

const SYSINFO_MANU: &[u8] = b"Manufacturer:\0";
const SYSINFO_TYPE: &[u8] = b"Type:\0";
const SYSINFO_MODEL: &[u8] = b"Model:\0";
const SRVLVL_CPUMF: &[u8] = b"CPU-MF:\0";
const SRVLVL_VERSION: &[u8] = b"version=\0";
const SRVLVL_AUTHORIZATION: &[u8] = b"authorization=\0";
const SYSINFO: &[u8] = b"/proc/sysinfo\0";
const SRVLVL: &[u8] = b"/proc/service_levels\0";

const EINVAL: c_int = 22;
const ENOBUFS: c_int = 105;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_cpu {
    _private: [u8; 0],
}

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

unsafe extern "C" {
    static mut errno: c_int;

    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fclose(stream: *mut FILE) -> c_int;
    fn getline(lineptr: *mut *mut c_char, n: *mut c_ulong, stream: *mut FILE) -> isize;
    fn free(ptr: *mut c_void);
    fn malloc(size: usize) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn strlen(s: *const c_char) -> usize;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn strtok_r(
        str: *mut c_char,
        delim: *const c_char,
        saveptr: *mut *mut c_char,
    ) -> *mut c_char;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn snprintf(str: *mut c_char, size: usize, format: *const c_char, ...) -> c_int;
    fn scnprintf(str: *mut c_char, size: usize, format: *const c_char, ...) -> c_int;
    fn zfree(ptr: *mut *mut c_char);
}

pub unsafe fn get_cpuid(buffer: *mut c_char, sz: usize, _cpu: perf_cpu) -> c_int {
    let mut cp: *mut c_char;
    let mut line: *mut c_char = core::ptr::null_mut();
    let mut line2: *mut c_char;
    let mut type_: [c_char; 8] = [0; 8];
    let mut model: [c_char; 33] = [0; 33];
    let mut version: [c_char; 8] = [0; 8];
    let mut manufacturer: [c_char; 32] = [0; 32];
    let mut authorization: [c_char; 8] = [0; 8];
    let mut tpsize: c_int = 0;
    let mut mdsize: c_int = 0;
    let mut vssize: c_int = 0;
    let mut mfsize: c_int = 0;
    let mut atsize: c_int = 0;
    let mut read: isize;
    let mut line_sz: c_ulong = 0;
    let nbytes: usize;
    let mut sysinfo: *mut FILE;

    /*
     * Scan /proc/sysinfo line by line and read out values for
     * Manufacturer:, Type: and Model:, for example:
     * Manufacturer:    IBM
     * Type:            2964
     * Model:           702              N96
     * The first word is the Model Capacity and the second word is
     * Model (can be omitted). Both words have a maximum size of 16
     * bytes.
     */
    unsafe {
        memset(
            manufacturer.as_mut_ptr() as *mut c_void,
            0,
            core::mem::size_of_val(&manufacturer),
        );
        memset(
            type_.as_mut_ptr() as *mut c_void,
            0,
            core::mem::size_of_val(&type_),
        );
        memset(
            model.as_mut_ptr() as *mut c_void,
            0,
            core::mem::size_of_val(&model),
        );
        memset(
            version.as_mut_ptr() as *mut c_void,
            0,
            core::mem::size_of_val(&version),
        );
        memset(
            authorization.as_mut_ptr() as *mut c_void,
            0,
            core::mem::size_of_val(&authorization),
        );

        sysinfo = fopen(SYSINFO.as_ptr() as *const c_char, b"r\0".as_ptr() as *const c_char);
        if sysinfo.is_null() {
            return errno;
        }

        loop {
            read = getline(&mut line, &mut line_sz, sysinfo);
            if read == -1 {
                break;
            }
            if strncmp(
                line,
                SYSINFO_MANU.as_ptr() as *const c_char,
                strlen(SYSINFO_MANU.as_ptr() as *const c_char),
            ) == 0
            {
                line2 = line.add(strlen(SYSINFO_MANU.as_ptr() as *const c_char));

                loop {
                    cp = strtok_r(
                        line2,
                        b"\n \0".as_ptr() as *const c_char,
                        &mut line2,
                    );
                    if cp.is_null() {
                        break;
                    }
                    mfsize += scnprintf(
                        manufacturer.as_mut_ptr().add(mfsize as usize),
                        core::mem::size_of_val(&manufacturer) - mfsize as usize,
                        b"%s\0".as_ptr() as *const c_char,
                        cp,
                    );
                }
            }

            if strncmp(
                line,
                SYSINFO_TYPE.as_ptr() as *const c_char,
                strlen(SYSINFO_TYPE.as_ptr() as *const c_char),
            ) == 0
            {
                line2 = line.add(strlen(SYSINFO_TYPE.as_ptr() as *const c_char));

                loop {
                    cp = strtok_r(
                        line2,
                        b"\n \0".as_ptr() as *const c_char,
                        &mut line2,
                    );
                    if cp.is_null() {
                        break;
                    }
                    tpsize += scnprintf(
                        type_.as_mut_ptr().add(tpsize as usize),
                        core::mem::size_of_val(&type_) - tpsize as usize,
                        b"%s\0".as_ptr() as *const c_char,
                        cp,
                    );
                }
            }

            if strncmp(
                line,
                SYSINFO_MODEL.as_ptr() as *const c_char,
                strlen(SYSINFO_MODEL.as_ptr() as *const c_char),
            ) == 0
            {
                line2 = line.add(strlen(SYSINFO_MODEL.as_ptr() as *const c_char));

                loop {
                    cp = strtok_r(
                        line2,
                        b"\n \0".as_ptr() as *const c_char,
                        &mut line2,
                    );
                    if cp.is_null() {
                        break;
                    }
                    mdsize += scnprintf(
                        model.as_mut_ptr().add(mdsize as usize),
                        core::mem::size_of_val(&model) - mdsize as usize,
                        b"%s%s\0".as_ptr() as *const c_char,
                        if model[0] != 0 {
                            b",\0".as_ptr() as *const c_char
                        } else {
                            b"\0".as_ptr() as *const c_char
                        },
                        cp,
                    );
                }
                break;
            }
        }
        fclose(sysinfo);

        /* Missing manufacturer, type or model information should not happen */
        if manufacturer[0] == 0 || type_[0] == 0 || model[0] == 0 {
            return EINVAL;
        }

        /*
         * Scan /proc/service_levels and return the CPU-MF counter facility
         * version number and authorization level.
         * Optional, does not exist on z/VM guests.
         */
        sysinfo = fopen(SRVLVL.as_ptr() as *const c_char, b"r\0".as_ptr() as *const c_char);
        if sysinfo.is_null() {
            free(line as *mut c_void);
        } else {
            loop {
                read = getline(&mut line, &mut line_sz, sysinfo);
                if read == -1 {
                    break;
                }
                if strncmp(
                    line,
                    SRVLVL_CPUMF.as_ptr() as *const c_char,
                    strlen(SRVLVL_CPUMF.as_ptr() as *const c_char),
                ) != 0
                {
                    continue;
                }

                line2 = line.add(strlen(SRVLVL_CPUMF.as_ptr() as *const c_char));
                loop {
                    cp = strtok_r(
                        line2,
                        b"\n \0".as_ptr() as *const c_char,
                        &mut line2,
                    );
                    if cp.is_null() {
                        break;
                    }
                    if strncmp(
                        cp,
                        SRVLVL_VERSION.as_ptr() as *const c_char,
                        strlen(SRVLVL_VERSION.as_ptr() as *const c_char),
                    ) == 0
                    {
                        let sep: *mut c_char = strchr(cp, '=' as c_int);

                        vssize += scnprintf(
                            version.as_mut_ptr().add(vssize as usize),
                            core::mem::size_of_val(&version) - vssize as usize,
                            b"%s\0".as_ptr() as *const c_char,
                            sep.add(1),
                        );
                    }
                    if strncmp(
                        cp,
                        SRVLVL_AUTHORIZATION.as_ptr() as *const c_char,
                        strlen(SRVLVL_AUTHORIZATION.as_ptr() as *const c_char),
                    ) == 0
                    {
                        let sep: *mut c_char = strchr(cp, '=' as c_int);

                        atsize += scnprintf(
                            authorization.as_mut_ptr().add(atsize as usize),
                            core::mem::size_of_val(&authorization) - atsize as usize,
                            b"%s\0".as_ptr() as *const c_char,
                            sep.add(1),
                        );
                    }
                }
            }
            fclose(sysinfo);

            free(line as *mut c_void);
        }

        if version[0] != 0 && authorization[0] != 0 {
            nbytes = snprintf(
                buffer,
                sz,
                b"%s,%s,%s,%s,%s\0".as_ptr() as *const c_char,
                manufacturer.as_ptr(),
                type_.as_ptr(),
                model.as_ptr(),
                version.as_ptr(),
                authorization.as_ptr(),
            ) as usize;
        } else {
            nbytes = snprintf(
                buffer,
                sz,
                b"%s,%s,%s\0".as_ptr() as *const c_char,
                manufacturer.as_ptr(),
                type_.as_ptr(),
                model.as_ptr(),
            ) as usize;
        }
    }
    if nbytes >= sz {
        ENOBUFS
    } else {
        0
    }
}

pub unsafe fn get_cpuid_str(cpu: perf_cpu) -> *mut c_char {
    let mut buf: *mut c_char = unsafe { malloc(128) as *mut c_char };

    if !buf.is_null() && unsafe { get_cpuid(buf, 128, cpu) } != 0 {
        unsafe {
            zfree(&mut buf);
        }
    }
    buf
}
