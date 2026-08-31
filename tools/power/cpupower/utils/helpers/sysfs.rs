// SPDX-License-Identifier: GPL-2.0-only
/*
 *  (C) 2004-2009  Dominik Brodowski <linux@dominikbrodowski.de>
 *  (C) 2011       Thomas Renninger <trenn@novell.com> Novell Inc.
 */

use libc::{
    c_char, c_int, c_ulong, c_ulonglong, c_void, close, open, read, stat, strtoull, write, EINVAL,
    EIO, ENODEV, ERANGE, O_RDONLY, O_WRONLY,
};
use std::ffi::CString;
use std::ptr;

use crate::{MAX_LINE_LEN, PATH_TO_CPU, SYSFS_PATH_MAX};

unsafe extern "C" {
    fn __errno_location() -> *mut c_int;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strlen(s: *const c_char) -> usize;
}

const S_IFMT: libc::mode_t = 0o170000;
const S_IFDIR: libc::mode_t = 0o040000;

fn s_isdir(mode: libc::mode_t) -> bool {
    (mode & S_IFMT) == S_IFDIR
}

unsafe fn errno_value() -> c_int {
    unsafe { *__errno_location() }
}

unsafe fn make_cstring(s: String) -> Option<CString> {
    CString::new(s).ok()
}

pub unsafe fn sysfs_read_file(path: *const c_char, buf: *mut c_char, buflen: usize) -> u32 {
    let fd: c_int;
    let numread: isize;

    fd = unsafe { open(path, O_RDONLY) };
    if fd == -1 {
        return 0;
    }

    numread = unsafe { read(fd, buf as *mut c_void, buflen - 1) };
    if numread < 1 {
        unsafe {
            close(fd);
        }
        return 0;
    }

    unsafe {
        *buf.add(numread as usize) = b'\0' as c_char;
        close(fd);
    }

    numread as u32
}

/*
 * Detect whether a CPU is online
 *
 * Returns:
 *     1 -> if CPU is online
 *     0 -> if CPU is offline
 *     negative errno values in error case
 */
pub unsafe fn sysfs_is_cpu_online(cpu: u32) -> c_int {
    let mut fd: c_int;
    let numread: isize;
    let value: c_ulonglong;
    let mut linebuf = [0 as c_char; MAX_LINE_LEN];
    let mut endp: *mut c_char = ptr::null_mut();
    let mut statbuf: stat = unsafe { std::mem::zeroed() };

    let mut path = unsafe { make_cstring(format!("{}cpu{}", PATH_TO_CPU, cpu)) }.unwrap();

    if unsafe { stat(path.as_ptr(), &mut statbuf) } != 0 {
        return 0;
    }

    /*
     * kernel without CONFIG_HOTPLUG_CPU
     * -> cpuX directory exists, but not cpuX/online file
     */
    path = unsafe { make_cstring(format!("{}cpu{}/online", PATH_TO_CPU, cpu)) }.unwrap();
    if unsafe { stat(path.as_ptr(), &mut statbuf) } != 0 {
        return 1;
    }

    fd = unsafe { open(path.as_ptr(), O_RDONLY) };
    if fd == -1 {
        return -unsafe { errno_value() };
    }

    numread = unsafe { read(fd, linebuf.as_mut_ptr() as *mut c_void, MAX_LINE_LEN - 1) };
    if numread < 1 {
        unsafe {
            close(fd);
        }
        return -EIO;
    }
    linebuf[numread as usize] = b'\0' as c_char;
    unsafe {
        close(fd);
    }

    value = unsafe { strtoull(linebuf.as_ptr(), &mut endp, 0) };
    if value > 1 {
        return -EINVAL;
    }

    value as c_int
}

/* CPUidle idlestate specific /sys/devices/system/cpu/cpuX/cpuidle/ access */


/* CPUidle idlestate specific /sys/devices/system/cpu/cpuX/cpuidle/ access */

/*
 * helper function to check whether a file under "../cpuX/cpuidle/stateX/" dir
 * exists.
 * For example the functionality to disable c-states was introduced in later
 * kernel versions, this function can be used to explicitly check for this
 * feature.
 *
 * returns 1 if the file exists, 0 otherwise.
 */
pub unsafe fn sysfs_idlestate_file_exists(
    cpu: u32,
    idlestate: u32,
    fname: *const c_char,
) -> u32 {
    let mut statbuf: stat = unsafe { std::mem::zeroed() };
    let fname = unsafe { std::ffi::CStr::from_ptr(fname) }.to_string_lossy();
    let path = unsafe {
        make_cstring(format!(
            "{}cpu{}/cpuidle/state{}/{}",
            PATH_TO_CPU, cpu, idlestate, fname
        ))
    }
    .unwrap();

    if unsafe { stat(path.as_ptr(), &mut statbuf) } != 0 {
        return 0;
    }
    1
}

