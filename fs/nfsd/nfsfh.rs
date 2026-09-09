// SPDX-License-Identifier: GPL-2.0
/* NFS server file handle treatment. */

// Kernel dependencies from the original includes are supplied by surrounding
// translated modules.

unsafe fn nfsd_acceptable(expv: *mut core::ffi::c_void, dentry: *mut dentry) -> i32 {
    let exp = expv as *mut svc_export;
    let mut tdentry: *mut dentry;
    let parent: *mut dentry;
    if (*exp).ex_flags & NFSEXP_NOSUBTREECHECK != 0 { return 1; }
    tdentry = dget(dentry);
    while tdentry != (*exp).ex_path.dentry && !IS_ROOT(tdentry) {
        let err: i32;
        parent = dget_parent(tdentry);
        err = inode_permission(&nop_mnt_idmap, d_inode(parent), MAY_EXEC);
        if err < 0 { dput(parent); break; }
        dput(tdentry);
        tdentry = parent;
    }
    if tdentry != (*exp).ex_path.dentry { dprintk!("nfsd_acceptable failed at %p %pd\n", tdentry, tdentry); }
    let rv = (tdentry == (*exp).ex_path.dentry) as i32;
    dput(tdentry);
    rv
}

unsafe fn nfsd_mode_check(dentry: *mut dentry, requested: umode_t) -> __be32 {
    let mode = (*d_inode(dentry)).i_mode & S_IFMT;
    if requested == 0 { return nfs_ok; }
    if mode == requested {
        if mode == S_IFDIR && !d_can_lookup(dentry) { return nfserr_notdir; }
        return nfs_ok;
    }
    if mode == S_IFLNK { return if requested == S_IFDIR { nfserr_symlink_not_dir } else { nfserr_symlink }; }
    if requested == S_IFDIR { return nfserr_notdir; }
    if mode == S_IFDIR { return nfserr_isdir; }
    nfserr_wrong_type
}

unsafe fn nfsd_originating_port_ok(rqstp: *mut svc_rqst, cred: *mut svc_cred, exp: *mut svc_export) -> bool {
    if nfsexp_flags(cred, exp) & NFSEXP_INSECURE_PORT != 0 { return true; }
    if (*cred).cr_flavor >= RPC_AUTH_GSS { return true; }
    test_bit(RQ_SECURE, &(*rqstp).rq_flags)
}

unsafe fn nfsd_setuser_and_check_port(rqstp: *mut svc_rqst, cred: *mut svc_cred, exp: *mut svc_export) -> __be32 {
    if !rqstp.is_null() && !nfsd_originating_port_ok(rqstp, cred, exp) {
        if IS_ENABLED(CONFIG_SUNRPC_DEBUG) {
            let mut buf = [0u8; RPC_MAX_ADDRBUFLEN];
            dprintk!("nfsd: request from insecure port %s!\n", svc_print_addr(rqstp, buf.as_mut_ptr(), buf.len()));
        }
        return nfserr_perm;
    }
    nfserrno(nfsd_setuser(cred, exp))
}

unsafe fn check_pseudo_root(dentry: *mut dentry, exp: *mut svc_export) -> __be32 {
    if (*exp).ex_flags & NFSEXP_V4ROOT == 0 { return nfs_ok; }
    if !d_is_dir(dentry) && !d_is_symlink(dentry) { return nfserr_stale; }
    if dentry != (*exp).ex_path.dentry { return nfserr_stale; }
    nfs_ok
}

pub unsafe fn fh_append_mac(fh: *mut knfsd_fh, fh_maxsize: i32, net: *mut net) -> bool {
    let nn = net_generic(net, nfsd_net_id);
    let fh_key = (*nn).fh_key;
    let hash: __le64;
    if fh_key.is_null() { pr_warn_ratelimited!("NFSD: unable to sign filehandles, fh_key not set.\n"); return false; }
    if (*fh).fh_size + core::mem::size_of::<__le64>() > fh_maxsize as usize {
        pr_warn_ratelimited!("NFSD: unable to sign filehandles, fh_size %zu would be greater than fh_maxsize %d.\n", (*fh).fh_size + core::mem::size_of::<__le64>(), fh_maxsize); return false;
    }
    hash = cpu_to_le64(siphash(&(*fh).fh_raw as *const _, (*fh).fh_size, fh_key));
    core::ptr::copy_nonoverlapping(&hash as *const _, (*fh).fh_raw.as_mut_ptr().add((*fh).fh_size), 1);
    (*fh).fh_size += core::mem::size_of::<__le64>();
    true
}

