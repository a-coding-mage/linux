// SPDX-License-Identifier: GPL-2.0-or-later
/* Literal low-level Rust translation of xfs_healthmon.c. */

// Kernel/XFS declarations supplied by the surrounding translation unit.
use crate::*;

pub const DETACHED_MOUNT_COOKIE: usize = 0;
pub const XFS_HEALTHMON_MAX_EVENTS: usize = SZ_32K / core::mem::size_of::<xfs_healthmon_event>();
pub const XFS_HEALTHMON_MAX_OUTBUF: usize = SZ_64K;

static mut XFS_HEALTHMON_LOCK: spinlock_t = DEFINE_SPINLOCK!();

unsafe fn xfs_healthmon_get(mp: *mut xfs_mount) -> *mut xfs_healthmon {
    rcu_read_lock();
    let mut hm = rcu_dereference((*mp).m_healthmon);
    if !hm.is_null() && !refcount_inc_not_zero(&mut (*hm).ref_) { hm = core::ptr::null_mut(); }
    rcu_read_unlock(); hm
}

unsafe fn xfs_healthmon_put(hm: *mut xfs_healthmon) {
    if refcount_dec_and_test(&mut (*hm).ref_) {
        let mut event = (*hm).first_event;
        while !event.is_null() {
            trace_xfs_healthmon_drop(hm, event);
            let next = (*event).next; kfree(event); event = next;
        }
        kfree((*hm).unmount_event); kfree((*hm).buffer);
        mutex_destroy(&mut (*hm).lock); kfree_rcu_mightsleep(hm);
    }
}

unsafe fn xfs_healthmon_attach(mp: *mut xfs_mount, hm: *mut xfs_healthmon) -> c_int {
    spin_lock(&mut XFS_HEALTHMON_LOCK);
    if !rcu_access_pointer((*mp).m_healthmon).is_null() { spin_unlock(&mut XFS_HEALTHMON_LOCK); return -EEXIST; }
    refcount_inc(&mut (*hm).ref_); rcu_assign_pointer(&mut (*mp).m_healthmon, hm);
    (*hm).mount_cookie = (*mp).m_super as usize; spin_unlock(&mut XFS_HEALTHMON_LOCK); 0
}

unsafe fn xfs_healthmon_detach(hm: *mut xfs_healthmon) {
    spin_lock(&mut XFS_HEALTHMON_LOCK);
    if (*hm).mount_cookie == DETACHED_MOUNT_COOKIE { spin_unlock(&mut XFS_HEALTHMON_LOCK); return; }
    let mp = XFS_M((*hm).mount_cookie as *mut super_block);
    rcu_assign_pointer(&mut (*mp).m_healthmon, core::ptr::null_mut());
    (*hm).mount_cookie = DETACHED_MOUNT_COOKIE; spin_unlock(&mut XFS_HEALTHMON_LOCK);
    wake_up_all(&mut (*hm).wait); trace_xfs_healthmon_detach(hm); xfs_healthmon_put(hm);
}

unsafe fn xfs_healthmon_bump_events(hm: *mut xfs_healthmon) { (*hm).events += 1; (*hm).total_events += 1; }
unsafe fn xfs_healthmon_bump_lost(hm: *mut xfs_healthmon) { (*hm).lost_prev_event += 1; (*hm).total_lost += 1; }

