// SPDX-License-Identifier: GPL-2.0
/* Translated from security/tomoyo/realpath.c. Kernel declarations are supplied
 * by the surrounding translation unit. */

use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)] pub struct path { pub dentry: *mut dentry }
#[repr(C)] pub struct dentry { pub d_sb: *mut super_block, pub d_op: *mut dentry_operations }
#[repr(C)] pub struct dentry_operations { pub d_dname: Option<unsafe extern "C" fn(*mut dentry, *mut c_char, c_int) -> *mut c_char> }
#[repr(C)] pub struct super_block { pub s_magic: c_uint, pub s_dev: u64, pub s_root: *mut dentry, pub s_type: *mut file_system_type }
#[repr(C)] pub struct file_system_type { pub name: *const c_char, pub fs_flags: c_uint }
#[repr(C)] pub struct inode_operations { pub rename: *const c_void }
#[repr(C)] pub struct inode { pub i_mode: c_uint, pub i_op: *mut inode_operations }
#[repr(C)] pub struct pid_namespace;
#[repr(C)] pub struct task_struct;
type pid_t = i32;
type dev_t = u64;

extern "C" {
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kmalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kfree(p: *mut c_void);
    fn strlen(s: *const c_char) -> usize;
    fn memmove(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn snprintf(dst: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn simple_strtoul(s: *const c_char, end: *mut *mut c_char, base: c_uint) -> c_ulong;
    fn d_absolute_path(path: *const path, buf: *mut c_char, buflen: c_int) -> *mut c_char;
    fn dentry_path_raw(dentry: *mut dentry, buf: *mut c_char, buflen: c_int) -> *mut c_char;
    fn d_backing_inode(dentry: *mut dentry) -> *mut inode;
    fn proc_pid_ns(sb: *mut super_block) -> *mut pid_namespace;
    fn task_tgid_nr_ns(task: *mut task_struct, ns: *mut pid_namespace) -> pid_t;
    fn kern_path(name: *const c_char, flags: c_uint, path: *mut path) -> c_int;
    fn path_put(path: *mut path);
    fn tomoyo_warn_oom(function: *const c_char);
    static mut current: *mut task_struct;
}
type c_ulong = usize;
const GFP_NOFS: c_uint = 0;
const PAGE_SIZE: usize = 4096;
const PROC_SUPER_MAGIC: c_uint = 0x9fa0;
const FS_REQUIRES_DEV: c_uint = 1;

#[inline] unsafe fn is_err(p: *mut c_char) -> bool { (p as isize) < 0 && (p as isize) >= -4095 }
#[inline] unsafe fn err_ptr<T>(e: isize) -> *mut T { e as *mut T }
#[inline] unsafe fn is_dir(mode: c_uint) -> bool { mode & 0o170000 == 0o040000 }
#[inline] unsafe fn major(dev: dev_t) -> c_uint { (dev >> 20) as c_uint }
#[inline] unsafe fn minor(dev: dev_t) -> c_uint { (dev & 0xfffff) as c_uint }

pub unsafe extern "C" fn tomoyo_encode2(str_: *const c_char, str_len: c_int) -> *mut c_char {
    if str_.is_null() { return core::ptr::null_mut(); }
    let mut len = 0usize;
    for i in 0..str_len as usize { let c = *(str_ as *const u8).add(i); len += if c == b'\\' { 2 } else if c > b' ' && c < 127 { 1 } else { 4 }; }
    len += 1;
    let cp = kzalloc(len + 10, GFP_NOFS) as *mut u8;
    if cp.is_null() { return core::ptr::null_mut(); }
    let mut out = cp;
    for i in 0..str_len as usize {
        let c = *(str_ as *const u8).add(i);
        if c == b'\\' { *out = b'\\'; out = out.add(1); *out = b'\\'; }
        else if c > b' ' && c < 127 { *out = c; }
        else { *out = b'\\'; out = out.add(1); *out = (c >> 6) + b'0'; out = out.add(1); *out = ((c >> 3) & 7) + b'0'; out = out.add(1); *out = (c & 7) + b'0'; }
        out = out.add(1);
    }
    cp as *mut c_char
}

pub unsafe extern "C" fn tomoyo_encode(str_: *const c_char) -> *mut c_char {
    if str_.is_null() { core::ptr::null_mut() } else { tomoyo_encode2(str_, strlen(str_) as c_int) }
}

unsafe fn tomoyo_get_absolute_path(p: *const path, buffer: *mut c_char, buflen: c_int) -> *mut c_char {
    let mut pos = err_ptr::<c_char>(-12);
    if buflen >= 256 {
        pos = d_absolute_path(p, buffer, buflen - 1);
        if !is_err(pos) && *pos == b'/' as c_char && *pos.add(1) != 0 {
            let inode = d_backing_inode((*p).dentry);
            if !inode.is_null() && is_dir((*inode).i_mode) { *buffer.add((buflen - 2) as usize) = b'/' as c_char; *buffer.add((buflen - 1) as usize) = 0; }
        }
    }
    pos
}

unsafe fn tomoyo_get_dentry_path(d: *mut dentry, buffer: *mut c_char, buflen: c_int) -> *mut c_char {
    let mut pos = err_ptr::<c_char>(-12);
    if buflen >= 256 {
        pos = dentry_path_raw(d, buffer, buflen - 1);
        if !is_err(pos) && *pos == b'/' as c_char && *pos.add(1) != 0 {
            let inode = d_backing_inode(d);
            if !inode.is_null() && is_dir((*inode).i_mode) { *buffer.add((buflen - 2) as usize) = b'/' as c_char; *buffer.add((buflen - 1) as usize) = 0; }
        }
    }
    pos
}

unsafe fn tomoyo_get_local_path(d: *mut dentry, buffer: *mut c_char, buflen: c_int) -> *mut c_char {
    let sb = (*d).d_sb; let mut pos = tomoyo_get_dentry_path(d, buffer, buflen); if is_err(pos) { return pos; }
    if (*sb).s_magic == PROC_SUPER_MAGIC && *pos == b'/' as c_char {
        let mut ep = core::ptr::null_mut(); let pid = simple_strtoul(pos.add(1), &mut ep, 10) as pid_t;
        if *ep == b'/' as c_char && pid != 0 && pid == task_tgid_nr_ns(current, proc_pid_ns(sb)) { pos = ep.sub(5); if pos < buffer { return err_ptr(-12); } memmove(pos as *mut c_void, b"/self\0".as_ptr() as *const c_void, 5); }
        return prepend_fs_name(sb, pos, buffer);
    }
    if major((*sb).s_dev) == 0 { return prepend_fs_name(sb, pos, buffer); }
    let inode = d_backing_inode((*sb).s_root); if (*inode).i_op.is_null() || (*(*inode).i_op).rename.is_null() { return prepend_fs_name(sb, pos, buffer); }
    let mut name = [0i8; 64]; let dev = (*sb).s_dev; name[63] = 0; snprintf(name.as_mut_ptr(), 63, b"dev(%u,%u):\0".as_ptr() as *const c_char, major(dev), minor(dev)); let n = strlen(name.as_ptr()); pos = pos.sub(n); if pos < buffer { return err_ptr(-12); } memmove(pos as *mut c_void, name.as_ptr() as *const c_void, n); pos
}

unsafe fn prepend_fs_name(sb: *mut super_block, mut pos: *mut c_char, buffer: *mut c_char) -> *mut c_char { let name = (*(*sb).s_type).name; let n = strlen(name); pos = pos.sub(n + 1); if pos < buffer { return err_ptr(-12); } memmove(pos as *mut c_void, name as *const c_void, n); *pos.add(n) = b':' as c_char; pos }

pub unsafe extern "C" fn tomoyo_realpath_from_path(p: *const path) -> *mut c_char {
    let mut buf = core::ptr::null_mut(); let mut name = core::ptr::null_mut(); let mut len = PAGE_SIZE / 2; let d = (*p).dentry; let sb = (*d).d_sb;
    loop { len <<= 1; kfree(buf as *mut c_void); buf = kmalloc(len, GFP_NOFS) as *mut c_char; if buf.is_null() { break; } *buf.add(len - 1) = 0; let pos;
        if !(*d).d_op.is_null() && (*(*d).d_op).d_dname.is_some() { pos = ((*(*d).d_op).d_dname.unwrap())(d, buf, (len - 1) as c_int); }
        else { let inode = d_backing_inode((*sb).s_root); pos = if ((*inode).i_op).is_null() || (*(*inode).i_op).rename.is_null() && (*sb).s_type as usize != 0 && ((*(*sb).s_type).fs_flags & FS_REQUIRES_DEV) == 0 { tomoyo_get_local_path(d, buf, (len - 1) as c_int) } else { let mut x = tomoyo_get_absolute_path(p, buf, (len - 1) as c_int); if x == err_ptr(-22) { x = tomoyo_get_local_path(d, buf, (len - 1) as c_int); } x }; }
        if is_err(pos) { continue; } name = tomoyo_encode(pos); break;
    }
    kfree(buf as *mut c_void); if name.is_null() { tomoyo_warn_oom(b"tomoyo_realpath_from_path\0".as_ptr() as *const c_char); } name
}

pub unsafe extern "C" fn tomoyo_realpath_nofollow(pathname: *const c_char) -> *mut c_char { let mut p = core::mem::MaybeUninit::<path>::uninit(); if !pathname.is_null() && kern_path(pathname, 0, p.as_mut_ptr()) == 0 { let p = p.assume_init(); let b = tomoyo_realpath_from_path(&p); path_put(&p as *const _ as *mut _); b } else { core::ptr::null_mut() } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
