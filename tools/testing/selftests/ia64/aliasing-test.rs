// SPDX-License-Identifier: GPL-2.0-only
/*
 * Exercise /dev/mem mmap cases that have been troublesome in the past
 *
 * (c) Copyright 2007 Hewlett-Packard Development Company, L.P.
 *	Bjorn Helgaas <bjorn.helgaas@hp.com>
 */

use std::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use std::ptr;

type off_t = c_long;
type size_t = usize;
type ssize_t = isize;
type mode_t = c_uint;

const O_RDWR: c_int = 0o2;
const PROT_READ: c_int = 0x1;
const PROT_WRITE: c_int = 0x2;
const MAP_SHARED: c_int = 0x01;
const PCIIOC_MMAP_IS_MEM: c_ulong = 0x80087003;
const S_IFMT: mode_t = 0o170000;
const S_IFDIR: mode_t = 0o040000;

#[repr(C)]
struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
struct dirent {
    d_ino: c_ulong,
    d_off: c_long,
    d_reclen: u16,
    d_type: u8,
    d_name: [c_char; 256],
}

#[repr(C)]
struct stat {
    st_dev: c_ulong,
    st_ino: c_ulong,
    st_nlink: c_ulong,
    st_mode: mode_t,
    st_uid: c_uint,
    st_gid: c_uint,
    __pad0: c_int,
    st_rdev: c_ulong,
    st_size: c_long,
    st_blksize: c_long,
    st_blocks: c_long,
    st_atime: c_long,
    st_atime_nsec: c_long,
    st_mtime: c_long,
    st_mtime_nsec: c_long,
    st_ctime: c_long,
    st_ctime_nsec: c_long,
    __unused: [c_long; 3],
}

