// SPDX-License-Identifier: GPL-2.0+
/*
 * Restartable sequences system call
 *
 * Direct Rust translation of rseq.c. Kernel-provided types, macros, globals,
 * and functions are intentionally referenced as external dependencies.
 */

// C build-time selection: RSEQ_BUILD_SLOW_PATH

#[allow(non_upper_case_globals)]
static mut rseq_debug_enabled: StaticKeyMaybe = StaticKeyMaybe::new();

#[inline]
unsafe fn rseq_control_debug(on: bool) {
    if on { static_branch_enable(&mut rseq_debug_enabled); }
    else { static_branch_disable(&mut rseq_debug_enabled); }
}

unsafe fn rseq_setup_debug(str_: *mut core::ffi::c_char) -> i32 {
    let mut on = false;
    if kstrtobool(str_, &mut on) != 0 { return -EINVAL; }
    rseq_control_debug(on);
    1
}

#[cfg(CONFIG_TRACEPOINTS)]
pub unsafe fn __rseq_trace_update(t: *mut task_struct) { trace_rseq_update(t); }

#[cfg(CONFIG_TRACEPOINTS)]
pub unsafe fn __rseq_trace_ip_fixup(ip: usize, start_ip: usize, offset: usize, abort_ip: usize) {
    trace_rseq_ip_fixup(ip, start_ip, offset, abort_ip);
}

#[cfg(CONFIG_RSEQ_STATS)]
static mut rseq_stats: PerCpu<rseq_stats> = PerCpu::new();

#[cfg(CONFIG_RSEQ_STATS)]
unsafe fn rseq_stats_show(m: *mut seq_file, _p: *mut core::ffi::c_void) -> i32 {
    let mut stats = rseq_stats { ..Default::default() };
    let mut cpu: u32 = 0;
    for_each_possible_cpu!(cpu) {
        stats.exit = stats.exit.wrapping_add(data_race(per_cpu!(rseq_stats.exit, cpu)));
        stats.signal = stats.signal.wrapping_add(data_race(per_cpu!(rseq_stats.signal, cpu)));
        stats.slowpath = stats.slowpath.wrapping_add(data_race(per_cpu!(rseq_stats.slowpath, cpu)));
        stats.fastpath = stats.fastpath.wrapping_add(data_race(per_cpu!(rseq_stats.fastpath, cpu)));
        stats.ids = stats.ids.wrapping_add(data_race(per_cpu!(rseq_stats.ids, cpu)));
        stats.cs = stats.cs.wrapping_add(data_race(per_cpu!(rseq_stats.cs, cpu)));
        stats.clear = stats.clear.wrapping_add(data_race(per_cpu!(rseq_stats.clear, cpu)));
        stats.fixup = stats.fixup.wrapping_add(data_race(per_cpu!(rseq_stats.fixup, cpu)));
        if cfg!(CONFIG_RSEQ_SLICE_EXTENSION) {
            stats.s_granted = stats.s_granted.wrapping_add(data_race(per_cpu!(rseq_stats.s_granted, cpu)));
            stats.s_expired = stats.s_expired.wrapping_add(data_race(per_cpu!(rseq_stats.s_expired, cpu)));
            stats.s_revoked = stats.s_revoked.wrapping_add(data_race(per_cpu!(rseq_stats.s_revoked, cpu)));
            stats.s_yielded = stats.s_yielded.wrapping_add(data_race(per_cpu!(rseq_stats.s_yielded, cpu)));
            stats.s_aborted = stats.s_aborted.wrapping_add(data_race(per_cpu!(rseq_stats.s_aborted, cpu)));
        }
    }
    seq_printf(m, "exit:   %16lu\n", stats.exit);
    seq_printf(m, "signal: %16lu\n", stats.signal);
    seq_printf(m, "slowp:  %16lu\n", stats.slowpath);
    seq_printf(m, "fastp:  %16lu\n", stats.fastpath);
    seq_printf(m, "ids:    %16lu\n", stats.ids);
    seq_printf(m, "cs:     %16lu\n", stats.cs);
    seq_printf(m, "clear:  %16lu\n", stats.clear);
    seq_printf(m, "fixup:  %16lu\n", stats.fixup);
    if cfg!(CONFIG_RSEQ_SLICE_EXTENSION) {
        seq_printf(m, "sgrant: %16lu\n", stats.s_granted);
        seq_printf(m, "sexpir: %16lu\n", stats.s_expired);
        seq_printf(m, "srevok: %16lu\n", stats.s_revoked);
        seq_printf(m, "syield: %16lu\n", stats.s_yielded);
        seq_printf(m, "sabort: %16lu\n", stats.s_aborted);
    }
    0
}

