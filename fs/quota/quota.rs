// SPDX-License-Identifier: GPL-2.0
/*
 * Quota code necessary even when VFS quota support is not compiled
 * into the kernel.  The interesting stuff is over in dquot.c, here
 * we have symbols for initial quotactl(2) handling, the sysctl(2)
 * variables, etc - things needed even when quota support disabled.
 */

// Kernel dependencies supplied by the surrounding translation unit/build.

unsafe fn check_quotactl_permission(sb: *mut super_block, typ: i32, cmd: i32, id: qid_t) -> i32 {
    match cmd {
        Q_GETFMT | Q_SYNC | Q_GETINFO | Q_XGETQSTAT | Q_XGETQSTATV | Q_XQUOTASYNC => {}
        Q_GETQUOTA | Q_XGETQUOTA => {
            if (typ == USRQUOTA && uid_eq(current_euid(), make_kuid(current_user_ns(), id))) ||
               (typ == GRPQUOTA && in_egroup_p(make_kgid(current_user_ns(), id))) { }
            else if !capable(CAP_SYS_ADMIN) { return -EPERM; }
        }
        _ => { if !capable(CAP_SYS_ADMIN) { return -EPERM; } }
    }
    security_quotactl(cmd, typ, id, sb)
}

unsafe fn quota_sync_one(sb: *mut super_block, arg: *mut core::ffi::c_void) {
    let typ = *(arg as *mut i32);
    if !(*sb).s_qcop.is_null() && !(*(*sb).s_qcop).quota_sync.is_none() &&
       ((*sb).s_quota_types & (1u32 << typ)) != 0 {
        ((*(*sb).s_qcop).quota_sync.unwrap())(sb, typ);
    }
}

unsafe fn quota_sync_all(typ: i32) -> i32 {
    let ret = security_quotactl(Q_SYNC, typ, 0, core::ptr::null_mut());
    if ret == 0 { iterate_supers(Some(quota_sync_one), &typ as *const _ as *mut _); }
    ret
}

pub unsafe fn qtype_enforce_flag(typ: i32) -> u32 {
    match typ { USRQUOTA => FS_QUOTA_UDQ_ENFD, GRPQUOTA => FS_QUOTA_GDQ_ENFD,
        PRJQUOTA => FS_QUOTA_PDQ_ENFD, _ => 0 }
}

unsafe fn quota_quotaon(sb: *mut super_block, typ: i32, id: qid_t, path: *const path) -> i32 {
    if (*(*sb).s_qcop).quota_on.is_none() && (*(*sb).s_qcop).quota_enable.is_none() { return -ENOSYS; }
    if let Some(f) = (*(*sb).s_qcop).quota_enable { return f(sb, qtype_enforce_flag(typ)); }
    if IS_ERR(path) { return PTR_ERR(path); }
    (*(*sb).s_qcop).quota_on.unwrap()(sb, typ, id, path)
}

unsafe fn quota_quotaoff(sb: *mut super_block, typ: i32) -> i32 {
    if (*(*sb).s_qcop).quota_off.is_none() && (*(*sb).s_qcop).quota_disable.is_none() { return -ENOSYS; }
    if let Some(f) = (*(*sb).s_qcop).quota_disable { return f(sb, qtype_enforce_flag(typ)); }
    (*(*sb).s_qcop).quota_off.unwrap()(sb, typ)
}

unsafe fn quota_getfmt(sb: *mut super_block, typ: i32, addr: *mut core::ffi::c_void) -> i32 {
    if !sb_has_quota_active(sb, typ) { return -ESRCH; }
    let fmt = (*sb_dqopt(sb)).info[typ as usize].dqi_format.as_ref().unwrap().qf_fmt_id;
    if copy_to_user(addr, &fmt as *const _ as *const _, core::mem::size_of_val(&fmt)) != 0 { return -EFAULT; }
    0
}

unsafe fn qbtos(blocks: qsize_t) -> qsize_t { blocks << QIF_DQBLKSIZE_BITS }
unsafe fn stoqb(space: qsize_t) -> qsize_t { (space + QIF_DQBLKSIZE - 1) >> QIF_DQBLKSIZE_BITS }

unsafe fn copy_to_if_dqblk(dst: *mut if_dqblk, src: *mut qc_dqblk) {
    core::ptr::write_bytes(dst, 0, 1);
    (*dst).dqb_bhardlimit = stoqb((*src).d_spc_hardlimit);
    (*dst).dqb_bsoftlimit = stoqb((*src).d_spc_softlimit);
    (*dst).dqb_curspace = (*src).d_space;
    (*dst).dqb_ihardlimit = (*src).d_ino_hardlimit;
    (*dst).dqb_isoftlimit = (*src).d_ino_softlimit;
    (*dst).dqb_curinodes = (*src).d_ino_count;
    (*dst).dqb_btime = (*src).d_spc_timer;
    (*dst).dqb_itime = (*src).d_ino_timer;
    (*dst).dqb_valid = QIF_ALL;
}

