// SPDX-License-Identifier: GPL-2.0
//
// Translated from lib/api/fs/fs.c.
// C includes referenced errno, limits, stdio/stdlib/string, statfs, pthread,
// unistd, mount, fs.h, ../io.h, and debug-internal.h.

use core::ffi::c_void;
use core::mem;
use core::ptr;

const SYSFS_MAGIC: libc::c_long = 0x62656572;
const PROC_SUPER_MAGIC: libc::c_long = 0x9fa0;
const DEBUGFS_MAGIC: libc::c_long = 0x64626720;
const TRACEFS_MAGIC: libc::c_long = 0x74726163;
const HUGETLBFS_MAGIC: libc::c_long = 0x958458f6;
const BPF_FS_MAGIC: libc::c_long = 0xcafe4a11;

const SYSFS: &[u8] = b"sysfs\0";
const PROCFS: &[u8] = b"procfs\0";
const DEBUGFS: &[u8] = b"debugfs\0";
const TRACEFS: &[u8] = b"tracefs\0";
const HUGETLBFS: &[u8] = b"hugetlbfs\0";
const BPF: &[u8] = b"bpf\0";

const PROC_MOUNTS: &[u8] = b"/proc/mounts\0";
const READ_MODE: &[u8] = b"r\0";
const FSCANF_MOUNTS_FMT: &[u8] = b"%*s %4096s %99s %*s %*d %*d\n\0";
const PERF_ENV_FMT: &[u8] = b"PERF_%s_ENVIRONMENT\0";
const PATH_JOIN_FMT: &[u8] = b"%s/%s\0";
const PROC_SYS_JOIN_FMT: &[u8] = b"%s/sys/%s\0";
const INT_FMT: &[u8] = b"%d\0";
const NAME_PATH_SUFFIX: &[u8] = b"_PATH\0";

static SYSFS_MP_SYS: &[u8] = b"/sys\0";
static PROCFS_MP_PROC: &[u8] = b"/proc\0";
static DEBUGFS_MP_DEFAULT: &[u8] = b"/sys/kernel/debug\0";
static DEBUGFS_MP_DEBUG: &[u8] = b"/debug\0";
static TRACEFS_MP_DEFAULT: &[u8] = b"/sys/kernel/tracing\0";
static TRACEFS_MP_DEBUG_TRACING: &[u8] = b"/sys/kernel/debug/tracing\0";
static TRACEFS_MP_TRACING: &[u8] = b"/tracing\0";
static TRACEFS_MP_TRACE: &[u8] = b"/trace\0";
static BPF_FS_MP_DEFAULT: &[u8] = b"/sys/fs/bpf\0";

static SYSFS_KNOWN_MOUNTPOINTS: [*const libc::c_char; 2] = [
    SYSFS_MP_SYS.as_ptr() as *const libc::c_char,
    ptr::null(),
];

static PROCFS_KNOWN_MOUNTPOINTS: [*const libc::c_char; 2] = [
    PROCFS_MP_PROC.as_ptr() as *const libc::c_char,
    ptr::null(),
];

static DEBUGFS_KNOWN_MOUNTPOINTS: [*const libc::c_char; 3] = [
    DEBUGFS_MP_DEFAULT.as_ptr() as *const libc::c_char,
    DEBUGFS_MP_DEBUG.as_ptr() as *const libc::c_char,
    ptr::null(),
];

static TRACEFS_KNOWN_MOUNTPOINTS: [*const libc::c_char; 5] = [
    TRACEFS_MP_DEFAULT.as_ptr() as *const libc::c_char,
    TRACEFS_MP_DEBUG_TRACING.as_ptr() as *const libc::c_char,
    TRACEFS_MP_TRACING.as_ptr() as *const libc::c_char,
    TRACEFS_MP_TRACE.as_ptr() as *const libc::c_char,
    ptr::null(),
];

static HUGETLBFS_KNOWN_MOUNTPOINTS: [*const libc::c_char; 1] = [ptr::null()];

