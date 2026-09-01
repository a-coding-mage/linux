// SPDX-License-Identifier: GPL-2.0-only
/*
 *  (C) 2004-2009  Dominik Brodowski <linux@dominikbrodowski.de>
 *  (C) 2011       Thomas Renninger <trenn@novell.com> Novell Inc.
 */

use core::ffi::{c_char, c_int, c_ulong, c_void};

type SizeT = usize;
type SSizeT = isize;

// Dependencies supplied by cpuidle.h and cpupower_intern.h in the original C.
const SYSFS_PATH_MAX: usize = 255;
const MAX_LINE_LEN: usize = 4096;
const PATH_TO_CPU: &[u8] = b"/sys/devices/system/cpu/";

const O_RDONLY: c_int = 0;
const O_WRONLY: c_int = 1;
const ERANGE: c_int = 34;
const S_IFMT: u32 = 0o170000;
const S_IFDIR: u32 = 0o040000;

#[repr(C)]
pub struct stat {
    _private: [u8; 0],
}

unsafe extern "C" {
    static mut errno: c_int;

    fn snprintf(str_: *mut c_char, size: SizeT, format: *const c_char, ...) -> c_int;
    fn stat(pathname: *const c_char, statbuf: *mut stat) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: SizeT) -> SSizeT;
    fn write(fd: c_int, buf: *const c_void, count: SizeT) -> SSizeT;
    fn close(fd: c_int) -> c_int;
    fn strtoull(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> u64;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strcspn(s: *const c_char, reject: *const c_char) -> SizeT;

    fn cpupower_read_sysfs(path: *const c_char, buf: *mut c_char, buflen: SizeT) -> u32;
}

unsafe fn S_ISDIR(mode: u32) -> bool {
    (mode & S_IFMT) == S_IFDIR
}

unsafe fn stat_mode(_statbuf: *const stat) -> u32 {
    // struct stat layout is supplied by the C library bindings in the full tree.
    0
}

/*
 * helper function to check whether a file under "../cpuX/cpuidle/stateX/" dir
 * exists.
 * For example the functionality to disable c-states was introduced in later
 * kernel versions, this function can be used to explicitly check for this
 * feature.
 *
 * returns 1 if the file exists, 0 otherwise.
 */
