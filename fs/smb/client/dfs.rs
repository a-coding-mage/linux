// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2022 Paulo Alcantara <palcantara@suse.de>
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// left external.

macro_rules! DFS_DOM {
    ($ctx:expr) => {
        if !$ctx.dfs_root_ses.is_null() {
            (*$ctx.dfs_root_ses).dns_dom
        } else {
            core::ptr::null_mut()
        }
    };
}

/**
 * dfs_parse_target_referral - set fs context for dfs target referral
 *
 * @full_path: full path in UNC format.
 * @ref: dfs referral pointer.
 * @ctx: smb3 fs context pointer.
 *
 * Return zero if dfs referral was parsed correctly, otherwise non-zero.
 */
pub unsafe fn dfs_parse_target_referral(
    full_path: *const core::ffi::c_char,
    ref_: *const dfs_info3_param,
    ctx: *mut smb3_fs_context,
) -> i32 {
    let mut rc: i32;
    let mut prepath: *const core::ffi::c_char = core::ptr::null();
    let path: *mut core::ffi::c_char;

    if full_path.is_null() || *full_path == 0 || ref_.is_null() || ctx.is_null() {
        return -EINVAL;
    }

    if WARN_ON_ONCE((*ref_).node_name.is_null() || (*ref_).path_consumed < 0) {
        return -EINVAL;
    }

    if strlen(full_path) - (*ref_).path_consumed as usize != 0 {
        prepath = full_path.add((*ref_).path_consumed as usize);
        if *prepath == b'/' as i8 || *prepath == b'\\' as i8 {
            prepath = prepath.add(1);
        }
    }

    path = cifs_build_devname((*ref_).node_name, prepath);
    if IS_ERR(path) {
        return PTR_ERR(path);
    }

    rc = smb3_parse_devname(path, ctx);
    if rc != 0 {
        kfree(path as *mut core::ffi::c_void);
        return rc;
    }

    rc = dns_resolve_unc(DFS_DOM!((*ctx)), path, &mut (*ctx).dstaddr as *mut _ as *mut sockaddr);
    kfree(path as *mut core::ffi::c_void);
    rc
}

unsafe fn get_session(mnt_ctx: *mut cifs_mount_ctx, full_path: *const core::ffi::c_char) -> i32 {
    let ctx = (*mnt_ctx).fs_ctx;
    (*ctx).leaf_fullpath = full_path as *mut core::ffi::c_char;
    (*ctx).dns_dom = DFS_DOM!((*ctx));
    let rc = cifs_mount_get_session(mnt_ctx);
    (*ctx).leaf_fullpath = core::ptr::null_mut();
    (*ctx).dns_dom = core::ptr::null_mut();
    rc
}

/* Get an active reference of @ses so that next call to cifs_put_tcon() won't
 * release it as any new DFS referrals must go through its IPC tcon. */
unsafe fn set_root_smb_session(mnt_ctx: *mut cifs_mount_ctx) {
    let ctx = (*mnt_ctx).fs_ctx;
    let ses = (*mnt_ctx).ses;
    if !ses.is_null() {
        spin_lock(&mut cifs_tcp_ses_lock);
        cifs_smb_ses_inc_refcount(ses);
        spin_unlock(&mut cifs_tcp_ses_lock);
    }
    (*ctx).dfs_root_ses = ses;
}

unsafe fn parse_dfs_target(ctx: *mut smb3_fs_context, rw: *mut dfs_ref_walk, tgt: *mut dfs_info3_param) -> i32 {
    let fpath = ref_walk_fpath(rw).add(1);
    let mut rc = ref_walk_get_tgt(rw, tgt);
    if rc == 0 {
        rc = dfs_parse_target_referral(fpath, tgt, ctx);
    }
    rc
}

