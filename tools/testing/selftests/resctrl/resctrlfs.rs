// SPDX-License-Identifier: GPL-2.0
/*
 * Basic resctrl file system operations
 *
 * Copyright (C) 2018 Intel Corporation
 *
 * Authors:
 *    Sai Praneeth Prakhya <sai.praneeth.prakhya@intel.com>,
 *    Fenghua Yu <fenghua.yu@intel.com>
 */

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct DIR {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dirent {
    pub d_ino: c_ulong,
    pub d_off: c_long,
    pub d_reclen: u16,
    pub d_type: u8,
    pub d_name: [c_char; 256],
}

#[repr(C)]
pub struct stat {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_event_attr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct resctrl_test {
    pub resource: *const c_char,
}

pub type pid_t = c_int;

#[repr(C)]
pub struct cpu_set_t {
    pub __bits: [c_ulong; 16],
}

pub const ENXIO: c_int = 6;
pub const ENOENT: c_int = 2;
pub const EOF: c_int = -1;
pub const PATH_MAX: usize = 4096;
pub const O_WRONLY: c_int = 1;
pub const STDOUT_FILENO: c_int = 1;
pub const SIGTERM: c_int = 15;
pub const __NR_perf_event_open: c_long = 298;

/* Constants supplied by resctrl.h in the original C translation unit. */
unsafe extern "C" {
    static RESCTRL_PATH: *const c_char;
    static INFO_PATH: *const c_char;
    static PHYS_ID_PATH: *const c_char;
    static mut errno: c_int;
}

unsafe extern "C" {
    fn fopen(path: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fclose(stream: *mut FILE) -> c_int;
    fn feof(stream: *mut FILE) -> c_int;
    fn fgets(s: *mut c_char, size: c_int, stream: *mut FILE) -> *mut c_char;
    fn fgetc(stream: *mut FILE) -> c_int;
    fn fscanf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn fflush(stream: *mut FILE) -> c_int;
    fn fdopen(fd: c_int, mode: *const c_char) -> *mut FILE;
    static mut stdout: *mut FILE;

    fn strtok(str_: *mut c_char, delim: *const c_char) -> *mut c_char;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn strncpy(dest: *mut c_char, src: *const c_char, n: usize) -> *mut c_char;
    fn strlen(s: *const c_char) -> usize;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn snprintf(s: *mut c_char, n: usize, format: *const c_char, ...) -> c_int;
    fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int;
    fn free(ptr: *mut c_void);

    fn mount(source: *const c_char, target: *const c_char, filesystemtype: *const c_char,
             mountflags: c_ulong, data: *const c_void) -> c_int;
    fn umount(target: *const c_char) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> isize;
    fn pipe(pipefd: *mut c_int) -> c_int;
    fn fork() -> pid_t;
    fn dup2(oldfd: c_int, newfd: c_int) -> c_int;
    fn execlp(file: *const c_char, arg: *const c_char, ...) -> c_int;
    fn exit(status: c_int) -> !;
    fn kill(pid: pid_t, sig: c_int) -> c_int;
    fn waitpid(pid: pid_t, wstatus: *mut c_int, options: c_int) -> pid_t;
    fn syscall(number: c_long, ...) -> c_long;

    fn opendir(name: *const c_char) -> *mut DIR;
    fn readdir(dirp: *mut DIR) -> *mut dirent;
    fn closedir(dirp: *mut DIR) -> c_int;
    fn mkdir(pathname: *const c_char, mode: c_int) -> c_int;
    fn stat(pathname: *const c_char, statbuf: *mut stat) -> c_int;

    fn sched_getaffinity(pid: pid_t, cpusetsize: usize, mask: *mut cpu_set_t) -> c_int;
    fn sched_setaffinity(pid: pid_t, cpusetsize: usize, mask: *const cpu_set_t) -> c_int;

    fn ksft_perror(msg: *const c_char);
    fn ksft_print_msg(format: *const c_char, ...);
}

#[no_mangle]
pub static mut snc_unreliable: c_int = 0;

unsafe fn cpu_zero(set: *mut cpu_set_t) {
    ptr::write_bytes(set as *mut u8, 0, size_of::<cpu_set_t>());
}

unsafe fn cpu_set(cpu: c_int, set: *mut cpu_set_t) {
    let bits_per_word = 8 * size_of::<c_ulong>() as c_int;
    let idx = (cpu / bits_per_word) as usize;
    let bit = (cpu % bits_per_word) as u32;
    (*set).__bits[idx] |= (1 as c_ulong) << bit;
}

unsafe fn isxdigit(c: c_int) -> bool {
    (c >= b'0' as c_int && c <= b'9' as c_int)
        || (c >= b'a' as c_int && c <= b'f' as c_int)
        || (c >= b'A' as c_int && c <= b'F' as c_int)
}

unsafe fn ffsl(mut i: c_long) -> c_int {
    if i == 0 {
        return 0;
    }
    let mut bit = 1;
    while (i & 1) == 0 {
        bit += 1;
        i >>= 1;
    }
    bit
}

unsafe fn find_resctrl_mount(buffer: *mut c_char) -> c_int {
    let mut line = [0 as c_char; 256];
    let mut fs: *mut c_char;
    let mut mntpoint: *mut c_char;

    let mounts = fopen(c"/proc/mounts".as_ptr(), c"r".as_ptr());
    if mounts.is_null() {
        ksft_perror(c"/proc/mounts".as_ptr());
        return -ENXIO;
    }
    while feof(mounts) == 0 {
        if fgets(line.as_mut_ptr(), 256, mounts).is_null() {
            break;
        }
        fs = strtok(line.as_mut_ptr(), c" \t".as_ptr());
        if fs.is_null() {
            continue;
        }
        mntpoint = strtok(ptr::null_mut(), c" \t".as_ptr());
        if mntpoint.is_null() {
            continue;
        }
        fs = strtok(ptr::null_mut(), c" \t".as_ptr());
        if fs.is_null() {
            continue;
        }
        if strcmp(fs, c"resctrl".as_ptr()) != 0 {
            continue;
        }

        fclose(mounts);
        if !buffer.is_null() {
            strncpy(buffer, mntpoint, 256);
        }

        return 0;
    }

    fclose(mounts);

    -ENOENT
}

/*
 * mount_resctrlfs - Mount resctrl FS at /sys/fs/resctrl
 *
 * Mounts resctrl FS. Fails if resctrl FS is already mounted to avoid
 * pre-existing settings interfering with the test results.
 *
 * Return: 0 on success, < 0 on error.
 */
#[no_mangle]
pub unsafe extern "C" fn mount_resctrlfs() -> c_int {
    let mut ret: c_int;

    ret = find_resctrl_mount(ptr::null_mut());
    if ret != -ENOENT {
        return -1;
    }

    ksft_print_msg(c"Mounting resctrl to \"%s\"\n".as_ptr(), RESCTRL_PATH);
    ret = mount(c"resctrl".as_ptr(), RESCTRL_PATH, c"resctrl".as_ptr(), 0, ptr::null());
    if ret != 0 {
        ksft_perror(c"mount".as_ptr());
    }

    ret
}

#[no_mangle]
pub unsafe extern "C" fn umount_resctrlfs() -> c_int {
    let mut mountpoint = [0 as c_char; 256];
    let ret: c_int;

    ret = find_resctrl_mount(mountpoint.as_mut_ptr());
    if ret == -ENOENT {
        return 0;
    }
    if ret != 0 {
        return ret;
    }

    if umount(mountpoint.as_ptr()) != 0 {
        ksft_perror(c"Unable to umount resctrl".as_ptr());

        return -1;
    }

    0
}

/*
 * get_cache_level - Convert cache level from string to integer
 * @cache_type:		Cache level as string
 *
 * Return: cache level as integer or -1 if @cache_type is invalid.
 */
unsafe fn get_cache_level(cache_type: *const c_char) -> c_int {
    if strcmp(cache_type, c"L3".as_ptr()) == 0 {
        return 3;
    }
    if strcmp(cache_type, c"L2".as_ptr()) == 0 {
        return 2;
    }

    ksft_print_msg(c"Invalid cache level\n".as_ptr());
    -1
}

unsafe fn get_resource_cache_level(resource: *const c_char) -> c_int {
    /* "MB" use L3 (LLC) as resource */
    if strcmp(resource, c"MB".as_ptr()) == 0 {
        return 3;
    }
    get_cache_level(resource)
}

/*
 * get_domain_id - Get resctrl domain ID for a specified CPU
 * @resource:	resource name
 * @cpu_no:	CPU number
 * @domain_id:	domain ID (cache ID; for MB, L3 cache ID)
 *
 * Return: >= 0 on success, < 0 on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn get_domain_id(
    resource: *const c_char,
    cpu_no: c_int,
    domain_id: *mut c_int,
) -> c_int {
    let mut phys_pkg_path = [0 as c_char; 1024];
    let cache_num: c_int;
    let fp: *mut FILE;

    cache_num = get_resource_cache_level(resource);
    if cache_num < 0 {
        return cache_num;
    }

    sprintf(
        phys_pkg_path.as_mut_ptr(),
        c"%s%d/cache/index%d/id".as_ptr(),
        PHYS_ID_PATH,
        cpu_no,
        cache_num,
    );

    fp = fopen(phys_pkg_path.as_ptr(), c"r".as_ptr());
    if fp.is_null() {
        ksft_perror(c"Failed to open cache id file".as_ptr());

        return -1;
    }
    if fscanf(fp, c"%d".as_ptr(), domain_id) <= 0 {
        ksft_perror(c"Could not get domain ID".as_ptr());
        fclose(fp);

        return -1;
    }
    fclose(fp);

    0
}

/*
 * Count number of CPUs in a /sys bitmap
 */
unsafe fn count_sys_bitmap_bits(name: *mut c_char) -> u32 {
    let fp = fopen(name, c"r".as_ptr());
    let mut count: c_int = 0;
    let mut c: c_int;

    if fp.is_null() {
        return 0;
    }

    loop {
        c = fgetc(fp);
        if c == EOF {
            break;
        }
        if !isxdigit(c) {
            continue;
        }
        match c as u8 {
            b'f' => {
                count += 1;
                count += 1;
                count += 1;
                count += 1;
            }
            b'7' | b'b' | b'd' | b'e' => {
                count += 1;
                count += 1;
                count += 1;
            }
            b'3' | b'5' | b'6' | b'9' | b'a' | b'c' => {
                count += 1;
                count += 1;
            }
            b'1' | b'2' | b'4' | b'8' => {
                count += 1;
            }
            _ => {}
        }
    }
    fclose(fp);

    count as u32
}

unsafe fn cpus_offline_empty() -> bool {
    let mut offline_cpus_str = [0 as c_char; 64];
    let fp: *mut FILE;

    fp = fopen(c"/sys/devices/system/cpu/offline".as_ptr(), c"r".as_ptr());
    if fp.is_null() {
        ksft_perror(c"Could not open /sys/devices/system/cpu/offline".as_ptr());
        return false;
    }

    if fscanf(fp, c"%63s".as_ptr(), offline_cpus_str.as_mut_ptr()) < 0 {
        if errno == 0 {
            fclose(fp);
            return true;
        }
        ksft_perror(c"Could not read /sys/devices/system/cpu/offline".as_ptr());
    }

    fclose(fp);

    false
}

/*
 * Detect SNC by comparing #CPUs in node0 with #CPUs sharing LLC with CPU0.
 * If any CPUs are offline declare the detection as unreliable.
 */
#[no_mangle]
pub unsafe extern "C" fn snc_nodes_per_l3_cache() -> c_int {
    let node_cpus: c_int;
    let cache_cpus: c_int;
    static mut SNC_MODE: c_int = 0;

    if SNC_MODE == 0 {
        SNC_MODE = 1;
        if !cpus_offline_empty() {
            ksft_print_msg(c"Runtime SNC detection unreliable due to offline CPUs.\n".as_ptr());
            ksft_print_msg(c"Setting SNC mode to disabled.\n".as_ptr());
            snc_unreliable = 1;
            return SNC_MODE;
        }
        node_cpus = count_sys_bitmap_bits(c"/sys/devices/system/node/node0/cpumap".as_ptr() as *mut c_char) as c_int;
        cache_cpus = count_sys_bitmap_bits(c"/sys/devices/system/cpu/cpu0/cache/index3/shared_cpu_map".as_ptr() as *mut c_char) as c_int;

        if node_cpus == 0 || cache_cpus == 0 {
            ksft_print_msg(c"Could not determine Sub-NUMA Cluster mode.\n".as_ptr());
            snc_unreliable = 1;
            return SNC_MODE;
        }
        SNC_MODE = cache_cpus / node_cpus;

        /*
         * On some platforms (e.g. Hygon),
         * cache_cpus < node_cpus, the calculated snc_mode is 0.
         *
         * Set snc_mode = 1 to indicate that SNC mode is not
         * supported on the platform.
         */
        if SNC_MODE == 0 {
            SNC_MODE = 1;
        }

        if SNC_MODE > 1 {
            ksft_print_msg(c"SNC-%d mode discovered.\n".as_ptr(), SNC_MODE);
        }
    }

    SNC_MODE
}

/*
 * get_cache_size - Get cache size for a specified CPU
 * @cpu_no:	CPU number
 * @cache_type:	Cache level L2/L3
 * @cache_size:	pointer to cache_size
 *
 * Return: = 0 on success, < 0 on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn get_cache_size(
    cpu_no: c_int,
    cache_type: *const c_char,
    cache_size: *mut c_ulong,
) -> c_int {
    let mut cache_path = [0 as c_char; 1024];
    let mut cache_str = [0 as c_char; 64];
    let length: c_int;
    let mut i: c_int;
    let cache_num: c_int;
    let fp: *mut FILE;

    cache_num = get_cache_level(cache_type);
    if cache_num < 0 {
        return cache_num;
    }

    sprintf(
        cache_path.as_mut_ptr(),
        c"/sys/bus/cpu/devices/cpu%d/cache/index%d/size".as_ptr(),
        cpu_no,
        cache_num,
    );
    fp = fopen(cache_path.as_ptr(), c"r".as_ptr());
    if fp.is_null() {
        ksft_perror(c"Failed to open cache size".as_ptr());

        return -1;
    }
    if fscanf(fp, c"%63s".as_ptr(), cache_str.as_mut_ptr()) <= 0 {
        ksft_perror(c"Could not get cache_size".as_ptr());
        fclose(fp);

        return -1;
    }
    fclose(fp);

    length = strlen(cache_str.as_ptr()) as c_int;

    *cache_size = 0;

    i = 0;
    while i < length {
        let ch = cache_str[i as usize] as u8;
        if ch >= b'0' && ch <= b'9' {
            *cache_size = *cache_size * 10 + (ch - b'0') as c_ulong;
        } else if ch == b'K' {
            *cache_size = *cache_size * 1024;
        } else if ch == b'M' {
            *cache_size = *cache_size * 1024 * 1024;
        } else {
            break;
        }
        i += 1;
    }

    /*
     * The amount of cache represented by each bit in the masks
     * in the schemata file is reduced by a factor equal to SNC
     * nodes per L3 cache.
     * E.g. on a SNC-2 system with a 100MB L3 cache a test that
     * allocates memory from its local SNC node (default behavior
     * without using libnuma) will only see 50 MB llc_occupancy
     * with a fully populated L3 mask in the schemata file.
     */
    if cache_num == 3 {
        *cache_size /= snc_nodes_per_l3_cache() as c_ulong;
    }
    0
}

const CORE_SIBLINGS_PATH: *const c_char = c"/sys/bus/cpu/devices/cpu".as_ptr();

/*
 * get_bit_mask - Get bit mask from given file
 * @filename:	File containing the mask
 * @mask:	The bit mask returned as unsigned long
 *
 * Return: = 0 on success, < 0 on failure.
 */
unsafe fn get_bit_mask(filename: *const c_char, mask: *mut c_ulong) -> c_int {
    let fp: *mut FILE;

    if filename.is_null() || mask.is_null() {
        return -1;
    }

    fp = fopen(filename, c"r".as_ptr());
    if fp.is_null() {
        ksft_print_msg(
            c"Failed to open bit mask file '%s': %s\n".as_ptr(),
            filename,
            strerror(errno),
        );
        return -1;
    }

    if fscanf(fp, c"%lx".as_ptr(), mask) <= 0 {
        ksft_print_msg(
            c"Could not read bit mask file '%s': %s\n".as_ptr(),
            filename,
            strerror(errno),
        );
        fclose(fp);

        return -1;
    }
    fclose(fp);

    0
}

/*
 * resource_info_unsigned_get - Read an unsigned value from
 * /sys/fs/resctrl/info/@resource/@filename
 * @resource:	Resource name that matches directory name in
 *		/sys/fs/resctrl/info
 * @filename:	File in /sys/fs/resctrl/info/@resource
 * @val:	Contains read value on success.
 *
 * Return: = 0 on success, < 0 on failure. On success the read
 * value is saved into @val.
 */
#[no_mangle]
pub unsafe extern "C" fn resource_info_unsigned_get(
    resource: *const c_char,
    filename: *const c_char,
    val: *mut u32,
) -> c_int {
    let mut file_path = [0 as c_char; PATH_MAX];
    let fp: *mut FILE;

    snprintf(
        file_path.as_mut_ptr(),
        size_of::<[c_char; PATH_MAX]>(),
        c"%s/%s/%s".as_ptr(),
        INFO_PATH,
        resource,
        filename,
    );

    fp = fopen(file_path.as_ptr(), c"r".as_ptr());
    if fp.is_null() {
        ksft_print_msg(c"Error opening %s: %m\n".as_ptr(), file_path.as_ptr());
        return -1;
    }

    if fscanf(fp, c"%u".as_ptr(), val) <= 0 {
        ksft_print_msg(c"Could not get contents of %s: %m\n".as_ptr(), file_path.as_ptr());
        fclose(fp);
        return -1;
    }

    fclose(fp);
    0
}

/*
 * create_bit_mask- Create bit mask from start, len pair
 * @start:	LSB of the mask
 * @len		Number of bits in the mask
 */
#[no_mangle]
pub extern "C" fn create_bit_mask(start: u32, len: u32) -> c_ulong {
    (((1 as c_ulong) << len) - 1) << start
}

/*
 * count_contiguous_bits - Returns the longest train of bits in a bit mask
 * @val		A bit mask
 * @start	The location of the least-significant bit of the longest train
 *
 * Return:	The length of the contiguous bits in the longest train of bits
 */
#[no_mangle]
pub unsafe extern "C" fn count_contiguous_bits(mut val: c_ulong, start: *mut u32) -> u32 {
    let mut last_val: c_ulong = 0;
    let mut count: u32 = 0;

    while val != 0 {
        last_val = val;
        val &= val >> 1;
        count += 1;
    }

    if !start.is_null() {
        if count != 0 {
            *start = (ffsl(last_val as c_long) - 1) as u32;
        } else {
            *start = 0;
        }
    }

    count
}

/*
 * get_full_cbm - Get full Cache Bit Mask (CBM)
 * @cache_type:	Cache type as "L2" or "L3"
 * @mask:	Full cache bit mask representing the maximal portion of cache
 *		available for allocation, returned as unsigned long.
 *
 * Return: = 0 on success, < 0 on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn get_full_cbm(cache_type: *const c_char, mask: *mut c_ulong) -> c_int {
    let mut cbm_path = [0 as c_char; PATH_MAX];
    let ret: c_int;

    if cache_type.is_null() {
        return -1;
    }

    snprintf(cbm_path.as_mut_ptr(), size_of::<[c_char; PATH_MAX]>(), c"%s/%s/cbm_mask".as_ptr(), INFO_PATH, cache_type);

    ret = get_bit_mask(cbm_path.as_ptr(), mask);
    if ret != 0 || *mask == 0 {
        return -1;
    }

    0
}

/*
 * get_shareable_mask - Get shareable mask from shareable_bits
 * @cache_type:		Cache type as "L2" or "L3"
 * @shareable_mask:	Shareable mask returned as unsigned long
 *
 * Return: = 0 on success, < 0 on failure.
 */
unsafe fn get_shareable_mask(cache_type: *const c_char, shareable_mask: *mut c_ulong) -> c_int {
    let mut mask_path = [0 as c_char; PATH_MAX];

    if cache_type.is_null() {
        return -1;
    }

    snprintf(mask_path.as_mut_ptr(), size_of::<[c_char; PATH_MAX]>(), c"%s/%s/shareable_bits".as_ptr(), INFO_PATH, cache_type);

    get_bit_mask(mask_path.as_ptr(), shareable_mask)
}

/*
 * get_mask_no_shareable - Get Cache Bit Mask (CBM) without shareable bits
 * @cache_type:		The largest exclusive portion of the cache out of the
 *			full CBM, returned as unsigned long
 *
 * Parts of a cache may be shared with other devices such as GPU. This function
 * calculates the largest exclusive portion of the cache where no other devices
 * besides CPU have access to the cache portion.
 *
 * Return: = 0 on success, < 0 on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn get_mask_no_shareable(cache_type: *const c_char, mask: *mut c_ulong) -> c_int {
    let mut full_mask: c_ulong = 0;
    let mut shareable_mask: c_ulong = 0;
    let mut start: u32 = 0;
    let len: u32;

    if get_full_cbm(cache_type, &mut full_mask) < 0 {
        return -1;
    }
    if get_shareable_mask(cache_type, &mut shareable_mask) < 0 {
        return -1;
    }

    len = count_contiguous_bits(full_mask & !shareable_mask, &mut start);
    if len == 0 {
        return -1;
    }

    *mask = create_bit_mask(start, len);

    0
}

/*
 * taskset_benchmark - Taskset PID (i.e. benchmark) to a specified cpu
 * @bm_pid:		PID that should be binded
 * @cpu_no:		CPU number at which the PID would be binded
 * @old_affinity:	When not NULL, set to old CPU affinity
 *
 * Return: 0 on success, < 0 on error.
 */
#[no_mangle]
pub unsafe extern "C" fn taskset_benchmark(
    bm_pid: pid_t,
    cpu_no: c_int,
    old_affinity: *mut cpu_set_t,
) -> c_int {
    let mut my_set: cpu_set_t = core::mem::zeroed();

    if !old_affinity.is_null() {
        cpu_zero(old_affinity);
        if sched_getaffinity(bm_pid, size_of::<cpu_set_t>(), old_affinity) != 0 {
            ksft_perror(c"Unable to read CPU affinity".as_ptr());
            return -1;
        }
    }

    cpu_zero(&mut my_set);
    cpu_set(cpu_no, &mut my_set);

    if sched_setaffinity(bm_pid, size_of::<cpu_set_t>(), &my_set) != 0 {
        ksft_perror(c"Unable to taskset benchmark".as_ptr());

        return -1;
    }

    0
}

/*
 * taskset_restore - Taskset PID to the earlier CPU affinity
 * @bm_pid:		PID that should be reset
 * @old_affinity:	The old CPU affinity to restore
 *
 * Return: 0 on success, < 0 on error.
 */
#[no_mangle]
pub unsafe extern "C" fn taskset_restore(bm_pid: pid_t, old_affinity: *mut cpu_set_t) -> c_int {
    if sched_setaffinity(bm_pid, size_of::<cpu_set_t>(), old_affinity) != 0 {
        ksft_perror(c"Unable to restore CPU affinity".as_ptr());
        return -1;
    }

    0
}

/*
 * create_grp - Create a group only if one doesn't exist
 * @grp_name:	Name of the group
 * @grp:	Full path and name of the group
 * @parent_grp:	Full path and name of the parent group
 *
 * Creates a group @grp_name if it does not exist yet. If @grp_name is NULL,
 * it is interpreted as the root group which always results in success.
 *
 * Return: 0 on success, < 0 on error.
 */
unsafe fn create_grp(grp_name: *const c_char, grp: *mut c_char, parent_grp: *const c_char) -> c_int {
    let mut found_grp: c_int = 0;
    let mut ep: *mut dirent;
    let dp: *mut DIR;

    if grp_name.is_null() {
        return 0;
    }

    /* Check if requested grp exists or not */
    dp = opendir(parent_grp);
    if !dp.is_null() {
        loop {
            ep = readdir(dp);
            if ep.is_null() {
                break;
            }
            if strcmp((*ep).d_name.as_ptr(), grp_name) == 0 {
                found_grp = 1;
            }
        }
        closedir(dp);
    } else {
        ksft_perror(c"Unable to open resctrl for group".as_ptr());

        return -1;
    }

    /* Requested grp doesn't exist, hence create it */
    if found_grp == 0 {
        if mkdir(grp, 0) == -1 {
            ksft_perror(c"Unable to create group".as_ptr());

            return -1;
        }
    }

    0
}

unsafe fn write_pid_to_tasks(tasks: *mut c_char, pid: pid_t) -> c_int {
    let fp: *mut FILE;

    fp = fopen(tasks, c"w".as_ptr());
    if fp.is_null() {
        ksft_perror(c"Failed to open tasks file".as_ptr());

        return -1;
    }
    if fprintf(fp, c"%d\n".as_ptr(), pid as c_int) < 0 {
        ksft_print_msg(c"Failed to write pid to tasks file\n".as_ptr());
        fclose(fp);

        return -1;
    }
    fclose(fp);

    0
}

/*
 * write_bm_pid_to_resctrl - Write a PID (i.e. benchmark) to resctrl FS
 * @bm_pid:		PID that should be written
 * @ctrlgrp:		Name of the control monitor group (con_mon grp)
 * @mongrp:		Name of the monitor group (mon grp)
 *
 * If a con_mon grp is requested, create it and write pid to it, otherwise
 * write pid to root con_mon grp.
 * If a mon grp is requested, create it and write pid to it, otherwise
 * pid is not written, this means that pid is in con_mon grp and hence
 * should consult con_mon grp's mon_data directory for results.
 *
 * Return: 0 on success, < 0 on error.
 */
#[no_mangle]
pub unsafe extern "C" fn write_bm_pid_to_resctrl(
    bm_pid: pid_t,
    ctrlgrp: *const c_char,
    mongrp: *const c_char,
) -> c_int {
    let mut controlgroup = [0 as c_char; 128];
    let mut monitorgroup = [0 as c_char; 512];
    let mut monitorgroup_p = [0 as c_char; 256];
    let mut tasks = [0 as c_char; 1024];
    let mut ret: c_int = 0;

    if !ctrlgrp.is_null() {
        sprintf(controlgroup.as_mut_ptr(), c"%s/%s".as_ptr(), RESCTRL_PATH, ctrlgrp);
    } else {
        sprintf(controlgroup.as_mut_ptr(), c"%s".as_ptr(), RESCTRL_PATH);
    }

    /* Create control and monitoring group and write pid into it */
    ret = create_grp(ctrlgrp, controlgroup.as_mut_ptr(), RESCTRL_PATH);
    if ret != 0 {
        goto_out(ret);
        return ret;
    }
    sprintf(tasks.as_mut_ptr(), c"%s/tasks".as_ptr(), controlgroup.as_ptr());
    ret = write_pid_to_tasks(tasks.as_mut_ptr(), bm_pid);
    if ret != 0 {
        goto_out(ret);
        return ret;
    }

    /* Create monitor group and write pid into if it is used */
    if !mongrp.is_null() {
        sprintf(monitorgroup_p.as_mut_ptr(), c"%s/mon_groups".as_ptr(), controlgroup.as_ptr());
        sprintf(monitorgroup.as_mut_ptr(), c"%s/%s".as_ptr(), monitorgroup_p.as_ptr(), mongrp);
        ret = create_grp(mongrp, monitorgroup.as_mut_ptr(), monitorgroup_p.as_ptr());
        if ret != 0 {
            goto_out(ret);
            return ret;
        }

        sprintf(tasks.as_mut_ptr(), c"%s/mon_groups/%s/tasks".as_ptr(), controlgroup.as_ptr(), mongrp);
        ret = write_pid_to_tasks(tasks.as_mut_ptr(), bm_pid);
        if ret != 0 {
            goto_out(ret);
            return ret;
        }
    }

    goto_out(ret);
    ret
}

unsafe fn goto_out(ret: c_int) {
    ksft_print_msg(c"Writing benchmark parameters to resctrl FS\n".as_ptr());
    if ret != 0 {
        ksft_print_msg(c"Failed writing to resctrlfs\n".as_ptr());
    }
}

/*
 * write_schemata - Update schemata of a con_mon grp
 * @ctrlgrp:		Name of the con_mon grp
 * @schemata:		Schemata that should be updated to
 * @cpu_no:		CPU number that the benchmark PID is binded to
 * @resource:		Resctrl resource (Eg: MB, L3, L2, etc.)
 *
 * Update schemata of a con_mon grp *only* if requested resctrl resource is
 * allocation type
 *
 * Return: 0 on success, < 0 on error.
 */
#[no_mangle]
pub unsafe extern "C" fn write_schemata(
    ctrlgrp: *const c_char,
    schemata: *mut c_char,
    cpu_no: c_int,
    resource: *const c_char,
) -> c_int {
    let mut controlgroup = [0 as c_char; 1024];
    let mut reason = [0 as c_char; 128];
    let mut schema = [0 as c_char; 1024];
    let mut domain_id: c_int = 0;
    let fd: c_int;
    let schema_len: c_int;
    let mut ret: c_int = 0;

    if schemata.is_null() {
        ksft_print_msg(c"Skipping empty schemata update\n".as_ptr());

        return -1;
    }

    if get_domain_id(resource, cpu_no, &mut domain_id) < 0 {
        sprintf(reason.as_mut_ptr(), c"Failed to get domain ID".as_ptr());
        ret = -1;

        ksft_print_msg(
            c"Write schema \"%s\" to resctrl FS%s%s\n".as_ptr(),
            schema.as_ptr(),
            if ret != 0 { c" # ".as_ptr() } else { c"".as_ptr() },
            if ret != 0 { reason.as_ptr() } else { c"".as_ptr() },
        );
        return ret;
    }

    if !ctrlgrp.is_null() {
        sprintf(controlgroup.as_mut_ptr(), c"%s/%s/schemata".as_ptr(), RESCTRL_PATH, ctrlgrp);
    } else {
        sprintf(controlgroup.as_mut_ptr(), c"%s/schemata".as_ptr(), RESCTRL_PATH);
    }

    schema_len = snprintf(
        schema.as_mut_ptr(),
        size_of::<[c_char; 1024]>(),
        c"%s:%d=%s\n".as_ptr(),
        resource,
        domain_id,
        schemata,
    );
    if schema_len < 0 || schema_len >= size_of::<[c_char; 1024]>() as c_int {
        snprintf(
            reason.as_mut_ptr(),
            size_of::<[c_char; 128]>(),
            c"snprintf() failed with return value : %d".as_ptr(),
            schema_len,
        );
        ret = -1;
        ksft_print_msg(
            c"Write schema \"%s\" to resctrl FS%s%s\n".as_ptr(),
            schema.as_ptr(),
            c" # ".as_ptr(),
            reason.as_ptr(),
        );
        return ret;
    }

    fd = open(controlgroup.as_ptr(), O_WRONLY);
    if fd < 0 {
        snprintf(
            reason.as_mut_ptr(),
            size_of::<[c_char; 128]>(),
            c"open() failed : %s".as_ptr(),
            strerror(errno),
        );
        ret = -1;
    } else {
        if write(fd, schema.as_ptr() as *const c_void, schema_len as usize) < 0 {
            snprintf(
                reason.as_mut_ptr(),
                size_of::<[c_char; 128]>(),
                c"write() failed : %s".as_ptr(),
                strerror(errno),
            );
            close(fd);
            ret = -1;
        } else {
            close(fd);
        }
    }

    schema[(schema_len - 1) as usize] = 0;
    ksft_print_msg(
        c"Write schema \"%s\" to resctrl FS%s%s\n".as_ptr(),
        schema.as_ptr(),
        if ret != 0 { c" # ".as_ptr() } else { c"".as_ptr() },
        if ret != 0 { reason.as_ptr() } else { c"".as_ptr() },
    );

    ret
}

#[no_mangle]
pub unsafe extern "C" fn check_resctrlfs_support() -> bool {
    let inf = fopen(c"/proc/filesystems".as_ptr(), c"r".as_ptr());
    let dp: *mut DIR;
    let res: *mut c_char;
    let mut ret = false;

    if inf.is_null() {
        return false;
    }

    res = fgrep(inf, c"nodev\tresctrl\n".as_ptr());

    if !res.is_null() {
        ret = true;
        free(res as *mut c_void);
    }

    fclose(inf);

    ksft_print_msg(
        c"%s Check kernel supports resctrl filesystem\n".as_ptr(),
        if ret { c"Pass:".as_ptr() } else { c"Fail:".as_ptr() },
    );

    if !ret {
        return ret;
    }

    dp = opendir(RESCTRL_PATH);
    ksft_print_msg(
        c"%s Check resctrl mountpoint \"%s\" exists\n".as_ptr(),
        if !dp.is_null() { c"Pass:".as_ptr() } else { c"Fail:".as_ptr() },
        RESCTRL_PATH,
    );
    if !dp.is_null() {
        closedir(dp);
    }

    ksft_print_msg(
        c"resctrl filesystem %s mounted\n".as_ptr(),
        if find_resctrl_mount(ptr::null_mut()) != 0 {
            c"not".as_ptr()
        } else {
            c"is".as_ptr()
        },
    );

    ret
}

#[no_mangle]
pub unsafe extern "C" fn fgrep(inf: *mut FILE, str_: *const c_char) -> *mut c_char {
    let mut line = [0 as c_char; 256];
    let slen = strlen(str_) as c_int;

    while feof(inf) == 0 {
        if fgets(line.as_mut_ptr(), 256, inf).is_null() {
            break;
        }
        if strncmp(line.as_ptr(), str_, slen as usize) != 0 {
            continue;
        }

        return strdup(line.as_ptr());
    }

    ptr::null_mut()
}

/*
 * resctrl_resource_exists - Check if a resource is supported.
 * @resource:	Resctrl resource (e.g., MB, L3, L2, L3_MON, etc.)
 *
 * Return: True if the resource is supported, else false. False is
 *         also returned if resctrl FS is not mounted.
 */
#[no_mangle]
pub unsafe extern "C" fn resctrl_resource_exists(resource: *const c_char) -> bool {
    let mut res_path = [0 as c_char; PATH_MAX];
    let mut statbuf: stat = core::mem::zeroed();
    let ret: c_int;

    if resource.is_null() {
        return false;
    }

    ret = find_resctrl_mount(ptr::null_mut());
    if ret != 0 {
        return false;
    }

    snprintf(res_path.as_mut_ptr(), size_of::<[c_char; PATH_MAX]>(), c"%s/%s".as_ptr(), INFO_PATH, resource);

    if stat(res_path.as_ptr(), &mut statbuf) != 0 {
        return false;
    }

    true
}

/*
 * resctrl_mon_feature_exists - Check if requested monitoring feature is valid.
 * @resource:	Resource that uses the mon_features file. Currently only L3_MON
 *		is valid.
 * @feature:	Required monitor feature (in mon_features file).
 *
 * Return: True if the feature is supported, else false.
 */
#[no_mangle]
pub unsafe extern "C" fn resctrl_mon_feature_exists(
    resource: *const c_char,
    feature: *const c_char,
) -> bool {
    let mut res_path = [0 as c_char; PATH_MAX];
    let res: *mut c_char;
    let inf: *mut FILE;

    if feature.is_null() || resource.is_null() {
        return false;
    }

    snprintf(res_path.as_mut_ptr(), size_of::<[c_char; PATH_MAX]>(), c"%s/%s/mon_features".as_ptr(), INFO_PATH, resource);
    inf = fopen(res_path.as_ptr(), c"r".as_ptr());
    if inf.is_null() {
        return false;
    }

    res = fgrep(inf, feature);
    free(res as *mut c_void);
    fclose(inf);

    !res.is_null()
}

/*
 * resource_info_file_exists - Check if a file is present inside
 * /sys/fs/resctrl/info/@resource.
 * @resource:	Required resource (Eg: MB, L3, L2, etc.)
 * @file:	Required file.
 *
 * Return: True if the /sys/fs/resctrl/info/@resource/@file exists, else false.
 */
#[no_mangle]
pub unsafe extern "C" fn resource_info_file_exists(
    resource: *const c_char,
    file: *const c_char,
) -> bool {
    let mut res_path = [0 as c_char; PATH_MAX];
    let mut statbuf: stat = core::mem::zeroed();

    if file.is_null() || resource.is_null() {
        return false;
    }

    snprintf(
        res_path.as_mut_ptr(),
        size_of::<[c_char; PATH_MAX]>(),
        c"%s/%s/%s".as_ptr(),
        INFO_PATH,
        resource,
        file,
    );

    if stat(res_path.as_ptr(), &mut statbuf) != 0 {
        return false;
    }

    true
}

#[no_mangle]
pub unsafe extern "C" fn test_resource_feature_check(test: *const resctrl_test) -> bool {
    resctrl_resource_exists((*test).resource)
}

#[no_mangle]
pub unsafe extern "C" fn filter_dmesg() -> c_int {
    let mut line = [0 as c_char; 1024];
    let mut fp: *mut FILE;
    let mut pipefds = [0 as c_int; 2];
    let pid: pid_t;
    let ret: c_int;

    ret = pipe(pipefds.as_mut_ptr());
    if ret != 0 {
        ksft_perror(c"pipe".as_ptr());
        return ret;
    }
    fflush(stdout);
    pid = fork();
    if pid == 0 {
        close(pipefds[0]);
        dup2(pipefds[1], STDOUT_FILENO);
        execlp(c"dmesg".as_ptr(), c"dmesg".as_ptr(), ptr::null::<c_char>());
        ksft_perror(c"Executing dmesg".as_ptr());
        exit(1);
    }
    close(pipefds[1]);
    fp = fdopen(pipefds[0], c"r".as_ptr());
    if fp.is_null() {
        ksft_perror(c"fdopen(pipe)".as_ptr());
        kill(pid, SIGTERM);

        return -1;
    }

    while !fgets(line.as_mut_ptr(), 1024, fp).is_null() {
        if !strstr(line.as_ptr(), c"intel_rdt:".as_ptr()).is_null() {
            ksft_print_msg(c"dmesg: %s".as_ptr(), line.as_ptr());
        }
        if !strstr(line.as_ptr(), c"resctrl:".as_ptr()).is_null() {
            ksft_print_msg(c"dmesg: %s".as_ptr(), line.as_ptr());
        }
    }
    fclose(fp);
    waitpid(pid, ptr::null_mut(), 0);

    0
}

#[no_mangle]
pub unsafe extern "C" fn perf_event_open(
    hw_event: *mut perf_event_attr,
    pid: pid_t,
    cpu: c_int,
    group_fd: c_int,
    flags: c_ulong,
) -> c_int {
    let ret: c_int;

    ret = syscall(__NR_perf_event_open, hw_event, pid, cpu, group_fd, flags) as c_int;
    ret
}

#[no_mangle]
pub extern "C" fn count_bits(mut n: c_ulong) -> u32 {
    let mut count: u32 = 0;

    while n != 0 {
        count += (n & 1) as u32;
        n >>= 1;
    }

    count
}

/**
 * snc_kernel_support - Check for existence of mon_sub_L3_00 file that indicates
 * SNC resctrl support on the kernel side.
 *
 * Return: 0 if not supported, 1 if SNC is disabled or SNC discovery is
 * unreliable or SNC is both enabled and supported.
 */
#[no_mangle]
pub unsafe extern "C" fn snc_kernel_support() -> c_int {
    let mut node_path = [0 as c_char; PATH_MAX];
    let mut statbuf: stat = core::mem::zeroed();
    let ret: c_int;

    ret = snc_nodes_per_l3_cache();
    /*
     * If SNC is disabled then its kernel support isn't important. If SNC
     * got disabled because the discovery process was unreliable the
     * snc_unreliable variable was set. It can be used to verify the SNC
     * discovery reliability elsewhere in the selftest.
     */
    if ret == 1 {
        return ret;
    }

    snprintf(
        node_path.as_mut_ptr(),
        size_of::<[c_char; PATH_MAX]>(),
        c"%s/%s".as_ptr(),
        RESCTRL_PATH,
        c"mon_data/mon_L3_00/mon_sub_L3_00".as_ptr(),
    );

    if stat(node_path.as_ptr(), &mut statbuf) == 0 {
        return 1;
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