static BPF_FS_KNOWN_MOUNTPOINTS: [*const libc::c_char; 2] = [
    BPF_FS_MP_DEFAULT.as_ptr() as *const libc::c_char,
    ptr::null(),
];

#[repr(C)]
pub struct io {
    pub fd: libc::c_int,
    _private: [u8; 0],
}

#[repr(C)]
struct fs {
    name: *const libc::c_char,
    mounts: *const *const libc::c_char,
    path: *mut libc::c_char,
    mount_mutex: libc::pthread_mutex_t,
    magic: libc::c_long,
}

extern "C" {
    fn io__init(io: *mut io, fd: libc::c_int, buf: *mut libc::c_char, size: libc::size_t);
    fn io__getdelim(
        io: *mut io,
        buf: *mut *mut libc::c_char,
        sizep: *mut libc::size_t,
        delim: libc::c_int,
    ) -> libc::c_int;
    fn io__get_char(io: *mut io) -> libc::c_int;
}

static mut FS__SYSFS: fs = fs {
    name: SYSFS.as_ptr() as *const libc::c_char,
    mounts: SYSFS_KNOWN_MOUNTPOINTS.as_ptr(),
    path: ptr::null_mut(),
    mount_mutex: libc::PTHREAD_MUTEX_INITIALIZER,
    magic: SYSFS_MAGIC,
};

static mut FS__PROCFS: fs = fs {
    name: PROCFS.as_ptr() as *const libc::c_char,
    mounts: PROCFS_KNOWN_MOUNTPOINTS.as_ptr(),
    path: ptr::null_mut(),
    mount_mutex: libc::PTHREAD_MUTEX_INITIALIZER,
    magic: PROC_SUPER_MAGIC,
};

static mut FS__DEBUGFS: fs = fs {
    name: DEBUGFS.as_ptr() as *const libc::c_char,
    mounts: DEBUGFS_KNOWN_MOUNTPOINTS.as_ptr(),
    path: ptr::null_mut(),
    mount_mutex: libc::PTHREAD_MUTEX_INITIALIZER,
    magic: DEBUGFS_MAGIC,
};

static mut FS__TRACEFS: fs = fs {
    name: TRACEFS.as_ptr() as *const libc::c_char,
    mounts: TRACEFS_KNOWN_MOUNTPOINTS.as_ptr(),
    path: ptr::null_mut(),
    mount_mutex: libc::PTHREAD_MUTEX_INITIALIZER,
    magic: TRACEFS_MAGIC,
};

static mut FS__HUGETLBFS: fs = fs {
    name: HUGETLBFS.as_ptr() as *const libc::c_char,
    mounts: HUGETLBFS_KNOWN_MOUNTPOINTS.as_ptr(),
    path: ptr::null_mut(),
    mount_mutex: libc::PTHREAD_MUTEX_INITIALIZER,
    magic: HUGETLBFS_MAGIC,
};

static mut FS__BPF_FS: fs = fs {
    name: BPF.as_ptr() as *const libc::c_char,
    mounts: BPF_FS_KNOWN_MOUNTPOINTS.as_ptr(),
    path: ptr::null_mut(),
    mount_mutex: libc::PTHREAD_MUTEX_INITIALIZER,
    magic: BPF_FS_MAGIC,
};

static mut SYSFS_INIT_ONCE: libc::pthread_once_t = libc::PTHREAD_ONCE_INIT;
static mut PROCFS_INIT_ONCE: libc::pthread_once_t = libc::PTHREAD_ONCE_INIT;
static mut DEBUGFS_INIT_ONCE: libc::pthread_once_t = libc::PTHREAD_ONCE_INIT;
static mut TRACEFS_INIT_ONCE: libc::pthread_once_t = libc::PTHREAD_ONCE_INIT;
static mut HUGETLBFS_INIT_ONCE: libc::pthread_once_t = libc::PTHREAD_ONCE_INIT;
static mut BPF_FS_INIT_ONCE: libc::pthread_once_t = libc::PTHREAD_ONCE_INIT;

unsafe extern "C" fn sysfs_init_once() {
    fs__init_once(&mut FS__SYSFS);
}

