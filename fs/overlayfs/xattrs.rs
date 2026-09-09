// SPDX-License-Identifier: GPL-2.0-only

// Dependencies supplied by linux/fs.h, linux/xattr.h, and overlayfs.h.

unsafe fn ovl_is_escaped_xattr(sb: *mut super_block, name: *const c_char) -> bool {
    let ofs = (*sb).s_fs_info as *mut ovl_fs;

    if (*ofs).config.userxattr {
        strncmp(name, OVL_XATTR_ESCAPE_USER_PREFIX, OVL_XATTR_ESCAPE_USER_PREFIX_LEN) == 0
    } else {
        strncmp(name, OVL_XATTR_ESCAPE_TRUSTED_PREFIX, OVL_XATTR_ESCAPE_TRUSTED_PREFIX_LEN) == 0
    }
}

unsafe fn ovl_is_own_xattr(sb: *mut super_block, name: *const c_char) -> bool {
    let ofs = OVL_FS(sb);

    if (*ofs).config.userxattr {
        strncmp(name, OVL_XATTR_USER_PREFIX, OVL_XATTR_USER_PREFIX_LEN) == 0
    } else {
        strncmp(name, OVL_XATTR_TRUSTED_PREFIX, OVL_XATTR_TRUSTED_PREFIX_LEN) == 0
    }
}

pub unsafe fn ovl_is_private_xattr(sb: *mut super_block, name: *const c_char) -> bool {
    ovl_is_own_xattr(sb, name) && !ovl_is_escaped_xattr(sb, name)
}

unsafe fn ovl_xattr_set(
    dentry: *mut dentry, inode: *mut inode, name: *const c_char,
    value: *const c_void, size: size_t, flags: c_int,
) -> c_int {
    let mut err: c_int;
    let ofs = OVL_FS((*dentry).d_sb);
    let upperdentry = ovl_i_dentry_upper(inode);
    let mut realdentry = if !upperdentry.is_null() { upperdentry } else { ovl_dentry_lower(dentry) };
    let mut realpath = path::default();

    if value.is_null() && upperdentry.is_null() {
        ovl_path_lower(dentry, &mut realpath);
        err = vfs_getxattr(mnt_idmap(realpath.mnt), realdentry, name, core::ptr::null_mut(), 0);
        if err < 0 { return err; }
    }

    if upperdentry.is_null() {
        err = ovl_copy_up(dentry);
        if err != 0 { return err; }
        realdentry = ovl_dentry_upper(dentry);
    }

    err = ovl_want_write(dentry);
    if err != 0 { return err; }

    if !value.is_null() {
        err = ovl_do_setxattr(ofs, realdentry, name, value, size, flags);
    } else {
        WARN_ON(flags != XATTR_REPLACE);
        err = ovl_do_removexattr(ofs, realdentry, name);
    }
    ovl_drop_write(dentry);

    // copy c/mtime
    ovl_copyattr(inode);
    err
}

unsafe fn ovl_xattr_get(
    dentry: *mut dentry, inode: *mut inode, name: *const c_char,
    value: *mut c_void, size: size_t,
) -> c_int {
    let mut realpath = path::default();
    ovl_i_path_real(inode, &mut realpath);
    // Use vfs_getxattr(), not __vfs_getxattr(): it idmaps the security.capability rootid.
    vfs_getxattr(mnt_idmap(realpath.mnt), realpath.dentry, name, value, size)
}

unsafe fn ovl_can_list(sb: *mut super_block, s: *const c_char) -> bool {
    // Never list private (.overlay)
    if ovl_is_private_xattr(sb, s) { return false; }
    // List all non-trusted xattrs
    if strncmp(s, XATTR_TRUSTED_PREFIX, XATTR_TRUSTED_PREFIX_LEN) != 0 { return true; }
    // list other trusted for superuser only
    ns_capable_noaudit(&init_user_ns, CAP_SYS_ADMIN)
}

pub unsafe fn ovl_listxattr(dentry: *mut dentry, list: *mut c_char, size: size_t) -> ssize_t {
    let realdentry = ovl_dentry_real(dentry);
    let ofs = OVL_FS((*dentry).d_sb);
    let mut res = vfs_listxattr(realdentry, list, size);
    if res <= 0 || size == 0 { return res; }

    let prefix_len = if (*ofs).config.userxattr { OVL_XATTR_USER_PREFIX_LEN } else { OVL_XATTR_TRUSTED_PREFIX_LEN };
    // filter out private xattrs
    let mut s = list;
    let mut len = res as size_t;
    while len != 0 {
        let slen = strnlen(s, len) + 1;
        // underlying fs providing us with an broken xattr list?
        if WARN_ON(slen > len) { return -EIO; }
        len -= slen;
        if !ovl_can_list((*dentry).d_sb, s) {
            res -= slen as ssize_t;
            memmove(s as *mut c_void, s.add(slen) as *const c_void, len);
        } else if ovl_is_escaped_xattr((*dentry).d_sb, s) {
            res -= OVL_XATTR_ESCAPE_PREFIX_LEN as ssize_t;
            let name_len = slen - prefix_len - OVL_XATTR_ESCAPE_PREFIX_LEN;
            s = s.add(prefix_len);
            memmove(s as *mut c_void, s.add(OVL_XATTR_ESCAPE_PREFIX_LEN) as *const c_void, name_len + len);
            s = s.add(name_len);
        } else { s = s.add(slen); }
    }
    res
}

