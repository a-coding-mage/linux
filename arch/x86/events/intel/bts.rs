// SPDX-License-Identifier: GPL-2.0-only
/*
 * BTS PMU driver for perf
 * Copyright (c) 2013-2014, Intel Corporation.
 */

// #define pr_fmt(fmt) KBUILD_MODNAME ": " fmt
// Linux and architecture headers are supplied by the surrounding translation unit.

#[repr(C)]
pub struct bts_ctx {
    pub handle: perf_output_handle,
    pub ds_back: debug_store,
    pub state: c_int,
}

pub const BTS_STATE_STOPPED: c_int = 0;
pub const BTS_STATE_INACTIVE: c_int = 1;
pub const BTS_STATE_ACTIVE: c_int = 2;

static mut bts_ctx: *mut bts_ctx = core::ptr::null_mut();

pub const BTS_RECORD_SIZE: usize = 24;
pub const BTS_SAFETY_MARGIN: usize = 4080;

#[repr(C)]
pub struct bts_phys {
    pub page: *mut page,
    pub size: c_ulong,
    pub offset: c_ulong,
    pub displacement: c_ulong,
}

#[repr(C)]
pub struct bts_buffer {
    pub real_size: usize,
    pub nr_pages: c_uint,
    pub nr_bufs: c_uint,
    pub cur_buf: c_uint,
    pub snapshot: bool,
    pub data_size: local_t,
    pub head: local_t,
    pub end: c_ulong,
    pub data_pages: *mut *mut c_void,
    pub buf: [bts_phys; 0],
}

static mut bts_pmu: pmu = core::mem::zeroed();

unsafe fn buf_nr_pages(page: *mut page) -> c_int {
    if !PagePrivate(page) { 1 } else { 1 << page_private(page) }
}

unsafe fn buf_size(page: *mut page) -> usize {
    (buf_nr_pages(page) as usize) * PAGE_SIZE
}

unsafe fn bts_buffer_setup_aux(event: *mut perf_event, pages: *mut *mut c_void,
                               nr_pages: c_int, overwrite: bool) -> *mut c_void {
    let cpu = (*event).cpu;
    let node = if cpu == -1 { cpu } else { cpu_to_node(cpu) };
    let size = (nr_pages as usize) << PAGE_SHIFT;
    let mut pg = 0;
    let mut nr_buf = 0;
    while pg < nr_pages {
        let p = virt_to_page(*pages.add(pg as usize));
        pg += buf_nr_pages(p);
        nr_buf += 1;
    }
    if overwrite && nr_buf > 1 { return core::ptr::null_mut(); }
    let bb = kzalloc_node(struct_size::<bts_buffer>(nr_buf as usize), GFP_KERNEL, node) as *mut bts_buffer;
    if bb.is_null() { return core::ptr::null_mut(); }
    (*bb).nr_pages = nr_pages as c_uint;
    (*bb).nr_bufs = nr_buf as c_uint;
    (*bb).snapshot = overwrite;
    (*bb).data_pages = pages;
    (*bb).real_size = size - size % BTS_RECORD_SIZE;
    pg = 0;
    nr_buf = 0;
    let mut offset = 0usize;
    let mut pad = 0usize;
    while nr_buf < (*bb).nr_bufs as c_int {
        let page = virt_to_page(*pages.add(pg as usize));
        let nr = buf_nr_pages(page) as usize;
        let phys = &mut (*bb).buf.as_mut_ptr().add(nr_buf as usize).as_mut().unwrap();
        (*phys).page = page;
        (*phys).offset = offset as c_ulong;
        (*phys).displacement = if pad != 0 { (BTS_RECORD_SIZE - pad) as c_ulong } else { 0 };
        (*phys).size = (buf_size(page) as c_ulong) - (*phys).displacement;
        pad = ((*phys).size as usize) % BTS_RECORD_SIZE;
        (*phys).size -= pad as c_ulong;
        pg += nr as c_int;
        offset += nr << PAGE_SHIFT;
        nr_buf += 1;
    }
    bb as *mut c_void
}

unsafe fn bts_buffer_free_aux(data: *mut c_void) { kfree(data); }

unsafe fn bts_buffer_offset(bb: *mut bts_buffer, idx: c_uint) -> c_ulong {
    (*bb).buf.as_ptr().add(idx as usize).read().offset + (*bb).buf.as_ptr().add(idx as usize).read().displacement
}