unsafe fn fh_verify_mac(fhp: *mut svc_fh, net: *mut net) -> bool {
    let nn = net_generic(net, nfsd_net_id);
    let fh = &mut (*fhp).fh_handle;
    let fh_key = (*nn).fh_key;
    if fh_key.is_null() { pr_warn_ratelimited!("NFSD: unable to verify signed filehandles, fh_key not set.\n"); return false; }
    let hash = cpu_to_le64(siphash(&fh.fh_raw as *const _, fh.fh_size - core::mem::size_of::<__le64>(), fh_key));
    crypto_memneq(fh.fh_raw.as_ptr().add(fh.fh_size - core::mem::size_of::<__le64>()), &hash as *const _, core::mem::size_of::<__le64>()) == 0
}

unsafe fn nfsd_set_fh_dentry(rqstp: *mut svc_rqst, net: *mut net, cred: *mut svc_cred, client: *mut auth_domain, gssclient: *mut auth_domain, fhp: *mut svc_fh) -> __be32 {
    let fh = &mut (*fhp).fh_handle;
    let mut fid: *mut fid = core::ptr::null_mut();
    let mut exp: *mut svc_export;
    let mut dentry: *mut dentry;
    let fileid_type: i32;
    let mut data_left = fh.fh_size / 4;
    let mut len: i32;
    let mut error = nfserr_badhandle;
    if fh.fh_size == 0 { return nfserr_nofilehandle; }
    if fh.fh_version != 1 { return error; }
    data_left -= 1; if data_left < 0 { return error; }
    if fh.fh_auth_type != 0 { return error; }
    len = key_len(fh.fh_fsid_type) / 4; if len == 0 { return error; }
    if fh.fh_fsid_type == FSID_MAJOR_MINOR {
        let fsid = fh_fsid(fh);
        len = key_len(FSID_ENCODE_DEV) / 4;
        fh.fh_fsid_type = FSID_ENCODE_DEV;
        *fsid.add(0) = new_encode_dev(MKDEV(ntohl(*fsid.add(0) as __be32), ntohl(*fsid.add(1) as __be32)));
        *fsid.add(1) = *fsid.add(2);
    }
    data_left -= len; if data_left < 0 { return error; }
    exp = rqst_exp_find(if rqstp.is_null() { core::ptr::null_mut() } else { &mut (*rqstp).rq_chandle }, net, client, gssclient, fh.fh_fsid_type, fh_fsid(fh));
    fid = (fh_fsid(fh) as *mut u32).add(len as usize) as *mut fid;
    error = nfserr_stale;
    if IS_ERR(exp) { trace_nfsd_set_fh_dentry_badexport(rqstp, fhp, PTR_ERR(exp)); return if PTR_ERR(exp) == -ENOENT { error } else { nfserrno(PTR_ERR(exp)) }; }
    if (*exp).ex_flags & NFSEXP_NOSUBTREECHECK != 0 {
        let new = prepare_creds(); if new.is_null() { error = nfserrno(-ENOMEM); goto out; }
        (*new).cap_effective = cap_raise_nfsd_set((*new).cap_effective, (*new).cap_permitted);
        put_cred(override_creds(new));
    } else { error = nfsd_setuser_and_check_port(rqstp, cred, exp); if error != 0 { goto out; } }
    fileid_type = fh.fh_fileid_type; error = nfserr_stale;
    if fileid_type == FILEID_ROOT { dentry = dget((*exp).ex_path.dentry); }
    else {
        if (*exp).ex_flags & NFSEXP_SIGN_FH != 0 { if !fh_verify_mac(fhp, net) { trace_nfsd_set_fh_dentry_badmac(rqstp, fhp, -ESTALE); goto out; } data_left -= FH_MAC_WORDS; }
        dentry = exportfs_decode_fh_raw((*exp).ex_path.mnt, fid, data_left, fileid_type, 0, nfsd_acceptable, exp);
        if IS_ERR_OR_NULL(dentry) { trace_nfsd_set_fh_dentry_badhandle(rqstp, fhp, if dentry.is_null() { -ESTALE } else { PTR_ERR(dentry) }); if PTR_ERR(dentry) != -ENOMEM && PTR_ERR(dentry) != -ETIMEDOUT { dentry = ERR_PTR(-ESTALE); } }
    }
    error = nfserr_badhandle; if dentry.is_null() { goto out; } if IS_ERR(dentry) { if PTR_ERR(dentry) != -EINVAL { error = nfserrno(PTR_ERR(dentry)); } goto out; }
    match (*fhp).fh_maxsize { NFS4_FHSIZE => { if (*(*dentry).d_sb).s_export_op.flags & EXPORT_OP_NOATOMIC_ATTR != 0 { (*fhp).fh_no_atomic_attr = true; } (*fhp).fh_64bit_cookies = true; }, NFS3_FHSIZE => { if (*(*dentry).d_sb).s_export_op.flags & EXPORT_OP_NOWCC != 0 { (*fhp).fh_no_wcc = true; } (*fhp).fh_64bit_cookies = true; if (*exp).ex_flags & NFSEXP_V4ROOT != 0 { dput(dentry); goto out; } }, NFS_FHSIZE => { (*fhp).fh_no_wcc = true; if EX_WGATHER(exp) { (*fhp).fh_use_wgather = true; } if (*exp).ex_flags & NFSEXP_V4ROOT != 0 { dput(dentry); goto out; } }, _ => {} }
    (*fhp).fh_dentry = dentry; (*fhp).fh_export = exp; return 0;
out: exp_put(exp); error
}

