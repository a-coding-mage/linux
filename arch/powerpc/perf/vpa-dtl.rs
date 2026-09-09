// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Perf interface to expose Dispatch Trace Log counters.
 *
 * Copyright (C) 2024 Kajol Jain, IBM Corporation
 */

// Conditional compilation intent: CONFIG_PPC_SPLPAR

const DTL_CEDE: u32 = 0x1;
const DTL_PREEMPT: u32 = 0x2;
const DTL_FAULT: u32 = 0x4;
const DTL_ALL: u32 = 0x7;

// GENERIC_EVENT_ATTR(dtl_cede, DTL_CEDE);
// GENERIC_EVENT_ATTR(dtl_preempt, DTL_PREEMPT);
// GENERIC_EVENT_ATTR(dtl_fault, DTL_FAULT);
// GENERIC_EVENT_ATTR(dtl_all, DTL_ALL);
// PMU_FORMAT_ATTR(event, "config:0-7");

#[repr(C)]
struct VpaDtl {
    buf: *mut DtlEntry,
    last_idx: u64,
}

#[repr(C)]
struct VpaPmuCtx {
    handle: PerfOutputHandle,
}

#[repr(C)]
struct VpaPmuBuf {
    nr_pages: i32,
    snapshot: bool,
    base: *mut u64,
    size: u64,
    head: u64,
    head_size: u64,
    // boot timebase and frequency needs to be saved only at once
    boottb_freq_saved: i32,
    threshold: u64,
    full: bool,
}

#[repr(C)]
struct BoottbFreq {
    boot_tb: u64,
    tb_freq: u64,
    timebase: u64,
    padded: [u64; 3],
}

static mut VPA_PMU_CTX: PerCpu<VpaPmuCtx> = PerCpu::new();
static mut VPA_DTL_CPU: PerCpu<VpaDtl> = PerCpu::new();
static mut DTL_GLOBAL_REFC: i32 = 0;
static mut DTL_GLOBAL_LOCK: Spinlock = Spinlock::unlocked();

/*
 * Capture DTL data in AUX buffer
 */
unsafe fn vpa_dtl_capture_aux(
    n_entries: *mut i64,
    buf: *mut VpaPmuBuf,
    dtl: *mut VpaDtl,
    index: i32,
) {
    let aux_copy_buf = (*buf).base as *mut DtlEntry;

    /*
     * check if there is enough space to contain the
     * DTL data. If not, save the data for available
     * memory and set full to true.
     */
    if (*buf).head + (*n_entries as u64) >= (*buf).threshold {
        *n_entries = ((*buf).threshold - (*buf).head) as i64;
        (*buf).full = true;
    }

    /* Copy to AUX buffer from per-thread address */
    memcpy(
        aux_copy_buf.add((*buf).head as usize),
        (*dtl).buf.add(index as usize),
        (*n_entries as usize) * core::mem::size_of::<DtlEntry>(),
    );

    if (*buf).full {
        /* Set head of private aux to zero when buffer is full */
        (*buf).head = 0;
        return;
    }
    (*buf).head += *n_entries as u64;
}

/*
 * Function to dump the dispatch trace log buffer data to the
 * perf data.
 */