// The remaining quota operation translations retain the C ABI data structures and
// helper calls supplied by the kernel headers.
unsafe fn quota_getinfo(sb: *mut super_block, typ: i32, addr: *mut core::ffi::c_void) -> i32 {
    let mut state: qc_state = core::mem::zeroed();
    if (*(*sb).s_qcop).get_state.is_none() { return -ENOSYS; }
    let ret = (*(*sb).s_qcop).get_state.unwrap()(sb, &mut state); if ret != 0 { return ret; }
    let t = &state.s_state[typ as usize]; if t.flags & QCI_ACCT_ENABLED == 0 { return -ESRCH; }
    let mut uinfo: if_dqinfo = core::mem::zeroed();
    uinfo.dqi_bgrace = t.spc_timelimit; uinfo.dqi_igrace = t.ino_timelimit;
    if t.flags & QCI_SYSFILE != 0 { uinfo.dqi_flags |= DQF_SYS_FILE; }
    if t.flags & QCI_ROOT_SQUASH != 0 { uinfo.dqi_flags |= DQF_ROOT_SQUASH; }
    uinfo.dqi_valid = IIF_ALL;
    if copy_to_user(addr, &uinfo as *const _ as *const _, core::mem::size_of_val(&uinfo)) != 0 { -EFAULT } else { 0 }
}

unsafe fn copy_from_if_dqblk(dst: *mut qc_dqblk, src: *mut if_dqblk) {
    (*dst).d_spc_hardlimit=qbtos((*src).dqb_bhardlimit); (*dst).d_spc_softlimit=qbtos((*src).dqb_bsoftlimit);
    (*dst).d_space=(*src).dqb_curspace; (*dst).d_ino_hardlimit=(*src).dqb_ihardlimit;
    (*dst).d_ino_softlimit=(*src).dqb_isoftlimit; (*dst).d_ino_count=(*src).dqb_curinodes;
    (*dst).d_spc_timer=(*src).dqb_btime; (*dst).d_ino_timer=(*src).dqb_itime; (*dst).d_fieldmask=0;
    if (*src).dqb_valid & QIF_BLIMITS != 0 { (*dst).d_fieldmask |= QC_SPC_SOFT|QC_SPC_HARD; }
    if (*src).dqb_valid & QIF_SPACE != 0 { (*dst).d_fieldmask |= QC_SPACE; }
    if (*src).dqb_valid & QIF_ILIMITS != 0 { (*dst).d_fieldmask |= QC_INO_SOFT|QC_INO_HARD; }
    if (*src).dqb_valid & QIF_INODES != 0 { (*dst).d_fieldmask |= QC_INO_COUNT; }
    if (*src).dqb_valid & QIF_BTIME != 0 { (*dst).d_fieldmask |= QC_SPC_TIMER; }
    if (*src).dqb_valid & QIF_ITIME != 0 { (*dst).d_fieldmask |= QC_INO_TIMER; }
}

unsafe fn quota_setinfo(sb:*mut super_block,typ:i32,addr:*mut core::ffi::c_void)->i32 {
    let mut info:if_dqinfo=core::mem::zeroed(); let mut qi:qc_info=core::mem::zeroed();
    if copy_from_user(&mut info as *mut _ as *mut _,addr,core::mem::size_of_val(&info))!=0{return -EFAULT;}
    if (*(*sb).s_qcop).set_info.is_none(){return -ENOSYS;} if info.dqi_valid & !(IIF_FLAGS|IIF_BGRACE|IIF_IGRACE)!=0{return -EINVAL;}
    if info.dqi_valid&IIF_FLAGS!=0 {if info.dqi_flags&!DQF_SETINFO_MASK!=0{return -EINVAL;} if info.dqi_flags&DQF_ROOT_SQUASH!=0{qi.i_flags|=QCI_ROOT_SQUASH;}qi.i_fieldmask|=QC_FLAGS;}
    if info.dqi_valid&IIF_BGRACE!=0{qi.i_spc_timelimit=info.dqi_bgrace;qi.i_fieldmask|=QC_SPC_TIMER;} if info.dqi_valid&IIF_IGRACE!=0{qi.i_ino_timelimit=info.dqi_igrace;qi.i_fieldmask|=QC_INO_TIMER;}
    (*(*sb).s_qcop).set_info.unwrap()(sb,typ,&mut qi)
}

