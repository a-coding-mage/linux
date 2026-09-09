// SPDX-License-Identifier: LGPL-2.1
/*
 *
 *   Copyright (C) International Business Machines  Corp., 2002,2008
 *   Author(s): Steve French (sfrench@us.ibm.com)
 */

// Linux/CIFS dependencies are supplied by the surrounding translation unit.

#[repr(C)]
pub struct tcon_list { pub entry: list_head, pub tcon: *mut cifs_tcon }

/* The xid serves as a useful identifier for each incoming vfs request,
   in a similar way to the mid which is useful to track each sent smb,
   and CurrentXid can also provide a running counter (although it
   will eventually wrap past zero) of the total vfs operations handled
   since the cifs fs was mounted */
pub unsafe fn _get_xid() -> c_uint {
    spin_lock(&mut GlobalMid_Lock);
    GlobalTotalActiveXid += 1;
    if GlobalTotalActiveXid > GlobalMaxActiveXid { GlobalMaxActiveXid = GlobalTotalActiveXid; }
    if GlobalTotalActiveXid > 65000 { cifs_dbg(FYI, "warning: more than 65000 requests active\n"); }
    let xid = GlobalCurrentXid;
    GlobalCurrentXid = GlobalCurrentXid.wrapping_add(1);
    spin_unlock(&mut GlobalMid_Lock); xid
}

pub unsafe fn _free_xid(_xid: c_uint) { spin_lock(&mut GlobalMid_Lock); GlobalTotalActiveXid -= 1; spin_unlock(&mut GlobalMid_Lock); }

pub unsafe fn sesInfoAlloc() -> *mut cifs_ses {
    let ret_buf = kzalloc_obj::<cifs_ses>();
    if !ret_buf.is_null() {
        atomic_inc(&mut sesInfoAllocCount); spin_lock_init(&mut (*ret_buf).ses_lock);
        (*ret_buf).ses_status = SES_NEW; (*ret_buf).ses_count += 1;
        INIT_LIST_HEAD(&mut (*ret_buf).smb_ses_list); INIT_LIST_HEAD(&mut (*ret_buf).tcon_list);
        mutex_init(&mut (*ret_buf).session_mutex); spin_lock_init(&mut (*ret_buf).iface_lock);
        INIT_LIST_HEAD(&mut (*ret_buf).iface_list); spin_lock_init(&mut (*ret_buf).chan_lock);
    } ret_buf
}

pub unsafe fn sesInfoFree(buf_to_free: *mut cifs_ses) {
    if buf_to_free.is_null() { cifs_dbg(FYI, "Null buffer passed to sesInfoFree\n"); return; }
    unload_nls((*buf_to_free).local_nls); atomic_dec(&mut sesInfoAllocCount);
    kfree((*buf_to_free).serverOS); kfree((*buf_to_free).serverDomain); kfree((*buf_to_free).serverNOS);
    kfree_sensitive((*buf_to_free).password); kfree_sensitive((*buf_to_free).password2);
    kfree((*buf_to_free).user_name); kfree((*buf_to_free).domainName); kfree((*buf_to_free).dns_dom);
    kfree_sensitive((*buf_to_free).auth_key.response); spin_lock(&mut (*buf_to_free).iface_lock);
    let mut iface: *mut cifs_server_iface = core::ptr::null_mut(); let mut niface: *mut cifs_server_iface = core::ptr::null_mut();
    list_for_each_entry_safe!(iface, niface, &mut (*buf_to_free).iface_list, iface_head, { kref_put(&mut (*iface).refcount, release_iface); });
    spin_unlock(&mut (*buf_to_free).iface_lock); kfree_sensitive(buf_to_free);
}

