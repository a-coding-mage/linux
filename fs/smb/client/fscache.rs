// SPDX-License-Identifier: LGPL-2.1
/*
 *   CIFS filesystem cache interface
 *
 *   Copyright (c) 2010 Novell, Inc.
 *   Author(s): Suresh Jayaraman <sjayaraman@suse.de>
 *
 */

// Dependencies supplied by the corresponding kernel/CIFS headers are intentionally external.

#[repr(C, packed)]
struct cifs_fscache_inode_key {
    uniqueid: __le64,       // server inode number
    createtime: __le64,     // creation time on server
    type_: u8,              // S_IFMT file type
}

unsafe fn cifs_fscache_fill_volume_coherency(
    tcon: *mut cifs_tcon,
    cd: *mut cifs_fscache_volume_coherency_data,
) {
    memset(cd as *mut c_void, 0, core::mem::size_of::<cifs_fscache_volume_coherency_data>());
    (*cd).resource_id = cpu_to_le64((*tcon).resource_id);
    (*cd).vol_create_time = (*tcon).vol_create_time;
    (*cd).vol_serial_number = cpu_to_le32((*tcon).vol_serial_number);
}

unsafe fn cifs_fscache_get_super_cookie(tcon: *mut cifs_tcon) -> c_int {
    let mut cd: cifs_fscache_volume_coherency_data = core::mem::zeroed();
    let server: *mut TCP_Server_Info = (*(*tcon).ses).server;
    let mut vcookie: *mut fscache_volume;
    let sa: *const sockaddr = &(*server).dstaddr as *const _ as *const sockaddr;
    let mut sharename: *mut c_char;
    let mut key: *mut c_char;
    let mut ret: c_int = -ENOMEM;

    if (*tcon).fscache_acquired {
        return 0;
    }

    mutex_lock(&mut (*tcon).fscache_lock);
    if (*tcon).fscache_acquired {
        mutex_unlock(&mut (*tcon).fscache_lock);
        return 0;
    }
    (*tcon).fscache_acquired = true;

    (*tcon).fscache = core::ptr::null_mut();
    match (*sa).sa_family as c_int {
        AF_INET | AF_INET6 => {}
        _ => {
            mutex_unlock(&mut (*tcon).fscache_lock);
            cifs_dbg(VFS, c_str!("Unknown network family '%d'\n"), (*sa).sa_family);
            return -EINVAL;
        }
    }

    key = core::ptr::null_mut();
    sharename = extract_sharename((*tcon).tree_name);
    if IS_ERR(sharename) {
        mutex_unlock(&mut (*tcon).fscache_lock);
        cifs_dbg(FYI, c_str!("%s: couldn't extract sharename\n"), __func__);
        return PTR_ERR(sharename);
    }

    strreplace(sharename, b'/' as c_char, b';' as c_char);

    key = kasprintf(GFP_KERNEL, c_str!("cifs,%pISpc,%s"), sa, sharename);
    if key.is_null() {
        goto_out!(out, ret);
    }

    cifs_fscache_fill_volume_coherency(tcon, &mut cd);
    vcookie = fscache_acquire_volume(
        key,
        core::ptr::null_mut(), // preferred_cache
        &cd as *const _ as *const c_void,
        core::mem::size_of_val(&cd),
    );
    cifs_dbg(FYI, c_str!("%s: (%s/0x%p)"), __func__, key, vcookie);
    if IS_ERR(vcookie) {
        if vcookie != ERR_PTR(-EBUSY) {
            ret = PTR_ERR(vcookie);
            goto_out!(out_2, ret);
        }
        pr_err(c_str!("Cache volume key already in use (%s)\n"), key);
        vcookie = core::ptr::null_mut();
        trace_smb3_tcon_ref((*tcon).debug_id, (*tcon).tc_count,
                            netfs_trace_tcon_ref_see_fscache_collision);
    } else {
        trace_smb3_tcon_ref((*tcon).debug_id, (*tcon).tc_count,
                            netfs_trace_tcon_ref_see_fscache_okay);
    }

    (*tcon).fscache = vcookie;
    ret = 0;
out_2:
    kfree(key as *mut c_void);
out:
    kfree(sharename as *mut c_void);
    mutex_unlock(&mut (*tcon).fscache_lock);
    ret
}

unsafe fn cifs_fscache_release_super_cookie(tcon: *mut cifs_tcon) {
    let mut cd: cifs_fscache_volume_coherency_data = core::mem::zeroed();

    cifs_dbg(FYI, c_str!("%s: (0x%p)\n"), __func__, (*tcon).fscache);
    cifs_fscache_fill_volume_coherency(tcon, &mut cd);
    fscache_relinquish_volume((*tcon).fscache, &cd, false);
    (*tcon).fscache = core::ptr::null_mut();
    trace_smb3_tcon_ref((*tcon).debug_id, (*tcon).tc_count,
                        netfs_trace_tcon_ref_see_fscache_relinq);
}

unsafe fn cifs_fscache_get_inode_cookie(inode: *mut inode) {
    let mut cd: cifs_fscache_inode_coherency_data = core::mem::zeroed();
    let mut key: cifs_fscache_inode_key = core::mem::zeroed();
    let cifsi: *mut cifsInodeInfo = CIFS_I(inode);
    let cifs_sb: *mut cifs_sb_info = CIFS_SB((*inode).i_sb);
    let tcon: *mut cifs_tcon = cifs_sb_master_tcon(cifs_sb);

    key.uniqueid = cpu_to_le64((*cifsi).uniqueid);
    key.createtime = cpu_to_le64((*cifsi).createtime);
    key.type_ = (((*inode).i_mode & S_IFMT) >> 12) as u8;
    cifs_fscache_fill_coherency(&mut (*cifsi).netfs.inode, &mut cd);

    (*cifsi).netfs.cache = fscache_acquire_cookie(
        (*tcon).fscache, 0,
        &key as *const _ as *const c_void, core::mem::size_of_val(&key),
        &cd as *const _ as *const c_void, core::mem::size_of_val(&cd),
        i_size_read(&(*cifsi).netfs.inode),
    );
    if !(*cifsi).netfs.cache.is_null() {
        mapping_set_release_always((*inode).i_mapping);
    }
}

unsafe fn cifs_fscache_unuse_inode_cookie(inode: *mut inode, update: bool) {
    if update {
        let mut cd: cifs_fscache_inode_coherency_data = core::mem::zeroed();
        let mut i_size: loff_t = i_size_read(inode);
        cifs_fscache_fill_coherency(inode, &mut cd);
        fscache_unuse_cookie(cifs_inode_cookie(inode), &cd, &mut i_size);
    } else {
        fscache_unuse_cookie(cifs_inode_cookie(inode), core::ptr::null(), core::ptr::null_mut());
    }
}

unsafe fn cifs_fscache_release_inode_cookie(inode: *mut inode) {
    let cifsi: *mut cifsInodeInfo = CIFS_I(inode);
    let cookie: *mut fscache_cookie = cifs_inode_cookie(inode);

    if !cookie.is_null() {
        cifs_dbg(FYI, c_str!("%s: (0x%p)\n"), __func__, cookie);
        fscache_relinquish_cookie(cookie, false);
        (*cifsi).netfs.cache = core::ptr::null_mut();
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
