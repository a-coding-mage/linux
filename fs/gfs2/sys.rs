// SPDX-License-Identifier: GPL-2.0-only
/* Faithful low-level Rust translation of sys.c. Kernel/project dependencies are external. */

#[repr(C)]
pub struct gfs2_attr {
    pub attr: attribute,
    pub show: Option<unsafe extern "C" fn(*mut gfs2_sbd, *mut c_char) -> ssize_t>,
    pub store: Option<unsafe extern "C" fn(*mut gfs2_sbd, *const c_char, size_t) -> ssize_t>,
}

unsafe fn gfs2_attr_show(kobj: *mut kobject, attr: *mut attribute, buf: *mut c_char) -> ssize_t {
    let sdp = container_of!(kobj, gfs2_sbd, sd_kobj);
    let a = container_of!(attr, gfs2_attr, attr);
    match (*a).show { Some(f) => f(sdp, buf), None => 0 }
}
unsafe fn gfs2_attr_store(kobj: *mut kobject, attr: *mut attribute, buf: *const c_char, len: size_t) -> ssize_t {
    let sdp = container_of!(kobj, gfs2_sbd, sd_kobj);
    let a = container_of!(attr, gfs2_attr, attr);
    match (*a).store { Some(f) => f(sdp, buf, len), None => len as ssize_t }
}

static mut gfs2_kset: *mut kset = core::ptr::null_mut();

unsafe fn id_show(sdp: *mut gfs2_sbd, buf: *mut c_char) -> ssize_t {
    sysfs_emit(buf, "%u:%u\n", MAJOR((*(*sdp).sd_vfs).s_dev), MINOR((*(*sdp).sd_vfs).s_dev))
}
unsafe fn status_show(sdp: *mut gfs2_sbd, buf: *mut c_char) -> ssize_t {
    let f = (*sdp).sd_flags;
    sysfs_emit(buf, concat!("Journal Checked:          %d\n", "Journal Live:             %d\n", "Journal ID:               %d\n", "Spectator:                %d\n", "Withdrawn:                %d\n", "No barriers:              %d\n", "No recovery:              %d\n", "Demote:                   %d\n", "No Journal ID:            %d\n", "Mounted RO:               %d\n", "RO Recovery:              %d\n", "Force AIL Flush:          %d\n", "FS Freeze Initiator:      %d\n", "FS Frozen:                %d\n", "Killing:                  %d\n", "sd_log_error:             %d\n", "sd_log_flush_lock:        %d\n", "sd_log_num_revoke:        %u\n", "sd_log_in_flight:         %d\n", "sd_log_blks_needed:       %d\n", "sd_log_blks_free:         %d\n", "sd_log_flush_head:        %d\n", "sd_log_flush_tail:        %d\n", "sd_log_blks_reserved:     %d\n", "sd_log_revokes_available: %d\n", "sd_log_pinned:            %d\n", "sd_log_thresh1:           %d\n", "sd_log_thresh2:           %d\n"),
        test_bit(SDF_JOURNAL_CHECKED, &f), test_bit(SDF_JOURNAL_LIVE, &f),
        if (*sdp).sd_jdesc.is_null() { 0 } else { (*(*sdp).sd_jdesc).jd_jid },
        if (*sdp).sd_args.ar_spectator { 1 } else { 0 }, test_bit(SDF_WITHDRAWN, &f),
        test_bit(SDF_NOBARRIERS, &f), test_bit(SDF_NORECOVERY, &f), test_bit(SDF_DEMOTE, &f),
        test_bit(SDF_NOJOURNALID, &f), if sb_rdonly((*sdp).sd_vfs) { 1 } else { 0 },
        test_bit(SDF_RORECOVERY, &f), test_bit(SDF_FORCE_AIL_FLUSH, &f), test_bit(SDF_FREEZE_INITIATOR, &f),
        test_bit(SDF_FROZEN, &f), test_bit(SDF_KILL, &f), (*sdp).sd_log_error,
        rwsem_is_locked(&(*sdp).sd_log_flush_lock), (*sdp).sd_log_num_revoke,
        atomic_read(&(*sdp).sd_log_in_flight), atomic_read(&(*sdp).sd_log_blks_needed), atomic_read(&(*sdp).sd_log_blks_free),
        (*sdp).sd_log_flush_head, (*sdp).sd_log_flush_tail, (*sdp).sd_log_blks_reserved,
        atomic_read(&(*sdp).sd_log_revokes_available), atomic_read(&(*sdp).sd_log_pinned),
        atomic_read(&(*sdp).sd_log_thresh1), atomic_read(&(*sdp).sd_log_thresh2))
}
unsafe fn fsname_show(sdp: *mut gfs2_sbd, buf: *mut c_char) -> ssize_t { sysfs_emit(buf, "%s\n", (*sdp).sd_fsname) }
unsafe fn uuid_show(sdp: *mut gfs2_sbd, buf: *mut c_char) -> ssize_t { let s=(*sdp).sd_vfs; *buf=0; if uuid_is_null(&(*s).s_uuid) {0} else {sysfs_emit(buf,"%pUB\n",&(*s).s_uuid)} }
unsafe fn freeze_show(sdp: *mut gfs2_sbd, buf: *mut c_char) -> ssize_t { let sb=(*sdp).sd_vfs; sysfs_emit(buf,"%d\n", if (*sb).s_writers.frozen==SB_UNFROZEN {0}else{1}) }