unsafe extern "C" {
    static mut stderr: *mut FILE;
    static alphasort: unsafe extern "C" fn(*const *const dirent, *const *const dirent) -> c_int;

    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn perror(s: *const c_char);
    fn fnmatch(pattern: *const c_char, string: *const c_char, flags: c_int) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn mmap(
        addr: *mut c_void,
        length: size_t,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: off_t,
    ) -> *mut c_void;
    fn munmap(addr: *mut c_void, length: size_t) -> c_int;
    fn scandir(
        dirp: *const c_char,
        namelist: *mut *mut *mut dirent,
        filter: Option<unsafe extern "C" fn(*const dirent) -> c_int>,
        compar: Option<unsafe extern "C" fn(*const *const dirent, *const *const dirent) -> c_int>,
    ) -> c_int;
    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn strlen(s: *const c_char) -> size_t;
    fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strcat(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn lstat(pathname: *const c_char, statbuf: *mut stat) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
}

static mut sum: c_int = 0;
static mut buf: [c_char; 1024] = [0; 1024];

fn s_isdir(mode: mode_t) -> bool {
    (mode & S_IFMT) == S_IFDIR
}

unsafe fn map_mem(path: *mut c_char, offset: off_t, length: size_t, touch: c_int) -> c_int {
    let fd: c_int;
    let mut rc: c_int;
    let addr: *mut c_void;
    let mut c: *mut c_int;

    fd = open(path, O_RDWR);
    if fd == -1 {
        perror(path);
        return -1;
    }

    if fnmatch(c"/proc/bus/pci/*".as_ptr(), path, 0) == 0 {
        rc = ioctl(fd, PCIIOC_MMAP_IS_MEM);
        if rc == -1 {
            perror(c"PCIIOC_MMAP_IS_MEM ioctl".as_ptr());
        }
    }

    addr = mmap(
        ptr::null_mut(),
        length,
        PROT_READ | PROT_WRITE,
        MAP_SHARED,
        fd,
        offset,
    );
    if addr == (-1_isize) as *mut c_void {
        return 1;
    }

    if touch != 0 {
        c = addr as *mut c_int;
        while c < (addr as *mut u8).add(length) as *mut c_int {
            sum += *c;
            c = c.add(1);
        }
    }

    rc = munmap(addr, length);
    if rc == -1 {
        perror(c"munmap".as_ptr());
        return -1;
    }

    close(fd);
    return 0;
}

unsafe fn scan_tree(
    path: *mut c_char,
    file: *mut c_char,
    offset: off_t,
    length: size_t,
    touch: c_int,
) -> c_int {
    let mut namelist: *mut *mut dirent = ptr::null_mut();
    let mut name: *mut c_char;
    let mut path2: *mut c_char;
    let mut i: c_int;
    let n: c_int;
    let mut r: c_int;
    let mut rc: c_int = 0;
    let mut result: c_int = 0;
    let mut stat_buf: stat = std::mem::zeroed();

    n = scandir(path, &mut namelist, None, Some(alphasort));
    if n < 0 {
        perror(c"scandir".as_ptr());
        return -1;
    }

    i = 0;
    while i < n {
        name = (*(*namelist.add(i as usize))).d_name.as_mut_ptr();

        if fnmatch(c".".as_ptr(), name, 0) == 0 {
            free(*namelist.add(i as usize) as *mut c_void);
            i += 1;
            continue;
        }
        if fnmatch(c"..".as_ptr(), name, 0) == 0 {
            free(*namelist.add(i as usize) as *mut c_void);
            i += 1;
            continue;
        }

        path2 = malloc(strlen(path) + strlen(name) + 3) as *mut c_char;
        strcpy(path2, path);
        strcat(path2, c"/".as_ptr());
        strcat(path2, name);

        if fnmatch(file, name, 0) == 0 {
            rc = map_mem(path2, offset, length, touch);
            if rc == 0 {
                fprintf(
                    stderr,
                    c"PASS: %s 0x%lx-0x%lx is %s\n".as_ptr(),
                    path2,
                    offset,
                    offset + length as off_t,
                    if touch != 0 {
                        c"readable".as_ptr()
                    } else {
                        c"mappable".as_ptr()
                    },
                );
            } else if rc > 0 {
                fprintf(
                    stderr,
                    c"PASS: %s 0x%lx-0x%lx not mappable\n".as_ptr(),
                    path2,
                    offset,
                    offset + length as off_t,
                );
            } else {
                fprintf(
                    stderr,
                    c"FAIL: %s 0x%lx-0x%lx not accessible\n".as_ptr(),
                    path2,
                    offset,
                    offset + length as off_t,
                );
                return rc;
            }
        } else {
            r = lstat(path2, &mut stat_buf);
            if r == 0 && s_isdir(stat_buf.st_mode) {
                rc = scan_tree(path2, file, offset, length, touch);
                if rc < 0 {
                    return rc;
                }
            }
        }

        result |= rc;
        free(path2 as *mut c_void);

        free(*namelist.add(i as usize) as *mut c_void);
        i += 1;
    }
    free(namelist as *mut c_void);
    return result;
}

unsafe fn read_rom(path: *mut c_char) -> c_int {
    let fd: c_int;
    let mut rc: c_int;
    let mut size: size_t = 0;

    fd = open(path, O_RDWR);
    if fd == -1 {
        perror(path);
        return -1;
    }

    rc = write(fd, c"1".as_ptr() as *const c_void, 2) as c_int;
    if rc <= 0 {
        close(fd);
        perror(c"write".as_ptr());
        return -1;
    }

    loop {
        rc = read(fd, buf.as_mut_ptr() as *mut c_void, buf.len()) as c_int;
        if rc > 0 {
            size += rc as size_t;
        }
        if rc <= 0 {
            break;
        }
    }

    close(fd);
    return size as c_int;
}

unsafe fn scan_rom(path: *mut c_char, file: *mut c_char) -> c_int {
    let mut namelist: *mut *mut dirent = ptr::null_mut();
    let mut name: *mut c_char;
    let mut path2: *mut c_char;
    let mut i: c_int;
    let n: c_int;
    let mut r: c_int;
    let mut rc: c_int = 0;
    let mut result: c_int = 0;
    let mut stat_buf: stat = std::mem::zeroed();

    n = scandir(path, &mut namelist, None, Some(alphasort));
    if n < 0 {
        perror(c"scandir".as_ptr());
        return -1;
    }

    i = 0;
    while i < n {
        name = (*(*namelist.add(i as usize))).d_name.as_mut_ptr();

        if fnmatch(c".".as_ptr(), name, 0) == 0 {
            free(*namelist.add(i as usize) as *mut c_void);
            i += 1;
            continue;
        }
        if fnmatch(c"..".as_ptr(), name, 0) == 0 {
            free(*namelist.add(i as usize) as *mut c_void);
            i += 1;
            continue;
        }

        path2 = malloc(strlen(path) + strlen(name) + 3) as *mut c_char;
        strcpy(path2, path);
        strcat(path2, c"/".as_ptr());
        strcat(path2, name);

        if fnmatch(file, name, 0) == 0 {
            rc = read_rom(path2);

            /*
             * It's OK if the ROM is unreadable.  Maybe there
             * is no ROM, or some other error occurred.  The
             * important thing is that no MCA happened.
             */
            if rc > 0 {
                fprintf(stderr, c"PASS: %s read %d bytes\n".as_ptr(), path2, rc);
            } else {
                fprintf(stderr, c"PASS: %s not readable\n".as_ptr(), path2);
                return rc;
            }
        } else {
            r = lstat(path2, &mut stat_buf);
            if r == 0 && s_isdir(stat_buf.st_mode) {
                rc = scan_rom(path2, file);
                if rc < 0 {
                    return rc;
                }
            }
        }

        result |= rc;
        free(path2 as *mut c_void);

        free(*namelist.add(i as usize) as *mut c_void);
        i += 1;
    }
    free(namelist as *mut c_void);
    return result;
}

fn main() {
    unsafe {
        let rc: c_int;

        if map_mem(c"/dev/mem".as_ptr() as *mut c_char, 0, 0xA0000, 1) == 0 {
            fprintf(stderr, c"PASS: /dev/mem 0x0-0xa0000 is readable\n".as_ptr());
        } else {
            fprintf(stderr, c"FAIL: /dev/mem 0x0-0xa0000 not accessible\n".as_ptr());
        }

        /*
         * It's not safe to blindly read the VGA frame buffer.  If you know
         * how to poke the card the right way, it should respond, but it's
         * not safe in general.  Many machines, e.g., Intel chipsets, cover
         * up a non-responding card by just returning -1, but others will
         * report the failure as a machine check.
         */
        if map_mem(c"/dev/mem".as_ptr() as *mut c_char, 0xA0000, 0x20000, 0) == 0 {
            fprintf(stderr, c"PASS: /dev/mem 0xa0000-0xc0000 is mappable\n".as_ptr());
        } else {
            fprintf(stderr, c"FAIL: /dev/mem 0xa0000-0xc0000 not accessible\n".as_ptr());
        }

        if map_mem(c"/dev/mem".as_ptr() as *mut c_char, 0xC0000, 0x40000, 1) == 0 {
            fprintf(stderr, c"PASS: /dev/mem 0xc0000-0x100000 is readable\n".as_ptr());
        } else {
            fprintf(stderr, c"FAIL: /dev/mem 0xc0000-0x100000 not accessible\n".as_ptr());
        }

        /*
         * Often you can map all the individual pieces above (0-0xA0000,
         * 0xA0000-0xC0000, and 0xC0000-0x100000), but can't map the whole
         * thing at once.  This is because the individual pieces use different
         * attributes, and there's no single attribute supported over the
         * whole region.
         */
        rc = map_mem(c"/dev/mem".as_ptr() as *mut c_char, 0, 1024 * 1024, 0);
        if rc == 0 {
            fprintf(stderr, c"PASS: /dev/mem 0x0-0x100000 is mappable\n".as_ptr());
        } else if rc > 0 {
            fprintf(stderr, c"PASS: /dev/mem 0x0-0x100000 not mappable\n".as_ptr());
        } else {
            fprintf(stderr, c"FAIL: /dev/mem 0x0-0x100000 not accessible\n".as_ptr());
        }

        scan_tree(
            c"/sys/class/pci_bus".as_ptr() as *mut c_char,
            c"legacy_mem".as_ptr() as *mut c_char,
            0,
            0xA0000,
            1,
        );
        scan_tree(
            c"/sys/class/pci_bus".as_ptr() as *mut c_char,
            c"legacy_mem".as_ptr() as *mut c_char,
            0xA0000,
            0x20000,
            0,
        );
        scan_tree(
            c"/sys/class/pci_bus".as_ptr() as *mut c_char,
            c"legacy_mem".as_ptr() as *mut c_char,
            0xC0000,
            0x40000,
            1,
        );
        scan_tree(
            c"/sys/class/pci_bus".as_ptr() as *mut c_char,
            c"legacy_mem".as_ptr() as *mut c_char,
            0,
            1024 * 1024,
            0,
        );

        scan_rom(
            c"/sys/devices".as_ptr() as *mut c_char,
            c"rom".as_ptr() as *mut c_char,
        );

        scan_tree(
            c"/proc/bus/pci".as_ptr() as *mut c_char,
            c"??.?".as_ptr() as *mut c_char,
            0,
            0xA0000,
            1,
        );
        scan_tree(
            c"/proc/bus/pci".as_ptr() as *mut c_char,
            c"??.?".as_ptr() as *mut c_char,
            0xA0000,
            0x20000,
            0,
        );
        scan_tree(
            c"/proc/bus/pci".as_ptr() as *mut c_char,
            c"??.?".as_ptr() as *mut c_char,
            0xC0000,
            0x40000,
            1,
        );
        scan_tree(
            c"/proc/bus/pci".as_ptr() as *mut c_char,
            c"??.?".as_ptr() as *mut c_char,
            0,
            1024 * 1024,
            0,
        );

        std::process::exit(rc);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