unsafe fn quota_enable(sb:*mut super_block,addr:*mut core::ffi::c_void)->i32{let mut f:u32=0;if copy_from_user(&mut f as *mut _ as *mut _,addr,4)!=0{return -EFAULT;}match (*(*sb).s_qcop).quota_enable{Some(x)=>x(sb,f),None=>-ENOSYS}}
unsafe fn quota_disable(sb:*mut super_block,addr:*mut core::ffi::c_void)->i32{let mut f:u32=0;if copy_from_user(&mut f as *mut _ as *mut _,addr,4)!=0{return -EFAULT;}match (*(*sb).s_qcop).quota_disable{Some(x)=>x(sb,f),None=>-ENOSYS}}

unsafe fn quota_state_to_flags(s:*mut qc_state)->i32 { let mut f=0; for (i,a,b) in [(USRQUOTA,FS_QUOTA_UDQ_ACCT,FS_QUOTA_UDQ_ENFD),(GRPQUOTA,FS_QUOTA_GDQ_ACCT,FS_QUOTA_GDQ_ENFD),(PRJQUOTA,FS_QUOTA_PDQ_ACCT,FS_QUOTA_PDQ_ENFD)] {if (*s).s_state[i].flags&QCI_ACCT_ENABLED!=0{f|=a;}if (*s).s_state[i].flags&QCI_LIMITS_ENFORCED!=0{f|=b;}} f }

unsafe fn quota_getquota(sb:*mut super_block,typ:i32,id:qid_t,addr:*mut core::ffi::c_void)->i32{let mut q=make_kqid(current_user_ns(),typ,id);let mut d:qc_dqblk=core::mem::zeroed();let mut u:if_dqblk=core::mem::zeroed();if (*(*sb).s_qcop).get_dqblk.is_none(){return -ENOSYS;}if !qid_has_mapping((*sb).s_user_ns,q){return -EINVAL;}let r=(*(*sb).s_qcop).get_dqblk.unwrap()(sb,q,&mut d);if r!=0{return r;}copy_to_if_dqblk(&mut u,&mut d);if copy_to_user(addr,&u as *const _ as *const _,core::mem::size_of_val(&u))!=0{-EFAULT}else{0}}
unsafe fn quota_setquota(sb:*mut super_block,typ:i32,id:qid_t,addr:*mut core::ffi::c_void)->i32{let mut u:if_dqblk=core::mem::zeroed();let mut d:qc_dqblk=core::mem::zeroed();if copy_from_user(&mut u as *mut _ as *mut _,addr,core::mem::size_of_val(&u))!=0{return -EFAULT;}if (*(*sb).s_qcop).set_dqblk.is_none(){return -ENOSYS;}let q=make_kqid(current_user_ns(),typ,id);if !qid_has_mapping((*sb).s_user_ns,q){return -EINVAL;}copy_from_if_dqblk(&mut d,&mut u);(*(*sb).s_qcop).set_dqblk.unwrap()(sb,q,&mut d)}

unsafe fn quota_getxstate(sb:*mut super_block,_typ:i32,addr:*mut core::ffi::c_void)->i32{if (*(*sb).s_qcop).get_state.is_none(){return -ENOSYS;}let mut s:fs_quota_stat=core::mem::zeroed();let mut q:qc_state=core::mem::zeroed();let r=(*(*sb).s_qcop).get_state.unwrap()(sb,&mut q);if r!=0{return r;}s.qs_version=FS_QSTAT_VERSION;s.qs_flags=quota_state_to_flags(&mut q);if s.qs_flags==0{return -ENOSYS;}if copy_to_user(addr,&s as *const _ as *const _,core::mem::size_of_val(&s))!=0{-EFAULT}else{0}}

unsafe fn quota_getxstatev(sb:*mut super_block,typ:i32,addr:*mut core::ffi::c_void)->i32{let mut v:fs_quota_statv=core::mem::zeroed();if copy_from_user(&mut v as *mut _ as *mut _,addr,1)!=0{return -EFAULT;}if v.qs_version!=FS_QSTATV_VERSION1{return -EINVAL;}let r=(*(*sb).s_qcop).get_state.unwrap()(sb,&mut *(core::ptr::addr_of_mut!(v) as *mut qc_state));let _=typ;if r!=0{return r;}if copy_to_user(addr,&v as *const _ as *const _,core::mem::size_of_val(&v))!=0{-EFAULT}else{0}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