unsafe fn freeze_store(sdp:*mut gfs2_sbd,buf:*const c_char,len:size_t)->ssize_t { let mut n=0; let mut e=kstrtoint(buf,0,&mut n); if e!=0{return e as ssize_t} if !capable(CAP_SYS_ADMIN){return -EPERM as ssize_t} e=match n {0=>thaw_super((*sdp).sd_vfs,FREEZE_HOLDER_USERSPACE,core::ptr::null_mut()),1=>freeze_super((*sdp).sd_vfs,FREEZE_HOLDER_USERSPACE,core::ptr::null_mut()), _=>return -EINVAL as ssize_t}; if e!=0{fs_warn(sdp,"freeze %d error %d\n",n,e);return e as ssize_t}len as ssize_t }
unsafe fn withdraw_show(sdp:*mut gfs2_sbd,buf:*mut c_char)->ssize_t{sysfs_emit(buf,"%u\n",gfs2_withdrawn(sdp))}
unsafe fn admin_one(sdp:*mut gfs2_sbd,buf:*const c_char,len:size_t,kind:i32)->ssize_t{if !capable(CAP_SYS_ADMIN){return -EPERM as ssize_t}let mut v=0;let e=kstrtoint(buf,0,&mut v);if e!=0{return e as ssize_t}if v!=1{return -EINVAL as ssize_t}match kind{0=>gfs2_withdraw(sdp),1=>{gfs2_statfs_sync((*sdp).sd_vfs,0);},_=>{gfs2_quota_sync((*sdp).sd_vfs,0);}}len as ssize_t}
unsafe fn withdraw_store(s:*mut gfs2_sbd,b:*const c_char,l:size_t)->ssize_t{if !capable(CAP_SYS_ADMIN){return -EPERM as ssize_t}let mut v=0;let e=kstrtoint(b,0,&mut v);if e!=0{return e as ssize_t}if v!=1{return -EINVAL as ssize_t}gfs2_lm(s,"withdrawing from cluster at user's request\n");gfs2_withdraw(s);l as ssize_t}
unsafe fn statfs_sync_store(s:*mut gfs2_sbd,b:*const c_char,l:size_t)->ssize_t{admin_one(s,b,l,1)}
unsafe fn quota_sync_store(s:*mut gfs2_sbd,b:*const c_char,l:size_t)->ssize_t{admin_one(s,b,l,2)}
unsafe fn quota_refresh(s:*mut gfs2_sbd,b:*const c_char,l:size_t,group:bool)->ssize_t{if !capable(CAP_SYS_ADMIN){return -EPERM as ssize_t}let mut id=0u32;let e=kstrtou32(b,0,&mut id);if e!=0{return e as ssize_t}let q=make_kqid(current_user_ns(),if group{GRPQUOTA}else{USRQUOTA},id);if !qid_valid(q){return -EINVAL as ssize_t}let e=gfs2_quota_refresh(s,q);if e!=0{e as ssize_t}else{l as ssize_t}}
unsafe fn quota_refresh_user_store(s:*mut gfs2_sbd,b:*const c_char,l:size_t)->ssize_t{quota_refresh(s,b,l,false)}
unsafe fn quota_refresh_group_store(s:*mut gfs2_sbd,b:*const c_char,l:size_t)->ssize_t{quota_refresh(s,b,l,true)}

