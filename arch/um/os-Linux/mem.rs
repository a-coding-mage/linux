// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
 */

use std::ffi::{c_char, c_int, c_ulonglong, c_void};
use std::ptr;

// Dependencies supplied by the surrounding UML tree and libc are intentionally
// referenced here rather than implemented in this translation unit.
extern "C" {
    fn os_info(fmt: *const c_char, ...);
    fn os_warn(fmt: *const c_char, ...);
    fn os_set_exec_close(fd: c_int) -> c_int;
}

const PROT_READ: c_int = 0x1;
const PROT_WRITE: c_int = 0x2;
const PROT_EXEC: c_int = 0x4;
const MAP_PRIVATE: c_int = 0x02;
const MAP_FIXED: c_int = 0x10;
const MAP_ANONYMOUS: c_int = 0x20;
const MAP_NORESERVE: c_int = 0x4000;
const MADV_DONTDUMP: c_int = 16;
const MADV_DONTFORK: c_int = 10;
const O_CLOEXEC: c_int = 0o2000000;
const O_RDWR: c_int = 0o2;
const O_EXCL: c_int = 0o200;
const O_TMPFILE: c_int = 0o20000000;
const SEEK_SET: c_int = 0;
const TMPFS_MAGIC: c_long = 0x01021994;
const EPERM: c_int = 1;
const EINVAL: c_int = 22;
const EISDIR: c_int = 21;
const EOPNOTSUPP: c_int = 95;

type c_long = i64;

#[repr(C)]
struct StatFs {
    _data: [u8; 256],
}

