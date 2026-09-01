// SPDX-License-Identifier: GPL-2.0
// C source defined _GNU_SOURCE and __SANE_USERSPACE_TYPES__ // Use ll64
// Dependencies from the original includes are expected to be supplied by the
// surrounding selftest build.

use std::ffi::{c_char, c_int, c_long, c_ulong, c_void};
use std::ptr;

const NR_TESTS: c_int = 9;

const O_RDONLY: c_int = 0;
const O_RDWR: c_int = 2;
const O_CREAT: c_int = 0o100;
const O_TRUNC: c_int = 0o1000;

const PROT_READ: c_int = 0x1;
const PROT_WRITE: c_int = 0x2;
const MAP_SHARED: c_int = 0x01;

const _SC_PAGESIZE: c_int = 30;
const ENOSYS: c_int = 38;
const EBADF: c_int = 9;
const TMPFS_MAGIC: c_long = 0x01021994;

const KSFT_PASS: c_int = 0;
const KSFT_FAIL: c_int = 1;
const KSFT_SKIP: c_int = 4;

const __NR_cachestat: c_long = 451;

type size_t = usize;
type ssize_t = isize;
type mode_t = u32;
type off_t = i64;

#[repr(C)]
struct cachestat {
    nr_cache: u64,
    nr_dirty: u64,
    nr_writeback: u64,
    nr_evicted: u64,
    nr_recently_evicted: u64,
}

#[repr(C)]
struct cachestat_range {
    off: u64,
    len: u64,
}

#[repr(C)]
struct statfs {
    f_type: c_long,
    f_bsize: c_long,
    f_blocks: u64,
    f_bfree: u64,
    f_bavail: u64,
    f_files: u64,
    f_ffree: u64,
    f_fsid: [c_int; 2],
    f_namelen: c_long,
    f_frsize: c_long,
    f_flags: c_long,
    f_spare: [c_long; 4],
}

unsafe extern "C" {
    fn open(pathname: *const c_char, flags: c_int, mode: mode_t) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn fstatfs(fd: c_int, buf: *mut statfs) -> c_int;
    fn fsync(fd: c_int) -> c_int;
    fn remove(pathname: *const c_char) -> c_int;
    fn sysconf(name: c_int) -> c_long;
    fn syscall(num: c_long, ...) -> c_long;
    fn shm_open(name: *const c_char, oflag: c_int, mode: mode_t) -> c_int;
    fn shm_unlink(name: *const c_char) -> c_int;
    fn ftruncate(fd: c_int, length: off_t) -> c_int;
    fn mmap(
        addr: *mut c_void,
        length: size_t,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: off_t,
    ) -> *mut c_void;
    fn __errno_location() -> *mut c_int;

    fn ksft_print_header();
    fn ksft_print_msg(fmt: *const c_char, ...);
    fn ksft_exit_skip(fmt: *const c_char, ...) -> !;
    fn ksft_set_plan(plan: c_int);
    fn ksft_test_result_pass(fmt: *const c_char, ...);
    fn ksft_test_result_fail(fmt: *const c_char, ...);
    fn ksft_test_result_skip(fmt: *const c_char, ...);
}

const MAP_FAILED: *mut c_void = !0usize as *mut c_void;

static DEV_FILES: [*const c_char; 5] = [
    b"/dev/zero\0".as_ptr() as *const c_char,
    b"/dev/null\0".as_ptr() as *const c_char,
    b"/dev/urandom\0".as_ptr() as *const c_char,
    b"/proc/version\0".as_ptr() as *const c_char,
    b"/proc\0".as_ptr() as *const c_char,
];

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum file_type {
    FILE_MMAP,
    FILE_SHMEM,
}

unsafe fn print_cachestat(cs: *mut cachestat) {
    unsafe {
        ksft_print_msg(
            b"Using cachestat: Cached: %llu, Dirty: %llu, Writeback: %llu, Evicted: %llu, Recently Evicted: %llu\n\0"
                .as_ptr() as *const c_char,
            (*cs).nr_cache,
            (*cs).nr_dirty,
            (*cs).nr_writeback,
            (*cs).nr_evicted,
            (*cs).nr_recently_evicted,
        );
    }
}