unsafe fn setup_dfs_ref(tgt: *mut dfs_info3_param, rw: *mut dfs_ref_walk) -> i32 {
    let cifs_sb = (*(*rw).mnt_ctx).cifs_sb;
    let ctx = (*(*rw).mnt_ctx).fs_ctx;
    let full_path = smb3_fs_context_fullpath(ctx, CIFS_DIR_SEP(cifs_sb));
    if IS_ERR(full_path) { return PTR_ERR(full_path); }
    let ref_path = if tgt.is_null() || ((*tgt).server_type == DFS_TYPE_LINK && DFS_INTERLINK!((*tgt).flags)) {
        dfs_get_path(cifs_sb, (*ctx).UNC)
    } else { dfs_get_path(cifs_sb, full_path) };
    if IS_ERR(ref_path) {
        let rc = PTR_ERR(ref_path);
        kfree(full_path as *mut core::ffi::c_void);
        return rc;
    }
    ref_walk_path(rw) = ref_path;
    ref_walk_fpath(rw) = full_path;
    dfs_get_referral((*rw).mnt_ctx, ref_walk_path(rw).add(1), ref_walk_tl(rw))
}

unsafe fn __dfs_referral_walk(rw: *mut dfs_ref_walk) -> i32 {
    let ctx = (*(*rw).mnt_ctx).fs_ctx;
    let mnt_ctx = (*rw).mnt_ctx;
    let mut tgt: dfs_info3_param = core::mem::zeroed();
    let mut rc = -ENOENT;
    'again: loop {
        (*ctx).dfs_root_ses = ref_walk_ses(rw);
        while ref_walk_next_tgt(rw) {
            rc = parse_dfs_target(ctx, rw, &mut tgt);
            if rc != 0 { continue; }
            cifs_mount_put_conns(mnt_ctx);
            rc = get_session(mnt_ctx, ref_walk_path(rw));
            if rc != 0 { continue; }
            rc = cifs_mount_get_tcon(mnt_ctx);
            if rc != 0 {
                if tgt.server_type == DFS_TYPE_LINK && DFS_INTERLINK!(tgt.flags) { rc = -EREMOTE; }
            } else {
                rc = cifs_is_path_remote(mnt_ctx);
                if rc == 0 { ref_walk_set_tgt_hint(rw); break; }
            }
            if rc == -EREMOTE {
                rc = ref_walk_advance(rw);
                if rc == 0 {
                    rc = setup_dfs_ref(&mut tgt, rw);
                    if rc != 0 { break; }
                    ref_walk_mark_end(rw);
                    continue 'again;
                }
            }
        }
        if !(rc != 0 && ref_walk_descend(rw)) { break; }
    }
    free_dfs_info_param(&mut tgt);
    rc
}

unsafe fn dfs_referral_walk(mnt_ctx: *mut cifs_mount_ctx, rw: *mut *mut dfs_ref_walk) -> i32 {
    *rw = ref_walk_alloc();
    if IS_ERR(*rw) { let rc = PTR_ERR(*rw); *rw = core::ptr::null_mut(); return rc; }
    ref_walk_init(*rw, mnt_ctx);
    let mut rc = setup_dfs_ref(core::ptr::null_mut(), *rw);
    if rc == 0 { rc = __dfs_referral_walk(*rw); }
    rc
}

unsafe fn __dfs_mount_share(mnt_ctx: *mut cifs_mount_ctx) -> i32 {
    let cifs_sb = (*mnt_ctx).cifs_sb;
    let ctx = (*mnt_ctx).fs_ctx;
    let mut rw: *mut dfs_ref_walk = core::ptr::null_mut();
    let origin_fullpath = dfs_get_path(cifs_sb, (*ctx).source);
    if IS_ERR(origin_fullpath) { return PTR_ERR(origin_fullpath); }
    let mut rc = dfs_referral_walk(mnt_ctx, &mut rw);
    if rc == 0 {
        if WARN_ON((*mnt_ctx).server.is_null()) { rc = -EHOSTDOWN; }
        else if WARN_ON((*mnt_ctx).ses.is_null()) { rc = -EACCES; }
        else if WARN_ON((*mnt_ctx).tcon.is_null()) { rc = -ENOENT; }
    }
    if rc == 0 {
        let tcon = (*mnt_ctx).tcon;
        spin_lock(&mut (*tcon).tc_lock);
        (*tcon).origin_fullpath = origin_fullpath;
        ref_walk_set_tcon(rw, tcon);
        spin_unlock(&mut (*tcon).tc_lock);
        queue_delayed_work(dfscache_wq, &mut (*tcon).dfs_cache_work, dfs_cache_get_ttl() * HZ);
    }
    ref_walk_free(rw);
    rc
}