unsafe fn bts_config_buffer(bb: *mut bts_buffer) {
    let cpu = raw_smp_processor_id();
    let ds = per_cpu(cpu_hw_events, cpu).ds;
    let phys = &(*bb).buf.as_ptr().add((*bb).cur_buf as usize).read();
    let mut index = local_read(&(*bb).head) as c_ulong;
    let mut thresh = 0;
    let mut end = phys.size;
    if !(*bb).snapshot {
        if (*bb).end < phys.offset + buf_size(phys.page) as c_ulong { end = (*bb).end - phys.offset - phys.displacement; }
        index -= phys.offset + phys.displacement;
        thresh = if end - index > BTS_SAFETY_MARGIN as c_ulong { end - BTS_SAFETY_MARGIN as c_ulong } else if end - index > BTS_RECORD_SIZE as c_ulong { end - BTS_RECORD_SIZE as c_ulong } else { end };
    }
    (*ds).bts_buffer_base = (page_address(phys.page) as c_ulong as u64) + phys.displacement as u64;
    (*ds).bts_index = (*ds).bts_buffer_base + index as u64;
    (*ds).bts_absolute_maximum = (*ds).bts_buffer_base + end as u64;
    (*ds).bts_interrupt_threshold = if !(*bb).snapshot { (*ds).bts_buffer_base + thresh as u64 } else { (*ds).bts_absolute_maximum + BTS_RECORD_SIZE as u64 };
}

unsafe fn bts_buffer_pad_out(phys: *mut bts_phys, head: c_ulong) {
    memset((page_address((*phys).page) as c_ulong + (head - (*phys).offset)) as *mut c_void, 0, ((*phys).size - (head - (*phys).offset)) as usize);
}

unsafe fn bts_update(bts: *mut bts_ctx) {
    let cpu = raw_smp_processor_id();
    let ds = per_cpu(cpu_hw_events, cpu).ds;
    let bb = perf_get_aux(&mut (*bts).handle);
    if bb.is_null() { return; }
    let index = ((*ds).bts_index - (*ds).bts_buffer_base) as c_ulong;
    let head = index + bts_buffer_offset(bb, (*bb).cur_buf);
    let old = local_xchg(&mut (*bb).head, head as _);
    if !(*bb).snapshot {
        if old == head as _ { return; }
        if (*ds).bts_index >= (*ds).bts_absolute_maximum { perf_aux_output_flag(&mut (*bts).handle, PERF_AUX_FLAG_TRUNCATED); }
        local_add((head as _).wrapping_sub(old), &mut (*bb).data_size);
    } else { local_set(&mut (*bb).data_size, head as _); }
    barrier();
}

unsafe fn bts_buffer_reset(bb: *mut bts_buffer, handle: *mut perf_output_handle) -> c_int;

unsafe fn __bts_event_start(event: *mut perf_event) {
    let bts = this_cpu_ptr(bts_ctx);
    let bb = perf_get_aux(&mut (*bts).handle);
    let mut config: u64 = 0;
    if !(*bb).snapshot { config |= ARCH_PERFMON_EVENTSEL_INT as u64; }
    if !(*event).attr.exclude_kernel { config |= ARCH_PERFMON_EVENTSEL_OS as u64; }
    if !(*event).attr.exclude_user { config |= ARCH_PERFMON_EVENTSEL_USR as u64; }
    bts_config_buffer(bb);
    wmb();
    WRITE_ONCE((*bts).state, BTS_STATE_ACTIVE);
    intel_pmu_enable_bts(config);
}

unsafe fn bts_event_start(event: *mut perf_event, _flags: c_int) {
    let cpuc = this_cpu_ptr(&mut cpu_hw_events);
    let bts = this_cpu_ptr(bts_ctx);
    let bb = perf_aux_output_begin(&mut (*bts).handle, event);
    if bb.is_null() { (*event).hw.state = PERF_HES_STOPPED; return; }
    if bts_buffer_reset(bb, &mut (*bts).handle) != 0 { perf_aux_output_end(&mut (*bts).handle, 0); (*event).hw.state = PERF_HES_STOPPED; return; }
    (*bts).ds_back.bts_buffer_base = (*(*cpuc).ds).bts_buffer_base;
    (*bts).ds_back.bts_absolute_maximum = (*(*cpuc).ds).bts_absolute_maximum;
    (*bts).ds_back.bts_interrupt_threshold = (*(*cpuc).ds).bts_interrupt_threshold;
    perf_event_itrace_started(event);
    (*event).hw.state = 0;
    __bts_event_start(event);
}

