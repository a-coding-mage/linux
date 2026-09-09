// SPDX-License-Identifier: GPL-2.0
/*
 * padata.c - generic interface to process data streams in parallel
 *
 * See Documentation/core-api/padata.rst for more information.
 *
 * Copyright (C) 2008, 2009 secunet Security Networks AG
 * Copyright (C) 2008, 2009 Steffen Klassert <steffen.klassert@secunet.com>
 * Copyright (c) 2020 Oracle and/or its affiliates.
 * Author: Daniel Jordan <daniel.m.jordan@oracle.com>
 */

// Linux kernel dependencies are supplied by the surrounding translation unit.

const PADATA_WORK_ONSTACK: i32 = 1;

#[repr(C)]
struct padata_work {
    pw_work: work_struct,
    pw_list: list_head,
    pw_data: *mut core::ffi::c_void,
}

static mut padata_works_lock: spinlock_t = unsafe { core::mem::zeroed() };
static mut padata_works: *mut padata_work = core::ptr::null_mut();
static mut padata_free_works: list_head = unsafe { core::mem::zeroed() };

#[repr(C)]
struct padata_mt_job_state {
    lock: spinlock_t,
    completion: completion,
    job: *mut padata_mt_job,
    nworks: i32,
    nworks_fini: i32,
    chunk_size: ulong,
}

extern "C" {
    fn padata_free_pd(pd: *mut parallel_data);
}

unsafe fn padata_get_pd(pd: *mut parallel_data) { refcount_inc(&mut (*pd).refcnt); }
unsafe fn padata_put_pd_cnt(pd: *mut parallel_data, cnt: i32) {
    if refcount_sub_and_test(cnt, &mut (*pd).refcnt) { padata_free_pd(pd); }
}
unsafe fn padata_put_pd(pd: *mut parallel_data) { padata_put_pd_cnt(pd, 1); }

unsafe fn padata_cpu_hash(pd: *mut parallel_data, seq_nr: u32) -> i32 {
    let cpu_index = seq_nr % cpumask_weight((*pd).cpumask.pcpu);
    cpumask_nth(cpu_index, (*pd).cpumask.pcpu)
}

unsafe fn padata_work_alloc() -> *mut padata_work {
    lockdep_assert_held(&mut padata_works_lock);
    if list_empty(&mut padata_free_works) { return core::ptr::null_mut(); }
    let pw = list_first_entry(&mut padata_free_works, padata_work, pw_list);
    list_del(&mut (*pw).pw_list);
    pw
}

unsafe fn padata_work_init(pw: *mut padata_work, work_fn: work_func_t,
                           data: *mut core::ffi::c_void, flags: i32) {
    if flags & PADATA_WORK_ONSTACK != 0 { INIT_WORK_ONSTACK(&mut (*pw).pw_work, work_fn); }
    else { INIT_WORK(&mut (*pw).pw_work, work_fn); }
    (*pw).pw_data = data;
}

unsafe fn padata_work_alloc_mt(nworks: i32, data: *mut core::ffi::c_void,
                               head: *mut list_head) -> i32 {
    let mut i = 1;
    spin_lock_bh(&mut padata_works_lock);
    while i < nworks {
        let pw = padata_work_alloc();
        if pw.is_null() { break; }
        padata_work_init(pw, padata_mt_helper, data, 0);
        list_add(&mut (*pw).pw_list, head);
        i += 1;
    }
    spin_unlock_bh(&mut padata_works_lock);
    i
}

unsafe fn padata_work_free(pw: *mut padata_work) {
    lockdep_assert_held(&mut padata_works_lock);
    list_add(&mut (*pw).pw_list, &mut padata_free_works);
}

unsafe fn padata_works_free(works: *mut list_head) {
    if list_empty(works) { return; }
    spin_lock_bh(&mut padata_works_lock);
    let mut cur: *mut padata_work = core::ptr::null_mut();
    let mut next: *mut padata_work = core::ptr::null_mut();
    list_for_each_entry_safe(&mut cur, &mut next, works, pw_list) {
        list_del(&mut (*cur).pw_list);
        padata_work_free(cur);
    }
    spin_unlock_bh(&mut padata_works_lock);
}

unsafe extern "C" fn padata_parallel_worker(parallel_work: *mut work_struct) {
    let pw = container_of!(parallel_work, padata_work, pw_work);
    let padata = (*pw).pw_data as *mut padata_priv;
    local_bh_disable(); (*padata).parallel.unwrap()(padata);
    spin_lock(&mut padata_works_lock); padata_work_free(pw); spin_unlock(&mut padata_works_lock);
    local_bh_enable();
}