/*
 * helper function to read file from /sys into given buffer
 * fname is a relative path under "cpuX/cpuidle/stateX/" dir
 * cstates starting with 0, C0 is not counted as cstate.
 * This means if you want C1 info, pass 0 as idlestate param
 */
pub unsafe fn sysfs_idlestate_read_file(
    cpu: u32,
    idlestate: u32,
    fname: *const c_char,
    buf: *mut c_char,
    buflen: usize,
) -> u32 {
    let fd: c_int;
    let numread: isize;
    let fname = unsafe { std::ffi::CStr::from_ptr(fname) }.to_string_lossy();
    let path = unsafe {
        make_cstring(format!(
            "{}cpu{}/cpuidle/state{}/{}",
            PATH_TO_CPU, cpu, idlestate, fname
        ))
    }
    .unwrap();

    fd = unsafe { open(path.as_ptr(), O_RDONLY) };
    if fd == -1 {
        return 0;
    }

    numread = unsafe { read(fd, buf as *mut c_void, buflen - 1) };
    if numread < 1 {
        unsafe {
            close(fd);
        }
        return 0;
    }

    unsafe {
        *buf.add(numread as usize) = b'\0' as c_char;
        close(fd);
    }

    numread as u32
}

/* 
 * helper function to write a new value to a /sys file
 * fname is a relative path under "../cpuX/cpuidle/cstateY/" dir
 *
 * Returns the number of bytes written or 0 on error
 */
unsafe fn sysfs_idlestate_write_file(
    cpu: u32,
    idlestate: u32,
    fname: *const c_char,
    value: *const c_char,
    len: usize,
) -> u32 {
    let fd: c_int;
    let numwrite: isize;
    let fname = unsafe { std::ffi::CStr::from_ptr(fname) }.to_string_lossy();
    let path = unsafe {
        make_cstring(format!(
            "{}cpu{}/cpuidle/state{}/{}",
            PATH_TO_CPU, cpu, idlestate, fname
        ))
    }
    .unwrap();

    fd = unsafe { open(path.as_ptr(), O_WRONLY) };
    if fd == -1 {
        return 0;
    }

    numwrite = unsafe { write(fd, value as *const c_void, len) };
    if numwrite < 1 {
        unsafe {
            close(fd);
        }
        return 0;
    }

    unsafe {
        close(fd);
    }

    numwrite as u32
}

/* read access to files which contain one numeric value */

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum idlestate_value {
    IDLESTATE_USAGE,
    IDLESTATE_POWER,
    IDLESTATE_LATENCY,
    IDLESTATE_TIME,
    IDLESTATE_DISABLE,
    MAX_IDLESTATE_VALUE_FILES,
}

static idlestate_value_files: [&[u8]; idlestate_value::MAX_IDLESTATE_VALUE_FILES as usize] = [
    b"usage\0",
    b"power\0",
    b"latency\0",
    b"time\0",
    b"disable\0",
];

unsafe fn sysfs_idlestate_get_one_value(
    cpu: u32,
    idlestate: u32,
    which: idlestate_value,
) -> c_ulonglong {
    let value: c_ulonglong;
    let len: u32;
    let mut linebuf = [0 as c_char; MAX_LINE_LEN];
    let mut endp: *mut c_char = ptr::null_mut();

    if which >= idlestate_value::MAX_IDLESTATE_VALUE_FILES {
        return 0;
    }

    len = unsafe {
        sysfs_idlestate_read_file(
            cpu,
            idlestate,
            idlestate_value_files[which as usize].as_ptr() as *const c_char,
            linebuf.as_mut_ptr(),
            linebuf.len(),
        )
    };
    if len == 0 {
        return 0;
    }

    value = unsafe { strtoull(linebuf.as_ptr(), &mut endp, 0) };

    if endp == linebuf.as_mut_ptr() || unsafe { errno_value() } == ERANGE {
        return 0;
    }

    value
}

/* read access to files which contain one string */

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum idlestate_string {
    IDLESTATE_DESC,
    IDLESTATE_NAME,
    MAX_IDLESTATE_STRING_FILES,
}

static idlestate_string_files: [&[u8]; idlestate_string::MAX_IDLESTATE_STRING_FILES as usize] = [
    b"desc\0",
    b"name\0",
];