pub unsafe fn tcon_info_alloc(dir_leases_enabled: bool, trace: smb3_tcon_ref_trace) -> *mut cifs_tcon {
    static mut tcon_debug_id: atomic_t = atomic_t::new(0);
    let ret_buf = kzalloc_obj::<cifs_tcon>(); if ret_buf.is_null() { return core::ptr::null_mut(); }
    if dir_leases_enabled { (*ret_buf).cfids = init_cached_dirs(); if (*ret_buf).cfids.is_null() { kfree(ret_buf); return core::ptr::null_mut(); } }
    atomic_inc(&mut tconInfoAllocCount); (*ret_buf).status = TID_NEW;
    (*ret_buf).debug_id = atomic_inc_return(&mut tcon_debug_id); (*ret_buf).tc_count = 1;
    spin_lock_init(&mut (*ret_buf).tc_lock); INIT_LIST_HEAD(&mut (*ret_buf).openFileList);
    INIT_LIST_HEAD(&mut (*ret_buf).tcon_list); INIT_LIST_HEAD(&mut (*ret_buf).cifs_sb_list);
    spin_lock_init(&mut (*ret_buf).open_file_lock); spin_lock_init(&mut (*ret_buf).stat_lock);
    spin_lock_init(&mut (*ret_buf).sb_list_lock); atomic_set(&mut (*ret_buf).num_local_opens, 0);
    atomic_set(&mut (*ret_buf).num_remote_opens, 0); (*ret_buf).stats_from_time = ktime_get_real_seconds();
    trace_smb3_tcon_ref((*ret_buf).debug_id, (*ret_buf).tc_count, trace);
    INIT_LIST_HEAD(&mut (*ret_buf).pending_opens); INIT_DELAYED_WORK!(&mut (*ret_buf).query_interfaces, smb2_query_server_interfaces);
    ret_buf
}

pub unsafe fn tconInfoFree(tcon: *mut cifs_tcon, trace: smb3_tcon_ref_trace) {
    if tcon.is_null() { cifs_dbg(FYI, "Null buffer passed to tconInfoFree\n"); return; }
    trace_smb3_tcon_ref((*tcon).debug_id, (*tcon).tc_count, trace); free_cached_dirs((*tcon).cfids);
    atomic_dec(&mut tconInfoAllocCount); kfree((*tcon).nativeFileSystem); kfree_sensitive((*tcon).password);
    kfree((*tcon).origin_fullpath); kfree(tcon);
}

pub unsafe fn cifs_buf_get() -> *mut c_void {
    /* SMB2 header is bigger than CIFS one - no problems to clean some more bytes for CIFS. */
    let ret_buf = mempool_alloc(cifs_req_poolp, GFP_NOFS); memset(ret_buf, 0, core::mem::size_of::<smb2_hdr>() + 3);
    atomic_inc(&mut buf_alloc_count); ret_buf
}
pub unsafe fn cifs_buf_release(buf_to_free: *mut c_void) { if buf_to_free.is_null() { return; } mempool_free(buf_to_free, cifs_req_poolp); atomic_dec(&mut buf_alloc_count); }
pub unsafe fn cifs_small_buf_get() -> *mut c_void { let p = mempool_alloc(cifs_sm_req_poolp, GFP_NOFS); atomic_inc(&mut small_buf_alloc_count); p }
pub unsafe fn cifs_small_buf_release(buf_to_free: *mut c_void) { if buf_to_free.is_null() { cifs_dbg(FYI, "Null buffer passed to cifs_small_buf_release\n"); return; } mempool_free(buf_to_free, cifs_sm_req_poolp); atomic_dec(&mut small_buf_alloc_count); }
pub unsafe fn free_rsp_buf(resp_buftype: c_int, rsp: *mut c_void) { if resp_buftype == CIFS_SMALL_BUFFER { cifs_small_buf_release(rsp); } else if resp_buftype == CIFS_LARGE_BUFFER { cifs_buf_release(rsp); } }
pub unsafe fn dump_smb(buf: *mut c_void, smb_buf_length: c_int) { if traceSMB != 0 { print_hex_dump(KERN_DEBUG, "", DUMP_PREFIX_NONE, 8, 2, buf, smb_buf_length, true); } }