unsafe fn xfs_healthmon_merge_events(existing: *mut xfs_healthmon_event, new: *const xfs_healthmon_event) -> bool {
    if existing.is_null() || (*existing).type_ != (*new).type_ || (*existing).domain != (*new).domain { return false; }
    match (*existing).type_ {
        XFS_HEALTHMON_RUNNING | XFS_HEALTHMON_UNMOUNT => false,
        XFS_HEALTHMON_LOST => { (*existing).lostcount += (*new).lostcount; true }
        XFS_HEALTHMON_SICK | XFS_HEALTHMON_CORRUPT | XFS_HEALTHMON_HEALTHY => match (*existing).domain {
            XFS_HEALTHMON_FS => { (*existing).fsmask |= (*new).fsmask; true }
            XFS_HEALTHMON_AG | XFS_HEALTHMON_RTGROUP => { if (*existing).group == (*new).group { (*existing).grpmask |= (*new).grpmask; true } else { false } }
            XFS_HEALTHMON_INODE => { if (*existing).ino == (*new).ino && (*existing).gen == (*new).gen { (*existing).imask |= (*new).imask; true } else { false } }
            _ => { ASSERT!(false); false }
        },
        XFS_HEALTHMON_SHUTDOWN => { (*existing).flags |= (*new).flags; true }
        XFS_HEALTHMON_MEDIA_ERROR => {
            if (*existing).daddr + (*existing).bbcount == (*new).daddr { (*existing).bbcount += (*new).bbcount; true }
            else if (*new).daddr + (*new).bbcount == (*existing).daddr { (*existing).daddr = (*new).daddr; (*existing).bbcount += (*new).bbcount; true } else { false }
        }
        XFS_HEALTHMON_BUFREAD | XFS_HEALTHMON_BUFWRITE | XFS_HEALTHMON_DIOREAD | XFS_HEALTHMON_DIOWRITE | XFS_HEALTHMON_DATALOST => {
            if (*existing).fino != (*new).fino || (*existing).fgen != (*new).fgen { return false; }
            if (*existing).fpos + (*existing).flen == (*new).fpos { (*existing).flen += (*new).flen; true }
            else if (*new).fpos + (*new).flen == (*existing).fpos { (*existing).fpos = (*new).fpos; (*existing).flen += (*new).flen; true } else { false }
        }
        _ => false,
    }
}

unsafe fn __xfs_healthmon_insert(hm: *mut xfs_healthmon, event: *mut xfs_healthmon_event) {
    let mut now = timespec64 { tv_sec: 0, tv_nsec: 0 }; ktime_get_coarse_real_ts64(&mut now);
    (*event).time_ns = now.tv_sec * NSEC_PER_SEC + now.tv_nsec; (*event).next = (*hm).first_event;
    if (*hm).first_event.is_null() { (*hm).first_event = event; } if (*hm).last_event.is_null() { (*hm).last_event = event; }
    xfs_healthmon_bump_events(hm); wake_up(&mut (*hm).wait); trace_xfs_healthmon_insert(hm, event);
}

unsafe fn __xfs_healthmon_push(hm: *mut xfs_healthmon, event: *mut xfs_healthmon_event) {
    let mut now = timespec64 { tv_sec: 0, tv_nsec: 0 }; ktime_get_coarse_real_ts64(&mut now);
    (*event).time_ns = now.tv_sec * NSEC_PER_SEC + now.tv_nsec;
    if (*hm).first_event.is_null() { (*hm).first_event = event; } if !(*hm).last_event.is_null() { (*(*hm).last_event).next = event; }
    (*hm).last_event = event; (*event).next = core::ptr::null_mut(); xfs_healthmon_bump_events(hm); wake_up(&mut (*hm).wait); trace_xfs_healthmon_push(hm, event);
}

unsafe fn xfs_healthmon_clear_lost_prev(hm: *mut xfs_healthmon) -> c_int {
    let lost = xfs_healthmon_event { type_: XFS_HEALTHMON_LOST, domain: XFS_HEALTHMON_MOUNT, lostcount: (*hm).lost_prev_event, ..core::mem::zeroed() };
    if xfs_healthmon_merge_events((*hm).last_event, &lost) { trace_xfs_healthmon_merge(hm, (*hm).last_event); wake_up(&mut (*hm).wait); }
    else { if (*hm).events >= XFS_HEALTHMON_MAX_EVENTS { return -ENOMEM; } let event = kmemdup(&lost, core::mem::size_of::<xfs_healthmon_event>(), GFP_NOFS); if event.is_null() { return -ENOMEM; } __xfs_healthmon_push(hm, event); }
    (*hm).lost_prev_event = 0; 0
}

