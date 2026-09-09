// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2002 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
 */

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_uint, c_void};
use std::ptr;

// Dependencies supplied by the surrounding UML implementation.
extern "C" {
    fn printk(fmt: *const c_char, ...);
    fn strscpy(dst: *mut c_char, src: *const c_char) -> isize;
    fn strlcat(dst: *mut c_char, src: *const c_char, size: usize) -> usize;
    fn os_warn(fmt: *const c_char, ...);
}

const UML_DIR: &[u8] = b"~/.uml/\0";
const UMID_LEN: usize = 64;

static mut UMID: [c_char; UMID_LEN] = [0; UMID_LEN];
static mut UML_DIR_PTR: *mut c_char = UML_DIR.as_ptr() as *mut c_char;
static mut UMID_SETUP: c_int = 0;

unsafe fn c_strlen(s: *const c_char) -> usize {
    CStr::from_ptr(s).to_bytes().len()
}

unsafe fn make_uml_dir() -> c_int {
    let mut dir = [0 as c_char; 512];
    let mut err: c_int;

    if *UML_DIR_PTR == b'~' as c_char {
        let home = std::env::var_os("HOME");
        err = -2; // -ENOENT
        let Some(home) = home else {
            // printk(UM_KERN_ERR "%s: no value in environment for $HOME\n", __func__)
            return err;
        };
        let home = CString::new(home.to_string_lossy().as_bytes()).unwrap();
        strscpy(dir.as_mut_ptr(), home.as_ptr());
        UML_DIR_PTR = UML_DIR_PTR.add(1);
    }
    strlcat(dir.as_mut_ptr(), UML_DIR_PTR, dir.len());
    let len = c_strlen(dir.as_ptr());
    if len > 0 && dir[len - 1] != b'/' as c_char {
        strlcat(dir.as_mut_ptr(), b"/\0".as_ptr() as *const c_char, dir.len());
    }

    err = -12; // -ENOMEM
    let size = c_strlen(dir.as_ptr()) + 1;
    let p = libc::malloc(size) as *mut c_char;
    if p.is_null() {
        return err;
    }
    ptr::copy_nonoverlapping(dir.as_ptr(), p, size);
    UML_DIR_PTR = p;

    if libc::mkdir(UML_DIR_PTR, 0o777) < 0 && *libc::__errno_location() != 17 {
        err = -*libc::__errno_location();
        libc::free(UML_DIR_PTR as *mut c_void);
        UML_DIR_PTR = ptr::null_mut();
        return err;
    }
    0
}

unsafe fn remove_files_and_dir(dir: *mut c_char) -> c_int {
    let directory = libc::opendir(dir);
    if directory.is_null() {
        if *libc::__errno_location() != 2 { return -*libc::__errno_location(); }
        return 0;
    }
    let mut ret = 0;
    loop {
        let ent = libc::readdir(directory);
        if ent.is_null() { break; }
        let name = (*ent).d_name.as_ptr();
        if libc::strcmp(name, b".\0".as_ptr() as *const c_char) == 0 ||
           libc::strcmp(name, b"..\0".as_ptr() as *const c_char) == 0 { continue; }
        let len = c_strlen(dir) + 1 + c_strlen(name) + 1;
        if len > 256 { ret = -7; break; } // -E2BIG
        let mut file = [0 as c_char; 256];
        libc::snprintf(file.as_mut_ptr(), file.len(), b"%s/%s\0".as_ptr() as *const c_char, dir, name);
        if libc::unlink(file.as_ptr()) < 0 && *libc::__errno_location() != 2 {
            ret = -*libc::__errno_location(); break;
        }
    }
    if ret == 0 && libc::rmdir(dir) < 0 && *libc::__errno_location() != 2 {
        ret = -*libc::__errno_location();
    }
    libc::closedir(directory);
    ret
}

unsafe fn is_umdir_used(dir: *mut c_char) -> c_int {
    let filelen = c_strlen(dir) + 6;
    let file = libc::malloc(filelen) as *mut c_char;
    if file.is_null() { return -12; }
    libc::snprintf(file, filelen, b"%s/pid\0".as_ptr() as *const c_char, dir);
    let fd = libc::open(file, libc::O_RDONLY);
    if fd < 0 { libc::free(file as *mut c_void); return 0; }
    let mut pid = [0 as c_char; 10];
    let n = libc::read(fd, pid.as_mut_ptr() as *mut c_void, pid.len());
    if n <= 0 { libc::close(fd); libc::free(file as *mut c_void); return 0; }
    let p = libc::strtoul(pid.as_ptr(), ptr::null_mut(), 0) as libc::pid_t;
    if libc::kill(p, 0) == 0 || *libc::__errno_location() != 3 { libc::close(fd); libc::free(file as *mut c_void); return 1; }
    libc::close(fd);
    libc::free(file as *mut c_void);
    0
}