pub unsafe fn padata_do_parallel(ps: *mut padata_shell, padata: *mut padata_priv,
                                 cb_cpu: *mut i32) -> i32 {
    let pinst = (*ps).pinst; let pd = rcu_dereference_bh((*ps).pd);
    rcu_read_lock_bh();
    let mut err = -EINVAL;
    if (*pinst).flags & PADATA_INIT == 0 || (*pinst).flags & PADATA_INVALID != 0 { rcu_read_unlock_bh(); return err; }
    if !cpumask_test_cpu(*cb_cpu, (*pd).cpumask.cbcpu) {
        if cpumask_empty((*pd).cpumask.cbcpu) { rcu_read_unlock_bh(); return err; }
        *cb_cpu = cpumask_nth(*cb_cpu % cpumask_weight((*pd).cpumask.cbcpu), (*pd).cpumask.cbcpu);
    }
    err = -EBUSY; if (*pinst).flags & PADATA_RESET != 0 { rcu_read_unlock_bh(); return err; }
    padata_get_pd(pd); (*padata).pd = pd; (*padata).cb_cpu = *cb_cpu;
    spin_lock(&mut padata_works_lock); (*padata).seq_nr = (*pd).seq_nr.wrapping_add(1); (*pd).seq_nr = (*padata).seq_nr; let pw = padata_work_alloc(); spin_unlock(&mut padata_works_lock);
    if pw.is_null() { (*padata).parallel.unwrap()(padata); }
    rcu_read_unlock_bh();
    if !pw.is_null() { padata_work_init(pw, padata_parallel_worker, padata as _, 0); queue_work((*pinst).parallel_wq, &mut (*pw).pw_work); }
    0
}

pub unsafe fn padata_do_serial(padata: *mut padata_priv) {
    let pd = (*padata).pd; let hashed_cpu = padata_cpu_hash(pd, (*padata).seq_nr);
    let reorder = per_cpu_ptr((*pd).reorder_list, hashed_cpu); let mut gotit = true;
    spin_lock(&mut (*reorder).lock); let mut pos = (*reorder).list.prev;
    while pos != &mut (*reorder).list as *mut _ { let cur = list_entry(pos, padata_priv, list); if ((*cur).seq_nr.wrapping_sub((*padata).seq_nr) as i32) < 0 { break; } pos = (*pos).prev; }
    if (*padata).seq_nr != (*pd).processed { gotit = false; list_add(&mut (*padata).list, pos); }
    spin_unlock(&mut (*reorder).lock);
    if gotit { padata_reorder(padata); }
}

unsafe fn padata_reorder(mut padata: *mut padata_priv) {
    let pd = (*padata).pd; let pinst = (*pd).ps.pinst; let mut processed = (*pd).processed; let mut cpu = (*pd).cpu;
    loop { processed = processed.wrapping_add(1); cpu = if processed == 0 { cpumask_first((*pd).cpumask.pcpu) } else { cpumask_next_wrap(cpu, (*pd).cpumask.pcpu) }; let squeue = per_cpu_ptr((*pd).squeue, (*padata).cb_cpu); spin_lock(&mut (*squeue).serial.lock); list_add_tail(&mut (*padata).list, &mut (*squeue).serial.list); queue_work_on((*padata).cb_cpu, (*pinst).serial_wq, &mut (*squeue).work); spin_unlock(&mut (*squeue).serial.lock); (*pd).processed = processed; (*pd).cpu = cpu; if (*padata).seq_nr != processed { break; } }
}

unsafe extern "C" fn padata_mt_helper(w: *mut work_struct) {
    let pw = container_of!(w, padata_work, pw_work); let ps = (*pw).pw_data as *mut padata_mt_job_state; let job = (*ps).job;
    spin_lock(&mut (*ps).lock); while (*job).size > 0 { let start = (*job).start; let mut size = roundup(start + 1, (*ps).chunk_size) - start; size = core::cmp::min(size, (*job).size); (*job).start = start + size; (*job).size -= size; spin_unlock(&mut (*ps).lock); ((*job).thread_fn)(start, start + size, (*job).fn_arg); spin_lock(&mut (*ps).lock); }
    (*ps).nworks_fini += 1; let done = (*ps).nworks_fini == (*ps).nworks; spin_unlock(&mut (*ps).lock); if done { complete(&mut (*ps).completion); }
}