unsafe fn write_exactly(fd: c_int, filesize: size_t) -> bool {
    let random_fd: c_int = unsafe { open(b"/dev/urandom\0".as_ptr() as *const c_char, O_RDONLY, 0) };
    let mut cursor: *mut c_char;
    let data: *mut c_char;
    let mut remained: c_int;
    let ret: bool;

    if random_fd < 0 {
        unsafe { ksft_print_msg(b"Unable to access urandom.\n\0".as_ptr() as *const c_char) };
        ret = false;
        return ret;
    }

    data = unsafe { malloc(filesize) as *mut c_char };
    if data.is_null() {
        unsafe { ksft_print_msg(b"Unable to allocate data.\n\0".as_ptr() as *const c_char) };
        ret = false;
        unsafe {
            close(random_fd);
        }
        return ret;
    }

    remained = filesize as c_int;
    cursor = data;

    while remained != 0 {
        let read_len: ssize_t =
            unsafe { read(random_fd, cursor as *mut c_void, remained as size_t) };

        if read_len <= 0 {
            unsafe { ksft_print_msg(b"Unable to read from urandom.\n\0".as_ptr() as *const c_char) };
            ret = false;
            unsafe {
                free(data as *mut c_void);
                close(random_fd);
            }
            return ret;
        }

        remained -= read_len as c_int;
        cursor = unsafe { cursor.add(read_len as usize) };
    }

    /* write random data to fd */
    remained = filesize as c_int;
    cursor = data;
    while remained != 0 {
        let write_len: ssize_t =
            unsafe { write(fd, cursor as *const c_void, remained as size_t) };

        if write_len <= 0 {
            unsafe { ksft_print_msg(b"Unable write random data to file.\n\0".as_ptr() as *const c_char) };
            ret = false;
            unsafe {
                free(data as *mut c_void);
                close(random_fd);
            }
            return ret;
        }

        remained -= write_len as c_int;
        cursor = unsafe { cursor.add(write_len as usize) };
    }

    ret = true;
    unsafe {
        free(data as *mut c_void);
        close(random_fd);
    }
    ret
}

/*
 * fsync() is implemented via noop_fsync() on tmpfs. This makes the fsync()
 * test fail below, so we need to check for test file living on a tmpfs.
 */
unsafe fn is_on_tmpfs(fd: c_int) -> bool {
    let mut statfs_buf: statfs = unsafe { std::mem::zeroed() };

    if unsafe { fstatfs(fd, &mut statfs_buf) } != 0 {
        return false;
    }

    statfs_buf.f_type == TMPFS_MAGIC
}

/*
 * Open/create the file at filename, (optionally) write random data to it
 * (exactly num_pages), then test the cachestat syscall on this file.
 *
 * If test_fsync == true, fsync the file, then check the number of dirty
 * pages.
 */
unsafe fn test_cachestat(
    filename: *const c_char,
    write_random: bool,
    create: bool,
    test_fsync: bool,
    num_pages: c_ulong,
    open_flags: c_int,
    open_mode: mode_t,
) -> c_int {
    let ps: size_t = unsafe { sysconf(_SC_PAGESIZE) as size_t };
    let filesize: c_int = (num_pages as size_t * ps) as c_int;
    let mut ret: c_int = KSFT_PASS;
    let mut syscall_ret: c_long;
    let mut cs: cachestat = unsafe { std::mem::zeroed() };
    let mut cs_range = cachestat_range {
        off: 0,
        len: filesize as u64,
    };

    let fd: c_int = unsafe { open(filename, open_flags, open_mode) };

    if fd == -1 {
        unsafe { ksft_print_msg(b"Unable to create/open file.\n\0".as_ptr() as *const c_char) };
        ret = KSFT_FAIL;
        return ret;
    } else {
        unsafe { ksft_print_msg(b"Create/open %s\n\0".as_ptr() as *const c_char, filename) };
    }

    if write_random {
        if !unsafe { write_exactly(fd, filesize as size_t) } {
            unsafe { ksft_print_msg(b"Unable to access urandom.\n\0".as_ptr() as *const c_char) };
            ret = KSFT_FAIL;
            unsafe {
                close(fd);
            }
            if create {
                unsafe {
                    remove(filename);
                }
            }
            return ret;
        }
    }

    syscall_ret = unsafe {
        syscall(
            __NR_cachestat,
            fd,
            &mut cs_range as *mut cachestat_range,
            &mut cs as *mut cachestat,
            0,
        )
    };

    unsafe {
        ksft_print_msg(
            b"Cachestat call returned %ld\n\0".as_ptr() as *const c_char,
            syscall_ret,
        )
    };

    if syscall_ret != 0 {
        unsafe { ksft_print_msg(b"Cachestat returned non-zero.\n\0".as_ptr() as *const c_char) };
        ret = KSFT_FAIL;
        unsafe {
            close(fd);
        }
        if create {
            unsafe {
                remove(filename);
            }
        }
        return ret;
    } else {
        unsafe { print_cachestat(&mut cs) };

        if write_random {
            if cs.nr_cache + cs.nr_evicted != num_pages as u64 {
                unsafe {
                    ksft_print_msg(
                        b"Total number of cached and evicted pages is off.\n\0".as_ptr()
                            as *const c_char,
                    )
                };
                ret = KSFT_FAIL;
            }
        }
    }

    if test_fsync {
        if unsafe { is_on_tmpfs(fd) } {
            ret = KSFT_SKIP;
        } else if unsafe { fsync(fd) } != 0 {
            unsafe { ksft_print_msg(b"fsync fails.\n\0".as_ptr() as *const c_char) };
            ret = KSFT_FAIL;
        } else {
            syscall_ret = unsafe {
                syscall(
                    __NR_cachestat,
                    fd,
                    &mut cs_range as *mut cachestat_range,
                    &mut cs as *mut cachestat,
                    0,
                )
            };

            unsafe {
                ksft_print_msg(
                    b"Cachestat call (after fsync) returned %ld\n\0".as_ptr() as *const c_char,
                    syscall_ret,
                )
            };

            if syscall_ret == 0 {
                unsafe { print_cachestat(&mut cs) };

                if cs.nr_dirty != 0 {
                    ret = KSFT_FAIL;
                    unsafe {
                        ksft_print_msg(
                            b"Number of dirty should be zero after fsync.\n\0".as_ptr()
                                as *const c_char,
                        )
                    };
                }
            } else {
                unsafe {
                    ksft_print_msg(
                        b"Cachestat (after fsync) returned non-zero.\n\0".as_ptr()
                            as *const c_char,
                    )
                };
                ret = KSFT_FAIL;
                unsafe {
                    close(fd);
                }
                if create {
                    unsafe {
                        remove(filename);
                    }
                }
                return ret;
            }
        }
    }

    unsafe {
        close(fd);
    }

    if create {
        unsafe {
            remove(filename);
        }
    }
    ret
}

