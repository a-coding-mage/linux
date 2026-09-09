// SPDX-License-Identifier: GPL-2.0-only
/*
 * Fence mechanism for dma-buf and to allow for asynchronous dma access
 *
 * Copyright (C) 2012 Canonical Ltd
 * Copyright (C) 2012 Texas Instruments
 *
 * Authors:
 * Rob Clark <robdclark@gmail.com>
 * Maarten Lankhorst <maarten.lankhorst@canonical.com>
 */

// C includes and tracepoint declarations are supplied by the kernel bindings.

static mut DMA_FENCE_STUB: dma_fence = unsafe { core::mem::zeroed() };
static mut DMA_FENCE_CONTEXT_COUNTER: atomic64_t = ATOMIC64_INIT(1);

unsafe extern "C" fn dma_fence_stub_get_name(_fence: *mut dma_fence) -> *const core::ffi::c_char {
    b"stub\0".as_ptr() as *const core::ffi::c_char
}

static DMA_FENCE_STUB_OPS: dma_fence_ops = dma_fence_ops {
    get_driver_name: Some(dma_fence_stub_get_name),
    get_timeline_name: Some(dma_fence_stub_get_name),
    ..unsafe { core::mem::zeroed() }
};

unsafe extern "C" fn dma_fence_init_stub() -> i32 {
    dma_fence_init(&raw mut DMA_FENCE_STUB, &raw const DMA_FENCE_STUB_OPS, core::ptr::null_mut(), 0, 0);
    set_bit(DMA_FENCE_FLAG_ENABLE_SIGNAL_BIT, &raw mut (*(&raw mut DMA_FENCE_STUB)).flags);
    dma_fence_signal(&raw mut DMA_FENCE_STUB);
    0
}

// subsys_initcall(dma_fence_init_stub);

/// Return a signaled fence.
pub unsafe extern "C" fn dma_fence_get_stub() -> *mut dma_fence {
    dma_fence_get(&raw mut DMA_FENCE_STUB)
}

/// Return a private, signaled fence.
pub unsafe extern "C" fn dma_fence_allocate_private_stub(timestamp: ktime_t) -> *mut dma_fence {
    let fence = kzalloc_obj::<dma_fence>();
    if fence.is_null() { return core::ptr::null_mut(); }
    dma_fence_init(fence, &raw const DMA_FENCE_STUB_OPS, core::ptr::null_mut(), 0, 0);
    set_bit(DMA_FENCE_FLAG_ENABLE_SIGNAL_BIT, &raw mut (*fence).flags);
    dma_fence_signal_timestamp(fence, timestamp);
    fence
}

pub unsafe extern "C" fn dma_fence_context_alloc(num: u32) -> u64 {
    WARN_ON(!num);
    atomic64_fetch_add(num as i64, &raw mut DMA_FENCE_CONTEXT_COUNTER)
}

#[cfg(CONFIG_LOCKDEP)]
static mut DMA_FENCE_LOCKDEP_MAP: lockdep_map = lockdep_map { name: b"dma_fence_map\0".as_ptr() as *const _ };

#[cfg(CONFIG_LOCKDEP)]
pub unsafe extern "C" fn dma_fence_begin_signalling() -> bool {
    if lock_is_held_type(&raw mut DMA_FENCE_LOCKDEP_MAP, 1) { return true; }
    if in_atomic() { return true; }
    lock_acquire(&raw mut DMA_FENCE_LOCKDEP_MAP, 0, 1, 1, 1, core::ptr::null_mut(), _RET_IP_);
    false
}

#[cfg(CONFIG_LOCKDEP)]
pub unsafe extern "C" fn dma_fence_end_signalling(cookie: bool) {
    if !cookie { lock_release(&raw mut DMA_FENCE_LOCKDEP_MAP, _RET_IP_); }
}

#[cfg(CONFIG_LOCKDEP)]
pub unsafe extern "C" fn __dma_fence_might_wait() {
    let tmp = lock_is_held_type(&raw mut DMA_FENCE_LOCKDEP_MAP, 1);
    if tmp { lock_release(&raw mut DMA_FENCE_LOCKDEP_MAP, _THIS_IP_); }
    lock_map_acquire(&raw mut DMA_FENCE_LOCKDEP_MAP);
    lock_map_release(&raw mut DMA_FENCE_LOCKDEP_MAP);
    if tmp { lock_acquire(&raw mut DMA_FENCE_LOCKDEP_MAP, 0, 1, 1, 1, core::ptr::null_mut(), _THIS_IP_); }
}

