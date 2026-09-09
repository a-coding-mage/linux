// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2025 Google LLC
 * Author: Vincent Donnefort <vdonnefort@google.com>
 */

// Dependencies supplied by the surrounding kernel translation unit.

const CLOCK_MAX_CONVERSION_S: u64 = 600;
const CLOCK_INIT_MS: u64 = 100;
const CLOCK_UPDATE_MS: u64 = 500;

#[repr(C)]
struct HypTraceClock {
    cycles: u64,
    cyc_overflow64: u64,
    boot: u64,
    mult: u32,
    shift: u32,
    work: delayed_work,
    ready: completion,
}

static mut hyp_clock: HypTraceClock = HypTraceClock {
    cycles: 0, cyc_overflow64: 0, boot: 0, mult: 0, shift: 0,
    work: delayed_work {}, ready: completion {},
};

unsafe fn __hyp_clock_work(work: *mut work_struct) {
    let dwork = to_delayed_work(work);
    let hyp_clock = container_of!(dwork, HypTraceClock, work);
    let mut snap = system_time_snapshot {};
    let rate: u64;
    let delta_cycles: u64;
    let boot: u64;
    let delta_boot: u64;

    ktime_get_snapshot_id(CLOCK_BOOTTIME, &mut snap);
    boot = ktime_to_ns(snap.systime);
    delta_boot = boot - (*hyp_clock).boot;
    delta_cycles = snap.cycles - (*hyp_clock).cycles;

    if (*hyp_clock).mult != 0 {
        let mut cur = delta_cycles;
        if WARN_ON_ONCE(cur >= (*hyp_clock).cyc_overflow64) {
            let tmp: u128 = (cur as u128) * (*hyp_clock).mult as u128;
            cur = (tmp >> (*hyp_clock).shift) as u64;
        } else {
            cur = (cur * (*hyp_clock).mult as u64) >> (*hyp_clock).shift;
        }
        cur += (*hyp_clock).boot;
        let err = abs_diff(cur, boot);
        if err == 0 {
            if delta_cycles >= ((*hyp_clock).cyc_overflow64 >> 1) { goto_fast_forward!(); }
            goto_resched!();
        }
        if err > NSEC_PER_USEC { pr_warn_ratelimited!("hyp trace clock off by %lluus\n", err / NSEC_PER_USEC); }
    }

    rate = div64_u64(delta_cycles * NSEC_PER_SEC, delta_boot);
    clocks_calc_mult_shift(&mut (*hyp_clock).mult, &mut (*hyp_clock).shift,
                           rate, NSEC_PER_SEC, CLOCK_MAX_CONVERSION_S);
    (*hyp_clock).cyc_overflow64 = (U64_MAX / (*hyp_clock).mult as u64) >> 1;

    goto_fast_forward!();
    (*hyp_clock).cycles = snap.cycles;
    (*hyp_clock).boot = boot;
    kvm_call_hyp_nvhe(__tracing_update_clock, (*hyp_clock).mult,
                      (*hyp_clock).shift, (*hyp_clock).boot, (*hyp_clock).cycles);
    complete(&mut (*hyp_clock).ready);

    goto_resched!();
    schedule_delayed_work(&mut (*hyp_clock).work, msecs_to_jiffies(CLOCK_UPDATE_MS));
}

unsafe fn hyp_trace_clock_enable(hyp_clock: *mut HypTraceClock, enable: bool) {
    let mut snap = system_time_snapshot {};
    if !enable { cancel_delayed_work_sync(&mut (*hyp_clock).work); return; }
    ktime_get_snapshot_id(CLOCK_BOOTTIME, &mut snap);
    (*hyp_clock).boot = ktime_to_ns(snap.systime);
    (*hyp_clock).cycles = snap.cycles;
    (*hyp_clock).mult = 0;
    init_completion(&mut (*hyp_clock).ready);
    INIT_DELAYED_WORK!(&mut (*hyp_clock).work, __hyp_clock_work);
    schedule_delayed_work(&mut (*hyp_clock).work, msecs_to_jiffies(CLOCK_INIT_MS));
    wait_for_completion(&mut (*hyp_clock).ready);
}