unsafe fn vpa_dtl_dump_sample_data(event: *mut PerfEvent) {
    let mut cur_idx: u64;
    let mut last_idx: u64;
    let mut i: u64;
    let boot_tb: u64;
    let mut boottb_freq: BoottbFreq;
    let mut n_read: i64 = 0;
    let mut read_size: i64 = 0;
    let mut n_req: i64;
    let vpa_ctx = this_cpu_ptr(&mut VPA_PMU_CTX);
    let aux_buf: *mut VpaPmuBuf;
    let dtl = &mut per_cpu(&mut VPA_DTL_CPU, (*event).cpu);
    let mut size: u64;

    cur_idx = be64_to_cpu(lppaca_of((*event).cpu).dtl_idx);
    last_idx = dtl.last_idx;
    if last_idx + N_DISPATCH_LOG as u64 <= cur_idx {
        last_idx = cur_idx - N_DISPATCH_LOG as u64 + 1;
    }
    n_req = (cur_idx - last_idx) as i64;
    if n_req <= 0 { return; }

    dtl.last_idx = last_idx + n_req as u64;
    boot_tb = get_boot_tb();
    i = last_idx % N_DISPATCH_LOG as u64;
    aux_buf = perf_aux_output_begin(&mut (*vpa_ctx).handle, event);
    if aux_buf.is_null() {
        pr_debug!("returning. no aux\n");
        return;
    }

    if (*aux_buf).boottb_freq_saved == 0 {
        pr_debug!("Copying boot tb to aux buffer: %lld\n", boot_tb);
        boottb_freq.boot_tb = boot_tb;
        boottb_freq.tb_freq = tb_ticks_per_sec;
        boottb_freq.timebase = 0;
        memcpy((*aux_buf).base, &boottb_freq, core::mem::size_of::<BoottbFreq>());
        (*aux_buf).head += 1;
        (*aux_buf).boottb_freq_saved = 1;
        n_read += 1;
    }

    if i + n_req as u64 > N_DISPATCH_LOG as u64 {
        read_size = (N_DISPATCH_LOG as u64 - i) as i64;
        vpa_dtl_capture_aux(&mut read_size, aux_buf, dtl, i as i32);
        n_req -= read_size;
        n_read += read_size;
        i = 0;
        if (*aux_buf).full {
            size = n_read as u64 * core::mem::size_of::<DtlEntry>() as u64;
            if size + (*aux_buf).head_size > (*aux_buf).size {
                size = (*aux_buf).size - (*aux_buf).head_size;
                perf_aux_output_end(&mut (*vpa_ctx).handle, size);
                (*aux_buf).head = 0;
                (*aux_buf).head_size = 0;
            } else {
                (*aux_buf).head_size += n_read as u64 * core::mem::size_of::<DtlEntry>() as u64;
                perf_aux_output_end(&mut (*vpa_ctx).handle, n_read as u64 * core::mem::size_of::<DtlEntry>() as u64);
            }
            (*aux_buf).full = false;
            return;
        }
    }

    vpa_dtl_capture_aux(&mut n_req, aux_buf, dtl, i as i32);
    size = (n_req + n_read) as u64 * core::mem::size_of::<DtlEntry>() as u64;
    if size + (*aux_buf).head_size > (*aux_buf).size {
        size = (*aux_buf).size - (*aux_buf).head_size;
        perf_aux_output_end(&mut (*vpa_ctx).handle, size);
        (*aux_buf).head = 0;
        (*aux_buf).head_size = 0;
    } else {
        (*aux_buf).head_size += (n_req + n_read) as u64 * core::mem::size_of::<DtlEntry>() as u64;
        perf_aux_output_end(&mut (*vpa_ctx).handle, (n_req + n_read) as u64 * core::mem::size_of::<DtlEntry>() as u64);
    }
    (*aux_buf).full = false;
}

unsafe fn vpa_dtl_hrtimer_handle(hrtimer: *mut Hrtimer) -> HrtimerRestart {
    let event = container_of!(hrtimer, PerfEvent, hw.hrtimer);
    if (*event).state != PERF_EVENT_STATE_ACTIVE { return HRTIMER_NORESTART; }
    vpa_dtl_dump_sample_data(event);
    let period = core::cmp::max(NSEC_PER_MSEC, (*event).hw.sample_period);
    hrtimer_forward_now(hrtimer, ns_to_ktime(period));
    HRTIMER_RESTART
}