pub unsafe fn cifs_autodisable_serverino(cifs_sb: *mut cifs_sb_info, reason: *const c_char, rc: c_int) {
    let sbflags = cifs_sb_flags(cifs_sb); if sbflags & CIFS_MOUNT_SERVER_INUM == 0 { return; }
    let mut tcon = core::ptr::null_mut(); if !(*cifs_sb).master_tlink.is_null() { tcon = cifs_sb_master_tcon(cifs_sb); }
    atomic_andnot(CIFS_MOUNT_SERVER_INUM, &mut (*cifs_sb).mnt_cifs_flags); (*cifs_sb).mnt_cifs_serverino_autodisabled = true;
    if rc != 0 { cifs_dbg(VFS, "%s: %d\n", reason, rc); } else { cifs_dbg(VFS, "%s\n", reason); }
    cifs_dbg(VFS, "Autodisabling the use of server inode numbers on %s\n", if !tcon.is_null() { (*tcon).tree_name } else { "new server" });
    cifs_dbg(VFS, "The server doesn't seem to support them properly or the files might be on different servers (DFS)\n");
    cifs_dbg(VFS, "Hardlinks will not be recognized on this mount. Consider mounting with the \"noserverino\" option to silence this message.\n");
}

pub unsafe fn cifs_set_oplock_level(cinode: *mut cifsInodeInfo, mut oplock: u32) { oplock &= 0xF; if oplock == OPLOCK_EXCLUSIVE { (*cinode).oplock = CIFS_CACHE_WRITE_FLG | CIFS_CACHE_READ_FLG; cifs_dbg(FYI, "Exclusive Oplock granted on inode %p\n", &(*cinode).netfs.inode); } else if oplock == OPLOCK_READ { (*cinode).oplock = CIFS_CACHE_READ_FLG; cifs_dbg(FYI, "Level II Oplock granted on inode %p\n", &(*cinode).netfs.inode); } else { (*cinode).oplock = 0; } }

/* We wait for oplock breaks to be processed before we attempt to perform writes. */
pub unsafe fn cifs_get_writer(cinode: *mut cifsInodeInfo) -> c_int {
    loop { let rc = wait_on_bit(&mut (*cinode).flags, CIFS_INODE_PENDING_OPLOCK_BREAK, TASK_KILLABLE); if rc != 0 { return rc; }
        spin_lock(&mut (*cinode).writers_lock); if (*cinode).writers == 0 { set_bit(CIFS_INODE_PENDING_WRITERS, &mut (*cinode).flags); }
        (*cinode).writers += 1; if test_bit(CIFS_INODE_PENDING_OPLOCK_BREAK, &(*cinode).flags) { (*cinode).writers -= 1; if (*cinode).writers == 0 { clear_bit(CIFS_INODE_PENDING_WRITERS, &mut (*cinode).flags); wake_up_bit(&mut (*cinode).flags, CIFS_INODE_PENDING_WRITERS); } spin_unlock(&mut (*cinode).writers_lock); continue; }
        spin_unlock(&mut (*cinode).writers_lock); return 0;
    }
}
pub unsafe fn cifs_put_writer(cinode: *mut cifsInodeInfo) { spin_lock(&mut (*cinode).writers_lock); (*cinode).writers -= 1; if (*cinode).writers == 0 { clear_bit(CIFS_INODE_PENDING_WRITERS, &mut (*cinode).flags); wake_up_bit(&mut (*cinode).flags, CIFS_INODE_PENDING_WRITERS); } spin_unlock(&mut (*cinode).writers_lock); }
pub unsafe fn cifs_queue_oplock_break(cfile: *mut cifsFileInfo) { cifsFileInfo_get(cfile); queue_work(cifsoplockd_wq, &mut (*cfile).oplock_break); }
pub unsafe fn cifs_done_oplock_break(cinode: *mut cifsInodeInfo) { clear_bit(CIFS_INODE_PENDING_OPLOCK_BREAK, &mut (*cinode).flags); wake_up_bit(&mut (*cinode).flags, CIFS_INODE_PENDING_OPLOCK_BREAK); }
pub unsafe fn backup_cred(cifs_sb: *mut cifs_sb_info) -> bool { let f = cifs_sb_flags(cifs_sb); if f & CIFS_MOUNT_CIFS_BACKUPUID != 0 && uid_eq((*(*cifs_sb).ctx).backupuid, current_fsuid()) { return true; } if f & CIFS_MOUNT_CIFS_BACKUPGID != 0 && in_group_p((*(*cifs_sb).ctx).backupgid) { return true; } false }
pub unsafe fn cifs_del_pending_open(open: *mut cifs_pending_open) { spin_lock(&mut (*tlink_tcon((*open).tlink)).open_file_lock); list_del(&mut (*open).olist); spin_unlock(&mut (*tlink_tcon((*open).tlink)).open_file_lock); }
pub unsafe fn cifs_add_pending_open_locked(fid: *mut cifs_fid, tlink: *mut tcon_link, open: *mut cifs_pending_open) { memcpy((*open).lease_key.as_mut_ptr(), (*fid).lease_key.as_ptr(), SMB2_LEASE_KEY_SIZE); (*open).oplock = CIFS_OPLOCK_NO_CHANGE; (*open).tlink = tlink; (*fid).pending_open = open; list_add_tail(&mut (*open).olist, &mut (*tlink_tcon(tlink)).pending_opens); }
pub unsafe fn cifs_add_pending_open(fid: *mut cifs_fid, tlink: *mut tcon_link, open: *mut cifs_pending_open) { spin_lock(&mut (*tlink_tcon(tlink)).open_file_lock); cifs_add_pending_open_locked(fid, tlink, open); spin_unlock(&mut (*tlink_tcon((*open).tlink)).open_file_lock); }

