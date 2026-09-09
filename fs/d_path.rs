/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

#[repr(C)]
pub struct prepend_buffer {
    pub buf: *mut c_char,
    pub len: c_int,
}

unsafe fn extract_string(p: *mut prepend_buffer) -> *mut c_char {
    if (*p).len >= 0 { (*p).buf } else { ERR_PTR(-ENAMETOOLONG) }
}

unsafe fn prepend_char(p: *mut prepend_buffer, c: u8) -> bool {
    if (*p).len > 0 {
        (*p).len -= 1;
        (*p).buf = (*p).buf.offset(-1);
        *(*p).buf = c as c_char;
        true
    } else {
        (*p).len = -1;
        false
    }
}

unsafe fn prepend_copy(dst: *mut c_void, src: *const c_void, len: c_int) -> bool {
    if copy_from_kernel_nofault(dst, src, len) != 0 {
        memset(dst, b'x' as c_int, len as usize);
        false
    } else { true }
}

unsafe fn prepend(p: *mut prepend_buffer, mut str_: *const c_char, namelen: c_int) -> bool {
    if (*p).len < 0 { return false; }
    if (*p).len < namelen {
        str_ = str_.offset((namelen - (*p).len) as isize);
        (*p).buf = (*p).buf.offset(-(*p).len as isize);
        prepend_copy((*p).buf as *mut c_void, str_ as *const c_void, (*p).len);
        (*p).len = -1;
        return false;
    }
    (*p).len -= namelen;
    (*p).buf = (*p).buf.offset(-namelen as isize);
    prepend_copy((*p).buf as *mut c_void, str_ as *const c_void, namelen)
}

unsafe fn prepend_name(p: *mut prepend_buffer, name: *const qstr) -> bool {
    let dname = smp_load_acquire(&(*name).name);
    let dlen = READ_ONCE((*name).len);
    prepend(p, dname, dlen as c_int) && prepend_char(p, b'/')
}

unsafe fn __prepend_path(mut dentry: *const dentry, mut mnt: *const mount,
                         root: *const path, p: *mut prepend_buffer) -> c_int {
    while dentry != (*root).dentry || &(*mnt).mnt as *const vfsmount != (*root).mnt {
        let parent = READ_ONCE((*dentry).d_parent);
        if dentry == (*mnt).mnt.mnt_root {
            let m = READ_ONCE((*mnt).mnt_parent);
            if mnt != m { dentry = READ_ONCE((*mnt).mnt_mountpoint); mnt = m; continue; }
            let mnt_ns = READ_ONCE((*mnt).mnt_ns);
            if !IS_ERR_OR_NULL(mnt_ns) && !is_anon_ns(mnt_ns) { return 1; }
            return 2;
        }
        if dentry == parent { return 3; }
        prefetch(parent);
        if !prepend_name(p, &(*dentry).d_name) { break; }
        dentry = parent;
    }
    0
}

unsafe fn prepend_path(path_: *const path, root: *const path, p: *mut prepend_buffer) -> c_int {
    let mut seq: c_uint = 0;
    let mut m_seq: c_uint = 0;
    let mut b: prepend_buffer;
    let mut error: c_int;
    rcu_read_lock();
    'restart_mnt: loop {
        read_seqbegin_or_lock(&mount_lock, &mut m_seq);
        seq = 0;
        rcu_read_lock();
        'restart: loop {
            b = *p;
            read_seqbegin_or_lock(&rename_lock, &mut seq);
            error = __prepend_path((*path_).dentry, real_mount((*path_).mnt), root, &mut b);
            if seq & 1 == 0 { rcu_read_unlock(); }
            if need_seqretry(&rename_lock, seq) { seq = 1; continue 'restart; }
            done_seqretry(&rename_lock, seq);
            break;
        }
        if m_seq & 1 == 0 { rcu_read_unlock(); }
        if need_seqretry(&mount_lock, m_seq) { m_seq = 1; continue 'restart_mnt; }
        done_seqretry(&mount_lock, m_seq);
        break;
    }
    if error == 3 { b = *p; }
    if b.len == (*p).len { prepend_char(&mut b,  b'/'); }
    *p = b;
    error
}