unsafe extern "C" fn procfs_init_once() {
    fs__init_once(&mut FS__PROCFS);
}

unsafe extern "C" fn debugfs_init_once() {
    fs__init_once(&mut FS__DEBUGFS);
}

unsafe extern "C" fn tracefs_init_once() {
    fs__init_once(&mut FS__TRACEFS);
}

unsafe extern "C" fn hugetlbfs_init_once() {
    fs__init_once(&mut FS__HUGETLBFS);
}

unsafe extern "C" fn bpf_fs_init_once() {
    fs__init_once(&mut FS__BPF_FS);
}

#[no_mangle]
pub unsafe extern "C" fn sysfs__mountpoint() -> *const libc::c_char {
    libc::pthread_once(&mut SYSFS_INIT_ONCE, Some(sysfs_init_once));
    fs__mountpoint(&FS__SYSFS)
}

#[no_mangle]
pub unsafe extern "C" fn sysfs__mount() -> *const libc::c_char {
    let mountpoint = sysfs__mountpoint();
    if !mountpoint.is_null() {
        return mountpoint;
    }
    fs__mount(&mut FS__SYSFS)
}

#[no_mangle]
pub unsafe extern "C" fn sysfs__configured() -> bool {
    !sysfs__mountpoint().is_null()
}

#[no_mangle]
pub unsafe extern "C" fn procfs__mountpoint() -> *const libc::c_char {
    libc::pthread_once(&mut PROCFS_INIT_ONCE, Some(procfs_init_once));
    fs__mountpoint(&FS__PROCFS)
}

#[no_mangle]
pub unsafe extern "C" fn procfs__mount() -> *const libc::c_char {
    let mountpoint = procfs__mountpoint();
    if !mountpoint.is_null() {
        return mountpoint;
    }
    fs__mount(&mut FS__PROCFS)
}

#[no_mangle]
pub unsafe extern "C" fn procfs__configured() -> bool {
    !procfs__mountpoint().is_null()
}

#[no_mangle]
pub unsafe extern "C" fn debugfs__mountpoint() -> *const libc::c_char {
    libc::pthread_once(&mut DEBUGFS_INIT_ONCE, Some(debugfs_init_once));
    fs__mountpoint(&FS__DEBUGFS)
}

#[no_mangle]
pub unsafe extern "C" fn debugfs__mount() -> *const libc::c_char {
    let mountpoint = debugfs__mountpoint();
    if !mountpoint.is_null() {
        return mountpoint;
    }
    fs__mount(&mut FS__DEBUGFS)
}

#[no_mangle]
pub unsafe extern "C" fn debugfs__configured() -> bool {
    !debugfs__mountpoint().is_null()
}

#[no_mangle]
pub unsafe extern "C" fn tracefs__mountpoint() -> *const libc::c_char {
    libc::pthread_once(&mut TRACEFS_INIT_ONCE, Some(tracefs_init_once));
    fs__mountpoint(&FS__TRACEFS)
}

#[no_mangle]
pub unsafe extern "C" fn tracefs__mount() -> *const libc::c_char {
    let mountpoint = tracefs__mountpoint();
    if !mountpoint.is_null() {
        return mountpoint;
    }
    fs__mount(&mut FS__TRACEFS)
}

#[no_mangle]
pub unsafe extern "C" fn tracefs__configured() -> bool {
    !tracefs__mountpoint().is_null()
}

#[no_mangle]
pub unsafe extern "C" fn hugetlbfs__mountpoint() -> *const libc::c_char {
    libc::pthread_once(&mut HUGETLBFS_INIT_ONCE, Some(hugetlbfs_init_once));
    fs__mountpoint(&FS__HUGETLBFS)
}

#[no_mangle]
pub unsafe extern "C" fn hugetlbfs__mount() -> *const libc::c_char {
    let mountpoint = hugetlbfs__mountpoint();
    if !mountpoint.is_null() {
        return mountpoint;
    }
    fs__mount(&mut FS__HUGETLBFS)
}