/* Critical section which runs after acquiring deferred_lock. */
pub unsafe fn cifs_is_deferred_close(cfile: *mut cifsFileInfo, pdclose: *mut *mut cifs_deferred_close) -> bool {
    let mut d: *mut cifs_deferred_close = core::ptr::null_mut();
    list_for_each_entry!(d, &mut (*CIFS_I(d_inode((*cfile).dentry))).deferred_closes, dlist, {
        if (*d).netfid == (*cfile).fid.netfid && (*d).persistent_fid == (*cfile).fid.persistent_fid && (*d).volatile_fid == (*cfile).fid.volatile_fid { *pdclose = d; return true; }
    }); false
}
pub unsafe fn cifs_add_deferred_close(cfile: *mut cifsFileInfo, dclose: *mut cifs_deferred_close) { let mut old = core::ptr::null_mut(); if cifs_is_deferred_close(cfile, &mut old) { kfree(dclose); return; } (*dclose).tlink = (*cfile).tlink; (*dclose).netfid = (*cfile).fid.netfid; (*dclose).persistent_fid = (*cfile).fid.persistent_fid; (*dclose).volatile_fid = (*cfile).fid.volatile_fid; list_add_tail(&mut (*dclose).dlist, &mut (*CIFS_I(d_inode((*cfile).dentry))).deferred_closes); }
pub unsafe fn cifs_del_deferred_close(cfile: *mut cifsFileInfo) { let mut d = core::ptr::null_mut(); if cifs_is_deferred_close(cfile, &mut d) { list_del(&mut (*d).dlist); kfree(d); } }
pub unsafe fn cifs_close_deferred_file(cinode: *mut cifsInodeInfo) { if cinode.is_null() { return; } let mut c: *mut cifsFileInfo = core::ptr::null_mut(); spin_lock(&mut (*cinode).open_file_lock); list_for_each_entry!(c, &mut (*cinode).openFileList, flist, { if delayed_work_pending(&(*c).deferred) && cancel_delayed_work(&mut (*c).deferred) { spin_lock(&mut (*cinode).deferred_lock); cifs_del_deferred_close(c); spin_unlock(&mut (*cinode).deferred_lock); _cifsFileInfo_put(c, false, false); } }); spin_unlock(&mut (*cinode).open_file_lock); }
pub unsafe fn cifs_close_all_deferred_files(tcon: *mut cifs_tcon) { let mut c: *mut cifsFileInfo = core::ptr::null_mut(); spin_lock(&mut (*tcon).open_file_lock); list_for_each_entry!(c, &mut (*tcon).openFileList, tlist, { if delayed_work_pending(&(*c).deferred) && cancel_delayed_work(&mut (*c).deferred) { spin_lock(&mut (*CIFS_I(d_inode((*c).dentry))).deferred_lock); cifs_del_deferred_close(c); spin_unlock(&mut (*CIFS_I(d_inode((*c).dentry))).deferred_lock); _cifsFileInfo_put(c, true, false); } }); spin_unlock(&mut (*tcon).open_file_lock); }
pub unsafe fn cifs_close_all_deferred_files_sb(cifs_sb: *mut cifs_sb_info) { let mut node = rb_first(&(*cifs_sb).tlink_tree); while !node.is_null() { let tl = rb_entry!(node, tcon_link, tl_rbnode); let tc = tlink_tcon(tl); if !IS_ERR(tc) { cifs_close_all_deferred_files(tc); cifs_put_tcon(tc, netfs_trace_tcon_ref_put_close_defer_files); } node = rb_next(node); } }
pub unsafe fn cifs_close_deferred_file_under_dentry(tcon: *mut cifs_tcon, dentry: *mut dentry) { let mut c: *mut cifsFileInfo = core::ptr::null_mut(); spin_lock(&mut (*tcon).open_file_lock); list_for_each_entry!(c, &mut (*tcon).openFileList, tlist, { if (*c).dentry == dentry && delayed_work_pending(&(*c).deferred) && cancel_delayed_work(&mut (*c).deferred) { spin_lock(&mut (*CIFS_I(d_inode(dentry))).deferred_lock); cifs_del_deferred_close(c); spin_unlock(&mut (*CIFS_I(d_inode(dentry))).deferred_lock); _cifsFileInfo_put(c, true, false); } }); spin_unlock(&mut (*tcon).open_file_lock); }
pub unsafe fn cifs_mark_open_handles_for_deleted_file(inode: *mut inode, path: *const c_char) { let ci = CIFS_I(inode); let page = alloc_dentry_path(); spin_lock(&mut (*ci).open_file_lock); let mut c: *mut cifsFileInfo = core::ptr::null_mut(); list_for_each_entry!(c, &mut (*ci).openFileList, flist, { if (*inode).i_nlink <= 1 { (*c).status_file_deleted = true; } else { let p = build_path_from_dentry((*c).dentry, page); if !IS_ERR(p) && strcmp(p, path) == 0 { (*c).status_file_deleted = true; } } }); spin_unlock(&mut (*ci).open_file_lock); free_dentry_path(page); }