extern "C" {
    fn mmap(addr: *mut c_void, len: usize, prot: c_int, flags: c_int, fd: c_int, offset: i64) -> *mut c_void;
    fn madvise(addr: *mut c_void, len: usize, advice: c_int) -> c_int;
    fn munmap(addr: *mut c_void, len: usize) -> c_int;
    fn statfs(path: *const c_char, buf: *mut StatFs) -> c_int;
    fn getenv(name: *const c_char) -> *mut c_char;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn strlen(s: *const c_char) -> usize;
    fn strcpy(dst: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strcat(dst: *mut c_char, src: *const c_char) -> *mut c_char;
    fn open(path: *const c_char, flags: c_int, mode: c_int) -> c_int;
    fn mkstemp(template: *mut c_char) -> c_int;
    fn unlink(path: *const c_char) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn lseek64(fd: c_int, offset: c_ulonglong, whence: c_int) -> i64;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> isize;
    fn strerror(err: c_int) -> *const c_char;
    fn perror(s: *const c_char);
    fn __errno_location() -> *mut c_int;
    fn exit(status: c_int) -> !;
}

// Set by make_tempfile() during early boot.
pub static mut tempdir: *mut c_char = ptr::null_mut();

/*
 * kasan_map_memory - maps memory from @start with a size of @len.
 * The allocated memory is filled with zeroes upon success.
 * @start: the start address of the memory to be mapped
 * @len: the length of the memory to be mapped
 *
 * This function is used to map shadow memory for KASAN in uml
 */
pub unsafe fn kasan_map_memory(start: *mut c_void, len: usize) {
    if mmap(start, len, PROT_READ | PROT_WRITE,
            MAP_FIXED | MAP_ANONYMOUS | MAP_PRIVATE | MAP_NORESERVE, -1, 0)
        == (-1isize) as *mut c_void
    {
        os_info(b"Couldn't allocate shadow memory: %s\n.\0".as_ptr() as *const c_char, strerror(*__errno_location()));
        exit(1);
    }
    if madvise(start, len, MADV_DONTDUMP) != 0 {
        os_info(b"Couldn't set MAD_DONTDUMP on shadow memory: %s\n.\0".as_ptr() as *const c_char, strerror(*__errno_location()));
        exit(1);
    }
    if madvise(start, len, MADV_DONTFORK) != 0 {
        os_info(b"Couldn't set MAD_DONTFORK on shadow memory: %s\n.\0".as_ptr() as *const c_char, strerror(*__errno_location()));
        exit(1);
    }
}

unsafe fn check_tmpfs(dir: *const c_char) -> c_int {
    let mut st = StatFs { _data: [0; 256] };
    os_info(b"Checking if %s is on tmpfs...\0".as_ptr() as *const c_char, dir);
    if statfs(dir, &mut st) < 0 {
        os_info(b"%s\n\0".as_ptr() as *const c_char, strerror(*__errno_location()));
    } else {
        // The f_type field is platform-specific; retain the source-level test.
        let f_type = *(st._data.as_ptr() as *const c_long);
        if f_type != TMPFS_MAGIC { os_info(b"no\n\0".as_ptr() as *const c_char); }
        else { os_info(b"OK\n\0".as_ptr() as *const c_char); return 0; }
    }
    -1
}

unsafe fn choose_tempdir() -> *mut c_char {
    let vars = [b"TMPDIR\0".as_ptr(), b"TMP\0".as_ptr(), b"TEMP\0".as_ptr(), ptr::null()];
    let fallback = b"/tmp\0";
    let dirs = [b"/dev/shm\0".as_ptr(), fallback.as_ptr(), ptr::null()];
    os_info(b"Checking environment variables for a tempdir...\0".as_ptr() as *const c_char);
    for var in vars.iter().take(3) {
        let dir = getenv(*var as *const c_char);
        if !dir.is_null() && *dir != 0 {
            os_info(b"%s\n\0".as_ptr() as *const c_char, dir);
            if check_tmpfs(dir) >= 0 { return strdup(dir); }
            os_warn(b"Warning: tempdir %s is not on tmpfs\n\0".as_ptr() as *const c_char, dir);
            return strdup(dir);
        }
    }
    os_info(b"none found\n\0".as_ptr() as *const c_char);
    for dir in dirs.iter().take(2) { if check_tmpfs(*dir as *const c_char) >= 0 { return strdup(*dir as *const c_char); } }
    os_warn(b"Warning: tempdir %s is not on tmpfs\n\0".as_ptr() as *const c_char, fallback.as_ptr());
    strdup(fallback.as_ptr() as *const c_char)
}

unsafe fn make_tempfile(template: *const c_char) -> c_int {
    if tempdir.is_null() {
        tempdir = choose_tempdir();
        if tempdir.is_null() { os_warn(b"Failed to choose tempdir: %s\n\0".as_ptr() as *const c_char, strerror(*__errno_location())); return -1; }
    }
    let fd = open(tempdir, O_CLOEXEC | O_RDWR | O_EXCL | O_TMPFILE, 0o700);
    if fd != -1 || (*__errno_location() != EINVAL && *__errno_location() != EISDIR && *__errno_location() != EOPNOTSUPP) { return fd; }
    let name = malloc(strlen(tempdir) + strlen(template) + 1) as *mut c_char;
    if name.is_null() { return -1; }
    strcpy(name, tempdir); strcat(name, template);
    let fd = mkstemp(name);
    if fd < 0 { os_warn(b"open - cannot create %s: %s\n\0".as_ptr() as *const c_char, name, strerror(*__errno_location())); free(name as *mut c_void); return -1; }
    if unlink(name) < 0 { perror(b"unlink\0".as_ptr() as *const c_char); close(fd); free(name as *mut c_void); return -1; }
    free(name as *mut c_void); fd
}

const TEMPNAME_TEMPLATE: &[u8] = b"/vm_file-XXXXXX\0";

unsafe fn create_tmp_file(len: c_ulonglong) -> c_int {
    let fd = make_tempfile(TEMPNAME_TEMPLATE.as_ptr() as *const c_char);
    if fd < 0 { exit(1); }
    if lseek64(fd, len - 1, SEEK_SET) < 0 { perror(b"lseek64\0".as_ptr() as *const c_char); exit(1); }
    let zero: u8 = 0;
    if write(fd, &zero as *const u8 as *const c_void, 1) != 1 { perror(b"write\0".as_ptr() as *const c_char); exit(1); }
    fd
}

pub unsafe fn create_mem_file(len: c_ulonglong) -> c_int {
    let fd = create_tmp_file(len);
    let err = os_set_exec_close(fd);
    if err < 0 { *__errno_location() = -err; perror(b"exec_close\0".as_ptr() as *const c_char); }
    fd
}

pub unsafe fn check_tmpexec() {
    const UM_KERN_PAGE_SIZE: usize = 4096;
    let fd = create_tmp_file(UM_KERN_PAGE_SIZE as c_ulonglong);
    let addr = mmap(ptr::null_mut(), UM_KERN_PAGE_SIZE, PROT_READ | PROT_WRITE | PROT_EXEC, MAP_PRIVATE, fd, 0);
    os_info(b"Checking PROT_EXEC mmap in %s...\0".as_ptr() as *const c_char, tempdir);
    if addr == (-1isize) as *mut c_void {
        let err = *__errno_location(); os_warn(b"%s\n\0".as_ptr() as *const c_char, strerror(err)); close(fd);
        if err == EPERM { os_warn(b"%s must be not mounted noexec\n\0".as_ptr() as *const c_char, tempdir); }
        exit(1);
    }
    os_info(b"OK\n\0".as_ptr() as *const c_char); munmap(addr, UM_KERN_PAGE_SIZE); close(fd);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