pub unsafe fn __d_path(path_: *const path, root: *const path, buf: *mut c_char, buflen: c_int) -> *mut c_char {
    let mut b = prepend_buffer { buf: buf.offset(buflen as isize), len: buflen };
    prepend_char(&mut b, 0);
    if prepend_path(path_, root, &mut b) > 0 { core::ptr::null_mut() } else { extract_string(&mut b) }
}

pub unsafe fn d_absolute_path(path_: *const path, buf: *mut c_char, buflen: c_int) -> *mut c_char {
    let root: path = core::mem::zeroed();
    let mut b = prepend_buffer { buf: buf.offset(buflen as isize), len: buflen };
    prepend_char(&mut b, 0);
    if prepend_path(path_, &root, &mut b) > 1 { ERR_PTR(-EINVAL) } else { extract_string(&mut b) }
}

unsafe fn get_fs_root_rcu(fs: *mut fs_struct, root: *mut path) {
    let mut seq;
    loop { seq = read_seqbegin(&(*fs).seq); *root = (*fs).root; if !read_seqretry(&(*fs).seq, seq) { break; } }
}

pub unsafe fn d_path(path_: *const path, buf: *mut c_char, buflen: c_int) -> *mut c_char {
    let mut b = prepend_buffer { buf: buf.offset(buflen as isize), len: buflen };
    let mut root: path = core::mem::zeroed();
    if (*path_).dentry.deref().d_op != core::ptr::null() && (*(*path_).dentry).d_op.d_dname != None &&
       (!IS_ROOT((*path_).dentry) || (*path_).dentry != (*(*path_).mnt).mnt_root || failfs_mnt((*path_).mnt)) {
        return ((*(*(*path_).dentry).d_op).d_dname.unwrap())((*path_).dentry, buf, buflen);
    }
    rcu_read_lock();
    get_fs_root_rcu((*current).fs, &mut root);
    if d_unlinked((*path_).dentry) { prepend(&mut b, b" (deleted)\0".as_ptr() as *const c_char, 11); } else { prepend_char(&mut b, 0); }
    prepend_path(path_, &root, &mut b);
    rcu_read_unlock();
    extract_string(&mut b)
}

pub unsafe fn dynamic_dname(buffer: *mut c_char, buflen: c_int, fmt: *const c_char, mut args: ...) -> *mut c_char {
    let sz = vsnprintf(buffer, buflen, fmt, args) + 1;
    if sz > NAME_MAX || sz > buflen { return ERR_PTR(-ENAMETOOLONG); }
    let start = buffer.offset((buflen - sz) as isize);
    memmove(start as *mut c_void, buffer as *const c_void, sz as usize) as *mut c_char
}

pub unsafe fn simple_dname(dentry_: *mut dentry, buffer: *mut c_char, buflen: c_int) -> *mut c_char {
    let mut b = prepend_buffer { buf: buffer.offset(buflen as isize), len: buflen };
    prepend(&mut b, b" (deleted)\0".as_ptr() as *const c_char, 11);
    prepend(&mut b, (*dentry_).d_name.name, (*dentry_).d_name.len as c_int);
    prepend_char(&mut b, b'/');
    extract_string(&mut b)
}

unsafe fn __dentry_path(d: *const dentry, p: *mut prepend_buffer) -> *mut c_char {
    let mut dentry = d;
    let mut b: prepend_buffer;
    let mut seq: c_uint = 0;
    rcu_read_lock();
    'restart: loop {
        dentry = d;
        b = *p;
        read_seqbegin_or_lock(&rename_lock, &mut seq);
        while !IS_ROOT(dentry) {
            let parent = (*dentry).d_parent;
            prefetch(parent);
            if !prepend_name(&mut b, &(*dentry).d_name) { break; }
            dentry = parent;
        }
        if seq & 1 == 0 { rcu_read_unlock(); }
        if need_seqretry(&rename_lock, seq) { seq = 1; continue 'restart; }
        done_seqretry(&rename_lock, seq);
        break;
    }
    if b.len == (*p).len { prepend_char(&mut b, b'/'); }
    extract_string(&mut b)
}