unsafe fn __bts_event_stop(_event: *mut perf_event, state: c_int) {
    let bts = this_cpu_ptr(bts_ctx);
    WRITE_ONCE((*bts).state, state);
    intel_pmu_disable_bts();
}

unsafe fn bts_event_stop(event: *mut perf_event, flags: c_int) {
    let cpuc = this_cpu_ptr(&mut cpu_hw_events);
    let bts = this_cpu_ptr(bts_ctx);
    let mut bb: *mut bts_buffer = core::ptr::null_mut();
    let state = READ_ONCE((*bts).state);
    if state == BTS_STATE_ACTIVE { __bts_event_stop(event, BTS_STATE_STOPPED); }
    if state != BTS_STATE_STOPPED { bb = perf_get_aux(&mut (*bts).handle); }
    (*event).hw.state |= PERF_HES_STOPPED;
    if flags & PERF_EF_UPDATE != 0 {
        bts_update(bts);
        if !bb.is_null() {
            if (*bb).snapshot { (*bts).handle.head = local_xchg(&mut (*bb).data_size, ((*bb).nr_pages as usize) << PAGE_SHIFT); }
            perf_aux_output_end(&mut (*bts).handle, local_xchg(&mut (*bb).data_size, 0));
        }
        (*(*cpuc).ds).bts_index = (*bts).ds_back.bts_buffer_base;
        (*(*cpuc).ds).bts_buffer_base = (*bts).ds_back.bts_buffer_base;
        (*(*cpuc).ds).bts_absolute_maximum = (*bts).ds_back.bts_absolute_maximum;
        (*(*cpuc).ds).bts_interrupt_threshold = (*bts).ds_back.bts_interrupt_threshold;
    }
}

pub unsafe fn intel_bts_enable_local() {
    if bts_ctx.is_null() { return; }
    let bts = this_cpu_ptr(bts_ctx);
    let state = READ_ONCE((*bts).state);
    if WARN_ON_ONCE(state == BTS_STATE_ACTIVE) || state == BTS_STATE_STOPPED { return; }
    if !(*bts).handle.event.is_null() { __bts_event_start((*bts).handle.event); }
}

pub unsafe fn intel_bts_disable_local() {
    if bts_ctx.is_null() { return; }
    let bts = this_cpu_ptr(bts_ctx);
    if READ_ONCE((*bts).state) != BTS_STATE_ACTIVE { return; }
    if !(*bts).handle.event.is_null() { __bts_event_stop((*bts).handle.event, BTS_STATE_INACTIVE); }
}

unsafe fn bts_buffer_reset(bb: *mut bts_buffer, handle: *mut perf_output_handle) -> c_int {
    if (*bb).snapshot { return 0; }
    let mut head = (*handle).head & (((*bb).nr_pages as usize) << PAGE_SHIFT).wrapping_sub(1);
    let mut cur = (*bb).cur_buf;
    let mut phys = (*bb).buf.as_mut_ptr().add(cur as usize);
    let mut space = (*phys).offset + (*phys).displacement + (*phys).size - head as c_ulong;
    let pad = space;
    if space > (*handle).size as c_ulong { space = (*handle).size as c_ulong; space -= space % BTS_RECORD_SIZE as c_ulong; }
    if space <= BTS_SAFETY_MARGIN as c_ulong {
        let next = if cur + 1 >= (*bb).nr_bufs { 0 } else { cur + 1 };
        let next_phys = (*bb).buf.as_mut_ptr().add(next as usize);
        let gap = buf_size((*phys).page) as c_ulong - (*phys).displacement - (*phys).size + (*next_phys).displacement;
        let skip = pad + gap;
        if (*handle).size as c_ulong >= skip {
            let mut next_space = (*next_phys).size;
            if next_space + skip > (*handle).size as c_ulong { next_space = (*handle).size as c_ulong - skip; next_space -= next_space % BTS_RECORD_SIZE as c_ulong; }
            if next_space > space || space == 0 {
                if pad != 0 { bts_buffer_pad_out(phys, head as c_ulong); }
                let ret = perf_aux_output_skip(handle, skip as _); if ret != 0 { return ret; }
                phys = next_phys; space = next_space; head = (*phys).offset + (*phys).displacement; cur = next; (*bb).cur_buf = cur; local_set(&mut (*bb).head, head as _);
            }
        }
    }
    let mut wakeup = BTS_SAFETY_MARGIN as c_ulong + BTS_RECORD_SIZE as c_ulong + (*handle).wakeup as c_ulong - (*handle).head as c_ulong;
    if space > wakeup { space = wakeup; space -= space % BTS_RECORD_SIZE as c_ulong; }
    (*bb).end = head as c_ulong + space;
    if space == 0 { return -ENOSPC; }
    0
}

