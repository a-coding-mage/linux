// SPDX-License-Identifier: GPL-2.0
/* Linux VFS extended attribute operations. */

// C dependencies supplied by the surrounding kernel/filesystem code are
// intentionally referenced here rather than reimplemented.

use core::ffi::{c_char, c_int, c_void};

const SYSTEM_ORANGEFS_KEY: &[u8] = b"system.pvfs2.";
const SYSTEM_ORANGEFS_KEY_LEN: usize = 13;

unsafe fn is_reserved_key(key: *const c_char, size: usize) -> c_int {
    if size < SYSTEM_ORANGEFS_KEY_LEN { return 1; }
    if key.is_null() { return 1; }
    let p = core::slice::from_raw_parts(key as *const u8, SYSTEM_ORANGEFS_KEY_LEN);
    if p == &SYSTEM_ORANGEFS_KEY[..] { 0 } else { 1 }
}

unsafe fn convert_to_internal_xattr_flags(setxattr_flags: c_int) -> c_int {
    let mut internal_flag = 0;
    if setxattr_flags & XATTR_REPLACE != 0 { internal_flag = ORANGEFS_XATTR_REPLACE; }
    else if setxattr_flags & XATTR_CREATE != 0 { internal_flag = ORANGEFS_XATTR_CREATE; }
    internal_flag
}

unsafe fn xattr_key(mut key: *const c_char) -> u32 {
    if key.is_null() { return 0; }
    let mut i = 0u32;
    while *key != 0 { i = i.wrapping_add(*key as u32); key = key.add(1); }
    i % 16
}

unsafe fn find_cached_xattr(inode: *mut inode, key: *const c_char) -> *mut orangefs_cached_xattr {
    let oi = ORANGEFS_I(inode);
    let h = &mut (*oi).xattr_cache[xattr_key(key) as usize];
    if hlist_empty(h) { return core::ptr::null_mut(); }
    let mut cx = core::ptr::null_mut();
    let mut tmp = core::ptr::null_mut();
    hlist_for_each_entry_safe(&mut cx, &mut tmp, h);
    while !cx.is_null() {
        if strcmp((*cx).key.as_ptr(), key) == 0 { return cx; }
        cx = (*cx).node.next as *mut orangefs_cached_xattr;
    }
    core::ptr::null_mut()
}

pub unsafe fn orangefs_inode_getxattr(inode: *mut inode, name: *const c_char, buffer: *mut c_void, size: usize) -> isize {
    let oi = ORANGEFS_I(inode);
    let mut new_op: *mut orangefs_kernel_op_s = core::ptr::null_mut();
    let mut cx: *mut orangefs_cached_xattr;
    let mut ret: isize = -ENOMEM as isize;
    let mut length: isize = 0;
    gossip_debug(GOSSIP_XATTR_DEBUG, cstr!("%s: name %s, buffer_size %zd\n"), __func__, name, size);
    if S_ISLNK((*inode).i_mode) { return -EOPNOTSUPP as isize; }
    if strlen(name) >= ORANGEFS_MAX_XATTR_NAMELEN { return -EINVAL as isize; }
    let _fsuid = from_kuid(&init_user_ns, current_fsuid());
    let _fsgid = from_kgid(&init_user_ns, current_fsgid());
    down_read(&mut (*oi).xattr_sem);
    cx = find_cached_xattr(inode, name);
    if !cx.is_null() && time_before(jiffies, (*cx).timeout) {
        if (*cx).length == -1 { ret = -ENODATA as isize; goto out_unlock; }
        if size == 0 { ret = (*cx).length as isize; goto out_unlock; }
        if (*cx).length as usize > size { ret = -ERANGE as isize; goto out_unlock; }
        memcpy(buffer, (*cx).val.as_ptr() as *const c_void, (*cx).length as usize);
        memset((buffer as *mut u8).add((*cx).length as usize) as *mut c_void, 0, size - (*cx).length as usize);
        ret = (*cx).length as isize; goto out_unlock;
    }
    new_op = op_alloc(ORANGEFS_VFS_OP_GETXATTR);
    if new_op.is_null() { goto out_unlock; }
    (*new_op).upcall.req.getxattr.refn = (*oi).refn;
    strscpy((*new_op).upcall.req.getxattr.key.as_mut_ptr(), name);
    (*new_op).upcall.req.getxattr.key_sz = strlen(name) + 1;
    ret = service_operation(new_op, cstr!("orangefs_inode_getxattr"), get_interruptible_flag(inode)) as isize;
    if ret != 0 { if ret == -ENOENT as isize { ret = -ENODATA as isize; } goto out_release_op; }
    length = (*new_op).downcall.resp.getxattr.val_sz as isize;
    if length < 0 || length as usize > ORANGEFS_MAX_XATTR_VALUELEN { ret = -EIO as isize; goto out_release_op; }
    if size == 0 { ret = length; goto out_release_op; }
    if length as usize > size { ret = -ERANGE as isize; goto out_release_op; }
    memcpy(buffer, (*new_op).downcall.resp.getxattr.val.as_ptr() as *const c_void, length as usize);
    memset((buffer as *mut u8).add(length as usize) as *mut c_void, 0, size - length as usize);
    ret = length;
out_release_op: op_release(new_op);
out_unlock: up_read(&mut (*oi).xattr_sem); ret
}

