// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Virtual Processor Dispatch Trace Log
 *
 * (C) Copyright IBM Corporation 2009
 *
 * Author: Jeremy Kerr <jk@ozlabs.org>
 */

// Dependencies are supplied by the surrounding kernel translation.

#[cfg(CONFIG_DTL)]
#[repr(C)]
struct dtl {
    buf: *mut dtl_entry,
    cpu: i32,
    buf_entries: i32,
    last_idx: u64,
    lock: spinlock_t,
}

#[cfg(CONFIG_DTL)]
static mut cpu_dtl: [dtl; 1] = [dtl { buf: core::ptr::null_mut(), cpu: 0, buf_entries: 0, last_idx: 0, lock: spinlock_t::new() }];

#[cfg(CONFIG_DTL)]
static mut dtl_event_mask: u8 = DTL_LOG_ALL;

#[cfg(CONFIG_DTL)]
static mut dtl_buf_entries: i32 = N_DISPATCH_LOG;

#[cfg(all(CONFIG_DTL, CONFIG_VIRT_CPU_ACCOUNTING_NATIVE))]
static mut dtl_consumer: Option<unsafe fn(*mut dtl_entry, u64)> = None;

#[cfg(all(CONFIG_DTL, CONFIG_VIRT_CPU_ACCOUNTING_NATIVE))]
#[repr(C)]
struct dtl_ring {
    write_index: u64,
    write_ptr: *mut dtl_entry,
    buf: *mut dtl_entry,
    buf_end: *mut dtl_entry,
}

#[cfg(all(CONFIG_DTL, CONFIG_VIRT_CPU_ACCOUNTING_NATIVE))]
static mut dtl_rings: [dtl_ring; 1] = [dtl_ring { write_index: 0, write_ptr: core::ptr::null_mut(), buf: core::ptr::null_mut(), buf_end: core::ptr::null_mut() }];
#[cfg(all(CONFIG_DTL, CONFIG_VIRT_CPU_ACCOUNTING_NATIVE))]
static mut dtl_count: atomic_t = atomic_t::new(0);

#[cfg(all(CONFIG_DTL, CONFIG_VIRT_CPU_ACCOUNTING_NATIVE))]
unsafe fn consume_dtle(dtle: *mut dtl_entry, index: u64) {
    let dtlr = this_cpu_ptr(&mut dtl_rings);
    let mut wp = (*dtlr).write_ptr;
    let vpa = (*local_paca).lppaca_ptr;
    if wp.is_null() { return; }
    core::ptr::write(wp, core::ptr::read(dtle));
    barrier();
    if index.wrapping_add(N_DISPATCH_LOG as u64) < be64_to_cpu((*vpa).dtl_idx) { return; }
    wp = wp.add(1);
    if wp == (*dtlr).buf_end { wp = (*dtlr).buf; }
    (*dtlr).write_ptr = wp;
    smp_wmb();
    (*dtlr).write_index = (*dtlr).write_index.wrapping_add(1);
}

#[cfg(all(CONFIG_DTL, CONFIG_VIRT_CPU_ACCOUNTING_NATIVE))]
unsafe fn dtl_start(dtl: *mut dtl) -> i32 {
    let dtlr = &mut per_cpu(&mut dtl_rings, (*dtl).cpu);
    (*dtlr).buf = (*dtl).buf;
    (*dtlr).buf_end = (*dtl).buf.add((*dtl).buf_entries as usize);
    (*dtlr).write_index = 0;
    smp_wmb();
    (*dtlr).write_ptr = (*dtl).buf;
    lppaca_of((*dtl).cpu).dtl_enable_mask |= dtl_event_mask;
    dtl_consumer = Some(consume_dtle);
    atomic_inc(&mut dtl_count);
    0
}

#[cfg(all(CONFIG_DTL, CONFIG_VIRT_CPU_ACCOUNTING_NATIVE))]
unsafe fn dtl_stop(dtl: *mut dtl) {
    let dtlr = &mut per_cpu(&mut dtl_rings, (*dtl).cpu);
    (*dtlr).write_ptr = core::ptr::null_mut();
    smp_wmb();
    (*dtlr).buf = core::ptr::null_mut();
    lppaca_of((*dtl).cpu).dtl_enable_mask = DTL_LOG_PREEMPT;
    if atomic_dec_and_test(&mut dtl_count) { dtl_consumer = None; }
}

#[cfg(all(CONFIG_DTL, CONFIG_VIRT_CPU_ACCOUNTING_NATIVE))]
unsafe fn dtl_current_index(dtl: *mut dtl) -> u64 { per_cpu(&mut dtl_rings, (*dtl).cpu).write_index }

#[cfg(all(CONFIG_DTL, not(CONFIG_VIRT_CPU_ACCOUNTING_NATIVE)))]
unsafe fn dtl_start(dtl: *mut dtl) -> i32 {
    let p = (*dtl).buf as *mut u32;
    *p.add(1) = cpu_to_be32(DISPATCH_LOG_BYTES);
    let hwcpu = get_hard_smp_processor_id((*dtl).cpu);
    let ret = register_dtl(hwcpu, __pa((*dtl).buf));
    if ret != 0 { printk(KERN_WARNING, "DTL registration for cpu failed\n"); return -EIO; }
    lppaca_of((*dtl).cpu).dtl_idx = 0;
    smp_wmb();
    lppaca_of((*dtl).cpu).dtl_enable_mask = dtl_event_mask;
    0
}