unsafe fn umdir_take_if_dead(dir: *mut c_char) -> c_int {
    if is_umdir_used(dir) != 0 { return -17; }
    remove_files_and_dir(dir)
}

unsafe fn create_pid_file() {
    let n = c_strlen(UML_DIR_PTR) + UMID_LEN + 6;
    let file = libc::malloc(n) as *mut c_char;
    if file.is_null() { return; }
    if umid_file_name(b"pid\0".as_ptr() as *mut c_char, file, n as c_int) != 0 { libc::free(file as *mut c_void); return; }
    let fd = libc::open(file, libc::O_RDWR | libc::O_CREAT | libc::O_EXCL, 0o644);
    if fd >= 0 {
        let mut pid = [0 as c_char; 16];
        libc::snprintf(pid.as_mut_ptr(), pid.len(), b"%d\n\0".as_ptr() as *const c_char, libc::getpid());
        libc::write(fd, pid.as_ptr() as *const c_void, c_strlen(pid.as_ptr()));
        libc::close(fd);
    }
    libc::free(file as *mut c_void);
}

unsafe fn set_uml_dir(name: *mut c_char, add: *mut c_int) -> c_int {
    *add = 0;
    if *name == 0 { return 0; }
    let len = c_strlen(name);
    if *name.add(len - 1) == b'/' as c_char {
        UML_DIR_PTR = name;
        return 0;
    }
    let p = libc::malloc(len + 2) as *mut c_char;
    if p.is_null() { return 0; }
    libc::sprintf(p, b"%s/\0".as_ptr() as *const c_char, name);
    UML_DIR_PTR = p;
    0
}

#[no_mangle]
pub unsafe extern "C" fn set_umid(name: *mut c_char) -> c_int {
    if c_strlen(name) > UMID_LEN - 1 { return -7; }
    strscpy(UMID.as_mut_ptr(), name);
    0
}

unsafe fn make_umid() -> c_int {
    if UMID_SETUP != 0 { return 0; }
    make_uml_dir();
    if UMID[0] == 0 {
        let mut tmp = [0 as c_char; 256];
        strscpy(tmp.as_mut_ptr(), UML_DIR_PTR);
        strlcat(tmp.as_mut_ptr(), b"XXXXXX\0".as_ptr() as *const c_char, tmp.len());
        let fd = libc::mkstemp(tmp.as_mut_ptr());
        if fd < 0 { return -*libc::__errno_location(); }
        libc::close(fd);
        set_umid(tmp.as_mut_ptr().add(c_strlen(UML_DIR_PTR)));
        if libc::unlink(tmp.as_ptr()) != 0 { return -*libc::__errno_location(); }
    }
    let mut tmp = [0 as c_char; 256];
    libc::snprintf(tmp.as_mut_ptr(), tmp.len(), b"%s%s\0".as_ptr() as *const c_char, UML_DIR_PTR, UMID.as_ptr());
    let mut err = libc::mkdir(tmp.as_ptr(), 0o777);
    if err < 0 {
        err = -*libc::__errno_location();
        if err != -17 { return err; }
        if umdir_take_if_dead(tmp.as_mut_ptr()) < 0 { return err; }
        err = libc::mkdir(tmp.as_ptr(), 0o777);
    }
    if err != 0 { return -*libc::__errno_location(); }
    UMID_SETUP = 1;
    create_pid_file();
    0
}

unsafe fn make_umid_init() -> c_int {
    if make_umid() == 0 { return 0; }
    UMID[0] = 0;
    make_umid();
    0
}

#[no_mangle]
pub unsafe extern "C" fn umid_file_name(name: *mut c_char, buf: *mut c_char, len: c_int) -> c_int {
    let err = make_umid();
    if err != 0 { return err; }
    let n = libc::snprintf(buf, len as usize, b"%s%s/%s\0".as_ptr() as *const c_char, UML_DIR_PTR, UMID.as_ptr(), name);
    if n >= len { return -7; }
    0
}

#[no_mangle]
pub unsafe extern "C" fn get_umid() -> *mut c_char { UMID.as_mut_ptr() }

unsafe fn remove_umid_dir() {
    let n = c_strlen(UML_DIR_PTR) + UMID_LEN + 1;
    let dir = libc::malloc(n) as *mut c_char;
    if dir.is_null() { return; }
    libc::snprintf(dir, n, b"%s%s\0".as_ptr() as *const c_char, UML_DIR_PTR, UMID.as_ptr());
    remove_files_and_dir(dir);
    libc::free(dir as *mut c_void);
}

// __initcall(make_umid_init);
// __uml_setup("uml_dir=", set_uml_dir, "uml_dir=<directory>\n    The location to place the pid and umid files.\n\n");
// __uml_exitcall(remove_umid_dir);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