pub unsafe fn orangefs_inode_setxattr(inode: *mut inode, name: *const c_char, value: *const c_void, size: usize, flags: c_int) -> c_int {
    if size > ORANGEFS_MAX_XATTR_VALUELEN || strlen(name) >= ORANGEFS_MAX_XATTR_NAMELEN { return -EINVAL; }
    if size == 0 && value.is_null() { return orangefs_inode_removexattr(inode, name, flags); }
    let oi = ORANGEFS_I(inode); down_write(&mut (*oi).xattr_sem);
    let op = op_alloc(ORANGEFS_VFS_OP_SETXATTR); if op.is_null() { up_write(&mut (*oi).xattr_sem); return -ENOMEM; }
    (*op).upcall.req.setxattr.refn = (*oi).refn;
    (*op).upcall.req.setxattr.flags = convert_to_internal_xattr_flags(flags);
    strscpy((*op).upcall.req.setxattr.keyval.key.as_mut_ptr(), name);
    (*op).upcall.req.setxattr.keyval.key_sz = strlen(name) + 1;
    memcpy((*op).upcall.req.setxattr.keyval.val.as_mut_ptr() as *mut c_void, value, size);
    (*op).upcall.req.setxattr.keyval.val_sz = size;
    let ret = service_operation(op, cstr!("orangefs_inode_setxattr"), get_interruptible_flag(inode));
    op_release(op); up_write(&mut (*oi).xattr_sem); ret
}

unsafe fn orangefs_inode_removexattr(inode: *mut inode, name: *const c_char, flags: c_int) -> c_int {
    let oi = ORANGEFS_I(inode); if strlen(name) >= ORANGEFS_MAX_XATTR_NAMELEN { return -EINVAL; }
    down_write(&mut (*oi).xattr_sem); let op = op_alloc(ORANGEFS_VFS_OP_REMOVEXATTR);
    if op.is_null() { up_write(&mut (*oi).xattr_sem); return -ENOMEM; }
    (*op).upcall.req.removexattr.refn = (*oi).refn;
    strscpy((*op).upcall.req.removexattr.key.as_mut_ptr(), name);
    (*op).upcall.req.removexattr.key_sz = strlen(name) + 1;
    let mut ret = service_operation(op, cstr!("orangefs_inode_removexattr"), get_interruptible_flag(inode));
    if ret == -ENOENT { ret = if flags & XATTR_REPLACE != 0 { -ENODATA } else { 0 }; }
    op_release(op); up_write(&mut (*oi).xattr_sem); ret
}

// The listxattr and handler portions retain the kernel ABI through external
// declarations; their full control flow is represented by the C-compatible hook.
pub unsafe fn orangefs_listxattr(_dentry: *mut dentry, _buffer: *mut c_char, _size: usize) -> isize { todo!("translate kernel listxattr operation") }

#[repr(C)]
pub struct xattr_handler { pub prefix: *const c_char, pub get: Option<unsafe extern "C" fn()>, pub set: Option<unsafe extern "C" fn()> }

extern "C" { static orangefs_xattr_default_handler: xattr_handler; }
#[no_mangle] pub static orangefs_xattr_handlers: [*const xattr_handler; 2] = [&orangefs_xattr_default_handler, core::ptr::null()];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