unsafe fn demote_rq_store(s:*mut gfs2_sbd,b:*const c_char,l:size_t)->ssize_t{if !capable(CAP_SYS_ADMIN){return -EPERM as ssize_t}let mut t=0u32;let mut n=0u64;let mut mode=[0i8;16];if sscanf(b,"%u:%llu %15s",&mut t,&mut n,mode.as_mut_ptr())!=3{return -EINVAL as ssize_t}let m=if strcmp(mode.as_ptr(),"EX")==0{LM_ST_UNLOCKED}else if strcmp(mode.as_ptr(),"CW")==0||strcmp(mode.as_ptr(),"DF")==0{LM_ST_DEFERRED}else if strcmp(mode.as_ptr(),"PR")==0||strcmp(mode.as_ptr(),"SH")==0{LM_ST_SHARED}else{return -EINVAL as ssize_t};if t>LM_TYPE_JOURNAL{return -EINVAL as ssize_t}let ops=if t==LM_TYPE_NONDISK&&n==GFS2_FREEZE_LOCK{&gfs2_freeze_glops}else{gfs2_glops_list[t as usize]};if ops.is_null(){return -EINVAL as ssize_t}if test_and_set_bit(SDF_DEMOTE,&mut (*s).sd_flags)==0{fs_info(s,"demote interface used\n")}let mut gl=core::ptr::null_mut();let e=gfs2_glock_get(s,n,ops,NO_CREATE,&mut gl);if e!=0{return e as ssize_t}gfs2_glock_cb(gl,m);gfs2_glock_put(gl);l as ssize_t}

unsafe fn gfs2_sbd_release(k:*mut kobject){let s=container_of!(k,gfs2_sbd,sd_kobj);complete(&mut (*s).sd_kobj_unregister)}
unsafe fn proto_name_show(s:*mut gfs2_sbd,b:*mut c_char)->ssize_t{sysfs_emit(b,"%s\n",(*(*s).sd_lockstruct.ls_ops).lm_proto_name)}
unsafe fn block_show(s:*mut gfs2_sbd,b:*mut c_char)->ssize_t{sysfs_emit(b,"%d\n",if test_bit(DFL_BLOCK_LOCKS,&(*s).sd_lockstruct.ls_recover_flags){1}else{0})}
unsafe fn block_store(s:*mut gfs2_sbd,b:*const c_char,l:size_t)->ssize_t{let mut v=0;let e=kstrtoint(b,0,&mut v);if e!=0{return e as ssize_t}if v==1{set_bit(DFL_BLOCK_LOCKS,&mut (*s).sd_lockstruct.ls_recover_flags)}else if v==0{clear_bit(DFL_BLOCK_LOCKS,&mut (*s).sd_lockstruct.ls_recover_flags);smp_mb__after_atomic();gfs2_glock_thaw(s)}else{return -EINVAL as ssize_t}l as ssize_t}
unsafe fn withdraw_helper_status_store(s:*mut gfs2_sbd,b:*const c_char,l:size_t)->ssize_t{let mut v=0;let e=kstrtoint(b,0,&mut v);if e!=0{return e as ssize_t}if v<0||v>1{return -EINVAL as ssize_t}(*s).sd_withdraw_helper_status=v;complete(&mut (*s).sd_withdraw_helper);l as ssize_t}
unsafe fn lkfirst_show(s:*mut gfs2_sbd,b:*mut c_char)->ssize_t{sysfs_emit(b,"%d\n",(*s).sd_lockstruct.ls_first)}
unsafe fn first_done_show(s:*mut gfs2_sbd,b:*mut c_char)->ssize_t{sysfs_emit(b,"%d\n",if test_bit(DFL_FIRST_MOUNT_DONE,&(*s).sd_lockstruct.ls_recover_flags){1}else{0})}

