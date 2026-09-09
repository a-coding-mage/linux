/*
 * JFFS2 -- Journalling Flash File System, Version 2.
 *
 * Copyright © 2006  NEC Corporation
 *
 * Created by KaiGai Kohei <kaigai@ak.jp.nec.com>
 *
 * For licensing information, see the file 'LICENCE' in this directory.
 */

// Dependencies supplied by the surrounding kernel/JFFS2 translation.

unsafe fn jffs2_acl_size(count: i32) -> usize {
    if count <= 4 {
        core::mem::size_of::<jffs2_acl_header>()
            + (count as usize) * core::mem::size_of::<jffs2_acl_entry_short>()
    } else {
        core::mem::size_of::<jffs2_acl_header>()
            + 4 * core::mem::size_of::<jffs2_acl_entry_short>()
            + ((count - 4) as usize) * core::mem::size_of::<jffs2_acl_entry>()
    }
}

unsafe fn jffs2_acl_count(mut size: usize) -> i32 {
    let s: usize;
    size -= core::mem::size_of::<jffs2_acl_header>();
    if size < 4 * core::mem::size_of::<jffs2_acl_entry_short>() {
        if size % core::mem::size_of::<jffs2_acl_entry_short>() != 0 {
            return -1;
        }
        return (size / core::mem::size_of::<jffs2_acl_entry_short>()) as i32;
    } else {
        s = size - 4 * core::mem::size_of::<jffs2_acl_entry_short>();
        if s % core::mem::size_of::<jffs2_acl_entry>() != 0 {
            return -1;
        }
        return (s / core::mem::size_of::<jffs2_acl_entry>() + 4) as i32;
    }
}

unsafe fn jffs2_acl_from_medium(value: *mut core::ffi::c_void, size: usize) -> *mut posix_acl {
    let mut value = value as *mut u8;
    let end = value.add(size);
    let header = value as *mut jffs2_acl_header;
    let mut entry: *mut jffs2_acl_entry;
    let acl: *mut posix_acl;
    let ver: u32;
    let mut i: i32;
    let count: i32;

    if value.is_null() {
        return core::ptr::null_mut();
    }
    if size < core::mem::size_of::<jffs2_acl_header>() {
        return ERR_PTR(-EINVAL);
    }
    ver = je32_to_cpu((*header).a_version);
    if ver != JFFS2_ACL_VERSION {
        JFFS2_WARNING!("Invalid ACL version. (=%u)\n", ver);
        return ERR_PTR(-EINVAL);
    }

    value = value.add(core::mem::size_of::<jffs2_acl_header>());
    count = jffs2_acl_count(size);
    if count < 0 {
        return ERR_PTR(-EINVAL);
    }
    if count == 0 {
        return core::ptr::null_mut();
    }

    acl = posix_acl_alloc(count, GFP_KERNEL);
    if acl.is_null() {
        return ERR_PTR(-ENOMEM);
    }

    i = 0;
    while i < count {
        entry = value as *mut jffs2_acl_entry;
        if value.add(core::mem::size_of::<jffs2_acl_entry_short>()) > end {
            goto_fail!(acl, -EINVAL);
        }
        (*acl).a_entries.add(i as usize).as_mut().unwrap().e_tag = je16_to_cpu((*entry).e_tag);
        (*acl).a_entries.add(i as usize).as_mut().unwrap().e_perm = je16_to_cpu((*entry).e_perm);
        match (*acl).a_entries.add(i as usize).as_ref().unwrap().e_tag {
            ACL_USER_OBJ | ACL_GROUP_OBJ | ACL_MASK | ACL_OTHER => {
                value = value.add(core::mem::size_of::<jffs2_acl_entry_short>());
            }
            ACL_USER => {
                value = value.add(core::mem::size_of::<jffs2_acl_entry>());
                if value > end { goto_fail!(acl, -EINVAL); }
                (*acl).a_entries.add(i as usize).as_mut().unwrap().e_uid =
                    make_kuid(&init_user_ns, je32_to_cpu((*entry).e_id));
            }
            ACL_GROUP => {
                value = value.add(core::mem::size_of::<jffs2_acl_entry>());
                if value > end { goto_fail!(acl, -EINVAL); }
                (*acl).a_entries.add(i as usize).as_mut().unwrap().e_gid =
                    make_kgid(&init_user_ns, je32_to_cpu((*entry).e_id));
            }
            _ => { goto_fail!(acl, -EINVAL); }
        }
        i += 1;
    }
    if value != end { goto_fail!(acl, -EINVAL); }
    acl
}