#[no_mangle]
pub unsafe extern "C" fn hugetlbfs__configured() -> bool {
    !hugetlbfs__mountpoint().is_null()
}

#[no_mangle]
pub unsafe extern "C" fn bpf_fs__mountpoint() -> *const libc::c_char {
    libc::pthread_once(&mut BPF_FS_INIT_ONCE, Some(bpf_fs_init_once));
    fs__mountpoint(&FS__BPF_FS)
}

#[no_mangle]
pub unsafe extern "C" fn bpf_fs__mount() -> *const libc::c_char {
    let mountpoint = bpf_fs__mountpoint();
    if !mountpoint.is_null() {
        return mountpoint;
    }
    fs__mount(&mut FS__BPF_FS)
}

#[no_mangle]
pub unsafe extern "C" fn bpf_fs__configured() -> bool {
    !bpf_fs__mountpoint().is_null()
}

unsafe fn fs__read_mounts(fs: *mut fs) -> bool {
    let mut type_: [libc::c_char; 100] = [0; 100];
    let mut path: [libc::c_char; libc::PATH_MAX as usize + 1] = [0; libc::PATH_MAX as usize + 1];

    let fp = libc::fopen(PROC_MOUNTS.as_ptr() as *const libc::c_char, READ_MODE.as_ptr() as *const libc::c_char);
    if fp.is_null() {
        return false;
    }

    while libc::fscanf(
        fp,
        FSCANF_MOUNTS_FMT.as_ptr() as *const libc::c_char,
        path.as_mut_ptr(),
        type_.as_mut_ptr(),
    ) == 2
    {
        if libc::strcmp(type_.as_ptr(), (*fs).name) == 0 {
            (*fs).path = libc::strdup(path.as_ptr());
            libc::fclose(fp);
            return !(*fs).path.is_null();
        }
    }
    libc::fclose(fp);
    false
}

unsafe fn fs__valid_mount(fs_path: *const libc::c_char, magic: libc::c_long) -> libc::c_int {
    let mut st_fs: libc::statfs = mem::zeroed();

    if libc::statfs(fs_path, &mut st_fs) < 0 {
        return -libc::ENOENT;
    } else if st_fs.f_type as libc::c_long != magic {
        return -libc::ENOENT;
    }

    0
}

unsafe fn fs__check_mounts(fs: *mut fs) -> bool {
    let mut ptr = (*fs).mounts;

    while !(*ptr).is_null() {
        if fs__valid_mount(*ptr, (*fs).magic) == 0 {
            (*fs).path = libc::strdup(*ptr);
            if (*fs).path.is_null() {
                return false;
            }
            return true;
        }
        ptr = ptr.add(1);
    }

    false
}

unsafe fn mem_toupper(mut f: *mut libc::c_char, mut len: libc::size_t) {
    while len != 0 {
        *f = libc::toupper(*f as libc::c_int) as libc::c_char;
        f = f.add(1);
        len -= 1;
    }
}

/*
 * Check for "NAME_PATH" environment variable to override fs location (for
 * testing). This matches the recommendation in Documentation/admin-guide/sysfs-rules.rst
 * for SYSFS_PATH.
 */
unsafe fn fs__env_override(fs: *mut fs) -> bool {
    let name_len = libc::strlen((*fs).name);
    /* name + "_PATH" + '\0' */
    let mut upper_name = vec![0 as libc::c_char; name_len + 5 + 1];

    libc::memcpy(
        upper_name.as_mut_ptr() as *mut c_void,
        (*fs).name as *const c_void,
        name_len,
    );
    mem_toupper(upper_name.as_mut_ptr(), name_len);
    libc::strcpy(
        upper_name.as_mut_ptr().add(name_len),
        NAME_PATH_SUFFIX.as_ptr() as *const libc::c_char,
    );

    let override_path = libc::getenv(upper_name.as_ptr());
    if override_path.is_null() {
        return false;
    }

    (*fs).path = libc::strdup(override_path);
    if (*fs).path.is_null() {
        return false;
    }
    true
}