unsafe fn update_fs_context_dstaddr(ctx: *mut smb3_fs_context) -> i32 {
    let addr = &mut (*ctx).dstaddr as *mut _ as *mut sockaddr;
    let mut rc = 0;
    if !(*ctx).nodfs && (*ctx).dfs_automount {
        rc = dns_resolve_unc(core::ptr::null_mut(), (*ctx).source, addr);
        if rc == 0 { cifs_set_port(addr, (*ctx).port); }
        (*ctx).dfs_automount = false;
    }
    rc
}

pub unsafe fn dfs_mount_share(mnt_ctx: *mut cifs_mount_ctx) -> i32 {
    let ctx = (*mnt_ctx).fs_ctx;
    let nodfs = (*ctx).nodfs;
    let mut rc = update_fs_context_dstaddr(ctx);
    if rc != 0 { return rc; }
    rc = get_session(mnt_ctx, core::ptr::null());
    if rc != 0 { return rc; }
    let mut nodfs = nodfs;
    if !nodfs {
        rc = dfs_get_referral(mnt_ctx, (*ctx).UNC.add(1), core::ptr::null_mut());
        if rc != 0 { cifs_dbg(FYI, b"%s: no dfs referral for %s: %d\n\0".as_ptr() as _, __func__, (*ctx).UNC.add(1), rc); nodfs = true; }
    }
    if nodfs {
        rc = cifs_mount_get_tcon(mnt_ctx);
        if rc == 0 { rc = cifs_is_path_remote(mnt_ctx); }
        return rc;
    }
    if !(*ctx).dfs_conn { (*ctx).dfs_conn = true; cifs_mount_put_conns(mnt_ctx); rc = get_session(mnt_ctx, core::ptr::null()); }
    if rc == 0 { rc = __dfs_mount_share(mnt_ctx); }
    rc
}

unsafe fn target_share_matches_server(server: *mut TCP_Server_Info, share: *mut core::ffi::c_char, target_match: *mut bool) -> i32 {
    let mut rc = 0;
    let mut dfs_host: *const core::ffi::c_char = core::ptr::null();
    let mut dfs_host_len = 0usize;
    *target_match = true;
    extract_unc_hostname(share, &mut dfs_host, &mut dfs_host_len);
    cifs_server_lock(server);
    if dfs_host_len != strlen((*server).hostname) || strncasecmp(dfs_host, (*server).hostname, dfs_host_len) != 0 {
        rc = match_target_ip(server, dfs_host, dfs_host_len, target_match);
    }
    cifs_server_unlock(server);
    rc
}