unsafe fn cpuidle_state_file_exists(cpu: u32, idlestate: u32, fname: *const c_char) -> u32 {
    let mut path = [0 as c_char; SYSFS_PATH_MAX];
    let mut statbuf: stat = core::mem::zeroed();

    snprintf(
        path.as_mut_ptr(),
        path.len(),
        b"%scpu%u/cpuidle/state%u/%s\0".as_ptr() as *const c_char,
        PATH_TO_CPU.as_ptr() as *const c_char,
        cpu,
        idlestate,
        fname,
    );
    if stat(path.as_ptr(), &mut statbuf) != 0 {
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
unsafe fn cpuidle_state_read_file(
    cpu: u32,
    idlestate: u32,
    fname: *const c_char,
    buf: *mut c_char,
    buflen: SizeT,
) -> u32 {
    let mut path = [0 as c_char; SYSFS_PATH_MAX];
    let fd: c_int;
    let numread: SSizeT;

    snprintf(
        path.as_mut_ptr(),
        path.len(),
        b"%scpu%u/cpuidle/state%u/%s\0".as_ptr() as *const c_char,
        PATH_TO_CPU.as_ptr() as *const c_char,
        cpu,
        idlestate,
        fname,
    );

    fd = open(path.as_ptr(), O_RDONLY);
    if fd == -1 {
        return 0;
    }

    numread = read(fd, buf as *mut c_void, buflen - 1);
    if numread < 1 {
        close(fd);
        return 0;
    }

    *buf.offset(numread) = b'\0' as c_char;
    close(fd);

    numread as u32
}

/*
 * helper function to write a new value to a /sys file
 * fname is a relative path under "../cpuX/cpuidle/cstateY/" dir
 *
 * Returns the number of bytes written or 0 on error
 */
unsafe fn cpuidle_state_write_file(
    cpu: u32,
    idlestate: u32,
    fname: *const c_char,
    value: *const c_char,
    len: SizeT,
) -> u32 {
    let mut path = [0 as c_char; SYSFS_PATH_MAX];
    let fd: c_int;
    let numwrite: SSizeT;

    snprintf(
        path.as_mut_ptr(),
        path.len(),
        b"%scpu%u/cpuidle/state%u/%s\0".as_ptr() as *const c_char,
        PATH_TO_CPU.as_ptr() as *const c_char,
        cpu,
        idlestate,
        fname,
    );

    fd = open(path.as_ptr(), O_WRONLY);
    if fd == -1 {
        return 0;
    }

    numwrite = write(fd, value as *const c_void, len);
    if numwrite < 1 {
        close(fd);
        return 0;
    }

    close(fd);

    numwrite as u32
}

/* read access to files which contain one numeric value */

#[repr(u32)]
enum idlestate_value {
    IDLESTATE_USAGE,
    IDLESTATE_POWER,
    IDLESTATE_LATENCY,
    IDLESTATE_RESIDENCY,
    IDLESTATE_TIME,
    IDLESTATE_DISABLE,
    MAX_IDLESTATE_VALUE_FILES,
}

static idlestate_value_files: [*const c_char; idlestate_value::MAX_IDLESTATE_VALUE_FILES as usize] = [
    b"usage\0".as_ptr() as *const c_char,
    b"power\0".as_ptr() as *const c_char,
    b"latency\0".as_ptr() as *const c_char,
    b"residency\0".as_ptr() as *const c_char,
    b"time\0".as_ptr() as *const c_char,
    b"disable\0".as_ptr() as *const c_char,
];

unsafe fn cpuidle_state_get_one_value(cpu: u32, idlestate: u32, which: idlestate_value) -> u64 {
    let mut value: u64;
    let len: u32;
    let mut linebuf = [0 as c_char; MAX_LINE_LEN];
    let mut endp: *mut c_char = core::ptr::null_mut();
    let which_index = which as usize;

    if which_index >= idlestate_value::MAX_IDLESTATE_VALUE_FILES as usize {
        return 0;
    }

    len = cpuidle_state_read_file(
        cpu,
        idlestate,
        idlestate_value_files[which_index],
        linebuf.as_mut_ptr(),
        linebuf.len(),
    );
    if len == 0 {
        return 0;
    }

    errno = 0;
    value = strtoull(linebuf.as_ptr(), &mut endp, 0);

    if endp == linebuf.as_mut_ptr() || errno == ERANGE {
        return 0;
    }

    value
}

/* read access to files which contain one string */

#[repr(u32)]
enum idlestate_string {
    IDLESTATE_DESC,
    IDLESTATE_NAME,
    MAX_IDLESTATE_STRING_FILES,
}

static idlestate_string_files: [*const c_char; idlestate_string::MAX_IDLESTATE_STRING_FILES as usize] = [
    b"desc\0".as_ptr() as *const c_char,
    b"name\0".as_ptr() as *const c_char,
];

unsafe fn cpuidle_state_get_one_string(
    cpu: u32,
    idlestate: u32,
    which: idlestate_string,
) -> *mut c_char {
    let mut linebuf = [0 as c_char; MAX_LINE_LEN];
    let result: *mut c_char;
    let len: u32;
    let which_index = which as usize;

    if which_index >= idlestate_string::MAX_IDLESTATE_STRING_FILES as usize {
        return core::ptr::null_mut();
    }

    len = cpuidle_state_read_file(
        cpu,
        idlestate,
        idlestate_string_files[which_index],
        linebuf.as_mut_ptr(),
        linebuf.len(),
    );
    if len == 0 {
        return core::ptr::null_mut();
    }

    result = strdup(linebuf.as_ptr());
    if result.is_null() {
        return core::ptr::null_mut();
    }

    *result.add(strcspn(result, b"\n\0".as_ptr() as *const c_char)) = b'\0' as c_char;

    result
}

/*
 * Returns:
 *    1  if disabled
 *    0  if enabled
 *    -1 if idlestate is not available
 *    -2 if disabling is not supported by the kernel
 */
#[no_mangle]
pub unsafe extern "C" fn cpuidle_is_state_disabled(cpu: u32, idlestate: u32) -> c_int {
    if cpuidle_state_count(cpu) <= idlestate {
        return -1;
    }

    if cpuidle_state_file_exists(
        cpu,
        idlestate,
        idlestate_value_files[idlestate_value::IDLESTATE_DISABLE as usize],
    ) == 0
    {
        return -2;
    }
    cpuidle_state_get_one_value(cpu, idlestate, idlestate_value::IDLESTATE_DISABLE) as c_int
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
#[no_mangle]
pub unsafe extern "C" fn cpuidle_state_disable(
    cpu: u32,
    idlestate: u32,
    disable: u32,
) -> c_int {
    let mut value = [0 as c_char; SYSFS_PATH_MAX];
    let bytes_written: c_int;
    let len: c_int;

    if cpuidle_state_count(cpu) <= idlestate {
        return -1;
    }

    if cpuidle_state_file_exists(
        cpu,
        idlestate,
        idlestate_value_files[idlestate_value::IDLESTATE_DISABLE as usize],
    ) == 0
    {
        return -2;
    }

    len = snprintf(
        value.as_mut_ptr(),
        SYSFS_PATH_MAX,
        b"%u\0".as_ptr() as *const c_char,
        disable,
    );

    bytes_written = cpuidle_state_write_file(
        cpu,
        idlestate,
        b"disable\0".as_ptr() as *const c_char,
        value.as_ptr(),
        len as SizeT,
    ) as c_int;
    if bytes_written != 0 {
        return 0;
    }
    -3
}

#[no_mangle]
pub unsafe extern "C" fn cpuidle_state_latency(cpu: u32, idlestate: u32) -> c_ulong {
    cpuidle_state_get_one_value(cpu, idlestate, idlestate_value::IDLESTATE_LATENCY) as c_ulong
}

#[no_mangle]
pub unsafe extern "C" fn cpuidle_state_residency(cpu: u32, idlestate: u32) -> c_ulong {
    cpuidle_state_get_one_value(cpu, idlestate, idlestate_value::IDLESTATE_RESIDENCY) as c_ulong
}

#[no_mangle]
pub unsafe extern "C" fn cpuidle_state_usage(cpu: u32, idlestate: u32) -> c_ulong {
    cpuidle_state_get_one_value(cpu, idlestate, idlestate_value::IDLESTATE_USAGE) as c_ulong
}

#[no_mangle]
pub unsafe extern "C" fn cpuidle_state_time(cpu: u32, idlestate: u32) -> u64 {
    cpuidle_state_get_one_value(cpu, idlestate, idlestate_value::IDLESTATE_TIME)
}

#[no_mangle]
pub unsafe extern "C" fn cpuidle_state_name(cpu: u32, idlestate: u32) -> *mut c_char {
    cpuidle_state_get_one_string(cpu, idlestate, idlestate_string::IDLESTATE_NAME)
}

#[no_mangle]
pub unsafe extern "C" fn cpuidle_state_desc(cpu: u32, idlestate: u32) -> *mut c_char {
    cpuidle_state_get_one_string(cpu, idlestate, idlestate_string::IDLESTATE_DESC)
}

/*
 * Returns number of supported C-states of CPU core cpu
 * Negativ in error case
 * Zero if cpuidle does not export any C-states
 */
#[no_mangle]
pub unsafe extern "C" fn cpuidle_state_count(cpu: u32) -> u32 {
    let mut file = [0 as c_char; SYSFS_PATH_MAX];
    let mut statbuf: stat = core::mem::zeroed();
    let mut idlestates: c_int = 1;

    snprintf(
        file.as_mut_ptr(),
        SYSFS_PATH_MAX,
        b"%scpuidle\0".as_ptr() as *const c_char,
        PATH_TO_CPU.as_ptr() as *const c_char,
    );
    if stat(file.as_ptr(), &mut statbuf) != 0 || !S_ISDIR(stat_mode(&statbuf)) {
        return 0;
    }

    snprintf(
        file.as_mut_ptr(),
        SYSFS_PATH_MAX,
        b"%scpu%u/cpuidle/state0\0".as_ptr() as *const c_char,
        PATH_TO_CPU.as_ptr() as *const c_char,
        cpu,
    );
    if stat(file.as_ptr(), &mut statbuf) != 0 || !S_ISDIR(stat_mode(&statbuf)) {
        return 0;
    }

    while stat(file.as_ptr(), &mut statbuf) == 0 && S_ISDIR(stat_mode(&statbuf)) {
        snprintf(
            file.as_mut_ptr(),
            SYSFS_PATH_MAX,
            b"%scpu%u/cpuidle/state%d\0".as_ptr() as *const c_char,
            PATH_TO_CPU.as_ptr() as *const c_char,
            cpu,
            idlestates,
        );
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
unsafe fn sysfs_cpuidle_read_file(fname: *const c_char, buf: *mut c_char, buflen: SizeT) -> u32 {
    let mut path = [0 as c_char; SYSFS_PATH_MAX];

    snprintf(
        path.as_mut_ptr(),
        path.len(),
        b"%scpuidle/%s\0".as_ptr() as *const c_char,
        PATH_TO_CPU.as_ptr() as *const c_char,
        fname,
    );

    cpupower_read_sysfs(path.as_ptr(), buf, buflen)
}

/* read access to files which contain one string */

#[repr(u32)]
enum cpuidle_string {
    CPUIDLE_GOVERNOR,
    CPUIDLE_GOVERNOR_RO,
    CPUIDLE_DRIVER,
    MAX_CPUIDLE_STRING_FILES,
}

static cpuidle_string_files: [*const c_char; cpuidle_string::MAX_CPUIDLE_STRING_FILES as usize] = [
    b"current_governor\0".as_ptr() as *const c_char,
    b"current_governor_ro\0".as_ptr() as *const c_char,
    b"current_driver\0".as_ptr() as *const c_char,
];

unsafe fn sysfs_cpuidle_get_one_string(which: cpuidle_string) -> *mut c_char {
    let mut linebuf = [0 as c_char; MAX_LINE_LEN];
    let result: *mut c_char;
    let len: u32;
    let which_index = which as usize;

    if which_index >= cpuidle_string::MAX_CPUIDLE_STRING_FILES as usize {
        return core::ptr::null_mut();
    }

    len = sysfs_cpuidle_read_file(cpuidle_string_files[which_index], linebuf.as_mut_ptr(), linebuf.len());
    if len == 0 {
        return core::ptr::null_mut();
    }

    result = strdup(linebuf.as_ptr());
    if result.is_null() {
        return core::ptr::null_mut();
    }

    *result.add(strcspn(result, b"\n\0".as_ptr() as *const c_char)) = b'\0' as c_char;

    result
}

#[no_mangle]
pub unsafe extern "C" fn cpuidle_get_governor() -> *mut c_char {
    let tmp = sysfs_cpuidle_get_one_string(cpuidle_string::CPUIDLE_GOVERNOR_RO);
    if tmp.is_null() {
        sysfs_cpuidle_get_one_string(cpuidle_string::CPUIDLE_GOVERNOR)
    } else {
        tmp
    }
}

#[no_mangle]
pub unsafe extern "C" fn cpuidle_get_driver() -> *mut c_char {
    sysfs_cpuidle_get_one_string(cpuidle_string::CPUIDLE_DRIVER)
}
/* CPUidle idlestate specific /sys/devices/system/cpu/cpuX/cpuidle/ access */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
