// SPDX-License-Identifier: GPL-2.0
//
// Translated from perf/arch/powerpc/util/header.c. C include dependencies are
// represented below as external declarations or file-local equivalents.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_ulong};

type size_t = usize;

const ENOBUFS: c_int = 105;
const PATH_MAX: usize = 4096;
const AT_PLATFORM: c_ulong = 15;
const AT_BASE_PLATFORM: c_ulong = 24;
const SPRN_PVR: c_ulong = 0x11f;

const PerChip: c_int = 0;

#[repr(C)]
pub struct perf_cpu {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pmu_metric {
    pub aggr_mode: c_int,
}

unsafe extern "C" {
    fn getauxval(type_: c_ulong) -> c_ulong;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn scnprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn asprintf(strp: *mut *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn strcat(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn sysfs__read_int(path: *const c_char, value: *mut c_int) -> c_int;

    fn mfspr(spr: c_ulong) -> c_ulong;
}

#[inline]
fn PVR_VER(pvr: c_ulong) -> c_ulong {
    pvr >> 16
}

#[inline]
fn PVR_REV(pvr: c_ulong) -> c_ulong {
    pvr & 0xffff
}

unsafe fn is_compat_mode() -> bool {
    let base_platform: c_ulong = getauxval(AT_BASE_PLATFORM);
    let platform: c_ulong = getauxval(AT_PLATFORM);

    if strcmp(platform as *const c_char, base_platform as *const c_char) == 0 {
        return false;
    }

    true
}

#[no_mangle]
pub unsafe extern "C" fn get_cpuid(
    buffer: *mut c_char,
    sz: size_t,
    _cpu: perf_cpu,
) -> c_int {
    let pvr: c_ulong;
    let nb: c_int;

    pvr = mfspr(SPRN_PVR);

    nb = scnprintf(
        buffer,
        sz,
        b"%lu,%lu$\0".as_ptr() as *const c_char,
        PVR_VER(pvr),
        PVR_REV(pvr),
    );

    /* look for end marker to ensure the entire data fit */
    if !strchr(buffer, b'$' as c_int).is_null() {
        *buffer.add((nb - 1) as usize) = b'\0' as c_char;
        return 0;
    }
    ENOBUFS
}

#[no_mangle]
pub unsafe extern "C" fn get_cpuid_str(_cpu: perf_cpu) -> *mut c_char {
    let mut bufp: *mut c_char;
    let pvr: c_ulong;

    /*
     * IBM Power System supports compatible mode. That is
     * Nth generation platform can support previous generation
     * OS in a mode called compatibile mode. For ex. LPAR can be
     * booted in a Power9 mode when the system is a Power10.
     *
     * In the compatible mode, care must be taken when generating
     * PVR value. When read, PVR will be of the AT_BASE_PLATFORM
     * To support generic events, return 0x00ffffff as pvr when
     * booted in compat mode. Based on this pvr value, json will
     * pick events from pmu-events/arch/powerpc/compat
     */
    if !is_compat_mode() {
        pvr = mfspr(SPRN_PVR);
    } else {
        pvr = 0x00ffffff;
    }

    if asprintf(&mut bufp, b"0x%.8lx\0".as_ptr() as *const c_char, pvr) < 0 {
        bufp = core::ptr::null_mut();
    }

    bufp
}

#[no_mangle]
pub unsafe extern "C" fn arch_get_runtimeparam(pm: *const pmu_metric) -> c_int {
    let mut count: c_int = 0;
    let mut path: [c_char; PATH_MAX] = [0; PATH_MAX];
    let init = b"/devices/hv_24x7/interface/\0";

    core::ptr::copy_nonoverlapping(
        init.as_ptr() as *const c_char,
        path.as_mut_ptr(),
        init.len(),
    );

    strcat(
        path.as_mut_ptr(),
        if (*pm).aggr_mode == PerChip {
            b"sockets\0".as_ptr() as *const c_char
        } else {
            b"coresperchip\0".as_ptr() as *const c_char
        },
    );
    if sysfs__read_int(path.as_ptr(), &mut count) < 0 {
        1
    } else {
        count
    }
}