unsafe fn fs__init_once(fs: *mut fs) {
    if !fs__env_override(fs) && !fs__check_mounts(fs) && !fs__read_mounts(fs) {
        assert!((*fs).path.is_null());
    } else {
        assert!(!(*fs).path.is_null());
    }
}

unsafe fn fs__mountpoint(fs: *const fs) -> *const libc::c_char {
    (*fs).path
}

unsafe fn mount_overload(fs: *mut fs) -> *const libc::c_char {
    let name_len = libc::strlen((*fs).name);
    /* "PERF_" + name + "_ENVIRONMENT" + '\0' */
    let mut upper_name = vec![0 as libc::c_char; 5 + name_len + 12 + 1];

    libc::snprintf(
        upper_name.as_mut_ptr(),
        upper_name.len(),
        PERF_ENV_FMT.as_ptr() as *const libc::c_char,
        (*fs).name,
    );
    mem_toupper(upper_name.as_mut_ptr(), libc::strlen(upper_name.as_ptr()));

    let overload = libc::getenv(upper_name.as_ptr());
    if !overload.is_null() {
        overload
    } else {
        *(*fs).mounts
    }
}

unsafe fn fs__mount(fs: *mut fs) -> *const libc::c_char {
    let mut mountpoint: *const libc::c_char;

    libc::pthread_mutex_lock(&mut (*fs).mount_mutex);

    /* Check if path found inside the mutex to avoid races with other callers of mount. */
    mountpoint = fs__mountpoint(fs);
    if !mountpoint.is_null() {
        libc::pthread_mutex_unlock(&mut (*fs).mount_mutex);
        return mountpoint;
    }

    mountpoint = mount_overload(fs);

    if libc::mount(
        ptr::null(),
        mountpoint,
        (*fs).name,
        0,
        ptr::null(),
    ) == 0
        && fs__valid_mount(mountpoint, (*fs).magic) == 0
    {
        (*fs).path = libc::strdup(mountpoint);
        mountpoint = (*fs).path;
    }

    libc::pthread_mutex_unlock(&mut (*fs).mount_mutex);
    mountpoint
}

#[no_mangle]
pub unsafe extern "C" fn filename__read_int(
    filename: *const libc::c_char,
    value: *mut libc::c_int,
) -> libc::c_int {
    let mut line: [libc::c_char; 64] = [0; 64];
    let fd = libc::open(filename, libc::O_RDONLY);
    let mut err = -1;
    let n: libc::ssize_t;

    if fd < 0 {
        return -*libc::__errno_location();
    }

    n = libc::read(fd, line.as_mut_ptr() as *mut c_void, mem::size_of_val(&line) - 1);
    if n > 0 {
        line[n as usize] = b'\0' as libc::c_char;
        *value = libc::atoi(line.as_ptr());
        err = 0;
    }

    libc::close(fd);
    err
}

unsafe fn filename__read_ull_base(
    filename: *const libc::c_char,
    value: *mut libc::c_ulonglong,
    base: libc::c_int,
) -> libc::c_int {
    let mut line: [libc::c_char; 64] = [0; 64];
    let fd = libc::open(filename, libc::O_RDONLY);
    let mut err = -1;
    let n: libc::ssize_t;

    if fd < 0 {
        return -*libc::__errno_location();
    }

    n = libc::read(fd, line.as_mut_ptr() as *mut c_void, mem::size_of_val(&line) - 1);
    if n > 0 {
        line[n as usize] = b'\0' as libc::c_char;
        *value = libc::strtoull(line.as_ptr(), ptr::null_mut(), base);
        if *value != libc::ULLONG_MAX {
            err = 0;
        }
    }

    libc::close(fd);
    err
}

/*
 * Parses @value out of @filename with strtoull.
 * By using 16 for base to treat the number as hex.
 */
#[no_mangle]
pub unsafe extern "C" fn filename__read_xll(
    filename: *const libc::c_char,
    value: *mut libc::c_ulonglong,
) -> libc::c_int {
    filename__read_ull_base(filename, value, 16)
}