#[cfg(not(CONFIG_RSEQ_STATS))]
#[inline] unsafe fn rseq_stats_init(_root_dir: *mut dentry) {}

unsafe fn rseq_debug_show(m: *mut seq_file, _p: *mut core::ffi::c_void) -> i32 {
    let on = static_branch_unlikely(&rseq_debug_enabled);
    seq_printf(m, "%d\n", on as i32);
    0
}

unsafe fn rseq_debug_write(_file: *mut file, ubuf: *const core::ffi::c_char, count: usize, _ppos: *mut loff_t) -> isize {
    let mut on = false;
    if kstrtobool_from_user(ubuf, count, &mut on) != 0 { return -EINVAL as isize; }
    rseq_control_debug(on);
    count as isize
}

unsafe fn rseq_debugfs_init() -> i32 {
    let root_dir = debugfs_create_dir(c"rseq".as_ptr(), core::ptr::null_mut());
    debugfs_create_file(c"debug".as_ptr(), 0o644, root_dir, core::ptr::null_mut(), &debug_ops);
    rseq_stats_init(root_dir);
    if cfg!(CONFIG_RSEQ_SLICE_EXTENSION) { rseq_slice_ext_init(root_dir); }
    0
}

unsafe fn rseq_handle_cs(t: *mut task_struct, regs: *mut pt_regs) -> bool {
    let urseq = (*t).rseq.usrptr;
    let mut csaddr: u64 = 0;
    if unsafe_get_user(&mut csaddr, &(*urseq).rseq_cs) != 0 { return false; }
    if csaddr == 0 { return true; }
    rseq_update_user_cs(t, regs, csaddr)
}

unsafe fn rseq_slowpath_update_usr(regs: *mut pt_regs) {
    let evt_mask = rseq_event { has_rseq: RSEQ_HAS_RSEQ_VERSION_MASK, user_irq: true, ..Default::default() };
    let t = current;
    let mut ids = rseq_ids::default();
    if (*t).flags & PF_EXITING != 0 { return; }
    rseq_stat_inc(rseq_stats.slowpath);
    let event;
    scoped_guard_irq! {
        event = (*t).rseq.event.sched_switch;
        (*t).rseq.event.all &= evt_mask.all;
        ids.cpu_id = task_cpu(t);
        ids.mm_cid = task_mm_cid(t);
    }
    if !event { return; }
    ids.node_id = cpu_to_node(ids.cpu_id);
    if !rseq_update_usr(t, regs, &ids) { (*t).rseq.event.error = 0; force_sig(SIGSEGV); }
}

pub unsafe fn __rseq_handle_slowpath(regs: *mut pt_regs) { if !regs.is_null() { rseq_slowpath_update_usr(regs); } }

pub unsafe fn __rseq_signal_deliver(sig: i32, regs: *mut pt_regs) {
    rseq_stat_inc(rseq_stats.signal);
    if !rseq_handle_cs(current, regs) { (*current).rseq.event.error = 0; force_sigsegv(sig); }
    if !rseq_v2(current) { rseq_force_update(); }
}

pub unsafe fn __rseq_debug_syscall_return(regs: *mut pt_regs) {
    let t = current;
    let mut csaddr = 0u64;
    if !(*t).rseq.event.has_rseq { return; }
    if get_user(&mut csaddr, &(*t).rseq.usrptr.as_ref().unwrap().rseq_cs) != 0 || (csaddr != 0 && (csaddr >= TASK_SIZE && !rseq_debug_update_user_cs(t, regs, csaddr))) { force_sig(SIGSEGV); }
}

#[cfg(CONFIG_DEBUG_RSEQ)]
pub unsafe fn rseq_syscall(regs: *mut pt_regs) { __rseq_debug_syscall_return(regs); }

unsafe fn rseq_reset_ids() -> bool {
    let rseq = (*current).rseq.usrptr;
    if unsafe_put_user(0, &mut (*rseq).cpu_id_start) != 0 || unsafe_put_user(RSEQ_CPU_ID_UNINITIALIZED, &mut (*rseq).cpu_id) != 0 || unsafe_put_user(0, &mut (*rseq).node_id) != 0 || unsafe_put_user(0, &mut (*rseq).mm_cid) != 0 { force_sig(SIGSEGV); return false; }
    true
}

