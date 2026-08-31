// SPDX-License-Identifier: GPL-2.0-only
/*
 * Implementation of get_cpuid().
 *
 * Author: Nikita Shubin <n.shubin@yadro.com>
 */

use libc::{c_char, c_int, c_void, size_t, FILE};

const CPUINFO_MVEN: &[u8] = b"mvendorid\0";
const CPUINFO_MARCH: &[u8] = b"marchid\0";
const CPUINFO_MIMP: &[u8] = b"mimpid\0";
const CPUINFO: &[u8] = b"/proc/cpuinfo\0";

extern "C" {
    fn asprintf(strp: *mut *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn fclose(stream: *mut FILE) -> c_int;
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn free(ptr: *mut c_void);
    fn getline(lineptr: *mut *mut c_char, n: *mut size_t, stream: *mut FILE) -> libc::ssize_t;
    fn scnprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
    fn strndup(s: *const c_char, n: size_t) -> *mut c_char;
    fn strrchr(s: *const c_char, c: c_int) -> *mut c_char;
}

unsafe fn _get_field(line: *const c_char) -> *mut c_char {
    let mut line2: *const c_char;
    let nl: *const c_char;

    line2 = strrchr(line, b' ' as c_int);
    if line2.is_null() {
        return std::ptr::null_mut();
    }

    line2 = line2.add(1);
    nl = strrchr(line, b'\n' as c_int);
    if nl.is_null() {
        return std::ptr::null_mut();
    }

    strndup(line2, nl.offset_from(line2) as size_t)
}

unsafe fn _get_cpuid() -> *mut c_char {
    let mut line: *mut c_char = std::ptr::null_mut();
    let mut mvendorid: *mut c_char = std::ptr::null_mut();
    let mut marchid: *mut c_char = std::ptr::null_mut();
    let mut mimpid: *mut c_char = std::ptr::null_mut();
    let mut cpuid: *mut c_char = std::ptr::null_mut();
    let mut read: libc::ssize_t;
    let mut line_sz: size_t = 0;
    let cpuinfo: *mut FILE;

    cpuinfo = fopen(CPUINFO.as_ptr() as *const c_char, b"r\0".as_ptr() as *const c_char);
    if cpuinfo.is_null() {
        return cpuid;
    }

    loop {
        read = getline(&mut line, &mut line_sz, cpuinfo);
        if read == -1 {
            break;
        }

        if strncmp(line, CPUINFO_MVEN.as_ptr() as *const c_char, strlen(CPUINFO_MVEN.as_ptr() as *const c_char)) == 0 {
            mvendorid = _get_field(line);
            if mvendorid.is_null() {
                goto_free(cpuinfo, mvendorid, marchid, mimpid);
                return cpuid;
            }
        } else if strncmp(line, CPUINFO_MARCH.as_ptr() as *const c_char, strlen(CPUINFO_MARCH.as_ptr() as *const c_char)) == 0 {
            marchid = _get_field(line);
            if marchid.is_null() {
                goto_free(cpuinfo, mvendorid, marchid, mimpid);
                return cpuid;
            }
        } else if strncmp(line, CPUINFO_MIMP.as_ptr() as *const c_char, strlen(CPUINFO_MIMP.as_ptr() as *const c_char)) == 0 {
            mimpid = _get_field(line);
            if mimpid.is_null() {
                goto_free(cpuinfo, mvendorid, marchid, mimpid);
                return cpuid;
            }

            break;
        }
    }

    if mvendorid.is_null() || marchid.is_null() || mimpid.is_null() {
        goto_free(cpuinfo, mvendorid, marchid, mimpid);
        return cpuid;
    }

    if asprintf(
        &mut cpuid,
        b"%s-%s-%s\0".as_ptr() as *const c_char,
        mvendorid,
        marchid,
        mimpid,
    ) < 0
    {
        cpuid = std::ptr::null_mut();
    }

    goto_free(cpuinfo, mvendorid, marchid, mimpid);

    cpuid
}

unsafe fn goto_free(
    cpuinfo: *mut FILE,
    mvendorid: *mut c_char,
    marchid: *mut c_char,
    mimpid: *mut c_char,
) {
    fclose(cpuinfo);
    free(mvendorid as *mut c_void);
    free(marchid as *mut c_void);
    free(mimpid as *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn get_cpuid(buffer: *mut c_char, sz: size_t, _cpu: perf_cpu) -> c_int {
    let cpuid: *mut c_char = _get_cpuid();
    let mut ret: c_int = 0;

    if sz < strlen(cpuid) {
        ret = -libc::EINVAL;
        free(cpuid as *mut c_void);
        return ret;
    }

    scnprintf(buffer, sz, b"%s\0".as_ptr() as *const c_char, cpuid);
    free(cpuid as *mut c_void);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn get_cpuid_str(_cpu: perf_cpu) -> *mut c_char {
    _get_cpuid()
}
