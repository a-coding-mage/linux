// SPDX-License-Identifier: GPL-2.0
/* Direct Rust translation of xfs_sysfs.c. Kernel-provided types and helpers
 * are intentionally referenced as external dependencies. */

#[repr(C)]
pub struct xfs_sysfs_attr {
    pub attr: attribute,
    pub show: Option<unsafe extern "C" fn(*mut kobject, *mut i8) -> isize>,
    pub store: Option<unsafe extern "C" fn(*mut kobject, *const i8, usize) -> isize>,
}

#[inline]
unsafe fn to_attr(attr: *mut attribute) -> *mut xfs_sysfs_attr {
    container_of(attr, offset_of!(xfs_sysfs_attr, attr))
}

unsafe extern "C" fn xfs_sysfs_object_show(kobject: *mut kobject, attr: *mut attribute, buf: *mut i8) -> isize {
    let a = &*to_attr(attr);
    a.show.map(|f| f(kobject, buf)).unwrap_or(0)
}

unsafe extern "C" fn xfs_sysfs_object_store(kobject: *mut kobject, attr: *mut attribute, buf: *const i8, count: usize) -> isize {
    let a = &*to_attr(attr);
    a.store.map(|f| f(kobject, buf, count)).unwrap_or(0)
}

static XFS_SYSFS_OPS: sysfs_ops = sysfs_ops { show: Some(xfs_sysfs_object_show), store: Some(xfs_sysfs_object_store) };

#[cfg(feature = "debug")]
unsafe extern "C" fn bug_on_assert_store(k: *mut kobject, buf: *const i8, count: usize) -> isize {
    let mut val = 0i32; let ret = kstrtoint(buf, 0, &mut val); if ret != 0 { return ret as isize; }
    if val == 1 { xfs_globals.bug_on_assert = true; } else if val == 0 { xfs_globals.bug_on_assert = false; } else { return -EINVAL as isize; } count as isize
}
#[cfg(feature = "debug")]
unsafe extern "C" fn bug_on_assert_show(_: *mut kobject, buf: *mut i8) -> isize { sysfs_emit(buf, "%d\n", xfs_globals.bug_on_assert) }

#[cfg(feature = "debug")]
unsafe extern "C" fn log_recovery_delay_store(_: *mut kobject, buf: *const i8, count: usize) -> isize { let mut v=0; let r=kstrtoint(buf,0,&mut v); if r!=0{return r as isize;} if v<0||v>60{return -EINVAL as isize;} xfs_globals.log_recovery_delay=v; count as isize }
#[cfg(feature = "debug")]
unsafe extern "C" fn log_recovery_delay_show(_: *mut kobject, buf: *mut i8) -> isize { sysfs_emit(buf,"%d\n",xfs_globals.log_recovery_delay) }
#[cfg(feature = "debug")]
unsafe extern "C" fn mount_delay_store(_: *mut kobject, buf: *const i8, count: usize) -> isize { let mut v=0; let r=kstrtoint(buf,0,&mut v); if r!=0{return r as isize;} if v<0||v>60{return -EINVAL as isize;} xfs_globals.mount_delay=v; count as isize }
#[cfg(feature = "debug")]
unsafe extern "C" fn mount_delay_show(_: *mut kobject, buf: *mut i8) -> isize { sysfs_emit(buf,"%d\n",xfs_globals.mount_delay) }
#[cfg(feature = "debug")]
unsafe extern "C" fn always_cow_store(_: *mut kobject, buf: *const i8, count: usize) -> isize { let r=kstrtobool(buf,&mut xfs_globals.always_cow); if r<0{return r as isize;} count as isize }
#[cfg(feature = "debug")]
unsafe extern "C" fn always_cow_show(_: *mut kobject, buf: *mut i8) -> isize { sysfs_emit(buf,"%d\n",xfs_globals.always_cow) }
#[cfg(feature = "debug")]
unsafe extern "C" fn pwork_threads_store(_: *mut kobject, buf: *const i8, count: usize) -> isize { let mut v=0; let r=kstrtoint(buf,0,&mut v); if r!=0{return r as isize;} if v < -1 || v > num_possible_cpus(){return -EINVAL as isize;} xfs_globals.pwork_threads=v; count as isize }
#[cfg(feature = "debug")]
unsafe extern "C" fn pwork_threads_show(_: *mut kobject, buf: *mut i8) -> isize { sysfs_emit(buf,"%d\n",xfs_globals.pwork_threads) }
#[cfg(feature = "debug")]
unsafe extern "C" fn larp_store(_: *mut kobject, buf: *const i8, count: usize) -> isize { let r=kstrtobool(buf,&mut xfs_globals.larp); if r<0{return r as isize;} count as isize }
#[cfg(feature = "debug")]
unsafe extern "C" fn larp_show(_: *mut kobject, buf: *mut i8) -> isize { snprintf(buf,PAGE_SIZE,"%d\n",xfs_globals.larp) }
#[cfg(feature = "debug")]
unsafe extern "C" fn bload_leaf_slack_store(_: *mut kobject, buf: *const i8, count: usize) -> isize { let mut v=0; let r=kstrtoint(buf,0,&mut v); if r!=0{return r as isize;} xfs_globals.bload_leaf_slack=v; count as isize }
#[cfg(feature = "debug")]
unsafe extern "C" fn bload_leaf_slack_show(_: *mut kobject, buf: *mut i8) -> isize { snprintf(buf,PAGE_SIZE,"%d\n",xfs_globals.bload_leaf_slack) }
#[cfg(feature = "debug")]
unsafe extern "C" fn bload_node_slack_store(_: *mut kobject, buf: *const i8, count: usize) -> isize { let mut v=0; let r=kstrtoint(buf,0,&mut v); if r!=0{return r as isize;} xfs_globals.bload_node_slack=v; count as isize }
#[cfg(feature = "debug")]
unsafe extern "C" fn bload_node_slack_show(_: *mut kobject, buf: *mut i8) -> isize { snprintf(buf,PAGE_SIZE,"%d\n",xfs_globals.bload_node_slack) }