// The remaining routines retain the original interfaces and operations.
pub unsafe fn fh_verify_local(net: *mut net, cred: *mut svc_cred, client: *mut auth_domain, fhp: *mut svc_fh, typ: umode_t, access: i32) -> __be32 { __fh_verify(core::ptr::null_mut(), net, cred, client, core::ptr::null_mut(), fhp, typ, access) }
pub unsafe fn fh_verify(rqstp: *mut svc_rqst, fhp: *mut svc_fh, typ: umode_t, access: i32) -> __be32 { __fh_verify(rqstp, SVC_NET(rqstp), &mut (*rqstp).rq_cred, (*rqstp).rq_client, (*rqstp).rq_gssclient, fhp, typ, access) }

unsafe fn __fh_verify(rqstp: *mut svc_rqst, net: *mut net, cred: *mut svc_cred, client: *mut auth_domain, gssclient: *mut auth_domain, fhp: *mut svc_fh, typ: umode_t, access: i32) -> __be32 {
    let nn = net_generic(net, nfsd_net_id); let mut exp = (*fhp).fh_export; let mut error;
    if (*fhp).fh_dentry.is_null() { error = nfsd_set_fh_dentry(rqstp, net, cred, client, gssclient, fhp); if error != 0 { goto out; } }
    let dentry = (*fhp).fh_dentry; exp = (*fhp).fh_export;
    trace_nfsd_fh_verify(rqstp, fhp, typ, access); error = check_pseudo_root(dentry, exp); if error != 0 { goto out; }
    error = nfsd_setuser_and_check_port(rqstp, cred, exp); if error != 0 { goto out; }
    error = nfsd_mode_check(dentry, typ); if error != 0 { goto out; }
    if rqstp.is_null() { goto check_permissions; }
    if access & NFSD_MAY_NLM != 0 && (*exp).ex_flags & NFSEXP_NOAUTHNLM != 0 { goto out; }
    if access & NFSD_MAY_NLM == 0 { error = check_xprtsec_policy(exp, rqstp); if error != 0 { goto out; } }
    let mut may_bypass_gss = access & NFSD_MAY_BYPASS_GSS != 0;
    if access & NFSD_MAY_BYPASS_GSS_ON_ROOT != 0 && (*exp).ex_path.dentry == dentry { may_bypass_gss = true; }
    error = check_security_flavor(exp, rqstp, may_bypass_gss); if error != 0 { goto out; }
    svc_xprt_set_valid((*rqstp).rq_xprt);
check_permissions: error = nfsd_permission(cred, exp, dentry, access);
out: trace_nfsd_fh_verify_err(rqstp, fhp, typ, access, error); if error == nfserr_stale { nfsd_stats_fh_stale_inc(nn, exp); } error
}

