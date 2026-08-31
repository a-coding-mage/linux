// SPDX-License-Identifier: GPL-2.0-only
/*
 *  (C) 2016 SUSE Software Solutions GmbH
 *           Thomas Renninger <trenn@suse.de>
 */

// C dependencies: sys/types.h, sys/stat.h, unistd.h, stdlib.h, string.h,
// fcntl.h, stdio.h, dirent.h, and "powercap.h".

use core::mem::size_of;
use core::ptr;
use std::os::raw::{c_char, c_int, c_longlong, c_uint, c_void};

pub type size_t = usize;
pub type ssize_t = isize;
pub type uint64_t = u64;
pub type mode_t = c_uint;

pub const O_RDONLY: c_int = 0;

pub const SYSFS_PATH_MAX: usize = 255;
pub const MAX_LINE_LEN: usize = 4096;
pub const POWERCAP_MAX_CHILD_ZONES: usize = 10;
pub const POWERCAP_MAX_TREE_DEPTH: c_int = 10;
pub const PATH_TO_POWERCAP: &[u8] = b"/sys/devices/virtual/powercap\0";
pub const PATH_TO_RAPL: &[u8] = b"/sys/devices/virtual/powercap/intel-rapl\0";

#[repr(C)]
pub struct powercap_zone {
    pub name: [c_char; MAX_LINE_LEN],
    pub sys_name: [c_char; SYSFS_PATH_MAX],
    pub parent: *mut powercap_zone,
    pub children: [*mut powercap_zone; POWERCAP_MAX_CHILD_ZONES],
    pub tree_depth: c_int,
    pub has_energy_uj: c_int,
    pub has_power_uw: c_int,
}

#[repr(C)]
pub struct DIR {
    _private: [u8; 0],
}

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dirent {
    pub d_ino: u64,
    pub d_off: i64,
    pub d_reclen: u16,
    pub d_type: u8,
    pub d_name: [c_char; 256],
}

#[repr(C)]
pub struct stat {
    pub st_dev: u64,
    pub st_ino: u64,
    pub st_nlink: u64,
    pub st_mode: mode_t,
    pub st_uid: c_uint,
    pub st_gid: c_uint,
    pub __pad0: c_int,
    pub st_rdev: u64,
    pub st_size: i64,
    pub st_blksize: i64,
    pub st_blocks: i64,
    pub st_atime: i64,
    pub st_atime_nsec: i64,
    pub st_mtime: i64,
    pub st_mtime_nsec: i64,
    pub st_ctime: i64,
    pub st_ctime_nsec: i64,
    pub __glibc_reserved: [i64; 3],
}