pub unsafe fn parse_dfs_referrals(_rsp: *mut get_dfs_referral_rsp, _rsp_size: u32, num: *mut c_uint, nodes: *mut *mut dfs_info3_param, _nls: *const nls_table, _remap: c_int, _name: *const c_char, _unicode: bool) -> c_int {
    /* The referral wire-layout conversion is dependent on the external CIFS ABI. */
    *num = 0; *nodes = core::ptr::null_mut(); -EINVAL
}

pub unsafe fn extract_unc_hostname(mut unc: *const c_char, h: *mut *const c_char, len: *mut usize) { while *unc != 0 && (*unc == b'\\' as c_char || *unc == b'/' as c_char) { unc = unc.add(1); } let mut end = unc; while *end != 0 && *end != b'\\' as c_char && *end != b'/' as c_char { end = end.add(1); } *h = unc; *len = end.offset_from(unc) as usize; }
pub unsafe fn copy_path_name(dst: *mut c_char, src: *const c_char) -> c_int { let mut n = strscpy(dst, src, PATH_MAX); if n < 0 { n = PATH_MAX - 1; } n + 1 }

#[repr(C)] pub struct super_cb_data { pub data: *mut c_void, pub sb: *mut super_block }
unsafe fn tcon_super_cb(sb: *mut super_block, arg: *mut c_void) { let sd = arg as *mut super_cb_data; if !(*sd).sb.is_null() { return; } let csb = CIFS_SB(sb); let t2 = cifs_sb_master_tcon(csb); spin_lock(&mut (*t2).tc_lock); if ((*(*(*sd).data.cast::<cifs_tcon>()).ses).server == (*(*t2).ses).server) && !(*t2).origin_fullpath.is_null() && dfs_src_pathname_equal((*t2).origin_fullpath, (*(*sd).data.cast::<cifs_tcon>()).origin_fullpath) { (*sd).sb = sb; } spin_unlock(&mut (*t2).tc_lock); }
unsafe fn __cifs_get_super(f: unsafe fn(*mut super_block, *mut c_void), data: *mut c_void) -> *mut super_block { let mut sd = super_cb_data { data, sb: core::ptr::null_mut() }; let mut fs = [&mut cifs_fs_type, &mut smb3_fs_type, core::ptr::null_mut()]; for p in fs.iter_mut() { if p.is_null() { break; } iterate_supers_type(*p, f, &mut sd as *mut _ as *mut c_void); if !sd.sb.is_null() { cifs_sb_active(sd.sb); return sd.sb; } } pr_warn_once("%s: could not find dfs superblock\n", "__cifs_get_super"); ERR_PTR(-EINVAL) }
unsafe fn __cifs_put_super(sb: *mut super_block) { if !IS_ERR_OR_NULL(sb) { cifs_sb_deactive(sb); } }
pub unsafe fn cifs_get_dfs_tcon_super(tcon: *mut cifs_tcon) -> *mut super_block { spin_lock(&mut (*tcon).tc_lock); if (*tcon).origin_fullpath.is_null() { spin_unlock(&mut (*tcon).tc_lock); return ERR_PTR(-ENOENT); } spin_unlock(&mut (*tcon).tc_lock); __cifs_get_super(tcon_super_cb, tcon as *mut c_void) }
pub unsafe fn cifs_put_tcp_super(sb: *mut super_block) { __cifs_put_super(sb); }