/*
 * Parses @value out of @filename with strtoull.
 * By using 0 for base, the strtoull detects the
 * base automatically (see man strtoull).
 */
#[no_mangle]
pub unsafe extern "C" fn filename__read_ull(
    filename: *const libc::c_char,
    value: *mut libc::c_ulonglong,
) -> libc::c_int {
    filename__read_ull_base(filename, value, 0)
}

#[no_mangle]
pub unsafe extern "C" fn filename__read_str(
    filename: *const libc::c_char,
    buf: *mut *mut libc::c_char,
    sizep: *mut libc::size_t,
) -> libc::c_int {
    let mut io: io = mem::zeroed();
    let mut bf: [libc::c_char; 128] = [0; 128];
    let mut err: libc::c_int;

    io.fd = libc::open(filename, libc::O_RDONLY);
    if io.fd < 0 {
        return -*libc::__errno_location();
    }
    io__init(&mut io, io.fd, bf.as_mut_ptr(), mem::size_of_val(&bf));
    *buf = ptr::null_mut();
    err = io__getdelim(&mut io, buf, sizep, -1);
    if err < 0 {
        libc::free(*buf as *mut c_void);
        *buf = ptr::null_mut();
    } else {
        err = 0;
    }
    libc::close(io.fd);
    err
}

#[no_mangle]
pub unsafe extern "C" fn filename__write_int(
    filename: *const libc::c_char,
    value: libc::c_int,
) -> libc::c_int {
    let fd = libc::open(filename, libc::O_WRONLY);
    let mut err = -1;
    let mut buf: [libc::c_char; 64] = [0; 64];
    let len: libc::c_int;

    if fd < 0 {
        return -*libc::__errno_location();
    }

    len = libc::sprintf(buf.as_mut_ptr(), INT_FMT.as_ptr() as *const libc::c_char, value);
    if libc::write(fd, buf.as_ptr() as *const c_void, len as libc::size_t) == len as libc::ssize_t {
        err = 0;
    }

    libc::close(fd);
    err
}

#[no_mangle]
pub unsafe extern "C" fn procfs__read_str(
    entry: *const libc::c_char,
    buf: *mut *mut libc::c_char,
    sizep: *mut libc::size_t,
) -> libc::c_int {
    let mut path: [libc::c_char; libc::PATH_MAX as usize] = [0; libc::PATH_MAX as usize];
    let procfs = procfs__mountpoint();

    if procfs.is_null() {
        return -1;
    }

    libc::snprintf(
        path.as_mut_ptr(),
        mem::size_of_val(&path),
        PATH_JOIN_FMT.as_ptr() as *const libc::c_char,
        procfs,
        entry,
    );

    filename__read_str(path.as_ptr(), buf, sizep)
}

unsafe fn sysfs__read_ull_base(
    entry: *const libc::c_char,
    value: *mut libc::c_ulonglong,
    base: libc::c_int,
) -> libc::c_int {
    let mut path: [libc::c_char; libc::PATH_MAX as usize] = [0; libc::PATH_MAX as usize];
    let sysfs = sysfs__mountpoint();

    if sysfs.is_null() {
        return -1;
    }

    libc::snprintf(
        path.as_mut_ptr(),
        mem::size_of_val(&path),
        PATH_JOIN_FMT.as_ptr() as *const libc::c_char,
        sysfs,
        entry,
    );

    filename__read_ull_base(path.as_ptr(), value, base)
}

#[no_mangle]
pub unsafe extern "C" fn sysfs__read_xll(
    entry: *const libc::c_char,
    value: *mut libc::c_ulonglong,
) -> libc::c_int {
    sysfs__read_ull_base(entry, value, 16)
}

#[no_mangle]
pub unsafe extern "C" fn sysfs__read_ull(
    entry: *const libc::c_char,
    value: *mut libc::c_ulonglong,
) -> libc::c_int {
    sysfs__read_ull_base(entry, value, 0)
}