pub unsafe fn dentry_path_raw(dentry_: *const dentry, buf: *mut c_char, buflen: c_int) -> *mut c_char {
    let mut b = prepend_buffer { buf: buf.offset(buflen as isize), len: buflen };
    prepend_char(&mut b, 0);
    __dentry_path(dentry_, &mut b)
}

pub unsafe fn dentry_path(dentry_: *const dentry, buf: *mut c_char, buflen: c_int) -> *mut c_char {
    let mut b = prepend_buffer { buf: buf.offset(buflen as isize), len: buflen };
    if d_unlinked(dentry_) { prepend(&mut b, b"//deleted\0".as_ptr() as *const c_char, 10); }
    else { prepend_char(&mut b, 0); }
    __dentry_path(dentry_, &mut b)
}

unsafe fn get_fs_root_and_pwd_rcu(fs: *mut fs_struct, root: *mut path, pwd: *mut path) {
    let mut seq;
    loop {
        seq = read_seqbegin(&(*fs).seq);
        *root = (*fs).root;
        *pwd = (*fs).pwd;
        if !read_seqretry(&(*fs).seq, seq) { break; }
    }
}

/* The syscall returns the filled length, including the terminating NUL. */
pub unsafe fn sys_getcwd(buf: *mut c_char, size: c_ulong) -> c_int {
    let mut error;
    let mut pwd: path = core::mem::zeroed();
    let mut root: path = core::mem::zeroed();
    let page = __getname();
    if page.is_null() { return -ENOMEM; }
    rcu_read_lock();
    get_fs_root_and_pwd_rcu((*current).fs, &mut root, &mut pwd);
    if d_unlinked(pwd.dentry) {
        rcu_read_unlock(); error = -ENOENT;
    } else {
        let mut b = prepend_buffer { buf: page.offset(PATH_MAX as isize), len: PATH_MAX };
        prepend_char(&mut b, 0);
        if prepend_path(&pwd, &root, &mut b) > 0 { prepend(&mut b, b"(unreachable)\0".as_ptr() as *const c_char, 13); }
        rcu_read_unlock();
        let len = PATH_MAX - b.len;
        if len > PATH_MAX { error = -ENAMETOOLONG; }
        else if len as c_ulong > size { error = -ERANGE; }
        else if copy_to_user(buf, b.buf, len as usize) != 0 { error = -EFAULT; }
        else { error = len; }
    }
    __putname(page);
    error
}

/* External kernel declarations and types are supplied by the surrounding translation unit. */
extern "C" {
    static mount_lock: seqcount_spinlock_t;
    static rename_lock: seqcount_spinlock_t;
    static current: *mut task_struct;
    fn copy_from_kernel_nofault(dst: *mut c_void, src: *const c_void, len: c_int) -> c_int;
    fn memset(dst: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn vsnprintf(buf: *mut c_char, size: c_int, fmt: *const c_char, args: ...) -> c_int;
    fn memmove(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn __getname() -> *mut c_char;
    fn __putname(name: *mut c_char);
    fn copy_to_user(dst: *mut c_char, src: *mut c_char, n: usize) -> c_int;
    fn rcu_read_lock(); fn rcu_read_unlock(); fn prefetch(p: *const c_void);
    fn real_mount(mnt: *const vfsmount) -> *const mount;
    fn failfs_mnt(mnt: *const vfsmount) -> bool;
    fn is_anon_ns(ns: *const mnt_namespace) -> bool;
    fn ERR_PTR(e: c_int) -> *mut c_char;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