#[cfg(all(CONFIG_DTL, not(CONFIG_VIRT_CPU_ACCOUNTING_NATIVE)))]
unsafe fn dtl_stop(dtl: *mut dtl) {
    let hwcpu = get_hard_smp_processor_id((*dtl).cpu);
    lppaca_of((*dtl).cpu).dtl_enable_mask = 0;
    unregister_dtl(hwcpu);
}

#[cfg(all(CONFIG_DTL, not(CONFIG_VIRT_CPU_ACCOUNTING_NATIVE)))]
unsafe fn dtl_current_index(dtl: *mut dtl) -> u64 { be64_to_cpu(lppaca_of((*dtl).cpu).dtl_idx) }

#[cfg(CONFIG_DTL)]
unsafe fn dtl_enable(dtl: *mut dtl) -> i32 {
    if dtl_cache.is_null() { return -ENOMEM; }
    if !(*dtl).buf.is_null() { return -EBUSY; }
    if !down_read_trylock(&mut dtl_access_lock) { return -EBUSY; }
    let n_entries = dtl_buf_entries;
    let buf = kmem_cache_alloc_node(dtl_cache, GFP_KERNEL, cpu_to_node((*dtl).cpu)) as *mut dtl_entry;
    if buf.is_null() { up_read(&mut dtl_access_lock); return -ENOMEM; }
    spin_lock(&mut (*dtl).lock);
    let mut rc = -EBUSY;
    if (*dtl).buf.is_null() {
        (*dtl).buf_entries = n_entries;
        (*dtl).buf = buf;
        (*dtl).last_idx = 0;
        rc = dtl_start(dtl);
        if rc != 0 { (*dtl).buf = core::ptr::null_mut(); }
    }
    spin_unlock(&mut (*dtl).lock);
    if rc != 0 { up_read(&mut dtl_access_lock); kmem_cache_free(dtl_cache, buf as *mut _); }
    rc
}

#[cfg(CONFIG_DTL)]
unsafe fn dtl_disable(dtl: *mut dtl) {
    spin_lock(&mut (*dtl).lock);
    dtl_stop(dtl);
    kmem_cache_free(dtl_cache, (*dtl).buf as *mut _);
    (*dtl).buf = core::ptr::null_mut();
    (*dtl).buf_entries = 0;
    spin_unlock(&mut (*dtl).lock);
    up_read(&mut dtl_access_lock);
}

#[cfg(CONFIG_VIRT_CPU_ACCOUNTING_NATIVE)]
unsafe fn scan_dispatch_log(stop_tb: u64) -> u64 {
    let mut i = (*local_paca).dtl_ridx;
    let mut dtl = (*local_paca).dtl_curr;
    let dtl_end = (*local_paca).dispatch_log_end;
    let vpa = (*local_paca).lppaca_ptr;
    let mut stolen = 0;
    if dtl.is_null() || i == be64_to_cpu((*vpa).dtl_idx) { return 0; }
    while i < be64_to_cpu((*vpa).dtl_idx) {
        let dtb = be64_to_cpu((*dtl).timebase);
        let tb_delta = be32_to_cpu((*dtl).enqueue_to_dispatch_time) as u64 + be32_to_cpu((*dtl).ready_to_enqueue_time) as u64;
        barrier();
        if i.wrapping_add(N_DISPATCH_LOG as u64) < be64_to_cpu((*vpa).dtl_idx) {
            i = be64_to_cpu((*vpa).dtl_idx) - N_DISPATCH_LOG as u64;
            dtl = (*local_paca).dispatch_log.add((i % N_DISPATCH_LOG as u64) as usize);
            continue;
        }
        if dtb > stop_tb { break; }
        #[cfg(CONFIG_DTL)]
        if let Some(f) = dtl_consumer { f(dtl, i); }
        stolen = stolen.wrapping_add(tb_delta);
        i += 1; dtl = dtl.add(1);
        if dtl == dtl_end { dtl = (*local_paca).dispatch_log; }
    }
    (*local_paca).dtl_ridx = i; (*local_paca).dtl_curr = dtl; stolen
}

#[cfg(CONFIG_VIRT_CPU_ACCOUNTING_NATIVE)]
pub unsafe fn pseries_accumulate_stolen_time() {
    let acct = &mut (*local_paca).accounting;
    let sst = scan_dispatch_log(acct.starttime_user);
    let ust = scan_dispatch_log(acct.starttime);
    acct.stime -= sst; acct.utime -= ust; acct.steal_time += ust + sst;
}

#[cfg(CONFIG_VIRT_CPU_ACCOUNTING_NATIVE)]
pub unsafe fn pseries_calculate_stolen_time(stop_tb: u64) -> u64 {
    if !firmware_has_feature(FW_FEATURE_SPLPAR) { return 0; }
    if (*get_paca()).dtl_ridx != be64_to_cpu((*get_lppaca()).dtl_idx) { scan_dispatch_log(stop_tb) } else { 0 }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
