// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2021 VMware Inc, Steven Rostedt <rostedt@goodmis.org>
 */
// Linux headers and trace.h provide the types, constants, and primitives used here.

use crate::*;

unsafe fn get_lower_chunk(pid_list: *mut trace_pid_list) -> *mut lower_chunk {
    lockdep_assert_held(&mut (*pid_list).lock);
    if (*pid_list).lower_list.is_null() { return core::ptr::null_mut(); }
    let chunk = (*pid_list).lower_list;
    (*pid_list).lower_list = (*chunk).next;
    (*pid_list).free_lower_chunks -= 1;
    WARN_ON_ONCE((*pid_list).free_lower_chunks < 0);
    (*chunk).next = core::ptr::null_mut();
    if (*pid_list).free_lower_chunks <= CHUNK_REALLOC { irq_work_queue(&mut (*pid_list).refill_irqwork); }
    chunk
}

unsafe fn get_upper_chunk(pid_list: *mut trace_pid_list) -> *mut upper_chunk {
    lockdep_assert_held(&mut (*pid_list).lock);
    if (*pid_list).upper_list.is_null() { return core::ptr::null_mut(); }
    let chunk = (*pid_list).upper_list;
    (*pid_list).upper_list = (*chunk).next;
    (*pid_list).free_upper_chunks -= 1;
    WARN_ON_ONCE((*pid_list).free_upper_chunks < 0);
    (*chunk).next = core::ptr::null_mut();
    if (*pid_list).free_upper_chunks <= CHUNK_REALLOC { irq_work_queue(&mut (*pid_list).refill_irqwork); }
    chunk
}

unsafe fn put_lower_chunk(pid_list: *mut trace_pid_list, chunk: *mut lower_chunk) {
    lockdep_assert_held(&mut (*pid_list).lock);
    (*chunk).next = (*pid_list).lower_list;
    (*pid_list).lower_list = chunk;
    (*pid_list).free_lower_chunks += 1;
}

unsafe fn put_upper_chunk(pid_list: *mut trace_pid_list, chunk: *mut upper_chunk) {
    lockdep_assert_held(&mut (*pid_list).lock);
    (*chunk).next = (*pid_list).upper_list;
    (*pid_list).upper_list = chunk;
    (*pid_list).free_upper_chunks += 1;
}

unsafe fn upper_empty(chunk: *mut upper_chunk) -> bool {
    bitmap_empty((*chunk).data.as_ptr() as *const _, BITS_PER_TYPE)
}

unsafe fn pid_split(pid: u32, upper1: *mut u32, upper2: *mut u32, lower: *mut u32) -> i32 {
    if pid >= MAX_PID { return -1; }
    *upper1 = (pid >> UPPER1_SHIFT) & UPPER_MASK;
    *upper2 = (pid >> UPPER2_SHIFT) & UPPER_MASK;
    *lower = pid & LOWER_MASK;
    0
}

fn pid_join(upper1: u32, upper2: u32, lower: u32) -> u32 {
    ((upper1 & UPPER_MASK) << UPPER1_SHIFT) |
        ((upper2 & UPPER_MASK) << UPPER2_SHIFT) | (lower & LOWER_MASK)
}

/// Test if the pid is set in the list.
pub unsafe fn trace_pid_list_is_set(pid_list: *mut trace_pid_list, pid: u32) -> bool {
    if pid_list.is_null() { return false; }
    let (mut upper1, mut upper2, mut lower) = (0, 0, 0);
    if pid_split(pid, &mut upper1, &mut upper2, &mut lower) < 0 { return false; }
    let mut ret;
    loop {
        let seq = read_seqcount_begin(&(*pid_list).seqcount);
        ret = false;
        let upper = (*pid_list).upper[upper1 as usize];
        if !upper.is_null() {
            let low = (*upper).data[upper2 as usize];
            if !low.is_null() { ret = test_bit(lower, (*low).data.as_ptr()); }
        }
        if !read_seqcount_retry(&(*pid_list).seqcount, seq) { break; }
    }
    ret
}

/// Add a pid to the list.
pub unsafe fn trace_pid_list_set(pid_list: *mut trace_pid_list, pid: u32) -> i32 {
    if pid_list.is_null() { return -ENODEV; }
    let (mut upper1, mut upper2, mut lower) = (0, 0, 0);
    if pid_split(pid, &mut upper1, &mut upper2, &mut lower) < 0 { return -EINVAL; }
    let mut flags = 0;
    raw_spin_lock_irqsave(&mut (*pid_list).lock, &mut flags);
    write_seqcount_begin(&mut (*pid_list).seqcount);
    let mut upper = (*pid_list).upper[upper1 as usize];
    let mut ret = 0;
    if upper.is_null() { upper = get_upper_chunk(pid_list); if upper.is_null() { ret = -ENOMEM; } else { (*pid_list).upper[upper1 as usize] = upper; } }
    if ret != 0 { write_seqcount_end(&mut (*pid_list).seqcount); raw_spin_unlock_irqrestore(&mut (*pid_list).lock, flags); return ret; }
    let mut low = (*upper).data[upper2 as usize];
    if low.is_null() { low = get_lower_chunk(pid_list); if low.is_null() { ret = -ENOMEM; } else { (*upper).data[upper2 as usize] = low; } }
    if ret != 0 { write_seqcount_end(&mut (*pid_list).seqcount); raw_spin_unlock_irqrestore(&mut (*pid_list).lock, flags); return ret; }
    set_bit(lower, (*low).data.as_mut_ptr());
    write_seqcount_end(&mut (*pid_list).seqcount);
    raw_spin_unlock_irqrestore(&mut (*pid_list).lock, flags);
    ret
}