const ORIG_RSEQ_SIZE: u32 = 32;
const RSEQ_FLAGS_SUPPORTED: u32 = RSEQ_FLAG_SLICE_EXT_DEFAULT_ON;

unsafe fn rseq_register(rseq: *mut rseq, rseq_len: u32, flags: i32, sig: u32) -> i64 {
    if !access_ok(rseq, rseq_len) { return -EFAULT; }
    let version = if cfg!(CONFIG_GENERIC_IRQ_ENTRY) && rseq_len > ORIG_RSEQ_SIZE { 2 } else { 1 };
    let mut rseqfl = 0u32;
    if cfg!(CONFIG_RSEQ_SLICE_EXTENSION) && version > 1 && rseq_slice_extension_enabled() { rseqfl |= RSEQ_CS_FLAG_SLICE_EXT_AVAILABLE; if flags & RSEQ_FLAG_SLICE_EXT_DEFAULT_ON as i32 != 0 { rseqfl |= RSEQ_CS_FLAG_SLICE_EXT_ENABLED; } }
    if unsafe_put_user(0u64, &mut (*rseq).rseq_cs) != 0 || unsafe_put_user(rseqfl, &mut (*rseq).flags) != 0 || unsafe_put_user(RSEQ_CPU_ID_UNINITIALIZED, &mut (*rseq).cpu_id_start) != 0 || unsafe_put_user(RSEQ_CPU_ID_UNINITIALIZED, &mut (*rseq).cpu_id) != 0 || unsafe_put_user(0u32, &mut (*rseq).node_id) != 0 || unsafe_put_user(0u32, &mut (*rseq).mm_cid) != 0 { return -EFAULT; }
    (*current).rseq.usrptr = rseq; (*current).rseq.len = rseq_len; (*current).rseq.sig = sig; (*current).rseq.event.has_rseq = version; rseq_force_update(); 0
}

unsafe fn rseq_unregister(rseq: *mut rseq, rseq_len: u32, flags: i32, sig: u32) -> i64 {
    if flags & !(RSEQ_FLAG_UNREGISTER as i32) != 0 || (*current).rseq.usrptr != rseq || rseq.is_null() || rseq_len != (*current).rseq.len { return -EINVAL; }
    if (*current).rseq.sig != sig { return -EPERM; }
    if !rseq_reset_ids() { return -EFAULT; }
    rseq_reset(current); 0
}

unsafe fn rseq_reregister(rseq: *mut rseq, rseq_len: u32, sig: u32) -> i64 {
    if (*current).rseq.usrptr != rseq || rseq_len != (*current).rseq.len { return -EINVAL; }
    if (*current).rseq.sig != sig { return -EPERM; }
    -EBUSY
}

unsafe fn rseq_length_valid(rseq: *mut rseq, rseq_len: u32) -> bool {
    if rseq_len < ORIG_RSEQ_SIZE { return false; }
    if rseq_len == ORIG_RSEQ_SIZE { IS_ALIGNED(rseq as usize, ORIG_RSEQ_SIZE as usize) }
    else { IS_ALIGNED(rseq as usize, rseq_alloc_align()) && rseq_len >= offset_of!(rseq, end) as u32 }
}

pub unsafe fn sys_rseq(rseq: *mut rseq, rseq_len: u32, flags: i32, sig: u32) -> i64 {
    if flags & RSEQ_FLAG_UNREGISTER as i32 != 0 { return rseq_unregister(rseq, rseq_len, flags, sig); }
    if flags & !(RSEQ_FLAGS_SUPPORTED as i32) != 0 { return -EINVAL; }
    if !(*current).rseq.usrptr.is_null() { return rseq_reregister(rseq, rseq_len, sig); }
    if !rseq_length_valid(rseq, rseq_len) { return -EINVAL; }
    rseq_register(rseq, rseq_len, flags, sig)
}

// CONFIG_RSEQ_SLICE_EXTENSION declarations and implementation are supplied by
// the corresponding kernel configuration and dependencies.
#[cfg(CONFIG_RSEQ_SLICE_EXTENSION)]
pub unsafe fn rseq_slice_extension_prctl(_arg2: usize, _arg3: usize) -> i32 { todo!("direct kernel dependency mapping") }

#[cfg(CONFIG_RSEQ_SLICE_EXTENSION)]
pub unsafe fn rseq_syscall_enter_work(_syscall: i64) { todo!("direct kernel dependency mapping") }

#[cfg(CONFIG_RSEQ_SLICE_EXTENSION)]
pub unsafe fn sys_rseq_slice_yield() -> i32 { todo!("direct kernel dependency mapping") }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