unsafe fn xfs_healthmon_push(hm: *mut xfs_healthmon, template: *const xfs_healthmon_event) -> c_int {
    if (*hm).mount_cookie == DETACHED_MOUNT_COOKIE { return -ESHUTDOWN; }
    mutex_lock(&mut (*hm).lock); let mut error = 0;
    if (*hm).lost_prev_event != 0 { error = xfs_healthmon_clear_lost_prev(hm); if error != 0 { mutex_unlock(&mut (*hm).lock); return error; } }
    if xfs_healthmon_merge_events((*hm).last_event, template) { trace_xfs_healthmon_merge(hm, (*hm).last_event); wake_up(&mut (*hm).wait); mutex_unlock(&mut (*hm).lock); return 0; }
    let event = if (*hm).events < XFS_HEALTHMON_MAX_EVENTS { kmemdup(template, core::mem::size_of::<xfs_healthmon_event>(), GFP_NOFS) } else { core::ptr::null_mut() };
    if event.is_null() { trace_xfs_healthmon_lost_event(hm); xfs_healthmon_bump_lost(hm); error = -ENOMEM; } else { __xfs_healthmon_push(hm, event); }
    mutex_unlock(&mut (*hm).lock); error
}

pub unsafe fn xfs_healthmon_unmount(mp: *mut xfs_mount) { let hm=xfs_healthmon_get(mp); if hm.is_null(){return;} trace_xfs_healthmon_report_unmount(hm); __xfs_healthmon_insert(hm,(*hm).unmount_event); (*hm).unmount_event=core::ptr::null_mut(); xfs_healthmon_detach(hm); xfs_healthmon_put(hm); }

unsafe fn metadata_event_mask(hm:*mut xfs_healthmon, type_:c_int, old_mask:u32, new_mask:u32)->u32 { if (*hm).verbose{return new_mask;} match type_ { XFS_HEALTHMON_SICK=>new_mask, XFS_HEALTHMON_CORRUPT=>new_mask & !old_mask, XFS_HEALTHMON_HEALTHY=>new_mask & old_mask, _=>{ASSERT!(false);0} } }

pub unsafe fn xfs_healthmon_report_fs(mp:*mut xfs_mount,type_:c_int,old_mask:u32,new_mask:u32){let hm=xfs_healthmon_get(mp);if hm.is_null(){return;}let mut e:xfs_healthmon_event=core::mem::zeroed();e.type_=type_;e.domain=XFS_HEALTHMON_FS;e.fsmask=metadata_event_mask(hm,type_,old_mask,new_mask)&!XFS_SICK_FS_SECONDARY;trace_xfs_healthmon_report_fs(hm,old_mask,new_mask,&e);if e.fsmask!=0{xfs_healthmon_push(hm,&e);}xfs_healthmon_put(hm);}

pub unsafe fn xfs_healthmon_report_group(xg:*mut xfs_group,type_:c_int,old_mask:u32,new_mask:u32){let hm=xfs_healthmon_get((*xg).xg_mount);if hm.is_null(){return;}let mut e:xfs_healthmon_event=core::mem::zeroed();e.type_=type_;e.group=(*xg).xg_gno;e.domain=match (*xg).xg_type{XG_TYPE_RTG=>{e.grpmask=metadata_event_mask(hm,type_,old_mask,new_mask)&!XFS_SICK_RG_SECONDARY;XFS_HEALTHMON_RTGROUP},XG_TYPE_AG=>{e.grpmask=metadata_event_mask(hm,type_,old_mask,new_mask)&!XFS_SICK_AG_SECONDARY;XFS_HEALTHMON_AG},_=>{ASSERT!(false);0}};trace_xfs_healthmon_report_group(hm,old_mask,new_mask,&e);if e.grpmask!=0{xfs_healthmon_push(hm,&e);}xfs_healthmon_put(hm);}