unsafe fn to_xstats(k: *mut kobject) -> *mut xstats { container_of(to_kobj(k), offset_of!(xstats, xs_kobj)) }
unsafe extern "C" fn stats_show(k:*mut kobject,b:*mut i8)->isize { xfs_stats_format((*to_xstats(k)).xs_stats,b) }
unsafe extern "C" fn stats_clear_store(k:*mut kobject,buf:*const i8,count:usize)->isize { let mut v=0; let r=kstrtoint(buf,0,&mut v); if r!=0{return r as isize;} if v!=1{return -EINVAL as isize;} xfs_stats_clearall((*to_xstats(k)).xs_stats); count as isize }

unsafe fn to_xlog(k:*mut kobject)->*mut xlog { container_of(to_kobj(k),offset_of!(xlog,l_kobj)) }
unsafe extern "C" fn log_head_lsn_show(k:*mut kobject,b:*mut i8)->isize { let l=to_xlog(k); let mut c=0;let mut bl=0; spin_lock(&mut (*l).l_icloglock);c=(*l).l_curr_cycle;bl=(*l).l_curr_block;spin_unlock(&mut (*l).l_icloglock);sysfs_emit(b,"%d:%d\n",c,bl) }
unsafe extern "C" fn log_tail_lsn_show(k:*mut kobject,b:*mut i8)->isize { let mut c=0;let mut bl=0;xlog_crack_atomic_lsn(&(*to_xlog(k)).l_tail_lsn,&mut c,&mut bl);sysfs_emit(b,"%d:%d\n",c,bl) }
unsafe extern "C" fn reserve_grant_head_bytes_show(k:*mut kobject,b:*mut i8)->isize { sysfs_emit(b,"%lld\n",atomic64_read(&(*to_xlog(k)).l_reserve_head.grant)) }
unsafe extern "C" fn write_grant_head_bytes_show(k:*mut kobject,b:*mut i8)->isize { sysfs_emit(b,"%lld\n",atomic64_read(&(*to_xlog(k)).l_write_head.grant)) }