#[no_mangle]
pub unsafe extern "C" fn sysfs__read_int(
    entry: *const libc::c_char,
    value: *mut libc::c_int,
) -> libc::c_int {
    let mut path: [libc::c_char; libc::PATH_MAX as usize] = [0; libc::PATH_MAX as usize];
    let sysfs = sysfs__mountpoint();

    if sysfs.is_null() {
        return -1;
    }

    libc::snprintf(
        path.as_mut_ptr(),
        mem::size_of_val(&path),
        PATH_JOIN_FMT.as_ptr() as *const libc::c_char,
        sysfs,
        entry,
    );

    filename__read_int(path.as_ptr(), value)
}

#[no_mangle]
pub unsafe extern "C" fn sysfs__read_str(
    entry: *const libc::c_char,
    buf: *mut *mut libc::c_char,
    sizep: *mut libc::size_t,
) -> libc::c_int {
    let mut path: [libc::c_char; libc::PATH_MAX as usize] = [0; libc::PATH_MAX as usize];
    let sysfs = sysfs__mountpoint();

    if sysfs.is_null() {
        return -1;
    }

    libc::snprintf(
        path.as_mut_ptr(),
        mem::size_of_val(&path),
        PATH_JOIN_FMT.as_ptr() as *const libc::c_char,
        sysfs,
        entry,
    );

    filename__read_str(path.as_ptr(), buf, sizep)
}

#[no_mangle]
pub unsafe extern "C" fn sysfs__read_bool(
    entry: *const libc::c_char,
    value: *mut bool,
) -> libc::c_int {
    let mut io: io = mem::zeroed();
    let mut bf: [libc::c_char; 16] = [0; 16];
    let mut ret = 0;
    let mut path: [libc::c_char; libc::PATH_MAX as usize] = [0; libc::PATH_MAX as usize];
    let sysfs = sysfs__mountpoint();

    if sysfs.is_null() {
        return -1;
    }

    libc::snprintf(
        path.as_mut_ptr(),
        mem::size_of_val(&path),
        PATH_JOIN_FMT.as_ptr() as *const libc::c_char,
        sysfs,
        entry,
    );
    io.fd = libc::open(path.as_ptr(), libc::O_RDONLY);
    if io.fd < 0 {
        return -*libc::__errno_location();
    }

    io__init(&mut io, io.fd, bf.as_mut_ptr(), mem::size_of_val(&bf));
    match io__get_char(&mut io) {
        x if x == b'1' as libc::c_int || x == b'y' as libc::c_int || x == b'Y' as libc::c_int => {
            *value = true;
        }
        x if x == b'0' as libc::c_int || x == b'n' as libc::c_int || x == b'N' as libc::c_int => {
            *value = false;
        }
        _ => {
            ret = -1;
        }
    }
    libc::close(io.fd);

    ret
}

#[no_mangle]
pub unsafe extern "C" fn sysctl__read_int(
    sysctl: *const libc::c_char,
    value: *mut libc::c_int,
) -> libc::c_int {
    let mut path: [libc::c_char; libc::PATH_MAX as usize] = [0; libc::PATH_MAX as usize];
    let procfs = procfs__mountpoint();

    if procfs.is_null() {
        return -1;
    }

    libc::snprintf(
        path.as_mut_ptr(),
        mem::size_of_val(&path),
        PROC_SYS_JOIN_FMT.as_ptr() as *const libc::c_char,
        procfs,
        sysctl,
    );

    filename__read_int(path.as_ptr(), value)
}

#[no_mangle]
pub unsafe extern "C" fn sysfs__write_int(
    entry: *const libc::c_char,
    value: libc::c_int,
) -> libc::c_int {
    let mut path: [libc::c_char; libc::PATH_MAX as usize] = [0; libc::PATH_MAX as usize];
    let sysfs = sysfs__mountpoint();

    if sysfs.is_null() {
        return -1;
    }

    if libc::snprintf(
        path.as_mut_ptr(),
        mem::size_of_val(&path),
        PATH_JOIN_FMT.as_ptr() as *const libc::c_char,
        sysfs,
        entry,
    ) >= libc::PATH_MAX
    {
        return -1;
    }

    filename__write_int(path.as_ptr(), value)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
