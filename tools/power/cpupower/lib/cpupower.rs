// SPDX-License-Identifier: GPL-2.0-only
/*
 *  (C) 2004-2009  Dominik Brodowski <linux@dominikbrodowski.de>
 */

use libc::{
    access, c_char, c_int, c_ulonglong, c_void, close, free, malloc, open, perror, qsort, read,
    snprintf, ssize_t, stat, stat as stat_fn, strcmp, strtol, strtoull, sysconf, write, EIO,
    EINVAL, ENOMEM, ERANGE, F_OK, O_RDONLY, O_WRONLY, _SC_NPROCESSORS_CONF,
};

// Header-provided constants from cpupower.h / cpupower_intern.h.
const SYSFS_PATH_MAX: usize = 255;
const MAX_LINE_LEN: usize = 4096;
const CPULIST_BUFFER: usize = 128;
const PATH_TO_CPU: &[u8] = b"/sys/devices/system/cpu/\0";

#[repr(C)]
pub struct cpuid_core_info {
    pub cpu: c_int,
    pub pkg: c_int,
    pub core: c_int,
    pub is_online: c_int,
    pub core_cpu_list: [c_char; CPULIST_BUFFER],
}

#[repr(C)]
pub struct cpupower_topology {
    pub core_info: *mut cpuid_core_info,
    pub pkgs: c_int,
    pub cores: c_int,
}

extern "C" {
    static mut errno: c_int;
}

#[no_mangle]
pub unsafe extern "C" fn is_valid_path(path: *const c_char) -> c_int {
    if access(path, F_OK) == -1 {
        return 0;
    }
    1
}

#[no_mangle]
pub unsafe extern "C" fn cpupower_read_sysfs(
    path: *const c_char,
    buf: *mut c_char,
    buflen: usize,
) -> libc::c_uint {
    let numread: ssize_t;
    let fd: c_int;

    fd = open(path, O_RDONLY);
    if fd == -1 {
        return 0;
    }

    numread = read(fd, buf as *mut c_void, buflen - 1);
    if numread < 1 {
        close(fd);
        return 0;
    }

    *buf.offset(numread as isize) = b'\0' as c_char;
    close(fd);

    numread as libc::c_uint
}

#[no_mangle]
pub unsafe extern "C" fn cpupower_write_sysfs(
    path: *const c_char,
    buf: *mut c_char,
    buflen: usize,
) -> libc::c_uint {
    let numwritten: ssize_t;
    let fd: c_int;

    fd = open(path, O_WRONLY);
    if fd == -1 {
        return 0;
    }

    numwritten = write(fd, buf as *const c_void, buflen - 1);
    if numwritten < 1 {
        perror(path);
        close(fd);
        return 0;
    }

    close(fd);

    numwritten as libc::c_uint
}

/*
 * Detect whether a CPU is online
 *
 * Returns:
 *     1 -> if CPU is online
 *     0 -> if CPU is offline
 *     negative errno values in error case
 */
#[no_mangle]
pub unsafe extern "C" fn cpupower_is_cpu_online(cpu: libc::c_uint) -> c_int {
    let mut path: [c_char; SYSFS_PATH_MAX] = [0; SYSFS_PATH_MAX];
    let fd: c_int;
    let numread: ssize_t;
    let value: c_ulonglong;
    let mut linebuf: [c_char; MAX_LINE_LEN] = [0; MAX_LINE_LEN];
    let mut endp: *mut c_char = core::ptr::null_mut();
    let mut statbuf: stat = core::mem::zeroed();

    snprintf(
        path.as_mut_ptr(),
        path.len(),
        b"%scpu%u\0".as_ptr() as *const c_char,
        PATH_TO_CPU.as_ptr() as *const c_char,
        cpu,
    );

    if stat_fn(path.as_ptr(), &mut statbuf) != 0 {
        return 0;
    }

    /*
     * kernel without CONFIG_HOTPLUG_CPU
     * -> cpuX directory exists, but not cpuX/online file
     */
    snprintf(
        path.as_mut_ptr(),
        path.len(),
        b"%scpu%u/online\0".as_ptr() as *const c_char,
        PATH_TO_CPU.as_ptr() as *const c_char,
        cpu,
    );
    if stat_fn(path.as_ptr(), &mut statbuf) != 0 {
        return 1;
    }

    fd = open(path.as_ptr(), O_RDONLY);
    if fd == -1 {
        return -errno;
    }

    numread = read(
        fd,
        linebuf.as_mut_ptr() as *mut c_void,
        MAX_LINE_LEN - 1,
    );
    if numread < 1 {
        close(fd);
        return -EIO;
    }
    linebuf[numread as usize] = b'\0' as c_char;
    close(fd);

    value = strtoull(linebuf.as_ptr(), &mut endp, 0);
    if value > 1 {
        return -EINVAL;
    }

    value as c_int
}