unsafe fn jffs2_acl_to_medium(acl: *const posix_acl, size: *mut usize) -> *mut core::ffi::c_void {
    let header: *mut jffs2_acl_header;
    let mut entry: *mut jffs2_acl_entry;
    let mut e: *mut u8;
    let mut i: usize;

    *size = jffs2_acl_size((*acl).a_count);
    header = kmalloc_flex::<jffs2_acl_header>((*acl).a_count, GFP_KERNEL);
    if header.is_null() { return ERR_PTR(-ENOMEM); }
    (*header).a_version = cpu_to_je32(JFFS2_ACL_VERSION);
    e = header.add(1) as *mut u8;
    i = 0;
    while i < (*acl).a_count as usize {
        let acl_e = &*(*acl).a_entries.add(i);
        entry = e as *mut jffs2_acl_entry;
        (*entry).e_tag = cpu_to_je16(acl_e.e_tag);
        (*entry).e_perm = cpu_to_je16(acl_e.e_perm);
        match acl_e.e_tag {
            ACL_USER => { (*entry).e_id = cpu_to_je32(from_kuid(&init_user_ns, acl_e.e_uid)); e = e.add(core::mem::size_of::<jffs2_acl_entry>()); }
            ACL_GROUP => { (*entry).e_id = cpu_to_je32(from_kgid(&init_user_ns, acl_e.e_gid)); e = e.add(core::mem::size_of::<jffs2_acl_entry>()); }
            ACL_USER_OBJ | ACL_GROUP_OBJ | ACL_MASK | ACL_OTHER => { e = e.add(core::mem::size_of::<jffs2_acl_entry_short>()); }
            _ => { kfree(header as *mut core::ffi::c_void); return ERR_PTR(-EINVAL); }
        }
        i += 1;
    }
    header as *mut core::ffi::c_void
}

pub unsafe fn jffs2_get_acl(inode: *mut inode, r#type: i32, rcu: bool) -> *mut posix_acl {
    let mut acl: *mut posix_acl;
    let mut value: *mut i8 = core::ptr::null_mut();
    let mut rc: i32;
    let xprefix: i32;
    if rcu { return ERR_PTR(-ECHILD); }
    xprefix = match r#type { ACL_TYPE_ACCESS => JFFS2_XPREFIX_ACL_ACCESS, ACL_TYPE_DEFAULT => JFFS2_XPREFIX_ACL_DEFAULT, _ => { BUG!(); 0 } };
    rc = do_jffs2_getxattr(inode, xprefix, b"\0".as_ptr() as *const i8, core::ptr::null_mut(), 0);
    if rc > 0 { value = kmalloc(rc as usize, GFP_KERNEL); if value.is_null() { return ERR_PTR(-ENOMEM); } rc = do_jffs2_getxattr(inode, xprefix, b"\0".as_ptr() as *const i8, value, rc as usize); }
    acl = if rc > 0 { jffs2_acl_from_medium(value as *mut core::ffi::c_void, rc as usize) } else if rc == -ENODATA || rc == -ENOSYS { core::ptr::null_mut() } else { ERR_PTR(rc) };
    kfree(value as *mut core::ffi::c_void); acl
}

unsafe fn __jffs2_set_acl(inode: *mut inode, xprefix: i32, acl: *mut posix_acl) -> i32 {
    let mut value: *mut core::ffi::c_void = core::ptr::null_mut(); let mut size = 0usize; let mut rc;
    if !acl.is_null() { value = jffs2_acl_to_medium(acl, &mut size); if IS_ERR(value) { return PTR_ERR(value); } }
    rc = do_jffs2_setxattr(inode, xprefix, b"\0".as_ptr() as *const i8, value, size, 0);
    if value.is_null() && rc == -ENODATA { rc = 0; } kfree(value); rc
}

pub unsafe fn jffs2_set_acl(_idmap: *mut mnt_idmap, dentry: *mut dentry, acl: *mut posix_acl, r#type: i32) -> i32 {
    let inode = d_inode(dentry); let xprefix;
    match r#type { ACL_TYPE_ACCESS => { xprefix = JFFS2_XPREFIX_ACL_ACCESS; if !acl.is_null() { let mut mode; let rc = posix_acl_update_mode(&nop_mnt_idmap, inode, &mut mode, &mut (acl as *mut posix_acl)); if rc != 0 { return rc; } if (*inode).i_mode != mode { let mut attr: iattr = core::mem::zeroed(); attr.ia_valid = ATTR_MODE | ATTR_CTIME; attr.ia_mode = mode; attr.ia_ctime = current_time(inode); let rc = jffs2_do_setattr(inode, &mut attr); if rc < 0 { return rc; } } } }, ACL_TYPE_DEFAULT => { xprefix = JFFS2_XPREFIX_ACL_DEFAULT; if !S_ISDIR((*inode).i_mode) { return if !acl.is_null() { -EACCES } else { 0 }; } }, _ => return -EINVAL }
    let rc = __jffs2_set_acl(inode, xprefix, acl); if rc == 0 { set_cached_acl(inode, r#type, acl); } rc
}

pub unsafe fn jffs2_init_acl_pre(dir_i: *mut inode, inode: *mut inode, i_mode: *mut umode_t) -> i32 {
    let mut default_acl = core::ptr::null_mut(); let mut acl = core::ptr::null_mut(); cache_no_acl(inode);
    let rc = posix_acl_create(dir_i, i_mode, &mut default_acl, &mut acl); if rc != 0 { return rc; }
    if !default_acl.is_null() { set_cached_acl(inode, ACL_TYPE_DEFAULT, default_acl); posix_acl_release(default_acl); }
    if !acl.is_null() { set_cached_acl(inode, ACL_TYPE_ACCESS, acl); posix_acl_release(acl); } 0
}

pub unsafe fn jffs2_init_acl_post(inode: *mut inode) -> i32 {
    let mut rc;
    if !(*inode).i_default_acl.is_null() { rc = __jffs2_set_acl(inode, JFFS2_XPREFIX_ACL_DEFAULT, (*inode).i_default_acl); if rc != 0 { return rc; } }
    if !(*inode).i_acl.is_null() { rc = __jffs2_set_acl(inode, JFFS2_XPREFIX_ACL_ACCESS, (*inode).i_acl); if rc != 0 { return rc; } } 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
