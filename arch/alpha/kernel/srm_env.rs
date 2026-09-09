// SPDX-License-Identifier: GPL-2.0-only
/*
 * srm_env.c - Access to SRM environment
 *             variables through linux' procfs
 *
 * (C) 2001,2002,2006 by Jan-Benedict Glaw <jbglaw@lug-owl.de>
 *
 * This driver is a modified version of Erik Mouw's example proc
 * interface, so: thank you, Erik! He can be reached via email at
 * <J.A.K.Mouw@its.tudelft.nl>. It is based on an idea
 * provided by DEC^WCompaq^WIntel's "Jumpstart" CD. They
 * included a patch like this as well. Thanks for idea!
 */

// Dependencies supplied by the surrounding kernel translation.
use core::ffi::{c_char, c_int, c_void};

const BASE_DIR: &[u8] = b"srm_environment\0"; // Subdir in /proc/
const NAMED_DIR: &[u8] = b"named_variables\0"; // Subdir for known variables
const NUMBERED_DIR: &[u8] = b"numbered_variables\0"; // Subdir for all variables
const VERSION: &[u8] = b"0.0.6\0"; // Module version
const NAME: &[u8] = b"srm_env\0"; // Module name

#[repr(C)]
struct ProcDirEntry {
    _private: [u8; 0],
}
#[repr(C)]
struct SeqFile {
    private: *mut c_void,
}
#[repr(C)]
struct Inode {
    _private: [u8; 0],
}
#[repr(C)]
struct File {
    _private: [u8; 0],
}
#[repr(C)]
struct ProcOps {
    proc_open: Option<unsafe extern "C" fn(*mut Inode, *mut File) -> c_int>,
    proc_read: Option<unsafe extern "C" fn()>,
    proc_lseek: Option<unsafe extern "C" fn()>,
    proc_release: Option<unsafe extern "C" fn()>,
    proc_write: Option<unsafe extern "C" fn(*mut File, *const c_char, usize, *mut i64) -> isize>,
}

#[repr(C)]
struct SrmEnv {
    name: *mut c_char,
    id: usize,
}

extern "C" {
    static mut base_dir: *mut ProcDirEntry;
    static mut named_dir: *mut ProcDirEntry;
    static mut numbered_dir: *mut ProcDirEntry;
    static alpha_using_srm: bool;

    static ENV_AUTO_ACTION: usize;
    static ENV_BOOT_DEV: usize;
    static ENV_BOOTDEF_DEV: usize;
    static ENV_BOOTED_DEV: usize;
    static ENV_BOOT_FILE: usize;
    static ENV_BOOTED_FILE: usize;
    static ENV_BOOT_OSFLAGS: usize;
    static ENV_BOOTED_OSFLAGS: usize;
    static ENV_BOOT_RESET: usize;
    static ENV_DUMP_DEV: usize;
    static ENV_ENABLE_AUDIT: usize;
    static ENV_LICENSE: usize;
    static ENV_CHAR_SET: usize;
    static ENV_LANGUAGE: usize;
    static ENV_TTY_DEV: usize;

    fn callback_getenv(id: usize, page: *mut c_char, size: usize) -> usize;
    fn callback_setenv(id: usize, page: *mut c_char, count: usize) -> usize;
    fn callback_save_env() -> usize;
    fn __get_free_page(flags: usize) -> *mut c_void;
    fn free_page(addr: usize);
    fn seq_write(m: *mut SeqFile, data: *const c_char, size: usize) -> isize;
    fn single_open(file: *mut File, show: unsafe extern "C" fn(*mut SeqFile, *mut c_void) -> c_int, data: *mut c_void) -> c_int;
    fn pde_data(inode: *mut Inode) -> *mut c_void;
    fn file_inode(file: *mut File) -> *mut Inode;
    fn copy_from_user(to: *mut c_char, from: *const c_char, count: usize) -> usize;
    fn proc_mkdir(name: *const c_char, parent: *mut ProcDirEntry) -> *mut ProcDirEntry;
    fn proc_create_data(name: *const c_char, mode: u16, parent: *mut ProcDirEntry, ops: *const ProcOps, data: *mut c_void) -> *mut ProcDirEntry;
    fn remove_proc_subtree(name: *const c_char, parent: *mut ProcDirEntry);
    fn printk(fmt: *const c_char, ...);
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn seq_read();
    fn seq_lseek();
    fn single_release();
}

