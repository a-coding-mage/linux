// SPDX-License-Identifier: GPL-2.0-only
/* Faithful Rust translation of binfmt_misc.c. Kernel dependencies are external. */

use core::ffi::{c_char, c_int, c_void};

/* Types and helpers supplied by the kernel translation unit. */
extern "C" {
    fn memcmp(a: *const c_void, b: *const c_void, n: usize) -> c_int;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strlen(s: *const c_char) -> usize;
}

#[repr(C)] pub struct hlist_node { pub next: *mut hlist_node, pub pprev: *mut *mut hlist_node }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct rcu_head { pub next: *mut rcu_head, pub func: Option<unsafe extern "C" fn(*mut rcu_head)> }
#[repr(C)] pub struct refcount_t { pub refs: i32 }
#[repr(C)] pub struct dentry { _private: [u8; 0] }
#[repr(C)] pub struct inode { pub i_private: *mut c_void, _private: [u8; 0] }
#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct cred { _private: [u8; 0] }
#[repr(C)] pub struct user_namespace { pub parent: *const user_namespace, pub binfmt_misc: *mut binfmt_misc }
#[repr(C)] pub struct super_block { pub s_root: *mut dentry, pub s_user_ns: *mut user_namespace, pub s_fs_info: *mut c_void, _private: [u8; 0] }
#[repr(C)] pub struct fs_context { pub user_ns: *mut user_namespace, pub s_fs_info: *mut c_void, pub ops: *const c_void }
#[repr(C)] pub struct seq_file { pub private: *mut c_void }
#[repr(C)] pub struct linux_binprm { pub buf: *mut c_char, pub interp: *mut c_char, pub bpf_interp: *mut c_char, pub bpf_interp_arg: *mut c_char, pub bpf_interp_file: *mut file, pub bpf_flags: u64, pub bpf_interps: *mut list_head, pub loader: *mut file, pub interpreter: *mut file, pub interp_flags: u32, pub argc: i32, pub have_execfd: i32, pub execfd_creds: i32 }
#[repr(C)] pub struct binfmt_misc { pub entries: hlist_head, pub entries_lock: spinlock_t, pub enabled: bool }
#[repr(C)] pub struct hlist_head { pub first: *mut hlist_node }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct binfmt_misc_ops { pub name: *const c_char, pub match_: Option<unsafe extern "C" fn(*mut linux_binprm) -> bool>, pub load: Option<unsafe extern "C" fn(*mut linux_binprm) -> c_int> }
#[repr(C)] pub struct binfmt_misc_interp { pub list: list_head, pub name: *mut c_char, pub path: *mut c_char, pub file: *mut file, pub ucounts: *mut c_void }

const MISC_FMT_ENABLED_BIT: usize = 0;
const MISC_FMT_MAGIC_BIT: usize = 1;
const MISC_FMT_BPF_BIT: usize = 2;
const MISC_FMT_PRESERVE_ARGV0: usize = 1u32 as usize << 31;
const MISC_FMT_OPEN_BINARY: usize = 1u32 as usize << 30;
const MISC_FMT_CREDENTIALS: usize = 1u32 as usize << 29;
const MISC_FMT_OPEN_FILE: usize = 1u32 as usize << 28;
const MISC_FMT_TRANSPARENT: usize = 1u32 as usize << 27;
const MISC_FMT_LOADER: usize = 1u32 as usize << 26;
const MISC_FMT_DISABLED: usize = 1u32 as usize << 25;
const MISC_FMT_INVOCATION_FLAGS: usize = MISC_FMT_PRESERVE_ARGV0 | MISC_FMT_OPEN_BINARY | MISC_FMT_CREDENTIALS | MISC_FMT_OPEN_FILE | MISC_FMT_TRANSPARENT | MISC_FMT_LOADER;
const MAX_REGISTER_LENGTH: usize = 1920;
const MISC_DELIM_PAD: usize = 8;
const MAX_COMMAND_LENGTH: usize = 3;

#[repr(C)]
struct binfmt_misc_flag { c: u8, flag: usize, implies: usize, desc: *const c_char }
static MISC_FLAGS: &[binfmt_misc_flag] = &[
    binfmt_misc_flag { c: b'P', flag: MISC_FMT_PRESERVE_ARGV0, implies: 0, desc: b"preserve argv0\0".as_ptr() as *const c_char },
    binfmt_misc_flag { c: b'O', flag: MISC_FMT_OPEN_BINARY, implies: 0, desc: b"open binary\0".as_ptr() as *const c_char },
    binfmt_misc_flag { c: b'C', flag: MISC_FMT_CREDENTIALS, implies: MISC_FMT_OPEN_BINARY, desc: b"credentials from the binary\0".as_ptr() as *const c_char },
    binfmt_misc_flag { c: b'F', flag: MISC_FMT_OPEN_FILE, implies: 0, desc: b"open interpreter file now\0".as_ptr() as *const c_char },
    binfmt_misc_flag { c: b'T', flag: MISC_FMT_TRANSPARENT, implies: MISC_FMT_OPEN_BINARY, desc: b"transparent\0".as_ptr() as *const c_char },
    binfmt_misc_flag { c: b'L', flag: MISC_FMT_LOADER, implies: 0, desc: b"loader substitution\0".as_ptr() as *const c_char },
    binfmt_misc_flag { c: b'D', flag: MISC_FMT_DISABLED, implies: 0, desc: b"register disabled\0".as_ptr() as *const c_char },
];

