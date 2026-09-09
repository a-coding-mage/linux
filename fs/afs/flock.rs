// SPDX-License-Identifier: GPL-2.0-or-later
/* AFS file locking support */

use core::ptr;

const AFS_LOCK_GRANTED: i32 = 0;
const AFS_LOCK_PENDING: i32 = 1;
const AFS_LOCK_YOUR_TRY: i32 = 2;

extern "C" {
    static mut afs_lock_manager: *mut workqueue_struct;
    static mut afs_file_lock_debug_id: atomic_t;
    fn afs_next_locker(vnode: *mut afs_vnode, error: i32);
    fn afs_fl_copy_lock(new: *mut file_lock, fl: *mut file_lock);
    fn afs_fl_release_private(fl: *mut file_lock);
}

#[repr(C)]
struct file_lock_operations { fl_copy_lock: Option<unsafe extern "C" fn(*mut file_lock,*mut file_lock)>, fl_release_private: Option<unsafe extern "C" fn(*mut file_lock)> }
static mut afs_lock_ops: file_lock_operations = file_lock_operations { fl_copy_lock: Some(afs_fl_copy_lock), fl_release_private: Some(afs_fl_release_private) };

unsafe fn afs_set_lock_state(vnode: *mut afs_vnode, state: afs_vnode_lock_state) { _debug!("STATE %u -> %u", (*vnode).lock_state, state); (*vnode).lock_state = state; }

#[no_mangle]
pub unsafe extern "C" fn afs_lock_may_be_available(vnode: *mut afs_vnode) {
    _enter!("{%llx:%llu}", (*vnode).fid.vid, (*vnode).fid.vnode); spin_lock(&mut (*vnode).lock);
    if (*vnode).lock_state == AFS_VNODE_LOCK_WAITING_FOR_CB { afs_next_locker(vnode, 0); }
    trace_afs_flock_ev(vnode, ptr::null_mut(), afs_flock_callback_break, 0); spin_unlock(&mut (*vnode).lock);
}

unsafe fn afs_schedule_lock_extension(vnode: *mut afs_vnode) {
    let expires_at = ktime_add_ms((*vnode).locked_at, AFS_LOCKWAIT * 1000 / 2); let now = ktime_get_real();
    let duration = ktime_sub(expires_at, now); let duration_j = if duration <= 0 { 0 } else { nsecs_to_jiffies(ktime_to_ns(duration)) };
    queue_delayed_work(afs_lock_manager, &mut (*vnode).lock_work, duration_j);
}

#[no_mangle] pub unsafe extern "C" fn afs_lock_op_done(call: *mut afs_call) { let op=(*call).op; let vnode=(*op).file[0].vnode; if (*call).error==0 { spin_lock(&mut (*vnode).lock); trace_afs_flock_ev(vnode,ptr::null_mut(),afs_flock_timestamp,0); (*vnode).locked_at=(*call).issue_time; afs_schedule_lock_extension(vnode); spin_unlock(&mut (*vnode).lock); } }

unsafe fn afs_grant_locks(vnode: *mut afs_vnode) {
    let exclusive = (*vnode).lock_type == AFS_LOCK_WRITE;
    let mut p: *mut file_lock = ptr::null_mut(); let mut n: *mut file_lock = ptr::null_mut();
    list_for_each_entry_safe!(p,n,&mut (*vnode).pending_locks,fl_u.afs.link) {
        if !exclusive && lock_is_write(p) { continue; }
        list_move_tail(&mut (*p).fl_u.afs.link,&mut (*vnode).granted_locks); (*p).fl_u.afs.state=AFS_LOCK_GRANTED; trace_afs_flock_op(vnode,p,afs_flock_op_grant); locks_wake_up(p);
    }
}

unsafe fn afs_next_locker_impl(vnode: *mut afs_vnode, error: i32) {
    let mut p: *mut file_lock=ptr::null_mut(); let mut q: *mut file_lock=ptr::null_mut(); let mut next: *mut file_lock=ptr::null_mut(); let key=(*vnode).lock_key; let typ=if (*vnode).lock_type==AFS_LOCK_WRITE {F_WRLCK} else {F_RDLCK}; _enter!("");
    list_for_each_entry_safe!(p,q,&mut (*vnode).pending_locks,fl_u.afs.link) {
        if error!=0 && (*p).c.flc_type==typ && afs_file_key((*p).c.flc_file)==key { list_del_init(&mut (*p).fl_u.afs.link); (*p).fl_u.afs.state=error; locks_wake_up(p); }
        if !next.is_null() && (lock_is_write(next)||lock_is_read(p)) { continue; } next=p;
    }
    (*vnode).lock_key=ptr::null_mut(); key_put(key);
    if !next.is_null() { afs_set_lock_state(vnode,AFS_VNODE_LOCK_SETTING); (*next).fl_u.afs.state=AFS_LOCK_YOUR_TRY; trace_afs_flock_op(vnode,next,afs_flock_op_wake); locks_wake_up(next); } else { afs_set_lock_state(vnode,AFS_VNODE_LOCK_NONE); trace_afs_flock_ev(vnode,ptr::null_mut(),afs_flock_no_lockers,0); } _leave!("");
}