unsafe fn ovl_xattr_escape_name(prefix: *const c_char, name: *const c_char) -> *mut c_char {
    let prefix_len = strlen(prefix);
    let name_len = strlen(name);
    let escaped_len = prefix_len + OVL_XATTR_ESCAPE_PREFIX_LEN + name_len;
    if escaped_len > XATTR_NAME_MAX { return ERR_PTR(-EOPNOTSUPP); }
    let escaped = kmalloc(escaped_len + 1, GFP_KERNEL);
    if escaped.is_null() { return ERR_PTR(-ENOMEM); }
    memcpy(escaped as *mut c_void, prefix as *const c_void, prefix_len);
    memcpy(escaped.add(prefix_len) as *mut c_void, OVL_XATTR_ESCAPE_PREFIX as *const c_void, OVL_XATTR_ESCAPE_PREFIX_LEN);
    memcpy(escaped.add(prefix_len + OVL_XATTR_ESCAPE_PREFIX_LEN) as *mut c_void, name as *const c_void, name_len + 1);
    escaped
}

unsafe fn ovl_own_xattr_get(handler: *const xattr_handler, dentry: *mut dentry, inode: *mut inode, name: *const c_char, buffer: *mut c_void, size: size_t) -> c_int {
    let escaped = ovl_xattr_escape_name((*handler).prefix, name);
    if IS_ERR(escaped) { return PTR_ERR(escaped); }
    let r = ovl_xattr_get(dentry, inode, escaped, buffer, size);
    kfree(escaped as *mut c_void); r
}

unsafe fn ovl_own_xattr_set(handler: *const xattr_handler, _idmap: *mut mnt_idmap, dentry: *mut dentry, inode: *mut inode, name: *const c_char, value: *const c_void, size: size_t, flags: c_int) -> c_int {
    let escaped = ovl_xattr_escape_name((*handler).prefix, name);
    if IS_ERR(escaped) { return PTR_ERR(escaped); }
    let r = ovl_xattr_set(dentry, inode, escaped, value, size, flags);
    kfree(escaped as *mut c_void); r
}

unsafe fn ovl_other_xattr_get(_handler: *const xattr_handler, dentry: *mut dentry, inode: *mut inode, name: *const c_char, buffer: *mut c_void, size: size_t) -> c_int { ovl_xattr_get(dentry, inode, name, buffer, size) }
unsafe fn ovl_other_xattr_set(_handler: *const xattr_handler, _idmap: *mut mnt_idmap, dentry: *mut dentry, inode: *mut inode, name: *const c_char, value: *const c_void, size: size_t, flags: c_int) -> c_int { ovl_xattr_set(dentry, inode, name, value, size, flags) }

static ovl_own_trusted_xattr_handler: xattr_handler = xattr_handler { prefix: OVL_XATTR_TRUSTED_PREFIX, get: ovl_own_xattr_get, set: ovl_own_xattr_set };
static ovl_own_user_xattr_handler: xattr_handler = xattr_handler { prefix: OVL_XATTR_USER_PREFIX, get: ovl_own_xattr_get, set: ovl_own_xattr_set };
static ovl_other_xattr_handler: xattr_handler = xattr_handler { prefix: b"\\0".as_ptr() as *const c_char, get: ovl_other_xattr_get, set: ovl_other_xattr_set };

static ovl_trusted_xattr_handlers: [*const xattr_handler; 3] = [&ovl_own_trusted_xattr_handler, &ovl_other_xattr_handler, core::ptr::null()];
static ovl_user_xattr_handlers: [*const xattr_handler; 3] = [&ovl_own_user_xattr_handler, &ovl_other_xattr_handler, core::ptr::null()];

pub unsafe fn ovl_xattr_handlers(ofs: *mut ovl_fs) -> *const *const xattr_handler {
    if (*ofs).config.userxattr { ovl_user_xattr_handlers.as_ptr() } else { ovl_trusted_xattr_handlers.as_ptr() }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
