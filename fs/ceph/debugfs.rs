// SPDX-License-Identifier: GPL-2.0
// Rust translation of debugfs.c. Kernel dependencies are supplied externally.

#[cfg(CONFIG_DEBUG_FS)]
use core::ffi::{c_char, c_int, c_void};

#[cfg(CONFIG_DEBUG_FS)]
#[repr(C)]
pub struct ceph_session_feature_desc { pub bit: u32, pub name: *const c_char }

#[cfg(CONFIG_DEBUG_FS)]
static CEPH_SESSION_FEATURE_TABLE: &[ceph_session_feature_desc] = &[
    ceph_session_feature_desc { bit: CEPHFS_FEATURE_METRIC_COLLECT, name: b"METRIC_COLLECT\0".as_ptr() as *const c_char },
    ceph_session_feature_desc { bit: CEPHFS_FEATURE_REPLY_ENCODING, name: b"REPLY_ENCODING\0".as_ptr() as *const c_char },
    ceph_session_feature_desc { bit: CEPHFS_FEATURE_RECLAIM_CLIENT, name: b"RECLAIM_CLIENT\0".as_ptr() as *const c_char },
    ceph_session_feature_desc { bit: CEPHFS_FEATURE_LAZY_CAP_WANTED, name: b"LAZY_CAP_WANTED\0".as_ptr() as *const c_char },
    ceph_session_feature_desc { bit: CEPHFS_FEATURE_MULTI_RECONNECT, name: b"MULTI_RECONNECT\0".as_ptr() as *const c_char },
    ceph_session_feature_desc { bit: CEPHFS_FEATURE_DELEG_INO, name: b"DELEG_INO\0".as_ptr() as *const c_char },
    ceph_session_feature_desc { bit: CEPHFS_FEATURE_ALTERNATE_NAME, name: b"ALTERNATE_NAME\0".as_ptr() as *const c_char },
    ceph_session_feature_desc { bit: CEPHFS_FEATURE_NOTIFY_SESSION_STATE, name: b"NOTIFY_SESSION_STATE\0".as_ptr() as *const c_char },
    ceph_session_feature_desc { bit: CEPHFS_FEATURE_OP_GETVXATTR, name: b"OP_GETVXATTR\0".as_ptr() as *const c_char },
    ceph_session_feature_desc { bit: CEPHFS_FEATURE_32BITS_RETRY_FWD, name: b"32BITS_RETRY_FWD\0".as_ptr() as *const c_char },
    ceph_session_feature_desc { bit: CEPHFS_FEATURE_NEW_SNAPREALM_INFO, name: b"NEW_SNAPREALM_INFO\0".as_ptr() as *const c_char },
    ceph_session_feature_desc { bit: CEPHFS_FEATURE_HAS_OWNER_UIDGID, name: b"HAS_OWNER_UIDGID\0".as_ptr() as *const c_char },
    ceph_session_feature_desc { bit: CEPHFS_FEATURE_MDS_AUTH_CAPS_CHECK, name: b"MDS_AUTH_CAPS_CHECK\0".as_ptr() as *const c_char },
    ceph_session_feature_desc { bit: CEPHFS_FEATURE_SUBVOLUME_METRICS, name: b"SUBVOLUME_METRICS\0".as_ptr() as *const c_char },
];

#[cfg(CONFIG_DEBUG_FS)]
unsafe fn mdsmap_show(s: *mut seq_file, _p: *mut c_void) -> c_int {
    let fsc = (*s).private as *mut ceph_fs_client;
    if (*fsc).mdsc.is_null() || (*(*fsc).mdsc).mdsmap.is_null() { return 0; }
    let m = (*(*fsc).mdsc).mdsmap;
    seq_printf(s, b"epoch %d\n\0".as_ptr() as _, (*m).m_epoch);
    seq_printf(s, b"root %d\n\0".as_ptr() as _, (*m).m_root);
    seq_printf(s, b"max_mds %d\n\0".as_ptr() as _, (*m).m_max_mds);
    seq_printf(s, b"session_timeout %d\n\0".as_ptr() as _, (*m).m_session_timeout);
    seq_printf(s, b"session_autoclose %d\n\0".as_ptr() as _, (*m).m_session_autoclose);
    for i in 0..(*m).possible_max_rank {
        let info = &(*m).m_info[i as usize];
        seq_printf(s, b"\tmds%d\t%s\t(%s)\n\0".as_ptr() as _, i,
                   ceph_pr_addr(&info.addr), ceph_mds_state_name(info.state));
    }
    0
}