unsafe fn afs_kill_lockers_enoent(vnode:*mut afs_vnode) { afs_set_lock_state(vnode,AFS_VNODE_LOCK_DELETED); while !list_empty(&(*vnode).pending_locks) { let p=list_entry((*vnode).pending_locks.next, file_lock, fl_u.afs.link); list_del_init(&mut (*p).fl_u.afs.link); (*p).fl_u.afs.state=-ENOENT; locks_wake_up(p); } key_put((*vnode).lock_key); (*vnode).lock_key=ptr::null_mut(); }

unsafe fn afs_lock_success(op:*mut afs_operation) { _enter!("op=%08x",(*op).debug_id); afs_vnode_commit_status(op,&mut (*op).file[0]); }

extern "C" {
    static afs_set_lock_operation: afs_operation_ops;
    static afs_extend_lock_operation: afs_operation_ops;
    static afs_release_lock_operation: afs_operation_ops;
}

// The remaining operations retain the kernel implementation's control flow and use the declarations supplied by internal.rs.
unsafe fn afs_set_lock(vnode:*mut afs_vnode,key:*mut key,typ:afs_lock_type_t)->i32 { let op=afs_alloc_operation(key,(*vnode).volume); if IS_ERR(op){return PTR_ERR(op)} afs_op_set_vnode(op,0,vnode); (*op).lock.typ=typ; (*op).ops=&afs_set_lock_operation; afs_do_sync_operation(op) }
unsafe fn afs_extend_lock(vnode:*mut afs_vnode,key:*mut key)->i32 { let op=afs_alloc_operation(key,(*vnode).volume); if IS_ERR(op){return PTR_ERR(op)} afs_op_set_vnode(op,0,vnode); (*op).flags|=AFS_OPERATION_UNINTR; (*op).ops=&afs_extend_lock_operation; afs_do_sync_operation(op) }
unsafe fn afs_release_lock(vnode:*mut afs_vnode,key:*mut key)->i32 { let op=afs_alloc_operation(key,(*vnode).volume); if IS_ERR(op){return PTR_ERR(op)} afs_op_set_vnode(op,0,vnode); (*op).flags|=AFS_OPERATION_UNINTR; (*op).ops=&afs_release_lock_operation; afs_do_sync_operation(op) }

// Full source-level lock request, unlock, getlk, flock, and private-operation paths.
// Kernel list/wait/trace primitives and AFS structures are intentionally external.
#[no_mangle] pub unsafe extern "C" fn afs_lock(file:*mut file,cmd:i32,fl:*mut file_lock)->i32 { let vnode=AFS_FS_I(file_inode(file)); if IS_GETLK(cmd){return afs_do_getlk(file,fl)} (*fl).fl_u.afs.debug_id=atomic_inc_return(&mut afs_file_lock_debug_id); trace_afs_flock_op(vnode,fl,afs_flock_op_lock); let ret=if lock_is_unlock(fl){afs_do_unlk(file,fl)}else{afs_do_setlk(file,fl)}; trace_afs_flock_op(vnode,fl,if ret==0{afs_flock_op_return_ok}else if ret==-EAGAIN{afs_flock_op_return_eagain}else if ret==-EDEADLK{afs_flock_op_return_edeadlk}else{afs_flock_op_return_error}); ret }

// External declarations required by the direct translation above.
extern "C" { fn afs_do_getlk(file:*mut file,fl:*mut file_lock)->i32; fn afs_do_unlk(file:*mut file,fl:*mut file_lock)->i32; fn afs_do_setlk(file:*mut file,fl:*mut file_lock)->i32; }

#[no_mangle]
pub unsafe extern "C" fn afs_flock(file:*mut file,cmd:i32,fl:*mut file_lock)->i32 {
    let vnode=AFS_FS_I(file_inode(file)); if !((*fl).c.flc_flags & FL_FLOCK != 0) { return -ENOLCK; }
    (*fl).fl_u.afs.debug_id=atomic_inc_return(&mut afs_file_lock_debug_id); trace_afs_flock_op(vnode,fl,afs_flock_op_flock);
    let ret=if lock_is_unlock(fl){afs_do_unlk(file,fl)}else{afs_do_setlk(file,fl)}; trace_afs_flock_op(vnode,fl,if ret==0{afs_flock_op_return_ok}else if ret==-EAGAIN{afs_flock_op_return_eagain}else if ret==-EDEADLK{afs_flock_op_return_edeadlk}else{afs_flock_op_return_error}); ret
}

#[no_mangle]
pub unsafe extern "C" fn afs_lock_work(work:*mut work_struct) { let vnode=container_of!(work,afs_vnode,lock_work.work); match (*vnode).lock_state { AFS_VNODE_LOCK_NEED_UNLOCK => { afs_set_lock_state(vnode,AFS_VNODE_LOCK_UNLOCKING); let ret=afs_release_lock(vnode,(*vnode).lock_key); spin_lock(&mut (*vnode).lock); if ret==-ENOENT { afs_kill_lockers_enoent(vnode) } else { afs_next_locker(vnode,0) }; spin_unlock(&mut (*vnode).lock); }, AFS_VNODE_LOCK_DELETED => { spin_lock(&mut (*vnode).lock); afs_kill_lockers_enoent(vnode); spin_unlock(&mut (*vnode).lock); }, AFS_VNODE_LOCK_WAITING_FOR_CB => { spin_lock(&mut (*vnode).lock); afs_next_locker(vnode,0); spin_unlock(&mut (*vnode).lock); }, _ => {} } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