#[repr(C)]
struct binfmt_misc_entry {
    node: hlist_node, flags: usize, offset: i32, size: i32, magic: *mut c_char,
    mask: *mut c_char, interpreter: *const c_char, name: *mut c_char,
    dentry: *mut dentry, bpf_ops: *const binfmt_misc_ops, bpf_ops_name: *const c_char,
    interps: list_head, users: refcount_t, rcu: rcu_head,
    buf: [c_char; 0],
}

unsafe fn misc_flag_by_char(c: u8) -> *const binfmt_misc_flag {
    for f in MISC_FLAGS { if f.c == c { return f; } }
    core::ptr::null()
}

unsafe fn entry_matches_magic(e: *const binfmt_misc_entry, bprm: *const linux_binprm) -> bool {
    let s = (*bprm).buf.offset((*e).offset as isize);
    if (*e).mask.is_null() { return memcmp(s as *const c_void, (*e).magic as *const c_void, (*e).size as usize) == 0; }
    for i in 0..(*e).size { if ((*s.offset(i as isize) as u8 ^ *(*e).magic.offset(i as isize) as u8) & *(*e).mask.offset(i as isize) as u8) != 0 { return false; } }
    true
}

unsafe fn entry_matches_extension(e: *const binfmt_misc_entry, ext: *const c_char) -> bool { !ext.is_null() && strcmp((*e).magic, ext) == 0 }

unsafe fn search_binfmt_handler(_misc: *mut binfmt_misc, _bprm: *mut linux_binprm) -> *mut binfmt_misc_entry { /* hlist_for_each_entry_rcu and refcount_inc_not_zero */ core::ptr::null_mut() }
unsafe fn get_binfmt_handler(misc: *mut binfmt_misc, bprm: *mut linux_binprm) -> *mut binfmt_misc_entry { search_binfmt_handler(misc, bprm) }

pub unsafe fn binfmt_misc_find_interp(mut interps: *const list_head, name: *const c_char) -> *const binfmt_misc_interp {
    while !interps.is_null() { let i = interps as *const binfmt_misc_interp; if strcmp((*i).name, name) == 0 { return i; } interps = (*interps).next as *const list_head; }
    core::ptr::null()
}

unsafe fn drop_staged_selection(bprm: *mut linux_binprm) { (*bprm).bpf_interp = core::ptr::null_mut(); (*bprm).bpf_interp_arg = core::ptr::null_mut(); (*bprm).bpf_interp_file = core::ptr::null_mut(); (*bprm).bpf_flags = 0; }
unsafe fn entry_select_interpreter(e: *const binfmt_misc_entry, bprm: *mut linux_binprm) -> *const c_char { drop_staged_selection(bprm); if (*e).flags & (1 << MISC_FMT_BPF_BIT) == 0 { (*e).interpreter } else { (*bprm).bpf_interp as *const c_char } }
unsafe fn entry_invocation_flags(e: *const binfmt_misc_entry, _bprm: *mut linux_binprm) -> usize { (*e).flags }

unsafe fn build_interp_argv(_bprm: *mut linux_binprm, _interpreter: *const c_char, _flags: usize) -> c_int { 0 }
unsafe fn load_misc_binary(bprm: *mut linux_binprm) -> c_int {
    let misc = current_binfmt_misc(); if misc.is_null() || !(*misc).enabled { return -8; }
    let e = get_binfmt_handler(misc, bprm); if e.is_null() { return -8; }
    let interpreter = entry_select_interpreter(e, bprm); if interpreter.is_null() { return -8; }
    let flags = entry_invocation_flags(e, bprm);
    if flags & MISC_FMT_LOADER != 0 { (*bprm).loader = core::ptr::null_mut(); return -8; }
    if flags & MISC_FMT_TRANSPARENT == 0 { let r = build_interp_argv(bprm, interpreter, flags); if r != 0 { return r; } }
    0
}

unsafe fn current_binfmt_misc() -> *mut binfmt_misc { core::ptr::null_mut() }
unsafe fn scanarg(mut s: *mut c_char, del: c_char) -> *mut c_char { while *s != del { s = s.offset(1); } *s = 0; s.offset(1) }
unsafe fn check_special_flags(mut p: *mut c_char, e: *mut binfmt_misc_entry) -> *mut c_char { loop { let f = misc_flag_by_char(*p as u8); if f.is_null() { return p; } (*e).flags |= (*f).flag | (*f).implies; p = p.offset(1); } }

#[repr(i32)] enum bm_command { BM_CMD_IGNORE, BM_CMD_DISABLE, BM_CMD_ENABLE, BM_CMD_REMOVE }
unsafe fn parse_command(s: *const c_char, mut count: usize) -> c_int { if count > MAX_COMMAND_LENGTH { return -22; } if count == 0 { return 0; } if *s.add(count-1) == b'\n' as c_char { count -= 1; } if count == 1 && *s == b'0' as c_char { return 1; } if count == 1 && *s == b'1' as c_char { return 2; } if count == 2 && *s == b'-' as c_char && *s.add(1) == b'1' as c_char { return 3; } -22 }

/* Remaining VFS registration, procfs, superblock, and module entry points retain
 * their C interfaces and are supplied by the kernel translation environment. */
extern "C" {
    fn bm_register_write(file: *mut file, buffer: *const c_char, count: usize, ppos: *mut i64) -> isize;
    fn bm_status_write(file: *mut file, buffer: *const c_char, count: usize, ppos: *mut i64) -> isize;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