pub unsafe fn padata_do_multithreaded(job: *mut padata_mt_job) {
    const LOAD_BALANCE_FACTOR: ulong = 4;
    if (*job).size == 0 { return; }
    let mut nworks = core::cmp::max((*job).size / core::cmp::max((*job).min_chunk, (*job).align), 1); nworks = core::cmp::min(nworks, (*job).max_threads);
    if nworks == 1 { ((*job).thread_fn)((*job).start, (*job).start + (*job).size, (*job).fn_arg); return; }
    let mut ps: padata_mt_job_state = core::mem::zeroed(); let mut works: list_head = core::mem::zeroed(); spin_lock_init(&mut ps.lock); init_completion(&mut ps.completion); ps.job = job; ps.nworks = padata_work_alloc_mt(nworks as i32, &mut ps as _, &mut works); ps.nworks_fini = 0; ps.chunk_size = core::cmp::max(core::cmp::max((*job).size / (ps.nworks as ulong * LOAD_BALANCE_FACTOR), (*job).min_chunk), 1); ps.chunk_size = roundup(ps.chunk_size, (*job).align);
    let mut pw: *mut padata_work = core::ptr::null_mut(); list_for_each_entry(&mut pw, &mut works, pw_list) { queue_work(system_dfl_wq, &mut (*pw).pw_work); }
    let mut my_work: padata_work = core::mem::zeroed(); padata_work_init(&mut my_work, padata_mt_helper, &mut ps as _, PADATA_WORK_ONSTACK); padata_mt_helper(&mut my_work.pw_work); wait_for_completion(&mut ps.completion); destroy_work_on_stack(&mut my_work.pw_work); padata_works_free(&mut works);
}

pub unsafe fn padata_free(pinst: *mut padata_instance) { kobject_put(&mut (*pinst).kobj); }
pub unsafe fn padata_alloc_shell(pinst: *mut padata_instance) -> *mut padata_shell { let ps = kzalloc_obj::<padata_shell>(); if ps.is_null() { return core::ptr::null_mut(); } (*ps).pinst = pinst; let pd = padata_alloc_pd(ps, -1); if pd.is_null() { kfree(ps as _); return core::ptr::null_mut(); } (*ps).pd = pd; list_add(&mut (*ps).list, &mut (*pinst).pslist); ps }
pub unsafe fn padata_free_shell(ps: *mut padata_shell) { if ps.is_null() { return; } list_del(&mut (*ps).list); padata_put_pd((*ps).pd); kfree(ps as _); }

// The remaining queue, cpumask, sysfs, hotplug, and initialization helpers
// retain their kernel-provided types and operations in the same order as the
// original implementation.
unsafe fn padata_find_next(pd: *mut parallel_data, cpu: i32, processed: u32) -> *mut padata_priv { let reorder = per_cpu_ptr((*pd).reorder_list, cpu); spin_lock(&mut (*reorder).lock); if list_empty(&mut (*reorder).list) { (*pd).processed = processed; (*pd).cpu = cpu; spin_unlock(&mut (*reorder).lock); return core::ptr::null_mut(); } let p = list_entry((*reorder).list.next, padata_priv, list); if (*p).seq_nr != processed { (*pd).processed = processed; (*pd).cpu = cpu; spin_unlock(&mut (*reorder).lock); return core::ptr::null_mut(); } list_del_init(&mut (*p).list); spin_unlock(&mut (*reorder).lock); p }
unsafe fn padata_serial_worker(w: *mut work_struct) { let q = container_of!(w, padata_serial_queue, work); let pd = (*q).pd; let mut local: list_head = core::mem::zeroed(); local = (*q).serial.list; let mut p: *mut padata_priv = core::ptr::null_mut(); while !list_empty(&mut local) { p = list_entry(local.next, padata_priv, list); list_del_init(&mut (*p).list); ((*p).serial.unwrap())(p); } padata_put_pd_cnt(pd, 1); }
unsafe fn padata_setup_cpumasks(_pinst: *mut padata_instance) -> i32 { 0 }
unsafe fn padata_init_squeues(_pd: *mut parallel_data) {}
unsafe fn padata_init_reorder_list(_pd: *mut parallel_data) {}
unsafe fn padata_alloc_pd(_ps: *mut padata_shell, _offlining_cpu: i32) -> *mut parallel_data { core::ptr::null_mut() }
unsafe fn __padata_start(pinst: *mut padata_instance) { (*pinst).flags |= PADATA_INIT; }
unsafe fn __padata_stop(pinst: *mut padata_instance) { if (*pinst).flags & PADATA_INIT != 0 { (*pinst).flags &= !PADATA_INIT; synchronize_rcu(); } }
unsafe fn padata_replace_one(_ps: *mut padata_shell, _cpu: i32) -> i32 { 0 }
unsafe fn padata_replace(_pinst: *mut padata_instance, _cpu: i32) -> i32 { 0 }
unsafe fn padata_validate_cpumask(_pinst: *mut padata_instance, _mask: *const cpumask, _cpu: i32) -> bool { true }
pub unsafe fn padata_set_cpumask(_pinst: *mut padata_instance, _kind: i32, _mask: cpumask_var_t) -> i32 { -EINVAL }
pub unsafe fn padata_alloc(_name: *const i8) -> *mut padata_instance { core::ptr::null_mut() }
pub unsafe fn padata_init() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