pub unsafe extern "C" fn dma_fence_signal_timestamp_locked(fence: *mut dma_fence, timestamp: ktime_t) {
    let ops: *const dma_fence_ops;
    let mut cur: *mut dma_fence_cb;
    let mut tmp: *mut dma_fence_cb;
    let mut cb_list: list_head = core::mem::zeroed();
    dma_fence_assert_held(fence);
    if test_and_set_bit(DMA_FENCE_FLAG_SIGNALED_BIT, &raw mut (*fence).flags) { return; }
    trace_dma_fence_signaled(fence);
    ops = rcu_dereference_protected((*fence).ops, true);
    if (*ops).release.is_none() && (*ops).wait.is_none() { RCU_INIT_POINTER((*fence).ops, core::ptr::null()); }
    list_replace(&raw mut (*fence).cb_list, &raw mut cb_list);
    (*fence).timestamp = timestamp;
    set_bit(DMA_FENCE_FLAG_TIMESTAMP_BIT, &raw mut (*fence).flags);
    list_for_each_entry_safe!(cur, tmp, &raw mut cb_list, node, {
        INIT_LIST_HEAD(&raw mut (*cur).node);
        ((*cur).func.unwrap())(fence, cur);
    });
}

pub unsafe extern "C" fn dma_fence_signal_timestamp(fence: *mut dma_fence, timestamp: ktime_t) {
    let mut flags = 0UL;
    if WARN_ON(fence.is_null()) { return; }
    dma_fence_lock_irqsave(fence, &raw mut flags);
    dma_fence_signal_timestamp_locked(fence, timestamp);
    dma_fence_unlock_irqrestore(fence, flags);
}

pub unsafe extern "C" fn dma_fence_signal_locked(fence: *mut dma_fence) { dma_fence_signal_timestamp_locked(fence, ktime_get()); }

pub unsafe extern "C" fn dma_fence_check_and_signal_locked(fence: *mut dma_fence) -> bool {
    let ret = dma_fence_test_signaled_flag(fence); dma_fence_signal_locked(fence); ret
}

pub unsafe extern "C" fn dma_fence_check_and_signal(fence: *mut dma_fence) -> bool {
    let mut flags = 0UL; dma_fence_lock_irqsave(fence, &raw mut flags);
    let ret = dma_fence_check_and_signal_locked(fence); dma_fence_unlock_irqrestore(fence, flags); ret
}

pub unsafe extern "C" fn dma_fence_signal(fence: *mut dma_fence) {
    let mut flags = 0UL; if WARN_ON(fence.is_null()) { return; }
    #[cfg(CONFIG_LOCKDEP)] let tmp = dma_fence_begin_signalling();
    dma_fence_lock_irqsave(fence, &raw mut flags); dma_fence_signal_timestamp_locked(fence, ktime_get()); dma_fence_unlock_irqrestore(fence, flags);
    #[cfg(CONFIG_LOCKDEP)] dma_fence_end_signalling(tmp);
}

pub unsafe extern "C" fn dma_fence_wait_timeout(fence: *mut dma_fence, intr: bool, timeout: i64) -> i64 {
    if WARN_ON(timeout < 0) { return -EINVAL as i64; }
    might_sleep();
    #[cfg(CONFIG_LOCKDEP)] __dma_fence_might_wait();
    dma_fence_enable_signaling(fence);
    rcu_read_lock();
    let ops = rcu_dereference((*fence).ops);
    trace_dma_fence_wait_start(fence);
    let ret = if !ops.is_null() && (*ops).wait.is_some() {
        rcu_read_unlock(); ((*ops).wait.unwrap())(fence, intr, timeout)
    } else { rcu_read_unlock(); dma_fence_default_wait(fence, intr, timeout) };
    if trace_dma_fence_wait_end_enabled() { rcu_read_lock(); trace_dma_fence_wait_end(fence); rcu_read_unlock(); }
    ret
}

pub unsafe extern "C" fn dma_fence_release(kref: *mut kref) {
    let fence = container_of!(kref, dma_fence, refcount); rcu_read_lock(); trace_dma_fence_destroy(fence);
    if !list_empty(&raw mut (*fence).cb_list) && !dma_fence_test_signaled_flag(fence) {
        let driver = dma_fence_driver_name(fence); let timeline = dma_fence_timeline_name(fence);
        WARN!(1, "Fence %s:%s:%llx:%llx released with pending signals!\n", driver, timeline, (*fence).context, (*fence).seqno);
        let mut flags = 0UL; dma_fence_lock_irqsave(fence, &raw mut flags); (*fence).error = -EDEADLK; dma_fence_signal_locked(fence); dma_fence_unlock_irqrestore(fence, flags);
    }
    let ops = rcu_dereference((*fence).ops); if !ops.is_null() && (*ops).release.is_some() { ((*ops).release.unwrap())(fence); } else { dma_fence_free(fence); } rcu_read_unlock();
}

