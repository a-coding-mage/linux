// SPDX-License-Identifier: GPL-2.0-only
/*
 * Implementation of get_cpuid().
 *
 * Author: Nikita Shubin <n.shubin@yadro.com>
 *         Bibo Mao <maobibo@loongson.cn>
 *         Huacai Chen <chenhuacai@loongson.cn>
 */

use core::ffi::{c_char, c_int, c_ulong, c_void};

type size_t = usize;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

// External type supplied by util/header.h in the original C build.
#[repr(C)]
pub struct perf_cpu {
    _private: [u8; 0],
}

/*
 * Output example from /proc/cpuinfo
 *   CPU Family              : Loongson-64bit
 *   Model Name              : Loongson-3C5000
 *   CPU Revision            : 0x10
 *   FPU Revision            : 0x01
 */
const CPUINFO_MODEL: &[u8] = b"Model Name\0";
const CPUINFO: &[u8] = b"/proc/cpuinfo\0";

const EINVAL: c_int = 22;
const ENOBUFS: c_int = 105;

unsafe extern "C" {
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fclose(stream: *mut FILE) -> c_int;
    fn getline(lineptr: *mut *mut c_char, n: *mut c_ulong, stream: *mut FILE) -> isize;
    fn strrchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn strndup(s: *const c_char, n: size_t) -> *mut c_char;
    fn free(ptr: *mut c_void);
    fn asprintf(strp: *mut *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn scnprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
}

unsafe fn _get_field(line: *const c_char) -> *mut c_char {
    let mut line2: *mut c_char;
    let nl: *mut c_char;

    line2 = unsafe { strrchr(line, b' ' as c_int) };
    if line2.is_null() {
        return core::ptr::null_mut();
    }

    line2 = unsafe { line2.add(1) };
    nl = unsafe { strrchr(line, b'\n' as c_int) };
    if nl.is_null() {
        return core::ptr::null_mut();
    }

    unsafe { strndup(line2, nl.offset_from(line2) as size_t) }
}

unsafe fn _get_cpuid() -> *mut c_char {
    let mut line_sz: c_ulong = 0;
    let mut line: *mut c_char;
    let mut model: *mut c_char;
    let mut cpuid: *mut c_char;
    let file: *mut FILE;

    file = unsafe { fopen(CPUINFO.as_ptr() as *const c_char, b"r\0".as_ptr() as *const c_char) };
    if file.is_null() {
        return core::ptr::null_mut();
    }

    line = core::ptr::null_mut();
    model = core::ptr::null_mut();
    cpuid = core::ptr::null_mut();
    while unsafe { getline(&mut line, &mut line_sz, file) } != -1 {
        if unsafe {
            strncmp(
                line,
                CPUINFO_MODEL.as_ptr() as *const c_char,
                strlen(CPUINFO_MODEL.as_ptr() as *const c_char),
            )
        } != 0
        {
            continue;
        }

        model = unsafe { _get_field(line) };
        if model.is_null() {
            unsafe {
                fclose(file);
                free(model as *mut c_void);
            }
            return cpuid;
        }
        break;
    }

    if !model.is_null()
        && unsafe {
            asprintf(
                &mut cpuid,
                b"%s\0".as_ptr() as *const c_char,
                model,
            )
        } < 0
    {
        cpuid = core::ptr::null_mut();
    }

    unsafe {
        fclose(file);
        free(model as *mut c_void);
    }
    cpuid
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_cpuid(
    buffer: *mut c_char,
    sz: size_t,
    cpu: perf_cpu,
) -> c_int {
    let mut ret: c_int = 0;
    let cpuid: *mut c_char = unsafe { _get_cpuid() };

    let _ = cpu;

    if cpuid.is_null() {
        return EINVAL;
    }

    if sz < unsafe { strlen(cpuid) } {
        ret = ENOBUFS;
        unsafe {
            free(cpuid as *mut c_void);
        }
        return ret;
    }

    unsafe {
        scnprintf(buffer, sz, b"%s\0".as_ptr() as *const c_char, cpuid);
        free(cpuid as *mut c_void);
    }
    ret
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_cpuid_str(cpu: perf_cpu) -> *mut c_char {
    let _ = cpu;
    unsafe { _get_cpuid() }
}