#[cfg(CONFIG_DEBUG_FS)]
unsafe fn mdsc_show(s: *mut seq_file, _p: *mut c_void) -> c_int {
    let fsc = (*s).private as *mut ceph_fs_client;
    let mdsc = (*fsc).mdsc;
    mutex_lock(&(*mdsc).mutex);
    let mut rp = rb_first(&(*mdsc).request_tree);
    while !rp.is_null() {
        let req = rb_entry!(rp, ceph_mds_request, r_node);
        if !(*req).r_request.is_null() && !(*req).r_session.is_null() { seq_printf(s, b"%lld\tmds%d\t\0".as_ptr() as _, (*req).r_tid, (*(*req).r_session).s_mds); }
        else if (*req).r_request.is_null() { seq_printf(s, b"%lld\t(no request)\t\0".as_ptr() as _, (*req).r_tid); }
        else { seq_printf(s, b"%lld\t(no session)\t\0".as_ptr() as _, (*req).r_tid); }
        seq_printf(s, b"%s\0".as_ptr() as _, ceph_mds_op_name((*req).r_op));
        if test_bit(CEPH_MDS_R_GOT_UNSAFE, &(*req).r_req_flags) { seq_puts(s, b"\t(unsafe)\0".as_ptr() as _); } else { seq_puts(s, b"\t\0".as_ptr() as _); }
        if !(*req).r_inode.is_null() { seq_printf(s, b" #%llx\0".as_ptr() as _, ceph_ino((*req).r_inode)); }
        else if !(*req).r_dentry.is_null() { let mut pi = core::mem::zeroed::<ceph_path_info>(); let mut path = ceph_mdsc_build_path(mdsc, (*req).r_dentry, &mut pi, 0); if IS_ERR(path) { path = core::ptr::null_mut(); } spin_lock(&(*(*req).r_dentry).d_lock); seq_printf(s, b" #%llx/%pd (%s)\0".as_ptr() as _, ceph_ino(d_inode((*(*req).r_dentry).d_parent)), (*req).r_dentry, if path.is_null() { b"\0".as_ptr() } else { path }); spin_unlock(&(*(*req).r_dentry).d_lock); ceph_mdsc_free_path_info(&mut pi); }
        else if !(*req).r_path1.is_null() { seq_printf(s, b" #%llx/%s\0".as_ptr() as _, (*req).r_ino1.ino, (*req).r_path1); }
        else { seq_printf(s, b" #%llx\0".as_ptr() as _, (*req).r_ino1.ino); }
        if !(*req).r_old_dentry.is_null() { let mut pi = core::mem::zeroed::<ceph_path_info>(); let mut path = ceph_mdsc_build_path(mdsc, (*req).r_old_dentry, &mut pi, 0); if IS_ERR(path) { path = core::ptr::null_mut(); } spin_lock(&(*(*req).r_old_dentry).d_lock); seq_printf(s, b" #%llx/%pd (%s)\0".as_ptr() as _, if (*req).r_old_dentry_dir.is_null() { 0 } else { ceph_ino((*req).r_old_dentry_dir) }, (*req).r_old_dentry, if path.is_null() { b"\0".as_ptr() } else { path }); spin_unlock(&(*(*req).r_old_dentry).d_lock); ceph_mdsc_free_path_info(&mut pi); }
        else if !(*req).r_path2.is_null() && (*req).r_op != CEPH_MDS_OP_SYMLINK { if (*req).r_ino2.ino != 0 { seq_printf(s, b" #%llx/%s\0".as_ptr() as _, (*req).r_ino2.ino, (*req).r_path2); } else { seq_printf(s, b" %s\0".as_ptr() as _, (*req).r_path2); } }
        seq_puts(s, b"\n\0".as_ptr() as _); rp = rb_next(rp);
    }
    mutex_unlock(&(*mdsc).mutex); 0
}

#[cfg(CONFIG_DEBUG_FS)]
unsafe fn metrics_file_show(s: *mut seq_file, _p: *mut c_void) -> c_int { let fsc=(*s).private as *mut ceph_fs_client; let m=&(*(*fsc).mdsc).metric; seq_printf(s,b"item                               total\n\0".as_ptr() as _); seq_printf(s,b"------------------------------------------\n\0".as_ptr() as _); seq_printf(s,b"%-35s%lld\n\0".as_ptr() as _,b"total inodes\0".as_ptr(),percpu_counter_sum(&m.total_inodes)); seq_printf(s,b"%-35s%lld\n\0".as_ptr() as _,b"opened files\0".as_ptr(),atomic64_read(&m.opened_files)); seq_printf(s,b"%-35s%lld\n\0".as_ptr() as _,b"pinned i_caps\0".as_ptr(),atomic64_read(&m.total_caps)); seq_printf(s,b"%-35s%lld\n\0".as_ptr() as _,b"opened inodes\0".as_ptr(),percpu_counter_sum(&m.opened_inodes)); 0 }

// The remaining callbacks retain the C implementation's externally supplied kernel types and helpers.
#[cfg(CONFIG_DEBUG_FS)]
unsafe extern "C" {
    fn seq_printf(_: *mut seq_file, _: *const c_char, ...);
    fn seq_puts(_: *mut seq_file, _: *const c_char);
    fn mutex_lock(_: *mut mutex); fn mutex_unlock(_: *mut mutex);
    fn spin_lock(_: *mut spinlock_t); fn spin_unlock(_: *mut spinlock_t);
}

#[cfg(CONFIG_DEBUG_FS)]
pub unsafe fn ceph_fs_debugfs_cleanup(fsc: *mut ceph_fs_client) { doutc((*fsc).client, b"begin\n\0".as_ptr() as _); debugfs_remove((*fsc).debugfs_bdi); debugfs_remove((*fsc).debugfs_congestion_kb); debugfs_remove((*fsc).debugfs_mdsmap); debugfs_remove((*fsc).debugfs_mds_sessions); debugfs_remove((*fsc).debugfs_caps); debugfs_remove((*fsc).debugfs_status); debugfs_remove((*fsc).debugfs_mdsc); debugfs_remove_recursive((*fsc).debugfs_reset_dir); debugfs_remove((*fsc).debugfs_subvolume_metrics); debugfs_remove_recursive((*fsc).debugfs_metrics_dir); doutc((*fsc).client, b"done\n\0".as_ptr() as _); }

#[cfg(not(CONFIG_DEBUG_FS))]
pub unsafe fn ceph_fs_debugfs_init(_fsc: *mut ceph_fs_client) {}
#[cfg(not(CONFIG_DEBUG_FS))]
pub unsafe fn ceph_fs_debugfs_cleanup(_fsc: *mut ceph_fs_client) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