static mut SRM_NAMED_ENTRIES: [SrmEnv; 16] = [
    SrmEnv { name: b"auto_action\0" as *const u8 as *mut c_char, id: unsafe { ENV_AUTO_ACTION } },
    SrmEnv { name: b"boot_dev\0" as *const u8 as *mut c_char, id: unsafe { ENV_BOOT_DEV } },
    SrmEnv { name: b"bootdef_dev\0" as *const u8 as *mut c_char, id: unsafe { ENV_BOOTDEF_DEV } },
    SrmEnv { name: b"booted_dev\0" as *const u8 as *mut c_char, id: unsafe { ENV_BOOTED_DEV } },
    SrmEnv { name: b"boot_file\0" as *const u8 as *mut c_char, id: unsafe { ENV_BOOT_FILE } },
    SrmEnv { name: b"booted_file\0" as *const u8 as *mut c_char, id: unsafe { ENV_BOOTED_FILE } },
    SrmEnv { name: b"boot_osflags\0" as *const u8 as *mut c_char, id: unsafe { ENV_BOOT_OSFLAGS } },
    SrmEnv { name: b"booted_osflags\0" as *const u8 as *mut c_char, id: unsafe { ENV_BOOTED_OSFLAGS } },
    SrmEnv { name: b"boot_reset\0" as *const u8 as *mut c_char, id: unsafe { ENV_BOOT_RESET } },
    SrmEnv { name: b"dump_dev\0" as *const u8 as *mut c_char, id: unsafe { ENV_DUMP_DEV } },
    SrmEnv { name: b"enable_audit\0" as *const u8 as *mut c_char, id: unsafe { ENV_ENABLE_AUDIT } },
    SrmEnv { name: b"license\0" as *const u8 as *mut c_char, id: unsafe { ENV_LICENSE } },
    SrmEnv { name: b"char_set\0" as *const u8 as *mut c_char, id: unsafe { ENV_CHAR_SET } },
    SrmEnv { name: b"language\0" as *const u8 as *mut c_char, id: unsafe { ENV_LANGUAGE } },
    SrmEnv { name: b"tty_dev\0" as *const u8 as *mut c_char, id: unsafe { ENV_TTY_DEV } },
    SrmEnv { name: core::ptr::null_mut(), id: 0 },
];

unsafe extern "C" fn srm_env_proc_show(m: *mut SeqFile, _v: *mut c_void) -> c_int {
    let mut ret: usize;
    let id = (*m).private as usize;
    let page = __get_free_page(0x20) as *mut c_char;
    if page.is_null() { return -12; }
    ret = callback_getenv(id, page, 4096);
    if (ret >> 61) == 0 { seq_write(m, page, ret); ret = 0; } else { ret = (-14isize) as usize; }
    free_page(page as usize);
    ret as c_int
}

unsafe extern "C" fn srm_env_proc_open(inode: *mut Inode, file: *mut File) -> c_int {
    single_open(file, srm_env_proc_show, pde_data(inode))
}

unsafe extern "C" fn srm_env_proc_write(file: *mut File, buffer: *const c_char, count: usize, _pos: *mut i64) -> isize {
    let id = pde_data(file_inode(file)) as usize;
    let buf = __get_free_page(0x20) as *mut c_char;
    if buf.is_null() { return -12; }
    let mut res: c_int;
    if count >= 4096 { free_page(buf as usize); return -22; }
    if copy_from_user(buf, buffer, count) != 0 { free_page(buf as usize); return -14; }
    *buf.add(count) = 0;
    let ret1 = callback_setenv(id, buf, count);
    if (ret1 >> 61) == 0 {
        let mut ret2;
        loop { ret2 = callback_save_env(); if (ret2 >> 61) != 1 { break; } }
        res = ret1 as c_int;
    } else { res = -14; }
    free_page(buf as usize);
    res as isize
}

static SRM_ENV_PROC_OPS: ProcOps = ProcOps {
    proc_open: Some(srm_env_proc_open), proc_read: Some(seq_read), proc_lseek: Some(seq_lseek),
    proc_release: Some(single_release), proc_write: Some(srm_env_proc_write),
};

unsafe extern "C" fn srm_env_init() -> c_int {
    if !alpha_using_srm { printk(b"srm_env: This Alpha system doesn't know about SRM (or you've booted SRM->MILO->Linux, which gets misdetected)...\n\0".as_ptr() as *const c_char); return -19; }
    base_dir = proc_mkdir(BASE_DIR.as_ptr() as *const c_char, core::ptr::null_mut());
    if base_dir.is_null() { return -12; }
    named_dir = proc_mkdir(NAMED_DIR.as_ptr() as *const c_char, base_dir);
    if named_dir.is_null() { remove_proc_subtree(BASE_DIR.as_ptr() as *const c_char, core::ptr::null_mut()); return -12; }
    numbered_dir = proc_mkdir(NUMBERED_DIR.as_ptr() as *const c_char, base_dir);
    if numbered_dir.is_null() { remove_proc_subtree(BASE_DIR.as_ptr() as *const c_char, core::ptr::null_mut()); return -12; }
    let mut entry = SRM_NAMED_ENTRIES.as_mut_ptr();
    while !(*entry).name.is_null() && (*entry).id != 0 {
        if proc_create_data((*entry).name, 0o644, named_dir, &SRM_ENV_PROC_OPS, (*entry).id as *mut c_void).is_null() { remove_proc_subtree(BASE_DIR.as_ptr() as *const c_char, core::ptr::null_mut()); return -12; }
        entry = entry.add(1);
    }
    let mut var_num = 0usize;
    while var_num <= 255 {
        let mut name = [0i8; 4];
        sprintf(name.as_mut_ptr(), b"%ld\0".as_ptr() as *const c_char, var_num);
        if proc_create_data(name.as_ptr(), 0o644, numbered_dir, &SRM_ENV_PROC_OPS, var_num as *mut c_void).is_null() { remove_proc_subtree(BASE_DIR.as_ptr() as *const c_char, core::ptr::null_mut()); return -12; }
        var_num += 1;
    }
    0
}

unsafe extern "C" fn srm_env_exit() { remove_proc_subtree(BASE_DIR.as_ptr() as *const c_char, core::ptr::null_mut()); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