unsafe extern "C" {
    static mut stderr: *mut FILE;

    fn open(path: *const c_char, flags: c_int, ...) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn close(fd: c_int) -> c_int;
    fn stat(path: *const c_char, buf: *mut stat) -> c_int;
    fn fstatat(dirfd: c_int, path: *const c_char, buf: *mut stat, flags: c_int) -> c_int;
    fn opendir(name: *const c_char) -> *mut DIR;
    fn readdir(dirp: *mut DIR) -> *mut dirent;
    fn closedir(dirp: *mut DIR) -> c_int;
    fn dirfd(dirp: *mut DIR) -> c_int;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strcat(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strlen(s: *const c_char) -> size_t;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
    fn strtoll(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_longlong;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
}

#[inline]
unsafe fn s_isdir(mode: mode_t) -> bool {
    (mode & 0o170000) == 0o040000
}

unsafe fn init_c_array<const N: usize>(s: &[u8]) -> [c_char; N] {
    let mut out = [0 as c_char; N];
    let mut i = 0usize;
    while i < s.len() && i < N {
        out[i] = s[i] as c_char;
        i += 1;
    }
    out
}

unsafe fn sysfs_read_file(path: *const c_char, buf: *mut c_char, buflen: size_t) -> c_uint {
    let fd: c_int;
    let numread: ssize_t;

    fd = open(path, O_RDONLY);
    if fd == -1 {
        return 0;
    }

    numread = read(fd, buf as *mut c_void, buflen - 1);
    if numread < 1 {
        close(fd);
        return 0;
    }

    *buf.offset(numread as isize) = '\0' as c_char;
    close(fd);

    numread as c_uint
}

unsafe fn sysfs_get_enabled(path: *mut c_char, mode: *mut c_int) -> c_int {
    let fd: c_int;
    let mut yes_no: c_char = 0;
    let mut ret: c_int = 0;

    *mode = 0;

    fd = open(path, O_RDONLY);
    if fd == -1 {
        ret = -1;
        return ret;
    }

    if read(fd, &mut yes_no as *mut c_char as *mut c_void, 1) != 1 {
        ret = -1;
        close(fd);
        return ret;
    }

    if yes_no == '1' as c_char {
        *mode = 1;
        close(fd);
        return ret;
    } else if yes_no == '0' as c_char {
        close(fd);
        return ret;
    } else {
        ret = -1;
        close(fd);
        return ret;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn powercap_get_enabled(mode: *mut c_int) -> c_int {
    let mut path: [c_char; SYSFS_PATH_MAX] =
        init_c_array(b"/sys/devices/virtual/powercap/intel-rapl/enabled\0");

    sysfs_get_enabled(path.as_mut_ptr(), mode)
}

/*
 * TODO: implement function. Returns dummy 0 for now.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn powercap_set_enabled(_mode: c_int) -> c_int {
    0
}

/*
 * Hardcoded, because rapl is the only powercap implementation
- * this needs to get more generic if more powercap implementations
 * should show up
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn powercap_get_driver(driver: *mut c_char, buflen: c_int) -> c_int {
    let mut file: [c_char; SYSFS_PATH_MAX] =
        init_c_array(b"/sys/devices/virtual/powercap/intel-rapl\0");

    let mut statbuf: stat = core::mem::zeroed();

    if stat(file.as_mut_ptr(), &mut statbuf) != 0 || !s_isdir(statbuf.st_mode) {
        let _driver = b"\0".as_ptr() as *const c_char;
        return -1;
    } else if buflen > 10 {
        strcpy(driver, b"intel-rapl\0".as_ptr() as *const c_char);
        return 0;
    } else {
        return -1;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum powercap_get64 {
    GET_ENERGY_UJ,
    GET_MAX_ENERGY_RANGE_UJ,
    GET_POWER_UW,
    GET_MAX_POWER_RANGE_UW,
    MAX_GET_64_FILES,
}

static POWERCAP_GET64_FILES: [*const c_char; powercap_get64::MAX_GET_64_FILES as usize] = [
    b"energy_uj\0".as_ptr() as *const c_char,
    b"max_energy_range_uj\0".as_ptr() as *const c_char,
    b"power_uw\0".as_ptr() as *const c_char,
    b"max_power_range_uw\0".as_ptr() as *const c_char,
];

unsafe fn sysfs_powercap_get64_val(
    zone: *mut powercap_zone,
    which: powercap_get64,
    val: *mut uint64_t,
) -> c_int {
    let mut file: [c_char; SYSFS_PATH_MAX] =
        init_c_array(b"/sys/devices/virtual/powercap/\0");
    let ret: c_int;
    let mut buf: [c_char; MAX_LINE_LEN] = [0; MAX_LINE_LEN];

    strcat(file.as_mut_ptr(), (*zone).sys_name.as_ptr());
    strcat(file.as_mut_ptr(), b"/\0".as_ptr() as *const c_char);
    strcat(
        file.as_mut_ptr(),
        POWERCAP_GET64_FILES[which as usize],
    );

    ret = sysfs_read_file(file.as_mut_ptr(), buf.as_mut_ptr(), MAX_LINE_LEN) as c_int;
    if ret < 0 {
        return ret;
    }
    if ret == 0 {
        return -1;
    }

    *val = strtoll(buf.as_mut_ptr(), ptr::null_mut(), 10) as uint64_t;
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn powercap_get_max_energy_range_uj(
    zone: *mut powercap_zone,
    val: *mut uint64_t,
) -> c_int {
    sysfs_powercap_get64_val(zone, powercap_get64::GET_MAX_ENERGY_RANGE_UJ, val)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn powercap_get_energy_uj(
    zone: *mut powercap_zone,
    val: *mut uint64_t,
) -> c_int {
    sysfs_powercap_get64_val(zone, powercap_get64::GET_ENERGY_UJ, val)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn powercap_get_max_power_range_uw(
    zone: *mut powercap_zone,
    val: *mut uint64_t,
) -> c_int {
    sysfs_powercap_get64_val(zone, powercap_get64::GET_MAX_POWER_RANGE_UW, val)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn powercap_get_power_uw(
    zone: *mut powercap_zone,
    val: *mut uint64_t,
) -> c_int {
    sysfs_powercap_get64_val(zone, powercap_get64::GET_POWER_UW, val)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn powercap_zone_get_enabled(
    zone: *mut powercap_zone,
    mode: *mut c_int,
) -> c_int {
    let mut path: [c_char; SYSFS_PATH_MAX] =
        init_c_array(b"/sys/devices/virtual/powercap\0");

    if (strlen(PATH_TO_POWERCAP.as_ptr() as *const c_char) + strlen((*zone).sys_name.as_ptr()))
        + strlen(b"/enabled\0".as_ptr() as *const c_char)
        + 1
        >= SYSFS_PATH_MAX
    {
        return -1;
    }

    strcat(path.as_mut_ptr(), b"/\0".as_ptr() as *const c_char);
    strcat(path.as_mut_ptr(), (*zone).sys_name.as_ptr());
    strcat(path.as_mut_ptr(), b"/enabled\0".as_ptr() as *const c_char);

    sysfs_get_enabled(path.as_mut_ptr(), mode)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn powercap_zone_set_enabled(
    _zone: *mut powercap_zone,
    _mode: c_int,
) -> c_int {
    /* To be done if needed */
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn powercap_read_zone(zone: *mut powercap_zone) -> c_int {
    let mut dent: *mut dirent;
    let zone_dir: *mut DIR;
    let mut sysfs_dir: [c_char; SYSFS_PATH_MAX] =
        init_c_array(b"/sys/devices/virtual/powercap\0");
    let mut child_zone: *mut powercap_zone;
    let mut file: [c_char; SYSFS_PATH_MAX] =
        init_c_array(b"/sys/devices/virtual/powercap\0");
    let mut i: c_int;
    let mut ret: c_int = 0;
    let mut val: uint64_t = 0;

    strcat(sysfs_dir.as_mut_ptr(), b"/\0".as_ptr() as *const c_char);
    strcat(sysfs_dir.as_mut_ptr(), (*zone).sys_name.as_ptr());

    zone_dir = opendir(sysfs_dir.as_mut_ptr());
    if zone_dir.is_null() {
        return -1;
    }

    strcat(file.as_mut_ptr(), b"/\0".as_ptr() as *const c_char);
    strcat(file.as_mut_ptr(), (*zone).sys_name.as_ptr());
    strcat(file.as_mut_ptr(), b"/name\0".as_ptr() as *const c_char);
    sysfs_read_file(file.as_mut_ptr(), (*zone).name.as_mut_ptr(), MAX_LINE_LEN);
    if !(*zone).parent.is_null() {
        (*zone).tree_depth = (*(*zone).parent).tree_depth + 1;
    }
    ret = powercap_get_energy_uj(zone, &mut val);
    if ret == 0 {
        (*zone).has_energy_uj = 1;
    }
    ret = powercap_get_power_uw(zone, &mut val);
    if ret == 0 {
        (*zone).has_power_uw = 1;
    }

    loop {
        dent = readdir(zone_dir);
        if dent.is_null() {
            break;
        }
        let mut st: stat = core::mem::zeroed();

        if strcmp((*dent).d_name.as_ptr(), b".\0".as_ptr() as *const c_char) == 0
            || strcmp((*dent).d_name.as_ptr(), b"..\0".as_ptr() as *const c_char) == 0
        {
            continue;
        }

        if stat((*dent).d_name.as_ptr(), &mut st) != 0 || !s_isdir(st.st_mode) {
            if fstatat(dirfd(zone_dir), (*dent).d_name.as_ptr(), &mut st, 0) < 0 {
                continue;
            }
        }

        if strncmp(
            (*dent).d_name.as_ptr(),
            b"intel-rapl:\0".as_ptr() as *const c_char,
            11,
        ) != 0
        {
            continue;
        }

        child_zone = calloc(1, size_of::<powercap_zone>()) as *mut powercap_zone;
        if child_zone.is_null() {
            return -1;
        }
        i = 0;
        while i < POWERCAP_MAX_CHILD_ZONES as c_int {
            if (*zone).children[i as usize].is_null() {
                (*zone).children[i as usize] = child_zone;
                break;
            }
            if i == POWERCAP_MAX_CHILD_ZONES as c_int - 1 {
                free(child_zone as *mut c_void);
                fprintf(
                    stderr,
                    b"Reached POWERCAP_MAX_CHILD_ZONES %d\n\0".as_ptr() as *const c_char,
                    POWERCAP_MAX_CHILD_ZONES as c_int,
                );
                return -1;
            }
            i += 1;
        }
        strcpy((*child_zone).sys_name.as_mut_ptr(), (*zone).sys_name.as_ptr());
        strcat(
            (*child_zone).sys_name.as_mut_ptr(),
            b"/\0".as_ptr() as *const c_char,
        );
        strcat((*child_zone).sys_name.as_mut_ptr(), (*dent).d_name.as_ptr());
        (*child_zone).parent = zone;
        if (*zone).tree_depth >= POWERCAP_MAX_TREE_DEPTH {
            fprintf(
                stderr,
                b"Maximum zone hierarchy depth[%d] reached\n\0".as_ptr() as *const c_char,
                POWERCAP_MAX_TREE_DEPTH,
            );
            ret = -1;
            break;
        }
        powercap_read_zone(child_zone);
    }
    closedir(zone_dir);
    ret
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn powercap_init_zones() -> *mut powercap_zone {
    let mut enabled: c_int = 0;
    let root_zone: *mut powercap_zone;
    let ret: c_int;
    let mut file: [c_char; SYSFS_PATH_MAX] =
        init_c_array(b"/sys/devices/virtual/powercap/intel-rapl/enabled\0");

    ret = sysfs_get_enabled(file.as_mut_ptr(), &mut enabled);

    if ret != 0 {
        return ptr::null_mut();
    }

    if enabled == 0 {
        return ptr::null_mut();
    }

    root_zone = calloc(1, size_of::<powercap_zone>()) as *mut powercap_zone;
    if root_zone.is_null() {
        return ptr::null_mut();
    }

    strcpy(
        (*root_zone).sys_name.as_mut_ptr(),
        b"intel-rapl/intel-rapl:0\0".as_ptr() as *const c_char,
    );

    powercap_read_zone(root_zone);

    root_zone
}

/* Call function *f on the passed zone and all its children */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn powercap_walk_zones(
    zone: *mut powercap_zone,
    f: Option<unsafe extern "C" fn(zone: *mut powercap_zone) -> c_int>,
) -> c_int {
    let mut i: c_int;
    let ret: c_int;

    if zone.is_null() {
        return -1;
    }

    ret = f.unwrap()(zone);
    if ret != 0 {
        return ret;
    }

    i = 0;
    while i < POWERCAP_MAX_CHILD_ZONES as c_int {
        if !(*zone).children[i as usize].is_null() {
            powercap_walk_zones((*zone).children[i as usize], f);
        }
        i += 1;
    }
    0
}