unsafe fn tree_connect_dfs_target(xid: u32, tcon: *mut cifs_tcon, cifs_sb: *mut cifs_sb_info, tree: *mut core::ffi::c_char, islink: bool, tl: *mut dfs_cache_tgt_list) -> i32 {
    let ops = (*(*tcon).ses).server;
    let server = (*(*tcon).ses).server;
    let mut tit = dfs_cache_get_tgt_iterator(tl);
    let mut share = core::ptr::null_mut();
    let mut prefix = core::ptr::null_mut();
    let mut rc = -ENOENT;
    while !tit.is_null() {
        kfree(share as *mut core::ffi::c_void); kfree(prefix as *mut core::ffi::c_void);
        share = core::ptr::null_mut(); prefix = core::ptr::null_mut();
        rc = dfs_cache_get_tgt_share((*server).leaf_fullpath.add(1), tit, &mut share, &mut prefix);
        if rc != 0 { break; }
        let mut target_match = false;
        rc = target_share_matches_server(server, share, &mut target_match);
        if rc != 0 { break; }
        if !target_match { rc = -EHOSTUNREACH; tit = dfs_cache_get_next_tgt(tl, tit); continue; }
        dfs_cache_noreq_update_tgthint((*server).leaf_fullpath.add(1), tit);
        scnprintf(tree, MAX_TREE_SIZE, b"\\%s\0".as_ptr() as _, share);
        rc = ((*ops).tree_connect)(xid, (*tcon).ses, tree, tcon, (*(*tcon).ses).local_nls);
        if islink && rc == 0 && !cifs_sb.is_null() { rc = cifs_update_super_prepath(cifs_sb, prefix); }
        break;
    }
    kfree(share as *mut core::ffi::c_void); kfree(prefix as *mut core::ffi::c_void);
    dfs_cache_free_tgts(tl);
    rc
}

pub unsafe fn cifs_tree_connect(xid: u32, tcon: *mut cifs_tcon) -> i32 {
    let server = (*(*tcon).ses).server;
    let ops = (*server).ops;
    DFS_CACHE_TGT_LIST!(tl);
    let mut cifs_sb = core::ptr::null_mut();
    let mut sb = core::ptr::null_mut();
    let mut ref_: dfs_info3_param = core::mem::zeroed();
    let tree = kzalloc(MAX_TREE_SIZE, GFP_KERNEL);
    if tree.is_null() { return -ENOMEM; }
    spin_lock(&mut (*tcon).tc_lock);
    if (*tcon).need_reconnect { (*tcon).status = TID_NEED_TCON; }
    if (*tcon).status == TID_GOOD { spin_unlock(&mut (*tcon).tc_lock); kfree(tree as _); return 0; }
    if (*tcon).status != TID_NEW && (*tcon).status != TID_NEED_TCON { spin_unlock(&mut (*tcon).tc_lock); kfree(tree as _); return -EHOSTDOWN; }
    (*tcon).status = TID_IN_TCON; spin_unlock(&mut (*tcon).tc_lock);
    let mut rc;
    if (*tcon).ipc {
        cifs_server_lock(server); scnprintf(tree, MAX_TREE_SIZE, b"\\\\%s\\IPC$\0".as_ptr() as _, (*server).hostname); cifs_server_unlock(server);
        rc = ((*ops).tree_connect)(xid, (*tcon).ses, tree, tcon, (*(*tcon).ses).local_nls);
    } else {
        sb = cifs_get_dfs_tcon_super(tcon); if !IS_ERR(sb) { cifs_sb = CIFS_SB(sb); }
        if (*server).leaf_fullpath.is_null() || dfs_cache_noreq_find((*server).leaf_fullpath.add(1), &mut ref_, &mut tl) {
            rc = ((*ops).tree_connect)(xid, (*tcon).ses, (*tcon).tree_name, tcon, (*(*tcon).ses).local_nls);
        } else { rc = tree_connect_dfs_target(xid, tcon, cifs_sb, tree, ref_.server_type == DFS_TYPE_LINK, &mut tl); free_dfs_info_param(&mut ref_); }
    }
    kfree(tree as _); cifs_put_tcp_super(sb);
    spin_lock(&mut (*tcon).tc_lock);
    if rc != 0 { if (*tcon).status == TID_IN_TCON { (*tcon).status = TID_NEED_TCON; } }
    else { if (*tcon).status == TID_IN_TCON { (*tcon).status = TID_GOOD; } (*tcon).need_reconnect = false; }
    spin_unlock(&mut (*tcon).tc_lock); rc
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