unsafe fn vpa_dtl_start_hrtimer(event: *mut PerfEvent) {
    let period = core::cmp::max(NSEC_PER_MSEC, (*event).hw.sample_period);
    hrtimer_start(&mut (*event).hw.hrtimer, ns_to_ktime(period), HRTIMER_MODE_REL_PINNED);
}

unsafe fn vpa_dtl_stop_hrtimer(event: *mut PerfEvent) { hrtimer_cancel(&mut (*event).hw.hrtimer); }

unsafe fn vpa_dtl_reset_global_refc(_event: *mut PerfEvent) {
    spin_lock(&mut DTL_GLOBAL_LOCK);
    DTL_GLOBAL_REFC -= 1;
    if DTL_GLOBAL_REFC <= 0 { DTL_GLOBAL_REFC = 0; up_write(&mut dtl_access_lock); }
    spin_unlock(&mut DTL_GLOBAL_LOCK);
}

unsafe fn vpa_dtl_mem_alloc(cpu: i32) -> i32 {
    let dtl = &mut per_cpu(&mut VPA_DTL_CPU, cpu);
    if dtl_cache.is_null() { return -ENOMEM; }
    let buf = kmem_cache_alloc_node(dtl_cache, GFP_KERNEL | GFP_ATOMIC, cpu_to_node(cpu));
    if buf.is_null() { pr_warn!("buffer allocation failed for cpu %d\n", cpu); return -ENOMEM; }
    dtl.buf = buf as *mut DtlEntry;
    0
}

unsafe fn vpa_dtl_event_init(event: *mut PerfEvent) -> i32 {
    let hwc = &mut (*event).hw;
    if (*event).attr.type_ != (*event).pmu.type_ { return -ENOENT; }
    if !perfmon_capable() { return -EACCES; }
    if !is_sampling_event(event) || has_branch_stack(event) { return -EOPNOTSUPP; }
    match (*event).attr.config {
        DTL_LOG_CEDE | DTL_LOG_PREEMPT | DTL_LOG_FAULT | DTL_LOG_ALL => (),
        _ => return -EINVAL,
    }
    spin_lock(&mut DTL_GLOBAL_LOCK);
    if DTL_GLOBAL_REFC == 0 && !down_write_trylock(&mut dtl_access_lock) { spin_unlock(&mut DTL_GLOBAL_LOCK); return -EBUSY; }
    if vpa_dtl_mem_alloc((*event).cpu) != 0 { spin_unlock(&mut DTL_GLOBAL_LOCK); return -ENOMEM; }
    DTL_GLOBAL_REFC += 1;
    spin_unlock(&mut DTL_GLOBAL_LOCK);
    hrtimer_setup(&mut hwc.hrtimer, vpa_dtl_hrtimer_handle, CLOCK_MONOTONIC, HRTIMER_MODE_REL);
    if (*event).attr.freq {
        let freq = (*event).attr.sample_freq;
        (*event).attr.sample_period = NSEC_PER_SEC / freq;
        hwc.sample_period = (*event).attr.sample_period;
        local64_set(&mut hwc.period_left, hwc.sample_period);
        hwc.last_period = hwc.sample_period;
        (*event).attr.freq = false;
    }
    (*event).destroy = Some(vpa_dtl_reset_global_refc);
    0
}

unsafe fn vpa_dtl_event_add(event: *mut PerfEvent, _flags: i32) -> i32 {
    let dtl = &mut per_cpu(&mut VPA_DTL_CPU, (*event).cpu);
    let p = dtl.buf as *mut u32;
    (*p.add(1)) = cpu_to_be32(DISPATCH_LOG_BYTES);
    dtl.last_idx = 0;
    let hwcpu = get_hard_smp_processor_id((*event).cpu);
    let ret = register_dtl(hwcpu, __pa(dtl.buf));
    if ret != 0 { pr_warn!("DTL registration for cpu %d (hw %d) failed with %d\n", (*event).cpu, hwcpu, ret); return ret; }
    lppaca_of((*event).cpu).dtl_idx = 0;
    smp_wmb();
    lppaca_of((*event).cpu).dtl_enable_mask = (*event).attr.config;
    vpa_dtl_start_hrtimer(event);
    0
}