pub unsafe extern "C" fn dma_fence_free(fence: *mut dma_fence) { kfree_rcu(fence, rcu); }

unsafe fn __dma_fence_enable_signaling(fence: *mut dma_fence) -> bool {
    dma_fence_assert_held(fence); let was_set = test_and_set_bit(DMA_FENCE_FLAG_ENABLE_SIGNAL_BIT, &raw mut (*fence).flags);
    if dma_fence_test_signaled_flag(fence) { return false; }
    rcu_read_lock(); let ops = rcu_dereference((*fence).ops);
    if !was_set && !ops.is_null() && (*ops).enable_signaling.is_some() { trace_dma_fence_enable_signal(fence); if !((*ops).enable_signaling.unwrap())(fence) { rcu_read_unlock(); dma_fence_signal_locked(fence); return false; } }
    rcu_read_unlock(); true
}

pub unsafe extern "C" fn dma_fence_enable_signaling(fence: *mut dma_fence) { let mut flags=0UL; dma_fence_lock_irqsave(fence,&raw mut flags); __dma_fence_enable_signaling(fence); dma_fence_unlock_irqrestore(fence,flags); }

pub unsafe extern "C" fn dma_fence_add_callback(fence:*mut dma_fence, cb:*mut dma_fence_cb, func:dma_fence_func_t)->i32 {
    let mut flags=0UL; if WARN_ON(fence.is_null() || func.is_none()) { return -EINVAL; }
    if dma_fence_test_signaled_flag(fence) { INIT_LIST_HEAD(&raw mut (*cb).node); return -ENOENT; }
    dma_fence_lock_irqsave(fence,&raw mut flags); let ret=if __dma_fence_enable_signaling(fence) { (*cb).func=func; list_add_tail(&raw mut (*cb).node,&raw mut (*fence).cb_list); 0 } else { INIT_LIST_HEAD(&raw mut (*cb).node); -ENOENT }; dma_fence_unlock_irqrestore(fence,flags); ret
}

pub unsafe extern "C" fn dma_fence_get_status(fence:*mut dma_fence)->i32 { let mut flags=0UL; dma_fence_lock_irqsave(fence,&raw mut flags); let status=dma_fence_get_status_locked(fence); dma_fence_unlock_irqrestore(fence,flags); status }

pub unsafe extern "C" fn dma_fence_remove_callback(fence:*mut dma_fence, cb:*mut dma_fence_cb)->bool { let mut flags=0UL; dma_fence_lock_irqsave(fence,&raw mut flags); let ret=!list_empty(&raw mut (*cb).node); if ret { list_del_init(&raw mut (*cb).node); } dma_fence_unlock_irqrestore(fence,flags); ret }

#[repr(C)] struct default_wait_cb { base: dma_fence_cb, task: *mut task_struct }
unsafe extern "C" fn dma_fence_default_wait_cb(_fence:*mut dma_fence, cb:*mut dma_fence_cb) { let wait=container_of!(cb,default_wait_cb,base); wake_up_state((*wait).task,TASK_NORMAL); }

pub unsafe extern "C" fn dma_fence_default_wait(fence:*mut dma_fence,intr:bool,timeout:i64)->i64 { let mut cb:default_wait_cb=core::mem::zeroed(); let mut flags=0UL; let mut ret=if timeout!=0 {timeout}else{1}; dma_fence_lock_irqsave(fence,&raw mut flags); if dma_fence_test_signaled_flag(fence){goto_out!();} if intr&&signal_pending(current()){ret=-ERESTARTSYS;goto_out!();} if timeout==0{ret=0;goto_out!();} cb.base.func=Some(dma_fence_default_wait_cb); cb.task=current(); list_add(&raw mut cb.base.node,&raw mut (*fence).cb_list); while !dma_fence_test_signaled_flag(fence)&&ret>0 { if intr {__set_current_state(TASK_INTERRUPTIBLE)} else {__set_current_state(TASK_UNINTERRUPTIBLE)} dma_fence_unlock_irqrestore(fence,flags); ret=schedule_timeout(ret); dma_fence_lock_irqsave(fence,&raw mut flags); if ret>0&&intr&&signal_pending(current()){ret=-ERESTARTSYS;} } if !list_empty(&raw mut cb.base.node){list_del(&raw mut cb.base.node);} __set_current_state(TASK_RUNNING); goto_out!(); dma_fence_unlock_irqrestore(fence,flags); ret }