/// Remove a pid from the list.
pub unsafe fn trace_pid_list_clear(pid_list: *mut trace_pid_list, pid: u32) -> i32 {
    if pid_list.is_null() { return -ENODEV; }
    let (mut u1, mut u2, mut low_i) = (0, 0, 0);
    if pid_split(pid, &mut u1, &mut u2, &mut low_i) < 0 { return -EINVAL; }
    let mut flags = 0; raw_spin_lock_irqsave(&mut (*pid_list).lock, &mut flags); write_seqcount_begin(&mut (*pid_list).seqcount);
    let upper = (*pid_list).upper[u1 as usize];
    if !upper.is_null() { let low = (*upper).data[u2 as usize]; if !low.is_null() { clear_bit(low_i, (*low).data.as_mut_ptr()); if find_first_bit((*low).data.as_ptr(), LOWER_MAX) >= LOWER_MAX { put_lower_chunk(pid_list, low); (*upper).data[u2 as usize] = core::ptr::null_mut(); if upper_empty(upper) { put_upper_chunk(pid_list, upper); (*pid_list).upper[u1 as usize] = core::ptr::null_mut(); } } } }
    write_seqcount_end(&mut (*pid_list).seqcount); raw_spin_unlock_irqrestore(&mut (*pid_list).lock, flags); 0
}

pub unsafe fn trace_pid_list_next(pid_list: *mut trace_pid_list, pid: u32, next: *mut u32) -> i32 {
    if pid_list.is_null() { return -ENODEV; }
    let (mut u1, mut u2, mut low) = (0, 0, 0); if pid_split(pid, &mut u1, &mut u2, &mut low) < 0 { return -EINVAL; }
    let mut flags = 0; raw_spin_lock_irqsave(&mut (*pid_list).lock, &mut flags);
    while u1 <= UPPER_MASK { let upper = (*pid_list).upper[u1 as usize]; if !upper.is_null() { while u2 <= UPPER_MASK { let l = (*upper).data[u2 as usize]; if !l.is_null() { low = find_next_bit((*l).data.as_ptr(), LOWER_MAX, low); if low < LOWER_MAX { raw_spin_unlock_irqrestore(&mut (*pid_list).lock, flags); *next = pid_join(u1,u2,low); return 0; } } u2 += 1; low=0; } } u1+=1; u2=0; }
    raw_spin_unlock_irqrestore(&mut (*pid_list).lock, flags); -1
}

pub unsafe fn trace_pid_list_first(pid_list: *mut trace_pid_list, pid: *mut u32) -> i32 { trace_pid_list_next(pid_list, 0, pid) }

unsafe extern "C" fn pid_list_refill_irq(iwork: *mut irq_work) {
    let pid_list = container_of!(iwork, trace_pid_list, refill_irqwork);
    let mut upper: *mut upper_chunk = core::ptr::null_mut(); let mut lower: *mut lower_chunk = core::ptr::null_mut();
    let mut uc = CHUNK_ALLOC - (*pid_list).free_upper_chunks; let mut lc = CHUNK_ALLOC - (*pid_list).free_lower_chunks;
    if uc <= 0 && lc <= 0 { return; }
    while uc > 0 { let c = kzalloc_obj::<upper_chunk>(GFP_NOWAIT); if c.is_null() { break; } (*c).next=upper; upper=c; uc-=1; }
    while lc > 0 { let c = kzalloc_obj::<lower_chunk>(GFP_NOWAIT); if c.is_null() { break; } (*c).next=lower; lower=c; lc-=1; }
    raw_spin_lock(&mut (*pid_list).lock); write_seqcount_begin(&mut (*pid_list).seqcount);
    if !upper.is_null() { (*pid_list).upper_list=upper; (*pid_list).free_upper_chunks += CHUNK_ALLOC-uc; }
    if !lower.is_null() { (*pid_list).lower_list=lower; (*pid_list).free_lower_chunks += CHUNK_ALLOC-lc; }
    write_seqcount_end(&mut (*pid_list).seqcount); raw_spin_unlock(&mut (*pid_list).lock);
}

pub unsafe fn trace_pid_list_alloc() -> *mut trace_pid_list {
    let p = kzalloc_obj::<trace_pid_list>(); if p.is_null() { return core::ptr::null_mut(); }
    init_irq_work(&mut (*p).refill_irqwork, Some(pid_list_refill_irq)); raw_spin_lock_init(&mut (*p).lock); seqcount_raw_spinlock_init(&mut (*p).seqcount, &mut (*p).lock);
    for _ in 0..CHUNK_ALLOC { let c=kzalloc_obj::<upper_chunk>(); if c.is_null(){break;} (*c).next=(*p).upper_list; (*p).upper_list=c; (*p).free_upper_chunks+=1; }
    for _ in 0..CHUNK_ALLOC { let c=kzalloc_obj::<lower_chunk>(); if c.is_null(){break;} (*c).next=(*p).lower_list; (*p).lower_list=c; (*p).free_lower_chunks+=1; }
    p
}

pub unsafe fn trace_pid_list_free(pid_list: *mut trace_pid_list) {
    if pid_list.is_null(){return;} irq_work_sync(&mut (*pid_list).refill_irqwork);
    while !(*pid_list).lower_list.is_null(){let c=(*pid_list).lower_list;(*pid_list).lower_list=(*c).next;kfree(c);}
    while !(*pid_list).upper_list.is_null(){let c=(*pid_list).upper_list;(*pid_list).upper_list=(*c).next;kfree(c);}
    for u in 0..UPPER1_SIZE { let up=(*pid_list).upper[u]; if !up.is_null(){for j in 0..UPPER2_SIZE{kfree((*up).data[j]);}kfree(up);} } kfree(pid_list);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