unsafe fn file_type_str(r#type: file_type) -> *const c_char {
    match r#type {
        file_type::FILE_SHMEM => b"shmem\0".as_ptr() as *const c_char,
        file_type::FILE_MMAP => b"mmap\0".as_ptr() as *const c_char,
    }
}

unsafe fn run_cachestat_test(r#type: file_type) -> bool {
    let ps: size_t = unsafe { sysconf(_SC_PAGESIZE) as size_t };
    let filesize: size_t = ps * 512 * 2; /* 2 2MB huge pages */
    let syscall_ret: c_int;
    let compute_len: size_t = ps * 512;
    let mut cs_range = cachestat_range {
        off: ps as u64,
        len: compute_len as u64,
    };
    let filename: *const c_char = b"tmpshmcstat\0".as_ptr() as *const c_char;
    let mut map: *mut c_char;
    let mut cs: cachestat = unsafe { std::mem::zeroed() };
    let mut ret: bool = true;
    let fd: c_int;
    let num_pages: c_ulong = (compute_len / ps) as c_ulong;

    if r#type == file_type::FILE_SHMEM {
        fd = unsafe { shm_open(filename, O_CREAT | O_RDWR, 0o600) };
    } else {
        fd = unsafe { open(filename, O_RDWR | O_CREAT | O_TRUNC, 0o666) };
    }

    if fd < 0 {
        unsafe {
            ksft_print_msg(
                b"Unable to create %s file.\n\0".as_ptr() as *const c_char,
                file_type_str(r#type),
            )
        };
        ret = false;
        return ret;
    }

    if unsafe { ftruncate(fd, filesize as off_t) } != 0 {
        unsafe {
            ksft_print_msg(
                b"Unable to truncate %s file.\n\0".as_ptr() as *const c_char,
                file_type_str(r#type),
            )
        };
        ret = false;
        unsafe {
            close(fd);
        }
        return ret;
    }
    match r#type {
        file_type::FILE_SHMEM => {
            if !unsafe { write_exactly(fd, filesize) } {
                unsafe { ksft_print_msg(b"Unable to write to file.\n\0".as_ptr() as *const c_char) };
                ret = false;
                unsafe {
                    close(fd);
                }
                return ret;
            }
        }
        file_type::FILE_MMAP => {
            map = unsafe {
                mmap(
                    ptr::null_mut(),
                    filesize,
                    PROT_READ | PROT_WRITE,
                    MAP_SHARED,
                    fd,
                    0,
                ) as *mut c_char
            };

            if map as *mut c_void == MAP_FAILED {
                unsafe { ksft_print_msg(b"mmap failed.\n\0".as_ptr() as *const c_char) };
                ret = false;
                unsafe {
                    close(fd);
                }
                return ret;
            }
            for i in 0..filesize {
                unsafe {
                    *map.add(i) = b'A' as c_char;
                }
            }
        }
    }
    syscall_ret = unsafe {
        syscall(
            __NR_cachestat,
            fd,
            &mut cs_range as *mut cachestat_range,
            &mut cs as *mut cachestat,
            0,
        ) as c_int
    };

    if syscall_ret != 0 {
        unsafe { ksft_print_msg(b"Cachestat returned non-zero.\n\0".as_ptr() as *const c_char) };
        ret = false;
        unsafe {
            close(fd);
        }
        return ret;
    } else {
        unsafe { print_cachestat(&mut cs) };
        if cs.nr_cache + cs.nr_evicted != num_pages as u64 {
            unsafe {
                ksft_print_msg(
                    b"Total number of cached and evicted pages is off.\n\0".as_ptr()
                        as *const c_char,
                )
            };
            ret = false;
        }
    }

    unsafe {
        shm_unlink(filename);
    }
    ret
}

