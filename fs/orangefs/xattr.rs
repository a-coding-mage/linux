// SPDX-License-Identifier: GPL-2.0
// OrangeFS extended attributes, translated from the retained xattr.c.
// Configured original C headers own every shared layout and declaration.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
    missing_docs, unsafe_op_in_unsafe_fn, clippy::all, unreachable_pub)]

#[allow(improper_ctypes)]
mod bindings {
    use kernel::ffi;
    include!(concat!(env!("OBJTREE"), "/rust/bindings/orangefs_xattr_generated.rs"));
}

use bindings::*;
use core::ptr::{addr_of, addr_of_mut, null, null_mut};
use kernel::ffi::{c_char, c_int, c_ulong, c_void};

#[inline]
fn neg(error: u32) -> c_int { -(error as c_int) }

unsafe fn is_reserved_key(key: *const c_char, size: usize) -> c_int {
    if size < 13 { return 1; }
    (strncmp(key, c"system.pvfs2.".as_ptr().cast(), 13) != 0) as c_int
}

fn convert_to_internal_xattr_flags(flags: c_int) -> c_int {
    if flags & XATTR_REPLACE as c_int != 0 { ORANGEFS_XATTR_REPLACE as c_int }
    else if flags & XATTR_CREATE as c_int != 0 { ORANGEFS_XATTR_CREATE as c_int }
    else { 0 }
}

unsafe fn xattr_key(mut key: *const c_char) -> usize {
    if key.is_null() { return 0; }
    let mut hash = 0u32;
    while *key != 0 {
        // Promote the target's C char, then convert to unsigned int as C does.
        hash = hash.wrapping_add(*key as c_int as u32);
        key = key.add(1);
    }
    (hash % 16) as usize
}

unsafe fn cache_from_node(node: *mut hlist_node) -> *mut orangefs_cached_xattr {
    node.cast::<u8>().sub(core::mem::offset_of!(orangefs_cached_xattr, node)).cast()
}

unsafe fn cache_bucket(oi: *mut orangefs_inode_s, name: *const c_char) -> *mut hlist_head {
    addr_of_mut!((*oi).xattr_cache).cast::<hlist_head>().add(xattr_key(name))
}

unsafe fn find_cached_xattr(oi: *mut orangefs_inode_s, name: *const c_char)
    -> *mut orangefs_cached_xattr
{
    let mut node = (*cache_bucket(oi, name)).first;
    while !node.is_null() {
        let next = (*node).next;
        let cx = cache_from_node(node);
        if strcmp(addr_of!((*cx).key).cast(), name) == 0 { return cx; }
        node = next;
    }
    null_mut()
}

unsafe fn invalidate_cached_xattr(oi: *mut orangefs_inode_s, name: *const c_char) {
    let mut node = (*cache_bucket(oi, name)).first;
    while !node.is_null() {
        let next = (*node).next;
        let cx = cache_from_node(node);
        if strcmp(addr_of!((*cx).key).cast(), name) == 0 {
            rust_orangefs_xattr_hlist_del(node);
            kfree(cx.cast());
            break;
        }
        node = next;
    }
}