unsafe fn sysfs_idlestate_get_one_string(
    cpu: u32,
    idlestate: u32,
    which: idlestate_string,
) -> *mut c_char {
    let mut linebuf = [0 as c_char; MAX_LINE_LEN];
    let result: *mut c_char;
    let len: u32;

    if which >= idlestate_string::MAX_IDLESTATE_STRING_FILES {
        return ptr::null_mut();
    }

    len = unsafe {
        sysfs_idlestate_read_file(
            cpu,
            idlestate,
            idlestate_string_files[which as usize].as_ptr() as *const c_char,
            linebuf.as_mut_ptr(),
            linebuf.len(),
        )
    };
    if len == 0 {
        return ptr::null_mut();
    }

    result = unsafe { strdup(linebuf.as_ptr()) };
    if result.is_null() {
        return ptr::null_mut();
    }

    unsafe {
        let last = strlen(result) - 1;
        if *result.add(last) == b'\n' as c_char {
            *result.add(last) = b'\0' as c_char;
        }
    }

    result
}

/*
 * Returns:
 *    1  if disabled
 *    0  if enabled
 *    -1 if idlestate is not available
 *    -2 if disabling is not supported by the kernel
 */
pub unsafe fn sysfs_is_idlestate_disabled(cpu: u32, idlestate: u32) -> c_int {
    if unsafe { sysfs_get_idlestate_count(cpu) } <= idlestate {
        return -1;
    }

    if unsafe {
        sysfs_idlestate_file_exists(
            cpu,
            idlestate,
            idlestate_value_files[idlestate_value::IDLESTATE_DISABLE as usize].as_ptr()
                as *const c_char,
        )
    } == 0
    {
        return -2;
    }
    unsafe { sysfs_idlestate_get_one_value(cpu, idlestate, idlestate_value::IDLESTATE_DISABLE) }
        as c_int
}

/*
 * Pass 1 as last argument to disable or 0 to enable the state
 * Returns:
 *    0  on success
 *    negative values on error, for example:
 *      -1 if idlestate is not available
 *      -2 if disabling is not supported by the kernel
 *      -3 No write access to disable/enable C-states
 */
pub unsafe fn sysfs_idlestate_disable(cpu: u32, idlestate: u32, disable: u32) -> c_int {
    let value = unsafe { make_cstring(format!("{}", disable)) }.unwrap();
    let bytes_written: c_int;

    if unsafe { sysfs_get_idlestate_count(cpu) } <= idlestate {
        return -1;
    }

    if unsafe {
        sysfs_idlestate_file_exists(
            cpu,
            idlestate,
            idlestate_value_files[idlestate_value::IDLESTATE_DISABLE as usize].as_ptr()
                as *const c_char,
        )
    } == 0
    {
        return -2;
    }

    bytes_written = unsafe {
        sysfs_idlestate_write_file(
            cpu,
            idlestate,
            b"disable\0".as_ptr() as *const c_char,
            value.as_ptr(),
            std::mem::size_of_val(&disable),
        )
    } as c_int;
    if bytes_written != 0 {
        return 0;
    }
    -3
}

pub unsafe fn sysfs_get_idlestate_latency(cpu: u32, idlestate: u32) -> c_ulong {
    unsafe { sysfs_idlestate_get_one_value(cpu, idlestate, idlestate_value::IDLESTATE_LATENCY) }
        as c_ulong
}

pub unsafe fn sysfs_get_idlestate_usage(cpu: u32, idlestate: u32) -> c_ulong {
    unsafe { sysfs_idlestate_get_one_value(cpu, idlestate, idlestate_value::IDLESTATE_USAGE) }
        as c_ulong
}

pub unsafe fn sysfs_get_idlestate_time(cpu: u32, idlestate: u32) -> c_ulonglong {
    unsafe { sysfs_idlestate_get_one_value(cpu, idlestate, idlestate_value::IDLESTATE_TIME) }
}

pub unsafe fn sysfs_get_idlestate_name(cpu: u32, idlestate: u32) -> *mut c_char {
    unsafe { sysfs_idlestate_get_one_string(cpu, idlestate, idlestate_string::IDLESTATE_NAME) }
}

pub unsafe fn sysfs_get_idlestate_desc(cpu: u32, idlestate: u32) -> *mut c_char {
    unsafe { sysfs_idlestate_get_one_string(cpu, idlestate, idlestate_string::IDLESTATE_DESC) }
}

/*
 * Returns number of supported C-states of CPU core cpu
 * Negativ in error case
 * Zero if cpuidle does not export any C-states
 */
