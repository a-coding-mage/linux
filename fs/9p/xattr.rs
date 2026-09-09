// SPDX-License-Identifier: LGPL-2.1
/*
 * Copyright IBM Corporation, 2010
 * Author Aneesh Kumar K.V <aneesh.kumar@linux.vnet.ibm.com>
 */

// Linux kernel headers and local headers from the C translation unit provide
// the declarations referenced below.

pub unsafe fn v9fs_fid_xattr_get(
    fid: *mut p9_fid,
    name: *const ::core::ffi::c_char,
    buffer: *mut ::core::ffi::c_void,
    buffer_size: usize,
) -> isize {
    let mut retval: isize;
    let mut attr_size: u64 = 0;
    let mut attr_fid: *mut p9_fid;
    let kvec = kvec { iov_base: buffer, iov_len: buffer_size };
    let mut to: iov_iter;
    let mut err: ::core::ffi::c_int = 0;

    iov_iter_kvec(&mut to, ITER_DEST, &kvec, 1, buffer_size);

    attr_fid = p9_client_xattrwalk(fid, name, &mut attr_size);
    if IS_ERR(attr_fid) {
        retval = PTR_ERR(attr_fid);
        p9_debug(P9_DEBUG_VFS, c"p9_client_attrwalk failed %zd\\n".as_ptr(), retval);
        return retval;
    }
    if attr_size > buffer_size as u64 {
        if buffer_size != 0 {
            retval = -ERANGE as isize;
        } else if attr_size > SSIZE_MAX as u64 {
            retval = -EOVERFLOW as isize;
        } else {
            retval = attr_size as isize;
        }
    } else {
        iov_iter_truncate(&mut to, attr_size);
        retval = p9_client_read(attr_fid, 0, &mut to, &mut err);
        if err != 0 {
            retval = err as isize;
        }
    }
    p9_fid_put(attr_fid);
    retval
}

/*
 * v9fs_xattr_get()
 *
 * Copy an extended attribute into the buffer provided, or compute the buffer
 * size required. Buffer is NULL to compute the size of the buffer required.
 *
 * Returns a negative error number on failure, or the number of bytes
 * used / required on success.
 */
pub unsafe fn v9fs_xattr_get(
    dentry: *mut dentry,
    name: *const ::core::ffi::c_char,
    buffer: *mut ::core::ffi::c_void,
    buffer_size: usize,
) -> isize {
    let fid: *mut p9_fid;
    p9_debug(P9_DEBUG_VFS, c"name = '%s' value_len = %zu\\n".as_ptr(), name, buffer_size);
    fid = v9fs_fid_lookup(dentry);
    if IS_ERR(fid) { return PTR_ERR(fid); }
    let ret = v9fs_fid_xattr_get(fid, name, buffer, buffer_size);
    p9_fid_put(fid);
    ret
}

/*
 * v9fs_xattr_set()
 *
 * Create, replace or remove an extended attribute for this inode. Buffer
 * is NULL to remove an existing extended attribute, and non-NULL to either
 * replace an existing extended attribute, or create a new extended attribute.
 * The flags XATTR_REPLACE and XATTR_CREATE specify attribute existence.
 *
 * Returns 0, or a negative error number on failure.
 */
pub unsafe fn v9fs_xattr_set(
    dentry: *mut dentry, name: *const ::core::ffi::c_char,
    value: *const ::core::ffi::c_void, value_len: usize, flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let fid = v9fs_fid_lookup(dentry);
    if IS_ERR(fid) { return PTR_ERR(fid) as ::core::ffi::c_int; }
    let ret = v9fs_fid_xattr_set(fid, name, value, value_len, flags);
    p9_fid_put(fid);
    ret
}

pub unsafe fn v9fs_fid_xattr_set(
    mut fid: *mut p9_fid, name: *const ::core::ffi::c_char,
    value: *const ::core::ffi::c_void, value_len: usize, flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let kvec = kvec { iov_base: value as *mut _, iov_len: value_len };
    let mut from: iov_iter;
    let mut retval: ::core::ffi::c_int;
    let mut err: ::core::ffi::c_int;
    iov_iter_kvec(&mut from, ITER_SOURCE, &kvec, 1, value_len);
    p9_debug(P9_DEBUG_VFS, c"name = %s value_len = %zu flags = %d\\n".as_ptr(), name, value_len, flags);
    fid = clone_fid(fid);
    if IS_ERR(fid) { return PTR_ERR(fid) as ::core::ffi::c_int; }
    retval = p9_client_xattrcreate(fid, name, value_len, flags);
    if retval < 0 { p9_debug(P9_DEBUG_VFS, c"p9_client_xattrcreate failed %d\\n".as_ptr(), retval); }
    else { p9_client_write(fid, 0, &mut from, &mut retval); }
    err = p9_fid_put(fid);
    if retval == 0 && err != 0 { retval = err; }
    retval
}

pub unsafe fn v9fs_listxattr(dentry: *mut dentry, buffer: *mut ::core::ffi::c_char, buffer_size: usize) -> isize {
    // Txattrwalk with an empty string lists xattrs instead.
    v9fs_xattr_get(dentry, c"".as_ptr(), buffer as *mut _, buffer_size)
}

unsafe fn v9fs_xattr_handler_get(handler: *const xattr_handler, dentry: *mut dentry, _inode: *mut inode, name: *const ::core::ffi::c_char, buffer: *mut ::core::ffi::c_void, size: usize) -> ::core::ffi::c_int {
    let full_name = xattr_full_name(handler, name);
    v9fs_xattr_get(dentry, full_name, buffer, size) as ::core::ffi::c_int
}

unsafe fn v9fs_xattr_handler_set(handler: *const xattr_handler, _idmap: *mut mnt_idmap, dentry: *mut dentry, _inode: *mut inode, name: *const ::core::ffi::c_char, value: *const ::core::ffi::c_void, size: usize, flags: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let full_name = xattr_full_name(handler, name);
    v9fs_xattr_set(dentry, full_name, value, size, flags)
}

static V9FS_XATTR_USER_HANDLER: xattr_handler = xattr_handler { prefix: XATTR_USER_PREFIX, get: Some(v9fs_xattr_handler_get), set: Some(v9fs_xattr_handler_set) };
static V9FS_XATTR_TRUSTED_HANDLER: xattr_handler = xattr_handler { prefix: XATTR_TRUSTED_PREFIX, get: Some(v9fs_xattr_handler_get), set: Some(v9fs_xattr_handler_set) };

#[cfg(feature = "CONFIG_9P_FS_SECURITY")]
static V9FS_XATTR_SECURITY_HANDLER: xattr_handler = xattr_handler { prefix: XATTR_SECURITY_PREFIX, get: Some(v9fs_xattr_handler_get), set: Some(v9fs_xattr_handler_set) };

pub static V9FS_XATTR_HANDLERS: &[*const xattr_handler] = &[
    &V9FS_XATTR_USER_HANDLER,
    &V9FS_XATTR_TRUSTED_HANDLER,
    #[cfg(feature = "CONFIG_9P_FS_SECURITY")]
    &V9FS_XATTR_SECURITY_HANDLER,
    core::ptr::null(),
];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