pub unsafe fn nfsd4_change_attribute(stat: *const kstat) -> u64 { let mut chattr; if (*stat).result_mask & STATX_CHANGE_COOKIE != 0 { chattr = (*stat).change_cookie; if S_ISREG((*stat).mode) && (*stat).attributes & STATX_ATTR_CHANGE_MONOTONIC == 0 { chattr += ((*stat).ctime.tv_sec as u64) << 30; chattr += (*stat).ctime.tv_nsec as u64; } } else { chattr = time_to_chattr(&(*stat).ctime); } chattr }

unsafe fn is_root_export(exp: *mut svc_export) -> bool { (*exp).ex_path.dentry == (*(*exp).ex_path.dentry).d_sb.s_root }
unsafe fn exp_sb(exp: *mut svc_export) -> *mut super_block { (*exp).ex_path.dentry.as_ref().unwrap().d_sb }
unsafe fn fsid_type_ok_for_exp(fsid_type: u8, exp: *mut svc_export) -> bool { match fsid_type { FSID_DEV => { if !old_valid_dev((*exp_sb(exp)).s_dev) { return false; } ((*exp_sb(exp)).s_type.fs_flags & FS_REQUIRES_DEV) != 0 }, FSID_MAJOR_MINOR | FSID_ENCODE_DEV => ((*exp_sb(exp)).s_type.fs_flags & FS_REQUIRES_DEV) != 0, FSID_NUM => (*exp).ex_flags & NFSEXP_FSID != 0, FSID_UUID8 | FSID_UUID16 => if !is_root_export(exp) { false } else { !(*exp).ex_uuid.is_null() }, FSID_UUID4_INUM | FSID_UUID16_INUM => !(*exp).ex_uuid.is_null(), _ => true } }
unsafe fn set_version_and_fsid_type(fhp: *mut svc_fh, exp: *mut svc_export, mut ref_fh: *mut svc_fh) { let mut version; let fsid_type; 'retry: loop { version = 1; if !ref_fh.is_null() && (*ref_fh).fh_export == exp { let v = (*ref_fh).fh_handle.fh_version; let mut f = (*ref_fh).fh_handle.fh_fsid_type; ref_fh = core::ptr::null_mut(); match v { 0xca => f = FSID_DEV, 1 => {}, _ => continue 'retry } if !fsid_type_ok_for_exp(f, exp) { continue 'retry; } fsid_type = f; } else if (*exp).ex_flags & NFSEXP_FSID != 0 { fsid_type = FSID_NUM; } else if !(*exp).ex_uuid.is_null() { fsid_type = if (*fhp).fh_maxsize >= 64 { if is_root_export(exp) { FSID_UUID16 } else { FSID_UUID16_INUM } } else if is_root_export(exp) { FSID_UUID8 } else { FSID_UUID4_INUM }; } else if !old_valid_dev((*exp_sb(exp)).s_dev) { fsid_type = FSID_ENCODE_DEV; } else { fsid_type = FSID_DEV; } break; } (*fhp).fh_handle.fh_version = version; if version != 0 { (*fhp).fh_handle.fh_fsid_type = fsid_type; } }

pub unsafe fn fh_put(fhp: *mut svc_fh) { let d = (*fhp).fh_dentry; let e = (*fhp).fh_export; if !d.is_null() { (*fhp).fh_dentry = core::ptr::null_mut(); dput(d); fh_clear_pre_post_attrs(fhp); } fh_drop_write(fhp); if !e.is_null() { exp_put(e); (*fhp).fh_export = core::ptr::null_mut(); } (*fhp).fh_no_wcc = false; }
pub unsafe fn fh_update(fhp: *mut svc_fh) -> __be32 { if (*fhp).fh_dentry.is_null() { printk!(KERN_ERR "fh_update: fh not verified!\n"); return nfserr_serverfault; } if d_really_is_negative((*fhp).fh_dentry) { printk!(KERN_ERR "fh_update: %pd2 still negative!\n", (*fhp).fh_dentry); return nfserr_serverfault; } if (*fhp).fh_handle.fh_fileid_type == FILEID_ROOT { _fh_update(fhp, (*fhp).fh_export, (*fhp).fh_dentry); if (*fhp).fh_handle.fh_fileid_type == FILEID_INVALID { return nfserr_stale; } } 0 }
pub unsafe fn fh_getattr(fhp: *const svc_fh, stat: *mut kstat) -> __be32 { let p = path { mnt: (*(*fhp).fh_export).ex_path.mnt, dentry: (*fhp).fh_dentry }; let inode = d_inode(p.dentry); let mut mask = STATX_BASIC_STATS; if S_ISREG((*inode).i_mode) { mask |= STATX_DIOALIGN | STATX_DIO_READ_ALIGN; } if (*fhp).fh_maxsize == NFS4_FHSIZE { mask |= STATX_BTIME | STATX_CHANGE_COOKIE; } nfserrno(vfs_getattr(&p, stat, mask, AT_STATX_SYNC_AS_STAT)) }
pub unsafe fn fh_fill_pre_attrs(fhp: *mut svc_fh) -> __be32 { if (*fhp).fh_no_wcc || (*fhp).fh_pre_saved { return nfs_ok; } let mut stat = core::mem::zeroed::<kstat>(); let e = fh_getattr(fhp, &mut stat); if e != 0 { return e; } if (*fhp).fh_maxsize == NFS4_FHSIZE { (*fhp).fh_pre_change = nfsd4_change_attribute(&stat); } (*fhp).fh_pre_mtime = stat.mtime; (*fhp).fh_pre_ctime = stat.ctime; (*fhp).fh_pre_size = stat.size; (*fhp).fh_pre_saved = true; nfs_ok }
pub unsafe fn fh_fill_post_attrs(fhp: *mut svc_fh) -> __be32 { if (*fhp).fh_no_wcc { return nfs_ok; } let e = fh_getattr(fhp, &mut (*fhp).fh_post_attr); if e != 0 { return e; } (*fhp).fh_post_saved = true; if (*fhp).fh_maxsize == NFS4_FHSIZE { (*fhp).fh_post_change = nfsd4_change_attribute(&(*fhp).fh_post_attr); } nfs_ok }
pub unsafe fn fh_fill_both_attrs(fhp: *mut svc_fh) -> __be32 { let e = fh_fill_post_attrs(fhp); if e != 0 { return e; } (*fhp).fh_pre_change = (*fhp).fh_post_change; (*fhp).fh_pre_mtime = (*fhp).fh_post_attr.mtime; (*fhp).fh_pre_ctime = (*fhp).fh_post_attr.ctime; (*fhp).fh_pre_size = (*fhp).fh_post_attr.size; (*fhp).fh_pre_saved = true; nfs_ok }

pub unsafe fn fh_compose(fhp: *mut svc_fh, exp: *mut svc_export, dentry: *mut dentry, ref_fh: *mut svc_fh) -> __be32 { let inode = d_inode(dentry); let dev = (*exp_sb(exp)).s_dev; set_version_and_fsid_type(fhp, exp, ref_fh); (*fhp).fh_no_wcc = if !ref_fh.is_null() { (*ref_fh).fh_no_wcc } else { false }; (*fhp).fh_dentry = dget(dentry); (*fhp).fh_export = exp_get(exp); (*fhp).fh_handle.fh_size = key_len((*fhp).fh_handle.fh_fsid_type) + 4; (*fhp).fh_handle.fh_auth_type = 0; mk_fsid((*fhp).fh_handle.fh_fsid_type, fh_fsid(&mut (*fhp).fh_handle), dev, (*d_inode((*exp).ex_path.dentry)).i_ino, (*exp).ex_fsid, (*exp).ex_uuid); if !inode.is_null() { _fh_update(fhp, exp, dentry); } if (*fhp).fh_handle.fh_fileid_type == FILEID_INVALID { fh_put(fhp); return nfserr_stale; } 0 }

pub unsafe fn fsid_source(fhp: *const svc_fh) -> fsid_source { fsid_source_fh(&(*fhp).fh_handle, (*fhp).fh_export) }
pub unsafe fn SVCFH_fmt(fhp: *mut svc_fh) -> *mut i8 { if (*fhp).fh_handle.fh_size > 64 { return b"bad-fh\0".as_ptr() as *mut i8; } core::ptr::null_mut() }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