pub unsafe fn intel_bts_interrupt() -> c_int {
    let ds = (*this_cpu_ptr(&mut cpu_hw_events)).ds;
    if bts_ctx.is_null() { return 0; }
    let bts = this_cpu_ptr(bts_ctx);
    let event = (*bts).handle.event;
    let mut handled = 0;
    if !ds.is_null() && (*ds).bts_index >= (*ds).bts_interrupt_threshold { handled = 1; }
    if READ_ONCE((*bts).state) == BTS_STATE_STOPPED { return handled; }
    let bb = perf_get_aux(&mut (*bts).handle); if bb.is_null() { return handled; }
    if (*bb).snapshot { return 0; }
    let old_head = local_read(&(*bb).head);
    bts_update(bts);
    if old_head == local_read(&(*bb).head) { return handled; }
    perf_aux_output_end(&mut (*bts).handle, local_xchg(&mut (*bb).data_size, 0));
    let bb = perf_aux_output_begin(&mut (*bts).handle, event);
    let err = if !bb.is_null() { bts_buffer_reset(bb, &mut (*bts).handle) } else { -ENOSPC };
    if err != 0 { WRITE_ONCE((*bts).state, BTS_STATE_STOPPED); if !bb.is_null() { barrier(); perf_aux_output_end(&mut (*bts).handle, 0); } }
    1
}

unsafe fn bts_event_del(event: *mut perf_event, _mode: c_int) { bts_event_stop(event, PERF_EF_UPDATE); }
unsafe fn bts_event_add(event: *mut perf_event, mode: c_int) -> c_int {
    let bts = this_cpu_ptr(bts_ctx); let cpuc = this_cpu_ptr(&mut cpu_hw_events); let hwc = &mut (*event).hw;
    (*event).hw.state = PERF_HES_STOPPED;
    if test_bit(INTEL_PMC_IDX_FIXED_BTS, (*cpuc).active_mask) != 0 || !(*bts).handle.event.is_null() { return -EBUSY; }
    if mode & PERF_EF_START != 0 { bts_event_start(event, 0); if hwc.state & PERF_HES_STOPPED != 0 { return -EINVAL; } }
    0
}
unsafe fn bts_event_destroy(_event: *mut perf_event) { x86_release_hardware(); x86_del_exclusive(x86_lbr_exclusive_bts); }
unsafe fn bts_event_init(event: *mut perf_event) -> c_int {
    if (*event).attr.type_ != bts_pmu.type_ { return -ENOENT; }
    if (*event).attr.exclude_kernel { let ret = perf_allow_kernel(); if ret != 0 { return ret; } }
    if x86_add_exclusive(x86_lbr_exclusive_bts) != 0 { return -EBUSY; }
    let ret = x86_reserve_hardware(); if ret != 0 { x86_del_exclusive(x86_lbr_exclusive_bts); return ret; }
    (*event).destroy = Some(bts_event_destroy); 0
}
unsafe fn bts_event_read(_event: *mut perf_event) {}

// Initialization is retained as a declaration-level translation; feature and PMU symbols are supplied externally.
unsafe fn bts_init() -> c_int {
    if !boot_cpu_has(X86_FEATURE_DTES64) { return -ENODEV; }
    x86_pmu.bts = boot_cpu_has(X86_FEATURE_BTS); if !x86_pmu.bts { return -ENODEV; }
    if boot_cpu_has(X86_FEATURE_PTI) { return -ENODEV; }
    bts_ctx = alloc_percpu::<bts_ctx>(); if bts_ctx.is_null() { return -ENOMEM; }
    bts_pmu.capabilities = PERF_PMU_CAP_AUX_NO_SG | PERF_PMU_CAP_ITRACE | PERF_PMU_CAP_EXCLUSIVE;
    bts_pmu.task_ctx_nr = perf_sw_context; bts_pmu.event_init = Some(bts_event_init); bts_pmu.add = Some(bts_event_add); bts_pmu.del = Some(bts_event_del); bts_pmu.start = Some(bts_event_start); bts_pmu.stop = Some(bts_event_stop); bts_pmu.read = Some(bts_event_read); bts_pmu.setup_aux = Some(bts_buffer_setup_aux); bts_pmu.free_aux = Some(bts_buffer_free_aux);
    perf_pmu_register(&mut bts_pmu, "intel_bts", -1)
}

// early_initcall(bts_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