pub unsafe extern "C" fn dma_fence_wait_any_timeout(fences:*mut *mut dma_fence,count:u32,intr:bool,timeout:i64,idx:*mut u32)->i64 { let mut ret=timeout; if WARN_ON(fences.is_null()||count==0||timeout<0){return -EINVAL as i64;} if timeout==0 {for i in 0..count {if dma_fence_is_signaled(*fences.add(i as usize)){if !idx.is_null(){*idx=i}return 1}}return 0;} let cb=kzalloc_objs::<default_wait_cb>(count); if cb.is_null(){return -ENOMEM as i64;} let mut i=0; while i<count {(*cb.add(i as usize)).task=current(); if dma_fence_add_callback(*fences.add(i as usize),&raw mut (*cb.add(i as usize)).base,Some(dma_fence_default_wait_cb))!=0 {if !idx.is_null(){*idx=i}break;} i+=1;} while ret>0 {if intr{set_current_state(TASK_INTERRUPTIBLE)}else{set_current_state(TASK_UNINTERRUPTIBLE)} for j in 0..count {if dma_fence_is_signaled(*fences.add(j as usize)){if !idx.is_null(){*idx=j}ret=1;break}} if ret<=0{break} ret=schedule_timeout(ret);if ret>0&&intr&&signal_pending(current()){ret=-ERESTARTSYS}} __set_current_state(TASK_RUNNING); while i>0{i-=1;dma_fence_remove_callback(*fences.add(i as usize),&raw mut (*cb.add(i as usize)).base);} kfree(cb); ret }

pub unsafe extern "C" fn dma_fence_set_deadline(fence:*mut dma_fence,deadline:ktime_t){rcu_read_lock();let ops=rcu_dereference((*fence).ops);if !ops.is_null()&&(*ops).set_deadline.is_some()&&!dma_fence_is_signaled(fence){((*ops).set_deadline.unwrap())(fence,deadline)}rcu_read_unlock();}

pub unsafe extern "C" fn dma_fence_describe(fence:*mut dma_fence,seq:*mut seq_file){let mut timeline=b"\0".as_ptr() as *const i8;let mut driver=b"\0".as_ptr() as *const i8;let mut signaled=b"\0".as_ptr() as *const i8;rcu_read_lock();if !dma_fence_is_signaled(fence){timeline=dma_fence_timeline_name(fence);driver=dma_fence_driver_name(fence);signaled=b"un\0".as_ptr() as *const i8;}seq_printf(seq,b"%llu:%llu %s %s %ssignalled\n\0".as_ptr() as *const i8,(*fence).context,(*fence).seqno,timeline,driver,signaled);rcu_read_unlock();}

unsafe fn __dma_fence_init(fence:*mut dma_fence,ops:*const dma_fence_ops,lock:*mut spinlock_t,context:u64,seqno:u64,flags:usize){BUG_ON(ops.is_null()||(*ops).get_driver_name.is_none()||(*ops).get_timeline_name.is_none());kref_init(&raw mut (*fence).refcount);RCU_INIT_POINTER((*fence).ops,ops);INIT_LIST_HEAD(&raw mut (*fence).cb_list);(*fence).context=context;(*fence).seqno=seqno;(*fence).flags=flags|BIT(DMA_FENCE_FLAG_INITIALIZED_BIT);if !lock.is_null(){(*fence).extern_lock=lock;}else{spin_lock_init(&raw mut (*fence).inline_lock);(*fence).flags|=BIT(DMA_FENCE_FLAG_INLINE_LOCK_BIT);}(*fence).error=0;trace_dma_fence_init(fence);}
pub unsafe extern "C" fn dma_fence_init(fence:*mut dma_fence,ops:*const dma_fence_ops,lock:*mut spinlock_t,context:u64,seqno:u64){__dma_fence_init(fence,ops,lock,context,seqno,0);}
pub unsafe extern "C" fn dma_fence_init64(fence:*mut dma_fence,ops:*const dma_fence_ops,lock:*mut spinlock_t,context:u64,seqno:u64){__dma_fence_init(fence,ops,lock,context,seqno,BIT(DMA_FENCE_FLAG_SEQNO64_BIT));}
pub unsafe extern "C" fn dma_fence_driver_name(fence:*mut dma_fence)->*const i8{let ops=rcu_dereference((*fence).ops);if !ops.is_null(){((*ops).get_driver_name.unwrap())(fence)}else{b"detached-driver\0".as_ptr() as *const i8}}
pub unsafe extern "C" fn dma_fence_timeline_name(fence:*mut dma_fence)->*const i8{let ops=rcu_dereference((*fence).ops);if !ops.is_null(){((*ops).get_timeline_name.unwrap())(fence)}else{b"signaled-timeline\0".as_ptr() as *const i8}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