unsafe fn to_error_cfg(k:*mut kobject)->*mut xfs_error_cfg { container_of(to_kobj(k),offset_of!(xfs_error_cfg,kobj)) }
unsafe fn err_to_mp(k:*mut kobject)->*mut xfs_mount { container_of(to_kobj(k),offset_of!(xfs_mount,m_error_kobj)) }
unsafe extern "C" fn max_retries_show(k:*mut kobject,b:*mut i8)->isize { let v=(*to_error_cfg(k)).max_retries;sysfs_emit(b,"%d\n",if v==XFS_ERR_RETRY_FOREVER{-1}else{v}) }
unsafe extern "C" fn max_retries_store(k:*mut kobject,buf:*const i8,count:usize)->isize { let mut v=0;let r=kstrtoint(buf,0,&mut v);if r!=0{return r as isize;}if v < -1{return -EINVAL as isize;}(*to_error_cfg(k)).max_retries=if v==-1{XFS_ERR_RETRY_FOREVER}else{v};count as isize }
unsafe extern "C" fn retry_timeout_seconds_show(k:*mut kobject,b:*mut i8)->isize { let v=(*to_error_cfg(k)).retry_timeout;let n=if v==XFS_ERR_RETRY_FOREVER{-1}else{jiffies_to_msecs(v)/MSEC_PER_SEC};sysfs_emit(b,"%d\n",n) }
unsafe extern "C" fn retry_timeout_seconds_store(k:*mut kobject,buf:*const i8,count:usize)->isize { let mut v=0;let r=kstrtoint(buf,0,&mut v);if r!=0{return r as isize;}if v < -1||v>86400{return -EINVAL as isize;}(*to_error_cfg(k)).retry_timeout=if v==-1{XFS_ERR_RETRY_FOREVER}else{secs_to_jiffies(v)};count as isize }
unsafe extern "C" fn fail_at_unmount_show(k:*mut kobject,b:*mut i8)->isize {sysfs_emit(b,"%d\n",(*err_to_mp(k)).m_fail_unmount)}
unsafe extern "C" fn fail_at_unmount_store(k:*mut kobject,buf:*const i8,count:usize)->isize {let mut v=0;let r=kstrtoint(buf,0,&mut v);if r!=0{return r as isize;}if v<0||v>1{return -EINVAL as isize;}(*err_to_mp(k)).m_fail_unmount=v;count as isize}

#[repr(C)] pub struct xfs_error_init { pub name:*mut i8, pub max_retries:i32, pub retry_timeout:i32 }
static XFS_ERROR_META_INIT:[xfs_error_init;4]=[
 xfs_error_init{name=cstr!("default"),max_retries:XFS_ERR_RETRY_FOREVER,retry_timeout:XFS_ERR_RETRY_FOREVER},
 xfs_error_init{name=cstr!("EIO"),max_retries:XFS_ERR_RETRY_FOREVER,retry_timeout:XFS_ERR_RETRY_FOREVER},
 xfs_error_init{name=cstr!("ENOSPC"),max_retries:XFS_ERR_RETRY_FOREVER,retry_timeout:XFS_ERR_RETRY_FOREVER},
 xfs_error_init{name=cstr!("ENODEV"),max_retries:0,retry_timeout:0}];

pub unsafe extern "C" fn xfs_error_sysfs_init_class(mp:*mut xfs_mount,class:i32,parent_name:*const i8,parent_kobj:*mut xfs_kobj,init:*const xfs_error_init)->i32 {
    ASSERT(class < XFS_ERR_CLASS_MAX);
    let mut error=xfs_sysfs_init(parent_kobj,&xfs_error_ktype,&mut (*mp).m_error_kobj,parent_name);
    if error!=0{return error;}
    let mut i=0;
    while i<XFS_ERR_ERRNO_MAX { let cfg=&mut (*mp).m_error_cfg[class as usize][i as usize]; let ent=&*init.add(i as usize);
        error=xfs_sysfs_init(&mut cfg.kobj,&xfs_error_cfg_ktype,parent_kobj,ent.name); if error!=0 { while i>0 {i-=1;xfs_sysfs_del(&mut (*mp).m_error_cfg[class as usize][i as usize].kobj);} xfs_sysfs_del(parent_kobj); return error; }
        cfg.max_retries=ent.max_retries; cfg.retry_timeout=if ent.retry_timeout==XFS_ERR_RETRY_FOREVER{XFS_ERR_RETRY_FOREVER}else{secs_to_jiffies(ent.retry_timeout)}; i+=1;
    } 0
}