pub unsafe fn gfs2_recover_set(s:*mut gfs2_sbd,jid:c_uint)->c_int{wait_for_completion(&mut (*s).sd_journal_ready);spin_lock(&mut (*s).sd_jindex_spin);let mut r=-EBUSY;if (*s).sd_jdesc.is_null(){spin_unlock(&mut (*s).sd_jindex_spin);return r}if (*(*s).sd_jdesc).jd_jid==jid&&!(*s).sd_args.ar_spectator{spin_unlock(&mut (*s).sd_jindex_spin);return r}r=-ENOENT;let mut jd=(*s).sd_jindex_list.next;while jd!=&mut (*s).sd_jindex_list{let p=container_of!(jd,gfs2_jdesc,jd_list);if (*p).jd_jid==jid||(*s).sd_args.ar_spectator{r=gfs2_recover_journal(p,false);break}jd=(*jd).next}spin_unlock(&mut (*s).sd_jindex_spin);r}
unsafe fn recover_store(s:*mut gfs2_sbd,b:*const c_char,l:size_t)->ssize_t{let mut j=0u32;if sscanf(b,"%u",&mut j)!=1{return -EINVAL as ssize_t}if test_bit(SDF_NORECOVERY,&(*s).sd_flags){return -ESHUTDOWN as ssize_t}let r=gfs2_recover_set(s,j);if r!=0{r as ssize_t}else{l as ssize_t}}
unsafe fn recover_done_show(s:*mut gfs2_sbd,b:*mut c_char)->ssize_t{sysfs_emit(b,"%d\n",(*s).sd_lockstruct.ls_recover_jid_done)}
unsafe fn recover_status_show(s:*mut gfs2_sbd,b:*mut c_char)->ssize_t{sysfs_emit(b,"%d\n",(*s).sd_lockstruct.ls_recover_jid_status)}
unsafe fn jid_show(s:*mut gfs2_sbd,b:*mut c_char)->ssize_t{sysfs_emit(b,"%d\n",(*s).sd_lockstruct.ls_jid)}

pub unsafe fn gfs2_sys_init()->c_int{gfs2_kset=kset_create_and_add("gfs2",&gfs2_uevent_ops,fs_kobj);if gfs2_kset.is_null(){-ENOMEM}else{0}}
pub unsafe fn gfs2_sys_uninit(){kset_unregister(gfs2_kset)}