fn main() {
    let mut ret: c_int;

    unsafe {
        ksft_print_header();

        ret = syscall(
            __NR_cachestat,
            -1,
            ptr::null::<c_void>(),
            ptr::null::<c_void>(),
            0,
        ) as c_int;
        if ret == -1 && *__errno_location() == ENOSYS {
            ksft_exit_skip(b"cachestat syscall not available\n\0".as_ptr() as *const c_char);
        }

        ksft_set_plan(NR_TESTS);

        if ret == -1 && *__errno_location() == EBADF {
            ksft_test_result_pass(b"bad file descriptor recognized\n\0".as_ptr() as *const c_char);
            ret = 0;
        } else {
            ksft_test_result_fail(b"bad file descriptor ignored\n\0".as_ptr() as *const c_char);
            ret = 1;
        }

        for i in 0..5 {
            let dev_filename: *const c_char = DEV_FILES[i];

            if test_cachestat(dev_filename, false, false, false, 4, O_RDONLY, 0o400) == KSFT_PASS {
                ksft_test_result_pass(
                    b"cachestat works with %s\n\0".as_ptr() as *const c_char,
                    dev_filename,
                );
            } else {
                ksft_test_result_fail(
                    b"cachestat fails with %s\n\0".as_ptr() as *const c_char,
                    dev_filename,
                );
                ret = 1;
            }
        }

        if test_cachestat(
            b"tmpfilecachestat\0".as_ptr() as *const c_char,
            true,
            true,
            false,
            4,
            O_CREAT | O_RDWR,
            0o600,
        ) == KSFT_PASS
        {
            ksft_test_result_pass(
                b"cachestat works with a normal file\n\0".as_ptr() as *const c_char,
            );
        } else {
            ksft_test_result_fail(
                b"cachestat fails with normal file\n\0".as_ptr() as *const c_char,
            );
            ret = 1;
        }

        match test_cachestat(
            b"tmpfilecachestat\0".as_ptr() as *const c_char,
            true,
            true,
            true,
            4,
            O_CREAT | O_RDWR,
            0o600,
        ) {
            KSFT_FAIL => {
                ksft_test_result_fail(
                    b"cachestat fsync fails with normal file\n\0".as_ptr() as *const c_char,
                );
                ret = KSFT_FAIL;
            }
            KSFT_PASS => {
                ksft_test_result_pass(
                    b"cachestat fsync works with a normal file\n\0".as_ptr() as *const c_char,
                );
            }
            KSFT_SKIP => {
                ksft_test_result_skip(
                    b"tmpfilecachestat is on tmpfs\n\0".as_ptr() as *const c_char,
                );
            }
            _ => {}
        }

        if run_cachestat_test(file_type::FILE_SHMEM) {
            ksft_test_result_pass(
                b"cachestat works with a shmem file\n\0".as_ptr() as *const c_char,
            );
        } else {
            ksft_test_result_fail(
                b"cachestat fails with a shmem file\n\0".as_ptr() as *const c_char,
            );
            ret = 1;
        }

        if run_cachestat_test(file_type::FILE_MMAP) {
            ksft_test_result_pass(
                b"cachestat works with a mmap file\n\0".as_ptr() as *const c_char,
            );
        } else {
            ksft_test_result_fail(
                b"cachestat fails with a mmap file\n\0".as_ptr() as *const c_char,
            );
            ret = 1;
        }
    }
    std::process::exit(ret);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