#[no_mangle]
pub unsafe extern "C" fn orangefs_inode_getxattr(
    inode: *mut inode, name: *const c_char, buffer: *mut c_void, size: usize,
) -> isize {
    let oi = rust_orangefs_xattr_inode(inode);
    rust_orangefs_xattr_debug_get_start(name, size);
    if rust_orangefs_xattr_is_symlink(inode) { return neg(EOPNOTSUPP) as isize; }
    if strlen(name) >= ORANGEFS_MAX_XATTR_NAMELEN as usize { return neg(EINVAL) as isize; }
    let fsuid = rust_orangefs_xattr_fsuid();
    let fsgid = rust_orangefs_xattr_fsgid();
    rust_orangefs_xattr_debug_get_ids(inode, name, fsuid, fsgid);
    let sem = addr_of_mut!((*oi).xattr_sem);
    down_read(sem);
    let mut cx = find_cached_xattr(oi, name);
    if !cx.is_null() && rust_orangefs_xattr_time_before(rust_orangefs_xattr_jiffies(), (*cx).timeout) {
        let ret = if (*cx).length == -1 {
            neg(ENODATA) as isize
        } else if size == 0 {
            (*cx).length
        } else if (*cx).length as usize > size {
            neg(ERANGE) as isize
        } else {
            let length = (*cx).length;
            rust_orangefs_xattr_copy(buffer, addr_of!((*cx).val).cast(), length as usize);
            rust_orangefs_xattr_zero(buffer.cast::<u8>().offset(length).cast(), size - length as usize);
            length
        };
        up_read(sem);
        return ret;
    }
    let op = op_alloc(ORANGEFS_VFS_OP_GETXATTR as c_int);
    if op.is_null() {
        up_read(sem);
        return neg(ENOMEM) as isize;
    }
    let ret = (|| -> isize {
        (*op).upcall.req.getxattr.refn = (*oi).refn;
        rust_orangefs_xattr_copy_name(addr_of_mut!((*op).upcall.req.getxattr.key).cast(), name);
        (*op).upcall.req.getxattr.key_sz = strlen(name).wrapping_add(1) as c_int;
        let service_ret = service_operation(op, c"orangefs_inode_getxattr".as_ptr().cast(),
            rust_orangefs_xattr_interruptible(inode));
        if service_ret != 0 {
            if service_ret == neg(ENOENT) {
                rust_orangefs_xattr_debug_get_missing(inode, addr_of!((*op).upcall.req.getxattr.key).cast());
                // The C original allocates a new negative entry even when an
                // expired entry of the same name already exists.
                cx = rust_orangefs_xattr_alloc_cache();
                if !cx.is_null() {
                    rust_orangefs_xattr_copy_name(addr_of_mut!((*cx).key).cast(), name);
                    (*cx).length = -1;
                    let ticks = orangefs_getattr_timeout_msecs
                        .wrapping_mul(RUST_ORANGEFS_XATTR_HZ as c_int) / 1000;
                    (*cx).timeout = rust_orangefs_xattr_jiffies().wrapping_add(ticks as c_ulong);
                    rust_orangefs_xattr_hlist_add_head(addr_of_mut!((*cx).node),
                        cache_bucket(oi, addr_of!((*cx).key).cast()));
                }
                return neg(ENODATA) as isize;
            }
            return service_ret as isize;
        }
        let length = (*op).downcall.resp.getxattr.val_sz as isize;
        if length < 0 || length > ORANGEFS_MAX_XATTR_VALUELEN as isize {
            return neg(EIO) as isize;
        }
        // Probes and short buffers do not populate or refresh the cache.
        if size == 0 { return length; }
        if length as usize > size { return neg(ERANGE) as isize; }
        rust_orangefs_xattr_copy(buffer, addr_of!((*op).downcall.resp.getxattr.val).cast(), length as usize);
        rust_orangefs_xattr_zero(buffer.cast::<u8>().offset(length).cast(), size - length as usize);
        // C logs the service return (zero), before assigning ret = length.
        rust_orangefs_xattr_debug_get_result(inode, addr_of!((*op).upcall.req.getxattr.key).cast(),
            (*op).upcall.req.getxattr.key_sz, service_ret);
        if !cx.is_null() {
            rust_orangefs_xattr_copy_name(addr_of_mut!((*cx).key).cast(), name);
            rust_orangefs_xattr_copy(addr_of_mut!((*cx).val).cast(), buffer, length as usize);
            (*cx).length = length;
            (*cx).timeout = rust_orangefs_xattr_jiffies().wrapping_add(RUST_ORANGEFS_XATTR_HZ as c_ulong);
        } else {
            cx = rust_orangefs_xattr_alloc_cache();
            if !cx.is_null() {
                rust_orangefs_xattr_copy_name(addr_of_mut!((*cx).key).cast(), name);
                rust_orangefs_xattr_copy(addr_of_mut!((*cx).val).cast(), buffer, length as usize);
                (*cx).length = length;
                (*cx).timeout = rust_orangefs_xattr_jiffies().wrapping_add(RUST_ORANGEFS_XATTR_HZ as c_ulong);
                rust_orangefs_xattr_hlist_add_head(addr_of_mut!((*cx).node),
                    cache_bucket(oi, addr_of!((*cx).key).cast()));
            }
        }
        length
    })();
    op_release(op);
    up_read(sem);
    ret
}