pub unsafe fn sysfs_get_idlestate_count(cpu: u32) -> u32 {
    let mut statbuf: stat = unsafe { std::mem::zeroed() };
    let mut idlestates: c_int = 1;

    let mut file = unsafe { make_cstring(format!("{}cpuidle", PATH_TO_CPU)) }.unwrap();
    if unsafe { stat(file.as_ptr(), &mut statbuf) } != 0 || !s_isdir(statbuf.st_mode) {
        return 0;
    }

    file = unsafe { make_cstring(format!("{}cpu{}/cpuidle/state0", PATH_TO_CPU, cpu)) }.unwrap();
    if unsafe { stat(file.as_ptr(), &mut statbuf) } != 0 || !s_isdir(statbuf.st_mode) {
        return 0;
    }

    while unsafe { stat(file.as_ptr(), &mut statbuf) } == 0 && s_isdir(statbuf.st_mode) {
        file = unsafe {
            make_cstring(format!(
                "{}cpu{}/cpuidle/state{}",
                PATH_TO_CPU, cpu, idlestates
            ))
        }
        .unwrap();
        idlestates += 1;
    }
    idlestates -= 1;
    idlestates as u32
}

/* CPUidle general /sys/devices/system/cpu/cpuidle/ sysfs access ********/

/*
 * helper function to read file from /sys into given buffer
 * fname is a relative path under "cpu/cpuidle/" dir
 */
unsafe fn sysfs_cpuidle_read_file(fname: *const c_char, buf: *mut c_char, buflen: usize) -> u32 {
    let fname = unsafe { std::ffi::CStr::from_ptr(fname) }.to_string_lossy();
    let path = unsafe { make_cstring(format!("{}cpuidle/{}", PATH_TO_CPU, fname)) }.unwrap();

    unsafe { sysfs_read_file(path.as_ptr(), buf, buflen) }
}



/* read access to files which contain one string */

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum cpuidle_string {
    CPUIDLE_GOVERNOR,
    CPUIDLE_GOVERNOR_RO,
    CPUIDLE_DRIVER,
    MAX_CPUIDLE_STRING_FILES,
}

static cpuidle_string_files: [&[u8]; cpuidle_string::MAX_CPUIDLE_STRING_FILES as usize] = [
    b"current_governor\0",
    b"current_governor_ro\0",
    b"current_driver\0",
];

unsafe fn sysfs_cpuidle_get_one_string(which: cpuidle_string) -> *mut c_char {
    let mut linebuf = [0 as c_char; MAX_LINE_LEN];
    let result: *mut c_char;
    let len: u32;

    if which >= cpuidle_string::MAX_CPUIDLE_STRING_FILES {
        return ptr::null_mut();
    }

    len = unsafe {
        sysfs_cpuidle_read_file(
            cpuidle_string_files[which as usize].as_ptr() as *const c_char,
            linebuf.as_mut_ptr(),
            linebuf.len(),
        )
    };
    if len == 0 {
        return ptr::null_mut();
    }

    result = unsafe { strdup(linebuf.as_ptr()) };
    if result.is_null() {
        return ptr::null_mut();
    }

    unsafe {
        let last = strlen(result) - 1;
        if *result.add(last) == b'\n' as c_char {
            *result.add(last) = b'\0' as c_char;
        }
    }

    result
}

pub unsafe fn sysfs_get_cpuidle_governor() -> *mut c_char {
    let tmp: *mut c_char =
        unsafe { sysfs_cpuidle_get_one_string(cpuidle_string::CPUIDLE_GOVERNOR_RO) };
    if tmp.is_null() {
        unsafe { sysfs_cpuidle_get_one_string(cpuidle_string::CPUIDLE_GOVERNOR) }
    } else {
        tmp
    }
}

pub unsafe fn sysfs_get_cpuidle_driver() -> *mut c_char {
    unsafe { sysfs_cpuidle_get_one_string(cpuidle_string::CPUIDLE_DRIVER) }
}

/* CPUidle idlestate specific /sys/devices/system/cpu/cpuX/cpuidle/ access */

/*
 * Get sched_mc or sched_smt settings
 * Pass "mc" or "smt" as argument
 *
 * Returns negative value on failure
 */
pub unsafe fn sysfs_get_sched(_smt_mc: *const c_char) -> c_int {
    -ENODEV
}

/*
 * Get sched_mc or sched_smt settings
 * Pass "mc" or "smt" as argument
 *
 * Returns negative value on failure
 */
pub unsafe fn sysfs_set_sched(_smt_mc: *const c_char, _val: c_int) -> c_int {
    -ENODEV
}
