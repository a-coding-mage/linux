// SPDX-License-Identifier: GPL-2.0
// Translated from perf/util/copyfile.c.
// Dependencies from: util/copyfile.h, util/namespaces.h, internal/lib.h,
// sys/mman.h, sys/stat.h, errno.h, fcntl.h, stdio.h, stdlib.h, string.h,
// unistd.h.

use core::ffi::{c_char, c_int, c_long, c_void};

type size_t = usize;
type ssize_t = isize;
type mode_t = u32;
type loff_t = i64;
type u64 = u64;

#[repr(C)]
pub struct nsinfo {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nscookie {
    _private: [u8; 0],
}

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct stat {
    pub st_dev: u64,
    pub st_ino: u64,
    pub st_nlink: u64,
    pub st_mode: u32,
    pub st_uid: u32,
    pub st_gid: u32,
    pub __pad0: c_int,
    pub st_rdev: u64,
    pub st_size: loff_t,
    pub st_blksize: c_long,
    pub st_blocks: c_long,
    pub st_atime: c_long,
    pub st_atime_nsec: c_long,
    pub st_mtime: c_long,
    pub st_mtime_nsec: c_long,
    pub st_ctime: c_long,
    pub st_ctime_nsec: c_long,
    pub __unused: [c_long; 3],
}

extern "C" {
    static page_size: loff_t;

    fn nsinfo__mountns_enter(nsi: *mut nsinfo, nsc: *mut nscookie);
    fn nsinfo__mountns_exit(nsc: *mut nscookie);

    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fclose(stream: *mut FILE) -> c_int;
    fn getline(lineptr: *mut *mut c_char, n: *mut size_t, stream: *mut FILE) -> ssize_t;
    fn fputs(s: *const c_char, stream: *mut FILE) -> c_int;
    fn free(ptr: *mut c_void);
    fn stat(pathname: *const c_char, statbuf: *mut stat) -> c_int;
    fn asprintf(strp: *mut *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn strrchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strlen(s: *const c_char) -> size_t;
    fn memmove(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn mkstemp(template: *mut c_char) -> c_int;
    fn fchmod(fd: c_int, mode: mode_t) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn link(oldpath: *const c_char, newpath: *const c_char) -> c_int;
    fn unlink(pathname: *const c_char) -> c_int;
    fn mmap(
        addr: *mut c_void,
        length: size_t,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: loff_t,
    ) -> *mut c_void;
    fn munmap(addr: *mut c_void, length: size_t) -> c_int;
    fn pwrite(fd: c_int, buf: *const c_void, count: size_t, offset: loff_t) -> ssize_t;
    fn __errno_location() -> *mut c_int;
}

const EOF: c_int = -1;
const EINTR: c_int = 4;
const O_RDONLY: c_int = 0;
const PROT_READ: c_int = 0x1;
const MAP_PRIVATE: c_int = 0x02;
const MAP_FAILED: *mut c_void = !0usize as *mut c_void;

unsafe fn slow_copyfile(from: *const c_char, to: *const c_char, nsi: *mut nsinfo) -> c_int {
    let mut err: c_int = -1;
    let mut line: *mut c_char = core::ptr::null_mut();
    let mut n: size_t = 0;
    let from_fp: *mut FILE;
    let to_fp: *mut FILE;
    let mut nsc = core::mem::MaybeUninit::<nscookie>::uninit();

    nsinfo__mountns_enter(nsi, nsc.as_mut_ptr());
    from_fp = fopen(from, b"r\0".as_ptr() as *const c_char);
    nsinfo__mountns_exit(nsc.as_mut_ptr());
    if from_fp.is_null() {
        return err;
    }

    to_fp = fopen(to, b"w\0".as_ptr() as *const c_char);
    if to_fp.is_null() {
        fclose(from_fp);
        return err;
    }

    while getline(&mut line, &mut n, from_fp) > 0 {
        if fputs(line, to_fp) == EOF {
            fclose(to_fp);
            free(line as *mut c_void);
            fclose(from_fp);
            return err;
        }
    }
    err = 0;

    fclose(to_fp);
    free(line as *mut c_void);
    fclose(from_fp);
    err
}

#[no_mangle]
pub unsafe extern "C" fn copyfile_offset(
    ifd: c_int,
    mut off_in: loff_t,
    ofd: c_int,
    mut off_out: loff_t,
    mut size: u64,
) -> c_int {
    let ptr: *mut c_void;
    let pgoff: loff_t;

    pgoff = off_in & !(page_size - 1);
    off_in -= pgoff;

    ptr = mmap(
        core::ptr::null_mut(),
        (off_in as u64).wrapping_add(size) as size_t,
        PROT_READ,
        MAP_PRIVATE,
        ifd,
        pgoff,
    );
    if ptr == MAP_FAILED {
        return -1;
    }

    while size != 0 {
        let ret = pwrite(
            ofd,
            (ptr as *mut u8).offset(off_in as isize) as *const c_void,
            size as size_t,
            off_out,
        );
        if ret < 0 && *__errno_location() == EINTR {
            continue;
        }
        if ret <= 0 {
            break;
        }

        size = size.wrapping_sub(ret as u64);
        off_in = off_in.wrapping_add(ret as loff_t);
        off_out = off_out.wrapping_add(ret as loff_t);
    }
    munmap(ptr, (off_in as u64).wrapping_add(size) as size_t);

    if size != 0 { -1 } else { 0 }
}

unsafe fn copyfile_mode_ns(
    from: *const c_char,
    to: *const c_char,
    mode: mode_t,
    nsi: *mut nsinfo,
) -> c_int {
    let mut fromfd: c_int;
    let mut tofd: c_int;
    let mut st = core::mem::MaybeUninit::<stat>::uninit();
    let mut err: c_int;
    let mut tmp: *mut c_char = core::ptr::null_mut();
    let mut ptr: *mut c_char = core::ptr::null_mut();
    let mut nsc = core::mem::MaybeUninit::<nscookie>::uninit();

    nsinfo__mountns_enter(nsi, nsc.as_mut_ptr());
    err = stat(from, st.as_mut_ptr());
    nsinfo__mountns_exit(nsc.as_mut_ptr());
    if err != 0 {
        free(tmp as *mut c_void);
        return err;
    }
    err = -1;

    /* extra 'x' at the end is to reserve space for '.' */
    if asprintf(&mut tmp, b"%s.XXXXXXx\0".as_ptr() as *const c_char, to) < 0 {
        tmp = core::ptr::null_mut();
        free(tmp as *mut c_void);
        return err;
    }
    ptr = strrchr(tmp, '/' as c_int);
    if ptr.is_null() {
        free(tmp as *mut c_void);
        return err;
    }
    ptr = memmove(
        ptr.offset(1) as *mut c_void,
        ptr as *const c_void,
        strlen(ptr).wrapping_sub(1),
    ) as *mut c_char;
    *ptr = b'.' as c_char;

    tofd = mkstemp(tmp);
    if tofd < 0 {
        free(tmp as *mut c_void);
        return err;
    }

    let st = st.assume_init();
    if st.st_size == 0 {
        /* /proc? do it slowly... */
        err = slow_copyfile(from, tmp, nsi);
        if err == 0 && fchmod(tofd, mode) != 0 {
            err = -1;
        }
        close(tofd);
        if err == 0 {
            err = link(tmp, to);
        }
        unlink(tmp);
        free(tmp as *mut c_void);
        return err;
    }

    if fchmod(tofd, mode) != 0 {
        close(tofd);
        unlink(tmp);
        free(tmp as *mut c_void);
        return err;
    }

    nsinfo__mountns_enter(nsi, nsc.as_mut_ptr());
    fromfd = open(from, O_RDONLY);
    nsinfo__mountns_exit(nsc.as_mut_ptr());
    if fromfd < 0 {
        close(tofd);
        unlink(tmp);
        free(tmp as *mut c_void);
        return err;
    }

    err = copyfile_offset(fromfd, 0, tofd, 0, st.st_size as u64);

    close(fromfd);
    close(tofd);
    if err == 0 {
        err = link(tmp, to);
    }
    unlink(tmp);
    free(tmp as *mut c_void);
    err
}

#[no_mangle]
pub unsafe extern "C" fn copyfile_ns(
    from: *const c_char,
    to: *const c_char,
    nsi: *mut nsinfo,
) -> c_int {
    copyfile_mode_ns(from, to, 0o755, nsi)
}

#[no_mangle]
pub unsafe extern "C" fn copyfile_mode(
    from: *const c_char,
    to: *const c_char,
    mode: mode_t,
) -> c_int {
    copyfile_mode_ns(from, to, mode, core::ptr::null_mut())
}

#[no_mangle]
pub unsafe extern "C" fn copyfile(from: *const c_char, to: *const c_char) -> c_int {
    copyfile_mode(from, to, 0o755)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