/* returns -1 on failure, 0 on success */
unsafe fn sysfs_topology_read_file(
    cpu: libc::c_uint,
    fname: *const c_char,
    result: *mut c_int,
) -> c_int {
    let mut linebuf: [c_char; MAX_LINE_LEN] = [0; MAX_LINE_LEN];
    let mut endp: *mut c_char = core::ptr::null_mut();
    let mut path: [c_char; SYSFS_PATH_MAX] = [0; SYSFS_PATH_MAX];

    snprintf(
        path.as_mut_ptr(),
        path.len(),
        b"%scpu%u/topology/%s\0".as_ptr() as *const c_char,
        PATH_TO_CPU.as_ptr() as *const c_char,
        cpu,
        fname,
    );
    if cpupower_read_sysfs(path.as_ptr(), linebuf.as_mut_ptr(), MAX_LINE_LEN) == 0 {
        return -1;
    }
    *result = strtol(linebuf.as_ptr(), &mut endp, 0) as c_int;
    if endp == linebuf.as_mut_ptr() || errno == ERANGE {
        return -1;
    }
    0
}

unsafe extern "C" fn __compare(t1: *const c_void, t2: *const c_void) -> c_int {
    let top1: *mut cpuid_core_info = t1 as *mut cpuid_core_info;
    let top2: *mut cpuid_core_info = t2 as *mut cpuid_core_info;
    if (*top1).pkg < (*top2).pkg {
        -1
    } else if (*top1).pkg > (*top2).pkg {
        1
    } else if (*top1).core < (*top2).core {
        -1
    } else if (*top1).core > (*top2).core {
        1
    } else if (*top1).cpu < (*top2).cpu {
        -1
    } else if (*top1).cpu > (*top2).cpu {
        1
    } else {
        0
    }
}

unsafe extern "C" fn __compare_core_cpu_list(t1: *const c_void, t2: *const c_void) -> c_int {
    let top1: *mut cpuid_core_info = t1 as *mut cpuid_core_info;
    let top2: *mut cpuid_core_info = t2 as *mut cpuid_core_info;

    strcmp(
        (*top1).core_cpu_list.as_ptr(),
        (*top2).core_cpu_list.as_ptr(),
    )
}

/*
 * Returns amount of cpus, negative on error, cpu_top must be
 * passed to cpu_topology_release to free resources
 *
 * Array is sorted after ->cpu_smt_list ->pkg, ->core
 */