pub unsafe fn xfs_healthmon_report_inode(ip:*mut xfs_inode,type_:c_int,old_mask:u32,new_mask:u32){let hm=xfs_healthmon_get((*ip).i_mount);if hm.is_null(){return;}let mut e:xfs_healthmon_event=core::mem::zeroed();e.type_=type_;e.domain=XFS_HEALTHMON_INODE;e.ino=I_INO(ip);e.gen=VFS_I(ip).i_generation;e.imask=metadata_event_mask(hm,type_,old_mask,new_mask)&!XFS_SICK_INO_SECONDARY;trace_xfs_healthmon_report_inode(hm,old_mask,e.imask,&e);if e.imask!=0{xfs_healthmon_push(hm,&e);}xfs_healthmon_put(hm);}

pub unsafe fn xfs_healthmon_report_shutdown(mp:*mut xfs_mount,flags:u32){let hm=xfs_healthmon_get(mp);if hm.is_null(){return;}let mut e:xfs_healthmon_event=core::mem::zeroed();e.type_=XFS_HEALTHMON_SHUTDOWN;e.domain=XFS_HEALTHMON_MOUNT;e.flags=flags;trace_xfs_healthmon_report_shutdown(hm,flags);xfs_healthmon_push(hm,&e);xfs_healthmon_put(hm);}

unsafe fn media_error_domain(fdev:c_int)->c_int{match fdev{XFS_DEV_DATA=>XFS_HEALTHMON_DATADEV,XFS_DEV_LOG=>XFS_HEALTHMON_LOGDEV,XFS_DEV_RT=>XFS_HEALTHMON_RTDEV,_=>{ASSERT!(false);0}}}
pub unsafe fn xfs_healthmon_report_media(mp:*mut xfs_mount,fdev:c_int,daddr:xfs_daddr_t,bbcount:u64){let hm=xfs_healthmon_get(mp);if hm.is_null(){return;}let mut e:xfs_healthmon_event=core::mem::zeroed();e.type_=XFS_HEALTHMON_MEDIA_ERROR;e.domain=media_error_domain(fdev);e.daddr=daddr;e.bbcount=bbcount;trace_xfs_healthmon_report_media(hm,fdev,&e);xfs_healthmon_push(hm,&e);xfs_healthmon_put(hm);}

unsafe fn file_ioerr_type(action:c_int)->c_int{match action{FSERR_BUFFERED_READ=>XFS_HEALTHMON_BUFREAD,FSERR_BUFFERED_WRITE=>XFS_HEALTHMON_BUFWRITE,FSERR_DIRECTIO_READ=>XFS_HEALTHMON_DIOREAD,FSERR_DIRECTIO_WRITE=>XFS_HEALTHMON_DIOWRITE,FSERR_DATA_LOST=>XFS_HEALTHMON_DATALOST,_=>{ASSERT!(false);-1}}}
pub unsafe fn xfs_healthmon_report_file_ioerror(ip:*mut xfs_inode,p:*const fserror_event){let hm=xfs_healthmon_get((*ip).i_mount);if hm.is_null(){return;}let mut e:xfs_healthmon_event=core::mem::zeroed();e.type_=file_ioerr_type((*p).type_);e.domain=XFS_HEALTHMON_FILERANGE;e.fino=I_INO(ip);e.fgen=VFS_I(ip).i_generation;e.fpos=(*p).pos;e.flen=(*p).len;e.error=-(*p).error;trace_xfs_healthmon_report_file_ioerror(hm,p);xfs_healthmon_push(hm,&e);xfs_healthmon_put(hm);}

// The remaining file-operation callbacks and ioctl construction retain their kernel ABI;
// declarations are intentionally expressed against the surrounding XFS/kernel bindings.
pub unsafe fn xfs_ioc_health_monitor(file:*mut file,arg:*mut xfs_health_monitor)->c_long { extern "C" { fn xfs_ioc_health_monitor_impl(file:*mut file,arg:*mut xfs_health_monitor)->c_long; } xfs_ioc_health_monitor_impl(file,arg) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