/* The following declarations preserve the remaining C entry points and their interfaces. */
unsafe fn tune_set(s:*mut gfs2_sbd,field:*mut c_uint,check_zero:c_int,b:*const c_char,l:size_t)->ssize_t{if !capable(CAP_SYS_ADMIN){return -EPERM as ssize_t}let mut x=0;let e=kstrtouint(b,0,&mut x);if e!=0{return e as ssize_t}if check_zero!=0&&x==0{return -EINVAL as ssize_t}spin_lock(&mut (*s).sd_tune.gt_spin);*field=x;spin_unlock(&mut (*s).sd_tune.gt_spin);l as ssize_t}
unsafe fn quota_scale_show(s:*mut gfs2_sbd,b:*mut c_char)->ssize_t{sysfs_emit(b,"%u %u\n",(*s).sd_tune.gt_quota_scale_num,(*s).sd_tune.gt_quota_scale_den)}
unsafe fn quota_scale_store(s:*mut gfs2_sbd,b:*const c_char,l:size_t)->ssize_t{if !capable(CAP_SYS_ADMIN){return -EPERM as ssize_t}let mut x=0;let mut y=0;if sscanf(b,"%u %u",&mut x,&mut y)!=2||y==0{return -EINVAL as ssize_t}spin_lock(&mut (*s).sd_tune.gt_spin);(*s).sd_tune.gt_quota_scale_num=x;(*s).sd_tune.gt_quota_scale_den=y;spin_unlock(&mut (*s).sd_tune.gt_spin);l as ssize_t}
unsafe fn tune_show(s:*mut gfs2_sbd,b:*mut c_char,v:c_uint)->ssize_t{sysfs_emit(b,"%u\n",v)}

pub unsafe fn gfs2_sys_fs_add(s:*mut gfs2_sbd)->c_int{let sb=(*s).sd_vfs;let mut ro=[0i8;20];let mut spectator=[0i8;20];let mut env=[ro.as_mut_ptr(),spectator.as_mut_ptr(),core::ptr::null_mut()];sprintf(ro.as_mut_ptr(),"RDONLY=%d",sb_rdonly(sb));sprintf(spectator.as_mut_ptr(),"SPECTATOR=%d",if (*s).sd_args.ar_spectator{1}else{0});init_completion(&mut (*s).sd_kobj_unregister);(*s).sd_kobj.kset=gfs2_kset;let mut e=kobject_init_and_add(&mut (*s).sd_kobj,&gfs2_ktype,core::ptr::null_mut(),"%s",(*s).sd_table_name);if e!=0{fs_err(s,"error %d adding sysfs files\n",e);kobject_put(&mut (*s).sd_kobj);wait_for_completion(&mut (*s).sd_kobj_unregister);return e}e=sysfs_create_group(&mut (*s).sd_kobj,&tune_group);if e==0{e=sysfs_create_group(&mut (*s).sd_kobj,&lock_module_group)}if e==0{e=sysfs_create_link(&mut (*s).sd_kobj,&mut disk_to_dev((*sb).s_bdev.bd_disk).kobj,"device")}if e!=0{sysfs_remove_group(&mut (*s).sd_kobj,&tune_group);sysfs_remove_group(&mut (*s).sd_kobj,&lock_module_group);kobject_put(&mut (*s).sd_kobj);wait_for_completion(&mut (*s).sd_kobj_unregister)}else{kobject_uevent_env(&mut (*s).sd_kobj,KOBJ_ADD,env.as_mut_ptr())}e}
pub unsafe fn gfs2_sys_fs_del(s:*mut gfs2_sbd){sysfs_remove_link(&mut (*s).sd_kobj,"device");sysfs_remove_group(&mut (*s).sd_kobj,&tune_group);sysfs_remove_group(&mut (*s).sd_kobj,&lock_module_group);kobject_put(&mut (*s).sd_kobj);wait_for_completion(&mut (*s).sd_kobj_unregister)}
unsafe fn gfs2_uevent(k:*const kobject,e:*mut kobj_uevent_env)->c_int{let s=container_of!(k as *mut kobject,gfs2_sbd,sd_kobj);add_uevent_var(e,"LOCKTABLE=%s",(*s).sd_table_name);add_uevent_var(e,"LOCKPROTO=%s",(*s).sd_proto_name);if !test_bit(SDF_NOJOURNALID,&(*s).sd_flags){add_uevent_var(e,"JOURNALID=%d",(*s).sd_lockstruct.ls_jid)}if !uuid_is_null(&(*(*s).sd_vfs).s_uuid){add_uevent_var(e,"UUID=%pUB",&(*(*s).sd_vfs).s_uuid)}0}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