unsafe fn orangefs_inode_removexattr(inode: *mut inode, name: *const c_char, flags: c_int) -> c_int {
    let oi = rust_orangefs_xattr_inode(inode);
    if strlen(name) >= ORANGEFS_MAX_XATTR_NAMELEN as usize { return neg(EINVAL); }
    let sem = addr_of_mut!((*oi).xattr_sem);
    down_write(sem);
    let op = op_alloc(ORANGEFS_VFS_OP_REMOVEXATTR as c_int);
    if op.is_null() { up_write(sem); return neg(ENOMEM); }
    (*op).upcall.req.removexattr.refn = (*oi).refn;
    rust_orangefs_xattr_copy_name(addr_of_mut!((*op).upcall.req.removexattr.key).cast(), name);
    (*op).upcall.req.removexattr.key_sz = strlen(name).wrapping_add(1) as c_int;
    rust_orangefs_xattr_debug_remove_key(addr_of!((*op).upcall.req.removexattr.key).cast(),
        (*op).upcall.req.removexattr.key_sz);
    let mut ret = service_operation(op, c"orangefs_inode_removexattr".as_ptr().cast(),
        rust_orangefs_xattr_interruptible(inode));
    if ret == neg(ENOENT) {
        ret = if flags & XATTR_REPLACE as c_int != 0 { neg(ENODATA) } else { 0 };
    }
    rust_orangefs_xattr_debug_remove_result(ret);
    op_release(op);
    // Preserve invalidation even when the serviced operation failed.
    invalidate_cached_xattr(oi, name);
    up_write(sem);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn orangefs_inode_setxattr(
    inode: *mut inode, name: *const c_char, value: *const c_void, size: usize, flags: c_int,
) -> c_int {
    let oi = rust_orangefs_xattr_inode(inode);
    rust_orangefs_xattr_debug_set_start(name, size);
    if size > ORANGEFS_MAX_XATTR_VALUELEN as usize { return neg(EINVAL); }
    if strlen(name) >= ORANGEFS_MAX_XATTR_NAMELEN as usize { return neg(EINVAL); }
    let internal_flag = convert_to_internal_xattr_flags(flags);
    if size == 0 && value.is_null() {
        rust_orangefs_xattr_debug_removing(name);
        return orangefs_inode_removexattr(inode, name, flags);
    }
    rust_orangefs_xattr_debug_set_inode(inode, name);
    let sem = addr_of_mut!((*oi).xattr_sem);
    down_write(sem);
    let op = op_alloc(ORANGEFS_VFS_OP_SETXATTR as c_int);
    if op.is_null() { up_write(sem); return neg(ENOMEM); }
    (*op).upcall.req.setxattr.refn = (*oi).refn;
    (*op).upcall.req.setxattr.flags = internal_flag;
    rust_orangefs_xattr_copy_name(addr_of_mut!((*op).upcall.req.setxattr.keyval.key).cast(), name);
    (*op).upcall.req.setxattr.keyval.key_sz = strlen(name).wrapping_add(1) as c_int;
    rust_orangefs_xattr_copy(addr_of_mut!((*op).upcall.req.setxattr.keyval.val).cast(), value, size);
    (*op).upcall.req.setxattr.keyval.val_sz = size as c_int;
    rust_orangefs_xattr_debug_set_key(addr_of!((*op).upcall.req.setxattr.keyval.key).cast(),
        (*op).upcall.req.setxattr.keyval.key_sz, size);
    let ret = service_operation(op, c"orangefs_inode_setxattr".as_ptr().cast(),
        rust_orangefs_xattr_interruptible(inode));
    rust_orangefs_xattr_debug_set_result(ret);
    op_release(op);
    invalidate_cached_xattr(oi, name);
    up_write(sem);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn orangefs_listxattr(dentry: *mut dentry, buffer: *mut c_char, size: usize) -> isize {
    let inode = rust_orangefs_xattr_d_inode(dentry);
    let oi = rust_orangefs_xattr_inode(inode);
    if size > 0 && buffer.is_null() {
        rust_orangefs_xattr_error_null();
        return neg(EINVAL) as isize;
    }
    let sem = addr_of_mut!((*oi).xattr_sem);
    down_read(sem);
    let op = op_alloc(ORANGEFS_VFS_OP_LISTXATTR as c_int);
    if op.is_null() { up_read(sem); return neg(ENOMEM) as isize; }
    if !buffer.is_null() && size > 0 { rust_orangefs_xattr_zero(buffer.cast(), size); }
    let mut token = ORANGEFS_ITERATE_START as u64;
    let mut total = 0isize;
    let mut count_keys: c_int = 0;
    let ret = 'done: loop {
        let mut key_size: c_int = 0;
        (*op).upcall.req.listxattr.refn = (*oi).refn;
        (*op).upcall.req.listxattr.token = token;
        (*op).upcall.req.listxattr.requested_count =
            if size == 0 { 0 } else { ORANGEFS_MAX_XATTR_LISTLEN as c_int };
        let ret = service_operation(op, c"orangefs_listxattr".as_ptr().cast(),
            rust_orangefs_xattr_interruptible(inode)) as isize;
        if ret != 0 { break ret; }
        if size == 0 {
            // C multiplies signed int before widening and does not validate
            // returned_count on the size-probe branch.
            total = (*op).downcall.resp.listxattr.returned_count
                .wrapping_mul(ORANGEFS_MAX_XATTR_NAMELEN as c_int) as isize;
            break 0;
        }
        let returned_count = (*op).downcall.resp.listxattr.returned_count;
        if returned_count < 0 || returned_count > ORANGEFS_MAX_XATTR_LISTLEN as c_int {
            rust_orangefs_xattr_error_count(returned_count);
            break neg(EIO) as isize;
        }
        for i in 0..returned_count {
            let length = *addr_of!((*op).downcall.resp.listxattr.lengths).cast::<c_int>().add(i as usize);
            if length < 0 || length > ORANGEFS_MAX_XATTR_NAMELEN as c_int {
                // The original diagnostic prints the length, not the index.
                rust_orangefs_xattr_error_length(length);
                break 'done neg(EIO) as isize;
            }
            // The original checks capacity before hiding reserved keys, and
            // returns the partial byte count with success on a short buffer.
            if total.wrapping_add(length as isize) as usize > size { break 'done 0; }
            let key = addr_of!((*op).downcall.resp.listxattr.key).cast::<c_char>().offset(key_size as isize);
            if is_reserved_key(key, length as usize) != 0 {
                rust_orangefs_xattr_debug_list_copy(i, key);
                rust_orangefs_xattr_copy(buffer.offset(total).cast(), key.cast(), length as usize);
                total = total.wrapping_add(length as isize);
                count_keys = count_keys.wrapping_add(1);
            } else {
                rust_orangefs_xattr_debug_list_reserved(i, key);
            }
            key_size = key_size.wrapping_add(length);
        }
        // Reuse the same allocated op across token pages, just as C does.
        token = (*op).downcall.resp.listxattr.token;
        if token == ORANGEFS_ITERATE_END as u64 { break 0; }
    };
    rust_orangefs_xattr_debug_list_result(if ret != 0 { ret as c_int } else { total as c_int },
        size as isize, count_keys);
    op_release(op);
    let ret = if ret == 0 { total } else { ret };
    up_read(sem);
    ret
}

unsafe extern "C" fn orangefs_xattr_set_default(
    _handler: *const xattr_handler, _idmap: *mut mnt_idmap, _unused: *mut dentry,
    inode: *mut inode, name: *const c_char, buffer: *const c_void, size: usize, flags: c_int,
) -> c_int {
    orangefs_inode_setxattr(inode, name, buffer, size, flags)
}

unsafe extern "C" fn orangefs_xattr_get_default(
    _handler: *const xattr_handler, _unused: *mut dentry, inode: *mut inode,
    name: *const c_char, buffer: *mut c_void, size: usize,
) -> c_int {
    orangefs_inode_getxattr(inode, name, buffer, size) as c_int
}

#[repr(transparent)]
struct XattrHandler(xattr_handler);
// Immutable callback descriptor; no interior mutation or Rust-owned objects.
unsafe impl Sync for XattrHandler {}
static orangefs_xattr_default_handler: XattrHandler = XattrHandler(xattr_handler {
    name: null(), prefix: c"".as_ptr().cast(), flags: 0, list: None,
    get: Some(orangefs_xattr_get_default), set: Some(orangefs_xattr_set_default),
});

#[repr(transparent)]
pub struct XattrHandlers([*const xattr_handler; 2]);
// The pointer table and its referent are immutable for the module's lifetime.
unsafe impl Sync for XattrHandlers {}
#[no_mangle]
pub static orangefs_xattr_handlers: XattrHandlers = XattrHandlers([
    addr_of!(orangefs_xattr_default_handler.0), null(),
]);