unsafe fn vpa_dtl_event_del(event: *mut PerfEvent, _flags: i32) {
    let hwcpu = get_hard_smp_processor_id((*event).cpu);
    let dtl = &mut per_cpu(&mut VPA_DTL_CPU, (*event).cpu);
    vpa_dtl_stop_hrtimer(event);
    unregister_dtl(hwcpu);
    kmem_cache_free(dtl_cache, dtl.buf as *mut core::ffi::c_void);
    dtl.buf = core::ptr::null_mut();
    lppaca_of((*event).cpu).dtl_enable_mask = 0;
}

/* Empty: vpa_dtl_dump_sample_data parses and dumps dispatch trace log data. */
unsafe fn vpa_dtl_event_read(_event: *mut PerfEvent) {}

unsafe fn vpa_dtl_setup_aux(event: *mut PerfEvent, pages: *mut *mut core::ffi::c_void, nr_pages: i32, _snapshot: bool) -> *mut core::ffi::c_void {
    let mut cpu = (*event).cpu;
    if nr_pages == 0 { return core::ptr::null_mut(); }
    if cpu == -1 { cpu = raw_smp_processor_id(); }
    let buf = kzalloc_node(core::mem::size_of::<VpaPmuBuf>(), GFP_KERNEL, cpu_to_node(cpu)) as *mut VpaPmuBuf;
    if buf.is_null() { return core::ptr::null_mut(); }
    let pglist = kzalloc_objs!(PagePtr, nr_pages as usize);
    if pglist.is_null() { return core::ptr::null_mut(); }
    for i in 0..nr_pages as usize { *pglist.add(i) = virt_to_page(*pages.add(i)); }
    (*buf).base = vmap(pglist, nr_pages, VM_MAP, PAGE_KERNEL) as *mut u64;
    if (*buf).base.is_null() { return core::ptr::null_mut(); }
    (*buf).nr_pages = nr_pages;
    (*buf).snapshot = false;
    (*buf).size = (nr_pages as u64) << PAGE_SHIFT;
    (*buf).head = 0; (*buf).head_size = 0; (*buf).boottb_freq_saved = 0;
    (*buf).threshold = ((*buf).size - 32) / core::mem::size_of::<DtlEntry>() as u64;
    buf as *mut core::ffi::c_void
}

unsafe fn vpa_dtl_free_aux(aux: *mut core::ffi::c_void) {
    let buf = aux as *mut VpaPmuBuf;
    vunmap((*buf).base as *mut core::ffi::c_void);
    kfree(buf as *mut core::ffi::c_void);
}

static mut VPA_DTL_PMU: Pmu = Pmu {
    task_ctx_nr: perf_invalid_context,
    name: "vpa_dtl",
    attr_groups: attr_groups,
    event_init: Some(vpa_dtl_event_init),
    add: Some(vpa_dtl_event_add),
    del: Some(vpa_dtl_event_del),
    read: Some(vpa_dtl_event_read),
    setup_aux: Some(vpa_dtl_setup_aux),
    free_aux: Some(vpa_dtl_free_aux),
    capabilities: PERF_PMU_CAP_NO_EXCLUDE | PERF_PMU_CAP_EXCLUSIVE,
};

unsafe fn vpa_dtl_init() -> i32 {
    if !firmware_has_feature(FW_FEATURE_SPLPAR) { pr_debug!("not a shared virtualized system, not enabling\n"); return -ENODEV; }
    if is_kvm_guest() { pr_debug!("Only supported for L1 host system\n"); return -ENODEV; }
    perf_pmu_register(&mut VPA_DTL_PMU, VPA_DTL_PMU.name, -1)
}

// device_initcall(vpa_dtl_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