unsafe fn zoned_to_mp(k:*mut kobject)->*mut xfs_mount {container_of(to_kobj(k),offset_of!(xfs_mount,m_zoned_kobj))}
unsafe extern "C" fn max_open_zones_show(k:*mut kobject,b:*mut i8)->isize{sysfs_emit(b,"%u\n",(*zoned_to_mp(k)).m_max_open_zones-XFS_OPEN_GC_ZONES)}
unsafe extern "C" fn nr_open_zones_show(k:*mut kobject,b:*mut i8)->isize{let z=(*zoned_to_mp(k)).m_zone_info;sysfs_emit(b,"%u\n",READ_ONCE((*z).zi_nr_open_zones))}
unsafe extern "C" fn zonegc_low_space_store(k:*mut kobject,buf:*const i8,count:usize)->isize{let m=zoned_to_mp(k);let mut v=0u32;let r=kstrtouint(buf,0,&mut v);if r!=0{return r as isize;}if v>100{return -EINVAL as isize;}if (*m).m_zonegc_low_space!=v{(*m).m_zonegc_low_space=v;xfs_zone_gc_wakeup(m);}count as isize}
unsafe extern "C" fn zonegc_low_space_show(k:*mut kobject,b:*mut i8)->isize{sysfs_emit(b,"%u\n",(*zoned_to_mp(k)).m_zonegc_low_space)}

pub unsafe extern "C" fn xfs_zoned_sysfs_init(mp:*mut xfs_mount)->i32{if !IS_ENABLED(CONFIG_XFS_RT)||!xfs_has_zoned(mp){return 0;}xfs_sysfs_init(&mut (*mp).m_zoned_kobj,&xfs_zoned_ktype,&mut (*mp).m_kobj,cstr!("zoned"))}
pub unsafe extern "C" fn xfs_zoned_sysfs_del(mp:*mut xfs_mount){if IS_ENABLED(CONFIG_XFS_RT)&&xfs_has_zoned(mp){xfs_sysfs_del(&mut (*mp).m_zoned_kobj);}}
pub unsafe extern "C" fn xfs_mount_sysfs_init(mp:*mut xfs_mount)->i32{let mut e=0;super_set_sysfs_name_id((*mp).m_super);e=xfs_sysfs_init(&mut (*mp).m_kobj,&xfs_mp_ktype,std::ptr::null_mut(),(*mp).m_super.s_id);if e!=0{return e;}e=xfs_sysfs_init(&mut (*mp).m_stats.xs_kobj,&xfs_stats_ktype,&mut (*mp).m_kobj,cstr!("stats"));if e!=0{ xfs_sysfs_del(&mut (*mp).m_kobj);return e;}e=xfs_sysfs_init(&mut (*mp).m_error_kobj,&xfs_error_ktype,&mut (*mp).m_kobj,cstr!("error"));if e!=0{xfs_sysfs_del(&mut (*mp).m_stats.xs_kobj);xfs_sysfs_del(&mut (*mp).m_kobj);}e}
pub unsafe extern "C" fn xfs_mount_sysfs_del(mp:*mut xfs_mount){for i in 0..XFS_ERR_CLASS_MAX{for j in 0..XFS_ERR_ERRNO_MAX{xfs_sysfs_del(&mut (*mp).m_error_cfg[i][j].kobj);}}xfs_sysfs_del(&mut (*mp).m_error_meta_kobj);xfs_sysfs_del(&mut (*mp).m_error_kobj);xfs_sysfs_del(&mut (*mp).m_stats.xs_kobj);xfs_sysfs_del(&mut (*mp).m_kobj)}
pub unsafe extern "C" fn xfs_error_get_cfg(mp:*mut xfs_mount,class:i32,mut error:i32)->*mut xfs_error_cfg{if error<0{error=-error;}let n=match error{EIO=>XFS_ERR_EIO,ENOSPC=>XFS_ERR_ENOSPC,ENODEV=>XFS_ERR_ENODEV,_=>XFS_ERR_DEFAULT};&mut (*mp).m_error_cfg[class as usize][n as usize]}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