#[no_mangle]
pub unsafe extern "C" fn get_cpu_topology(cpu_top: *mut cpupower_topology) -> c_int {
    let mut cpu: c_int;
    let mut last_pkg: c_int;
    let cpus: c_int = sysconf(_SC_NPROCESSORS_CONF) as c_int;
    let mut path: [c_char; SYSFS_PATH_MAX] = [0; SYSFS_PATH_MAX];
    let mut last_cpu_list: *mut c_char;

    (*cpu_top).core_info = malloc(core::mem::size_of::<cpuid_core_info>() * cpus as usize)
        as *mut cpuid_core_info;
    if (*cpu_top).core_info.is_null() {
        return -ENOMEM;
    }
    (*cpu_top).pkgs = 0;
    (*cpu_top).cores = (*cpu_top).pkgs;
    cpu = 0;
    while cpu < cpus {
        (*(*cpu_top).core_info.offset(cpu as isize)).cpu = cpu;
        (*(*cpu_top).core_info.offset(cpu as isize)).is_online =
            cpupower_is_cpu_online(cpu as libc::c_uint);
        if sysfs_topology_read_file(
            cpu as libc::c_uint,
            b"physical_package_id\0".as_ptr() as *const c_char,
            &mut (*(*cpu_top).core_info.offset(cpu as isize)).pkg,
        ) < 0
        {
            (*(*cpu_top).core_info.offset(cpu as isize)).pkg = -1;
            (*(*cpu_top).core_info.offset(cpu as isize)).core = -1;
            cpu += 1;
            continue;
        }
        if sysfs_topology_read_file(
            cpu as libc::c_uint,
            b"core_id\0".as_ptr() as *const c_char,
            &mut (*(*cpu_top).core_info.offset(cpu as isize)).core,
        ) < 0
        {
            (*(*cpu_top).core_info.offset(cpu as isize)).pkg = -1;
            (*(*cpu_top).core_info.offset(cpu as isize)).core = -1;
            cpu += 1;
            continue;
        }
        if (*(*cpu_top).core_info.offset(cpu as isize)).core == -1 {
            libc::strncpy(
                (*(*cpu_top).core_info.offset(cpu as isize))
                    .core_cpu_list
                    .as_mut_ptr(),
                b"-1\0".as_ptr() as *const c_char,
                CPULIST_BUFFER,
            );
            cpu += 1;
            continue;
        }
        snprintf(
            path.as_mut_ptr(),
            path.len(),
            b"%scpu%u/topology/%s\0".as_ptr() as *const c_char,
            PATH_TO_CPU.as_ptr() as *const c_char,
            cpu as libc::c_uint,
            b"core_cpus_list\0".as_ptr() as *const c_char,
        );
        if cpupower_read_sysfs(
            path.as_ptr(),
            (*(*cpu_top).core_info.offset(cpu as isize))
                .core_cpu_list
                .as_mut_ptr(),
            CPULIST_BUFFER,
        ) < 1
        {
            libc::printf(
                b"Warning CPU%u has a 0 size core_cpus_list string\0".as_ptr()
                    as *const c_char,
                cpu as libc::c_uint,
            );
        }
        cpu += 1;
    }

    /* Count the number of distinct cpu lists to get the physical core
     * count.
     */
    qsort(
        (*cpu_top).core_info as *mut c_void,
        cpus as usize,
        core::mem::size_of::<cpuid_core_info>(),
        Some(__compare_core_cpu_list),
    );

    last_cpu_list = (*(*cpu_top).core_info.offset(0)).core_cpu_list.as_mut_ptr();
    (*cpu_top).cores = 1;
    cpu = 1;
    while cpu < cpus {
        if strcmp(
            (*(*cpu_top).core_info.offset(cpu as isize))
                .core_cpu_list
                .as_ptr(),
            last_cpu_list,
        ) != 0
            && (*(*cpu_top).core_info.offset(cpu as isize)).pkg != -1
        {
            last_cpu_list = (*(*cpu_top).core_info.offset(cpu as isize))
                .core_cpu_list
                .as_mut_ptr();
            (*cpu_top).cores += 1;
        }
        cpu += 1;
    }

    qsort(
        (*cpu_top).core_info as *mut c_void,
        cpus as usize,
        core::mem::size_of::<cpuid_core_info>(),
        Some(__compare),
    );

    /* Count the number of distinct pkgs values. This works
       because the primary sort of the core_info struct was just
       done by pkg value. */
    last_pkg = (*(*cpu_top).core_info.offset(0)).pkg;
    cpu = 1;
    while cpu < cpus {
        if (*(*cpu_top).core_info.offset(cpu as isize)).pkg != last_pkg
            && (*(*cpu_top).core_info.offset(cpu as isize)).pkg != -1
        {
            last_pkg = (*(*cpu_top).core_info.offset(cpu as isize)).pkg;
            (*cpu_top).pkgs += 1;
        }
        cpu += 1;
    }
    if !((*(*cpu_top).core_info.offset(0)).pkg == -1) {
        (*cpu_top).pkgs += 1;
    }

    cpus
}

#[no_mangle]
pub unsafe extern "C" fn cpu_topology_release(cpu_top: cpupower_topology) {
    free(cpu_top.core_info as *mut c_void);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