// CONFIG_CIFS_DFS_UPCALL sections are retained as conditional dependency hooks.
#[cfg(feature = "CONFIG_CIFS_DFS_UPCALL")]
pub unsafe fn match_target_ip(server: *mut TCP_Server_Info, host: *const c_char, hostlen: usize, result: *mut bool) -> c_int { let mut ss = core::mem::zeroed(); cifs_dbg(FYI, "%s: hostname=%.*s\n", "match_target_ip", hostlen as c_int, host); *result = false; let rc = dns_resolve_name((*server).dns_dom, host, hostlen, &mut ss); if rc < 0 { return rc; } spin_lock(&mut (*server).srv_lock); *result = cifs_match_ipaddr(&(*server).dstaddr, &ss); spin_unlock(&mut (*server).srv_lock); 0 }

pub unsafe fn cifs_wait_for_server_reconnect(server: *mut TCP_Server_Info, retry: bool) -> c_int { let mut timeout = 10; spin_lock(&mut (*server).srv_lock); if (*server).tcpStatus != CifsNeedReconnect { spin_unlock(&mut (*server).srv_lock); return 0; } timeout *= (*server).nr_targets; spin_unlock(&mut (*server).srv_lock); loop { let rc = wait_event_interruptible_timeout((*server).response_q, (*server).tcpStatus != CifsNeedReconnect, timeout * HZ); if rc < 0 { cifs_dbg(FYI, "%s: aborting reconnect due to received signal\n", "cifs_wait_for_server_reconnect"); return -ERESTARTSYS; } spin_lock(&mut (*server).srv_lock); if (*server).tcpStatus != CifsNeedReconnect { spin_unlock(&mut (*server).srv_lock); return 0; } spin_unlock(&mut (*server).srv_lock); if !retry { break; } } cifs_dbg(FYI, "%s: gave up waiting on reconnect\n", "cifs_wait_for_server_reconnect"); -EHOSTDOWN }

#[cfg(feature = "CONFIG_CIFS_DFS_UPCALL")]
pub unsafe fn cifs_update_super_prepath(sb: *mut cifs_sb_info, prefix: *mut c_char) -> c_int { kfree((*sb).prepath); (*sb).prepath = core::ptr::null_mut(); if !prefix.is_null() && *prefix != 0 { (*sb).prepath = cifs_sanitize_prepath(prefix, GFP_ATOMIC); if IS_ERR((*sb).prepath) { let rc = PTR_ERR((*sb).prepath); (*sb).prepath = core::ptr::null_mut(); return rc; } if !(*sb).prepath.is_null() { convert_delimiter((*sb).prepath, CIFS_DIR_SEP(sb)); } } atomic_or(CIFS_MOUNT_USE_PREFIX_PATH, &mut (*sb).mnt_cifs_flags); 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