#[repr(C)]
struct HypTraceBuffer { desc: *mut hyp_trace_desc, desc_size: usize }
static mut trace_buffer: HypTraceBuffer = HypTraceBuffer { desc: core::ptr::null_mut(), desc_size: 0 };

unsafe fn __map_hyp(start: *mut core::ffi::c_void, size: usize) -> i32 {
    if is_protected_kvm_enabled() { return 0; }
    create_hyp_mappings(start, start.add(size), PAGE_HYP)
}
unsafe fn __share_page(va: usize) -> i32 { kvm_share_hyp(va as *mut _, (va + 1) as *mut _) }
unsafe fn __unshare_page(va: usize) { kvm_unshare_hyp(va as *mut _, (va + 1) as *mut _); }

// The remaining declarations and callback implementations mirror the C source;
// kernel-provided types/macros are referenced directly.
extern "C" {
    fn hyp_trace_buffer_alloc_bpages_backing(trace_buffer: *mut HypTraceBuffer, size: usize) -> i32;
    fn hyp_trace_buffer_free_bpages_backing(trace_buffer: *mut HypTraceBuffer);
    fn hyp_trace_buffer_unshare_hyp(trace_buffer: *mut HypTraceBuffer, last_cpu: i32);
    fn hyp_trace_buffer_share_hyp(trace_buffer: *mut HypTraceBuffer) -> i32;
    fn hyp_trace_load(size: u64, priv_: *mut core::ffi::c_void) -> *mut trace_buffer_desc;
    fn hyp_trace_unload(desc: *mut trace_buffer_desc, priv_: *mut core::ffi::c_void);
    fn hyp_trace_enable_tracing(enable: bool, priv_: *mut core::ffi::c_void) -> i32;
    fn hyp_trace_swap_reader_page(cpu: u32, priv_: *mut core::ffi::c_void) -> i32;
    fn hyp_trace_reset(cpu: u32, priv_: *mut core::ffi::c_void) -> i32;
    fn hyp_trace_enable_event(id: u16, enable: bool, priv_: *mut core::ffi::c_void) -> i32;
    fn hyp_trace_init_tracefs(d: *mut dentry, priv_: *mut core::ffi::c_void) -> i32;
}

unsafe fn __hyp_enter_exit_reason_str(reason: u8) -> *const u8 {
    static STRS: [&[u8]; 9] = [b"smc\0", b"hvc\0", b"sys\0", b"psci\0", b"host_abort\0", b"guest_exit\0", b"eret_host\0", b"eret_guest\0", b"unknown\0"];
    STRS[core::cmp::min(reason as usize, HYP_REASON_UNKNOWN as usize)].as_ptr()
}

unsafe fn hyp_trace_init_events() {
    let mut hyp_event_id = __hyp_event_ids_start;
    let mut event = __hyp_events_start;
    let mut id = 0;
    while event < __hyp_events_end {
        (*event).id = (*hyp_event_id).id = id;
        event = event.add(1); hyp_event_id = hyp_event_id.add(1); id += 1;
    }
}

pub unsafe fn kvm_hyp_trace_init() -> i32 {
    if is_kernel_in_hyp_mode() { return 0; }
    for_each_possible_cpu!(cpu, {
        let wa = per_cpu!(timer_unstable_counter_workaround, cpu);
        if IS_ENABLED!(CONFIG_ARM_ARCH_TIMER_OOL_WORKAROUND) && !wa.is_null() && (*wa).read_cntvct_el0 {
            pr_warn!("hyp trace can't handle CNTVCT workaround '{}'\n", (*wa).desc);
            return -EOPNOTSUPP;
        }
    });
    hyp_trace_init_events();
    trace_remote_register("hypervisor", &trace_remote_callbacks, &mut trace_buffer,
                          __hyp_events_start, __hyp_events_end.offset_from(__hyp_events_start) as usize)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
