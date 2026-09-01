// SPDX-License-Identifier: GPL-2.0
// Translated from perf/util/util.c. C include dependencies are expected to be
// supplied by the surrounding translated repository.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem;
use core::ptr;

type bool_ = bool;
type size_t = usize;
type mode_t = c_uint;
type u64 = u64;

const PERF_MAX_STACK_DEPTH: c_int = 127;
const PERF_MAX_CONTEXTS_PER_STACK: c_int = 8;
const PATH_MAX: usize = 4096;
const MAX_NR_CPUS: usize = 2048;
const INT_MAX: c_int = c_int::MAX;
const ENOMEM: c_int = 12;
const ENOSYS: c_int = 38;
const O_PATH: c_int = 0o10000000;
const S_IFDIR: mode_t = 0o040000;
const CAP_SYS_ADMIN: c_int = 21;
const CAP_PERFMON: c_int = 38;

#[repr(C)]
pub struct perf_event_attr {
    pub size: u32,
    pub exclude_host: u64,
    pub exclude_guest: u64,
}

#[repr(C)]
pub struct stat {
    pub st_mode: mode_t,
}

#[repr(C)]
pub struct DIR {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dirent {
    pub d_name: [c_char; 256],
}

#[repr(C)]
pub struct strlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct str_node {
    pub s: *mut c_char,
}

#[repr(C)]
pub struct strlist_config {
    pub dirname: *const c_char,
    pub file_only: bool_,
}

#[repr(C)]
pub struct perf_debuginfod {
    pub set: bool_,
    pub urls: *const c_char,
}

unsafe extern "C" {
    static graph_dotted_line: *const c_char;
    static mut errno: c_int;

    fn sysctl__read_int(path: *const c_char, value: *mut c_int) -> c_int;
    fn stat(path: *const c_char, st: *mut stat) -> c_int;
    fn lstat(path: *const c_char, st: *mut stat) -> c_int;
    fn mkdir(path: *const c_char, mode: mode_t) -> c_int;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strrchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn strlen(s: *const c_char) -> size_t;
    fn strncpy(dst: *mut c_char, src: *const c_char, n: size_t) -> *mut c_char;
    fn strcpy(dst: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memcpy(dst: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn unlink(path: *const c_char) -> c_int;
    fn rmdir(path: *const c_char) -> c_int;
    fn opendir(name: *const c_char) -> *mut DIR;
    fn readdir(dirp: *mut DIR) -> *mut dirent;
    fn closedir(dirp: *mut DIR) -> c_int;
    fn snprintf(s: *mut c_char, maxlen: size_t, format: *const c_char, ...) -> c_int;
    fn scnprintf(s: *mut c_char, maxlen: size_t, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn strglobmatch(str_: *const c_char, pat: *const c_char) -> bool_;
    fn bitmap_scnprintf(
        bitmap: *const c_ulong,
        nbits: c_int,
        buf: *mut c_char,
        size: size_t,
    ) -> c_int;
    fn strlist__new(name: *const c_char, config: *const strlist_config) -> *mut strlist;
    fn strlist__add(slist: *mut strlist, entry: *const c_char) -> c_int;
    fn strlist__delete(slist: *mut strlist);
    fn strlist__nr_entries(slist: *const strlist) -> c_int;
    fn strlist__entry(slist: *const strlist, idx: c_int) -> *mut str_node;
    fn random() -> c_long;
    fn asprintf(strp: *mut *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn readlink(path: *const c_char, buf: *mut c_char, bufsiz: size_t) -> isize;
    fn setenv(name: *const c_char, value: *const c_char, overwrite: c_int) -> c_int;
    fn getenv(name: *const c_char) -> *mut c_char;
    fn pr_debug(format: *const c_char, ...);
    fn pr_warning(format: *const c_char, ...);
    fn perf_cap__capable(cap: c_int) -> bool_;
    fn syscall(num: c_long, ...) -> c_long;
    fn openat(dirfd: c_int, pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn scandir(
        dirp: *const c_char,
        namelist: *mut *mut *mut dirent,
        filter: Option<unsafe extern "C" fn(*const dirent) -> c_int>,
        compar: Option<unsafe extern "C" fn(*const *mut dirent, *const *mut dirent) -> c_int>,
    ) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn getpid() -> c_int;
}

#[unsafe(no_mangle)]
pub static mut input_name: *const c_char = ptr::null();

#[unsafe(no_mangle)]
pub static mut perf_singlethreaded: bool_ = true;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_set_singlethreaded() {
    unsafe {
        perf_singlethreaded = true;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_set_multithreaded() {
    unsafe {
        perf_singlethreaded = false;
    }
}

#[unsafe(no_mangle)]
pub static mut sysctl_perf_event_max_stack: c_int = PERF_MAX_STACK_DEPTH;
#[unsafe(no_mangle)]
pub static mut sysctl_perf_event_max_contexts_per_stack: c_int = PERF_MAX_CONTEXTS_PER_STACK;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sysctl__max_stack() -> c_int {
    let mut value: c_int = 0;

    unsafe {
        if sysctl__read_int(c"kernel/perf_event_max_stack".as_ptr(), &mut value) == 0 {
            sysctl_perf_event_max_stack = value;
        }

        if sysctl__read_int(
            c"kernel/perf_event_max_contexts_per_stack".as_ptr(),
            &mut value,
        ) == 0
        {
            sysctl_perf_event_max_contexts_per_stack = value;
        }

        sysctl_perf_event_max_stack
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sysctl__nmi_watchdog_enabled() -> bool_ {
    static mut cached: bool_ = false;
    static mut nmi_watchdog: bool_ = false;
    let mut value: c_int = 0;

    unsafe {
        if cached {
            return nmi_watchdog;
        }

        if sysctl__read_int(c"kernel/nmi_watchdog".as_ptr(), &mut value) < 0 {
            return false;
        }

        nmi_watchdog = if value > 0 { true } else { false };
        cached = true;

        nmi_watchdog
    }
}

#[unsafe(no_mangle)]
pub static mut exclude_GH_default: bool_ = false;
#[unsafe(no_mangle)]
pub static mut perf_host: bool_ = true;
#[unsafe(no_mangle)]
pub static mut perf_guest: bool_ = false;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn event_attr_init(attr: *mut perf_event_attr) {
    unsafe {
        (*attr).size = mem::size_of::<perf_event_attr>() as u32;

        if !exclude_GH_default {
            return;
        }

        if !perf_host {
            (*attr).exclude_host = 1;
        }
        if !perf_guest {
            (*attr).exclude_guest = 1;
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mkdir_p(path: *mut c_char, mode: mode_t) -> c_int {
    let mut st: stat = unsafe { mem::zeroed() };
    let mut err: c_int;
    let mut d = path;

    unsafe {
        if *d != b'/' as c_char {
            return -1;
        }

        if stat(path, &mut st) == 0 {
            return 0;
        }

        loop {
            d = d.add(1);
            if *d != b'/' as c_char {
                break;
            }
        }

        loop {
            d = strchr(d, b'/' as c_int);
            if d.is_null() {
                break;
            }
            *d = 0;
            err = ((stat(path, &mut st) != 0) && (mkdir(path, mode) != 0)) as c_int;
            *d = b'/' as c_char;
            d = d.add(1);
            if err != 0 {
                return -1;
            }
            while *d == b'/' as c_char {
                d = d.add(1);
            }
        }
        if (stat(path, &mut st) != 0) && (mkdir(path, mode) != 0) {
            -1
        } else {
            0
        }
    }
}

unsafe fn match_pat(file: *mut c_char, pat: *const *const c_char) -> bool_ {
    let mut i: isize = 0;

    unsafe {
        if pat.is_null() {
            return true;
        }

        while !(*pat.offset(i)).is_null() {
            if strglobmatch(file, *pat.offset(i)) {
                return true;
            }

            i += 1;
        }

        false
    }
}

/*
 * The depth specify how deep the removal will go.
 * 0       - will remove only files under the 'path' directory
 * 1 .. x  - will dive in x-level deep under the 'path' directory
 *
 * If specified the pat is array of string patterns ended with NULL,
 * which are checked upon every file/directory found. Only matching
 * ones are removed.
 *
 * The function returns:
 *    0 on success
 *   -1 on removal failure with errno set
 *   -2 on pattern failure
 */
unsafe fn rm_rf_depth_pat(path: *const c_char, depth: c_int, pat: *const *const c_char) -> c_int {
    let mut ret: c_int;
    let mut statbuf: stat = unsafe { mem::zeroed() };
    let mut namebuf = [0 as c_char; PATH_MAX];

    unsafe {
        ret = lstat(path, &mut statbuf);
        if ret != 0 {
            return 0;
        }

        if (statbuf.st_mode & S_IFDIR) == 0 {
            return unlink(path);
        }

        let dir = opendir(path);
        if dir.is_null() {
            return -1;
        }

        loop {
            let d = readdir(dir);
            if d.is_null() || ret != 0 {
                break;
            }

            if strcmp((*d).d_name.as_ptr(), c".".as_ptr()) == 0
                || strcmp((*d).d_name.as_ptr(), c"..".as_ptr()) == 0
            {
                continue;
            }

            if !match_pat((*d).d_name.as_mut_ptr(), pat) {
                ret = -2;
                break;
            }

            scnprintf(
                namebuf.as_mut_ptr(),
                namebuf.len(),
                c"%s/%s".as_ptr(),
                path,
                (*d).d_name.as_ptr(),
            );

            ret = lstat(namebuf.as_ptr(), &mut statbuf);
            if ret < 0 {
                pr_debug(c"stat failed: %s\n".as_ptr(), namebuf.as_ptr());
                break;
            }

            if (statbuf.st_mode & S_IFDIR) == S_IFDIR {
                ret = if depth != 0 {
                    rm_rf_depth_pat(namebuf.as_ptr(), depth - 1, pat)
                } else {
                    0
                };
            } else {
                ret = unlink(namebuf.as_ptr());
            }
        }
        closedir(dir);

        if ret < 0 {
            return ret;
        }

        rmdir(path)
    }
}

unsafe fn rm_rf_a_kcore_dir(path: *const c_char, name: *const c_char) -> c_int {
    let mut kcore_dir_path = [0 as c_char; PATH_MAX];
    let pat = [
        c"kcore".as_ptr(),
        c"kallsyms".as_ptr(),
        c"modules".as_ptr(),
        ptr::null(),
    ];

    unsafe {
        snprintf(
            kcore_dir_path.as_mut_ptr(),
            kcore_dir_path.len(),
            c"%s/%s".as_ptr(),
            path,
            name,
        );

        rm_rf_depth_pat(kcore_dir_path.as_ptr(), 0, pat.as_ptr())
    }
}

unsafe extern "C" fn kcore_dir_filter(_name: *const c_char, d: *mut dirent) -> bool_ {
    let pat = [
        c"kcore_dir".as_ptr(),
        c"kcore_dir__[1-9]*".as_ptr(),
        ptr::null(),
    ];

    unsafe { match_pat((*d).d_name.as_mut_ptr(), pat.as_ptr()) }
}

unsafe fn rm_rf_kcore_dir(path: *const c_char) -> c_int {
    unsafe {
        let kcore_dirs = lsdir(path, Some(kcore_dir_filter));
        if kcore_dirs.is_null() {
            return 0;
        }

        let mut ret = 0;
        let nr = strlist__nr_entries(kcore_dirs);
        let mut i = 0;
        while i < nr {
            let nd = strlist__entry(kcore_dirs, i);
            ret = rm_rf_a_kcore_dir(path, (*nd).s);
            if ret != 0 {
                return ret;
            }
            i += 1;
        }

        strlist__delete(kcore_dirs);

        0
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cpumask_to_cpulist(cpumask: *mut c_char, cpulist: *mut c_char) {
    let mut i: c_int;
    let mut j: c_int;
    let mut len = unsafe { strlen(cpumask) as c_int };
    let bm_size: c_int;
    let nbits: c_int;
    let bm: *mut c_ulong;
    let mut cpus = [0 as c_char; MAX_NR_CPUS];

    unsafe {
        i = 0;
        while i < len {
            if *cpumask.offset(i as isize) == b',' as c_char {
                j = i;
                while j < len {
                    *cpumask.offset(j as isize) = *cpumask.offset((j + 1) as isize);
                    j += 1;
                }
            }
            i += 1;
        }

        len = strlen(cpumask) as c_int;
        bm_size = (len + 15) / 16;
        nbits = bm_size * 64;
        if nbits <= 0 {
            return;
        }

        bm = calloc(bm_size as size_t, mem::size_of::<c_ulong>()) as *mut c_ulong;
        if bm.is_null() {
            free(bm as *mut c_void);
            return;
        }

        i = 0;
        while i < bm_size {
            let mut blk = [0 as c_char; 17];
            let blklen = if len > 16 { 16 } else { len };

            strncpy(
                blk.as_mut_ptr(),
                cpumask.offset((len - blklen) as isize),
                blklen as size_t,
            );
            blk[blklen as usize] = 0;
            *bm.offset(i as isize) = strtoul(blk.as_ptr(), ptr::null_mut(), 16);
            *cpumask.offset((len - blklen) as isize) = 0;
            len = strlen(cpumask) as c_int;
            i += 1;
        }

        bitmap_scnprintf(bm, nbits, cpus.as_mut_ptr(), cpus.len());
        strcpy(cpulist, cpus.as_ptr());

        free(bm as *mut c_void);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn print_separator2(
    pre_dash_cnt: c_int,
    s: *const c_char,
    post_dash_cnt: c_int,
) {
    unsafe {
        printf(
            c"%.*s%s%.*s\n".as_ptr(),
            pre_dash_cnt,
            graph_dotted_line,
            s,
            post_dash_cnt,
            graph_dotted_line,
        );
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rm_rf_perf_data(path: *const c_char) -> c_int {
    let pat = [c"data".as_ptr(), c"data.*".as_ptr(), ptr::null()];

    unsafe {
        rm_rf_kcore_dir(path);
        rm_rf_depth_pat(path, 0, pat.as_ptr())
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rm_rf(path: *const c_char) -> c_int {
    unsafe { rm_rf_depth_pat(path, INT_MAX, ptr::null()) }
}

/* A filter which removes dot files */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lsdir_no_dot_filter(_name: *const c_char, d: *mut dirent) -> bool_ {
    unsafe { (*d).d_name[0] != b'.' as c_char }
}

/* lsdir reads a directory and store it in strlist */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lsdir(
    name: *const c_char,
    filter: Option<unsafe extern "C" fn(*const c_char, *mut dirent) -> bool_>,
) -> *mut strlist {
    unsafe {
        let dir = opendir(name);
        if dir.is_null() {
            return ptr::null_mut();
        }

        let list = strlist__new(ptr::null(), ptr::null());
        if list.is_null() {
            errno = ENOMEM;
            closedir(dir);
            return list;
        }

        loop {
            let d = readdir(dir);
            if d.is_null() {
                break;
            }
            if filter.is_none() || filter.unwrap()(name, d) {
                strlist__add(list, (*d).d_name.as_ptr());
            }
        }

        closedir(dir);
        list
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn hex_width(mut v: u64) -> size_t {
    let mut n: size_t = 1;

    while {
        v >>= 4;
        v != 0
    } {
        n += 1;
    }

    n
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_event_paranoid() -> c_int {
    let mut value: c_int = 0;

    unsafe {
        if sysctl__read_int(c"kernel/perf_event_paranoid".as_ptr(), &mut value) != 0 {
            return INT_MAX;
        }

        value
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_event_paranoid_check(max_level: c_int) -> bool_ {
    unsafe {
        perf_cap__capable(CAP_SYS_ADMIN)
            || perf_cap__capable(CAP_PERFMON)
            || perf_event_paranoid() <= max_level
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_tip(strp: *mut *mut c_char, dirpath: *const c_char) -> c_int {
    let conf = strlist_config {
        dirname: dirpath,
        file_only: true,
    };
    let mut ret: c_int = 0;

    unsafe {
        *strp = ptr::null_mut();
        let tips = strlist__new(c"tips.txt".as_ptr(), &conf);
        if tips.is_null() {
            return -errno;
        }

        if strlist__nr_entries(tips) == 0 {
            strlist__delete(tips);
            return ret;
        }

        let node = strlist__entry(tips, (random() % strlist__nr_entries(tips) as c_long) as c_int);
        if asprintf(strp, c"Tip: %s".as_ptr(), (*node).s) < 0 {
            ret = -ENOMEM;
        }

        strlist__delete(tips);

        ret
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_exe(buf: *mut c_char, len: c_int) -> *mut c_char {
    unsafe {
        if len <= 0 {
            return buf;
        }

        let n = readlink(c"/proc/self/exe".as_ptr(), buf, (len - 1) as size_t);
        if n > 0 {
            *buf.offset(n as isize) = 0;
            return buf;
        }
        if len < c"perf".to_bytes_with_nul().len() as c_int {
            *buf = 0;
            return buf;
        }

        strcpy(buf, c"perf".as_ptr())
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_debuginfod_setup(di: *mut perf_debuginfod) {
    /*
     * By default '!di->set' we clear DEBUGINFOD_URLS, so debuginfod
     * processing is not triggered, otherwise we set it to 'di->urls'
     * value. If 'di->urls' is "system" we keep DEBUGINFOD_URLS value.
     */
    unsafe {
        if !(*di).set {
            setenv(c"DEBUGINFOD_URLS".as_ptr(), c"".as_ptr(), 1);
        } else if !(*di).urls.is_null() && strcmp((*di).urls, c"system".as_ptr()) != 0 {
            setenv(c"DEBUGINFOD_URLS".as_ptr(), (*di).urls, 1);
        }

        pr_debug(
            c"DEBUGINFOD_URLS=%s\n".as_ptr(),
            getenv(c"DEBUGINFOD_URLS".as_ptr()),
        );

        // Original C conditional: #ifndef HAVE_DEBUGINFOD_SUPPORT
        #[cfg(not(HAVE_DEBUGINFOD_SUPPORT))]
        {
            if (*di).set {
                pr_warning(c"WARNING: debuginfod support requested, but perf is not built with it\n".as_ptr());
            }
        }
    }
}

/*
 * Return a new filename prepended with task's root directory if it's in
 * a chroot.  Callers should free the returned string.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn filename_with_chroot(
    pid: c_int,
    filename: *const c_char,
) -> *mut c_char {
    let mut buf = [0 as c_char; PATH_MAX];
    let mut proc_root = [0 as c_char; 32];
    let mut new_name: *mut c_char = ptr::null_mut();

    unsafe {
        scnprintf(
            proc_root.as_mut_ptr(),
            proc_root.len(),
            c"/proc/%d/root".as_ptr(),
            pid,
        );
        let ret = readlink(proc_root.as_ptr(), buf.as_mut_ptr(), buf.len() - 1);
        if ret <= 0 {
            return ptr::null_mut();
        }

        buf[ret as usize] = 0;

        if strcmp(buf.as_ptr(), c"/".as_ptr()) == 0 {
            return ptr::null_mut();
        }

        if !strstr(buf.as_ptr(), c"(deleted)".as_ptr()).is_null() {
            return ptr::null_mut();
        }

        if asprintf(&mut new_name, c"%s/%s".as_ptr(), buf.as_ptr(), filename) < 0 {
            return ptr::null_mut();
        }

        new_name
    }
}

/*
 * Reallocate an array *arr of size *arr_sz so that it is big enough to contain
 * x elements of size msz, initializing new entries to *init_val or zero if
 * init_val is NULL
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_realloc_array_as_needed(
    arr: *mut *mut c_void,
    arr_sz: *mut size_t,
    x: size_t,
    msz: size_t,
    init_val: *const c_void,
) -> c_int {
    unsafe {
        let mut new_sz = *arr_sz;
        let mut i: size_t;

        if new_sz == 0 {
            new_sz = if msz >= 64 { 1 } else { roundup(64, msz) };
        }
        while x >= new_sz {
            match new_sz.checked_mul(2) {
                Some(v) => new_sz = v,
                None => return -ENOMEM,
            }
        }
        if new_sz == *arr_sz {
            return 0;
        }
        let new_arr = calloc(new_sz, msz);
        if new_arr.is_null() {
            return -ENOMEM;
        }
        if *arr_sz != 0 {
            memcpy(new_arr, *arr, *arr_sz * msz);
        }
        if !init_val.is_null() {
            i = *arr_sz;
            while i < new_sz {
                memcpy((new_arr as *mut u8).add(i * msz) as *mut c_void, init_val, msz);
                i += 1;
            }
        }
        *arr = new_arr;
        *arr_sz = new_sz;
        0
    }
}

fn roundup(x: size_t, y: size_t) -> size_t {
    (((x) + ((y) - 1)) / (y)) * (y)
}

// Original C conditional: #ifndef HAVE_SCHED_GETCPU_SUPPORT
#[cfg(not(HAVE_SCHED_GETCPU_SUPPORT))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sched_getcpu() -> c_int {
    unsafe {
        // Original C conditional: #ifdef __NR_getcpu
        #[cfg(__NR_getcpu)]
        {
            let mut cpu: c_uint = 0;
            let err = syscall(__NR_getcpu as c_long, &mut cpu as *mut c_uint, ptr::null::<c_void>(), ptr::null::<c_void>());

            if err == 0 {
                return cpu as c_int;
            }
        }
        #[cfg(not(__NR_getcpu))]
        {
            errno = ENOSYS;
        }
        -1
    }
}

// Original C conditional: #ifndef HAVE_SCANDIRAT_SUPPORT
#[cfg(not(HAVE_SCANDIRAT_SUPPORT))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn scandirat(
    dirfd: c_int,
    dirp: *const c_char,
    namelist: *mut *mut *mut dirent,
    filter: Option<unsafe extern "C" fn(*const dirent) -> c_int>,
    compar: Option<unsafe extern "C" fn(*const *mut dirent, *const *mut dirent) -> c_int>,
) -> c_int {
    let mut path = [0 as c_char; PATH_MAX];

    unsafe {
        let fd = openat(dirfd, dirp, O_PATH);
        if fd < 0 {
            return fd;
        }

        snprintf(
            path.as_mut_ptr(),
            path.len(),
            c"/proc/%d/fd/%d".as_ptr(),
            getpid(),
            fd,
        );
        let err = scandir(path.as_ptr(), namelist, filter, compar);
        close(fd);
        err
    }
}

/* basename version that takes a const input string */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_basename(path: *const c_char) -> *const c_char {
    unsafe {
        let base = strrchr(path, b'/' as c_int);

        if !base.is_null() {
            base.add(1)
        } else {
            path
        }
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
