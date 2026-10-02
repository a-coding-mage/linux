// SPDX-License-Identifier: GPL-2.0+
/* Restartable sequences. The syscall, event and slice-extension algorithms
 * are translated from kernel/rseq.c. The out-of-line critical-section debug
 * implementation and slow-path ID handling follow include/linux/rseq_entry.h.
 * Kernel and UAPI layouts come exclusively from the configured C headers.
 */
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
    missing_docs, unsafe_op_in_unsafe_fn, clippy::all, unreachable_pub)]
#[allow(improper_ctypes)]
mod bindings {
    use kernel::ffi;
    include!(concat!(env!("OBJTREE"), "/rust/bindings/rseq_generated.rs"));
}
use bindings::*;
use core::mem::{offset_of, size_of, zeroed};
use core::ptr::{addr_of, addr_of_mut};
#[cfg(CONFIG_RSEQ_SLICE_EXTENSION)]
use core::ptr::{read_volatile, write_volatile};
use kernel::ffi::{c_char, c_int, c_long, c_ulong, c_void};

const ORIG_RSEQ_SIZE: u32 = 32;
const STAT_SIGNAL: u32 = 1;
const STAT_SLOWPATH: u32 = 2;
const STAT_IDS: u32 = 4;
const STAT_CS: u32 = 5;
const STAT_CLEAR: u32 = 6;
const STAT_FIXUP: u32 = 7;
const STAT_EXPIRED: u32 = 9;
const STAT_REVOKED: u32 = 10;
const STAT_YIELDED: u32 = 11;
const STAT_ABORTED: u32 = 12;

#[inline]
unsafe fn event(d: *mut rseq_data) -> *mut rseq_event__bindgen_ty_1__bindgen_ty_1 {
    addr_of_mut!((*d).event.__bindgen_anon_1.__bindgen_anon_1)
}
#[inline]
unsafe fn event_bits(d: *mut rseq_data) -> *mut rseq_event__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1 {
    addr_of_mut!((*event(d)).__bindgen_anon_1.__bindgen_anon_1)
}
#[inline]
unsafe fn fatal(d: *mut rseq_data) -> bool {
    (*event(d)).__bindgen_anon_2.__bindgen_anon_1.fatal = 1;
    false
}
#[inline]
unsafe fn clear_error(d: *mut rseq_data) { (*event(d)).__bindgen_anon_2.error = 0; }
#[inline]
unsafe fn v2(d: *mut rseq_data) -> bool { cfg!(CONFIG_GENERIC_IRQ_ENTRY) && (*event(d)).has_rseq > 1 }
#[inline]
fn store32(offset: usize, value: u32) -> rust_rseq_user_write {
    rust_rseq_user_write { offset, value: value as u64, wide: false }
}
#[inline]
fn store64(offset: usize, value: u64) -> rust_rseq_user_write {
    rust_rseq_user_write { offset, value, wide: true }
}
#[inline]
unsafe fn write_fields(p: *mut rseq, fields: &[rust_rseq_user_write], rw: bool) -> bool {
    rust_rseq_write_fields(p, fields.as_ptr().cast(), fields.len(), rw)
}
#[inline]
unsafe fn clear_cs(d: *mut rseq_data) -> bool {
    write_fields((*d).usrptr, &[store64(offset_of!(rseq, rseq_cs), 0)], true)
}
#[inline]
unsafe fn stat(_field: u32) {
    #[cfg(CONFIG_RSEQ_STATS)]
    rust_rseq_stat_inc(_field);
}

#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn rust_rseq_setup_debug(s: *mut c_char) -> c_int {
    let mut on = false;
    if kstrtobool(s, &mut on) != 0 { return -(EINVAL as c_int); }
    rust_rseq_control_debug(on);
    1
}

#[cfg(CONFIG_TRACEPOINTS)]
#[no_mangle]
pub unsafe extern "C" fn __rseq_trace_update(t: *mut task_struct) { rust_rseq_trace_update(t); }
#[cfg(CONFIG_TRACEPOINTS)]
#[no_mangle]
pub unsafe extern "C" fn __rseq_trace_ip_fixup(ip: c_ulong, start: c_ulong, offset: c_ulong, abort: c_ulong) {
    rust_rseq_trace_ip_fixup(ip, start, offset, abort);
}
#[inline]
unsafe fn trace_fixup(_ip: c_ulong, _start: u64, _offset: u64, _abort: u64) {
    #[cfg(CONFIG_TRACEPOINTS)]
    rust_rseq_trace_fixup_if_enabled(_ip, _start as c_ulong, _offset as c_ulong, _abort as c_ulong);
}

/* The out-of-line debug definition emitted by rseq_entry.h in the C
 * provider. Keep the ordered, conditional user reads and unsigned wrapping
 * arithmetic: malformed descriptor checks are part of the security ABI.
 */
#[no_mangle]
pub unsafe extern "C" fn rseq_debug_update_user_cs(t: *mut task_struct, regs: *mut pt_regs, csaddr: c_ulong) -> bool {
    let d = rust_rseq_data(t);
    let ucs = csaddr as *mut rseq_cs;
    let tasksize = rust_rseq_task_size() as u64;
    let ip = rust_rseq_ip(regs);
    let (mut start, mut offset, mut abort, mut head) = (0u64, 0u64, 0u64, 0u64);
    if !rust_rseq_read_cs_u64(ucs, offset_of!(rseq_cs, start_ip), &mut start) { return false; }
    if start >= tasksize { return fatal(d); }
    if (ip as u64) < start {
        if !clear_cs(d) { return false; }
        stat(STAT_CLEAR);
        return true;
    }
    if !rust_rseq_read_cs_u64(ucs, offset_of!(rseq_cs, post_commit_offset), &mut offset) { return false; }
    let end = start.wrapping_add(offset);
    if end >= tasksize || end < start { return fatal(d); }
    if ip as u64 >= end {
        if !clear_cs(d) { return false; }
        stat(STAT_CLEAR);
        return true;
    }
    if !rust_rseq_read_cs_u64(ucs, offset_of!(rseq_cs, abort_ip), &mut abort) { return false; }
    if abort >= tasksize || abort < size_of::<u32>() as u64 { return fatal(d); }
    if abort.wrapping_sub(start) < offset { return fatal(d); }
    if !rust_rseq_read_cs_u64(ucs, 0, &mut head) { return false; }
    if head != 0 { return fatal(d); }
    let mut sig = 0u32;
    if rust_rseq_get_u32(abort.wrapping_sub(size_of::<u32>() as u64) as usize as *const u32, &mut sig) != 0 { return false; }
    if sig != (*d).sig { return fatal(d); }
    if cfg!(CONFIG_GENERIC_IRQ_ENTRY) && (*event_bits(d)).user_irq == 0 { return fatal(d); }
    if !clear_cs(d) { return false; }
    rust_rseq_set_ip(regs, abort as c_ulong);
    stat(STAT_FIXUP);
    trace_fixup(ip, start, offset, abort);
    true
}

unsafe fn update_user_cs(t: *mut task_struct, regs: *mut pt_regs, csaddr: c_ulong) -> bool {
    let d = rust_rseq_data(t);
    let ucs = csaddr as *mut rseq_cs;
    let ip = rust_rseq_ip(regs);
    let tasksize = rust_rseq_task_size();
    stat(STAT_CS);
    if csaddr >= tasksize { return fatal(d); }
    if rust_rseq_debug_enabled() { return rseq_debug_update_user_cs(t, regs, csaddr); }
    let (mut start, mut offset, mut abort) = (0u64, 0u64, 0u64);
    if !rust_rseq_read_cs_u64(ucs, offset_of!(rseq_cs, start_ip), &mut start) ||
       !rust_rseq_read_cs_u64(ucs, offset_of!(rseq_cs, post_commit_offset), &mut offset) ||
       !rust_rseq_read_cs_u64(ucs, offset_of!(rseq_cs, abort_ip), &mut abort) { return false; }
    if (ip as u64).wrapping_sub(start) >= offset {
        if !clear_cs(d) { return false; }
        stat(STAT_CLEAR);
        return true;
    }
    if abort >= tasksize as u64 || abort < size_of::<u32>() as u64 { return fatal(d); }
    let mut sig = 0u32;
    if rust_rseq_get_u32(abort.wrapping_sub(size_of::<u32>() as u64) as usize as *const u32, &mut sig) != 0 { return false; }
    if sig != (*d).sig { return fatal(d); }
    if !clear_cs(d) { return false; }
    rust_rseq_set_ip(regs, abort as c_ulong);
    stat(STAT_FIXUP);
    trace_fixup(ip, start, offset, abort);
    true
}

unsafe fn update_usr(t: *mut task_struct, regs: *mut pt_regs, ids: &rseq_ids) -> bool {
    let d = rust_rseq_data(t);
    let p = (*d).usrptr;
    if rust_rseq_debug_enabled() || v2(d) {
        let cache = &(*d).ids;
        let mut cpu = 0;
        let mut value = 0;
        if !rust_rseq_read_rseq_u32(p, offset_of!(rseq, cpu_id_start), &mut cpu) { return false; }
        if cpu != cache.__bindgen_anon_1.__bindgen_anon_1.cpu_id { return fatal(d); }
        if !rust_rseq_read_rseq_u32(p, offset_of!(rseq, cpu_id), &mut value) { return false; }
        if value != cpu { return fatal(d); }
        if !rust_rseq_read_rseq_u32(p, offset_of!(rseq, node_id), &mut value) { return false; }
        if value != cache.node_id { return fatal(d); }
        if !rust_rseq_read_rseq_u32(p, offset_of!(rseq, mm_cid), &mut value) { return false; }
        if value != cache.__bindgen_anon_1.__bindgen_anon_1.mm_cid { return fatal(d); }
    }
    let fields = [store32(offset_of!(rseq, cpu_id_start), ids.__bindgen_anon_1.__bindgen_anon_1.cpu_id),
        store32(offset_of!(rseq, cpu_id), ids.__bindgen_anon_1.__bindgen_anon_1.cpu_id),
        store32(offset_of!(rseq, node_id), ids.node_id),
        store32(offset_of!(rseq, mm_cid), ids.__bindgen_anon_1.__bindgen_anon_1.mm_cid)];
    if !write_fields(p, &fields, true) { return false; }
    let mut csaddr = 0;
    if !rust_rseq_read_csaddr(p, &mut csaddr) { return false; }
    #[cfg(CONFIG_RSEQ_SLICE_EXTENSION)]
    {
        if v2(d) && rust_rseq_slice_enabled() &&
           !write_fields(p, &[store32(offset_of!(rseq, slice_ctrl), 0)], true) { return false; }
        if (*d).slice.state.__bindgen_anon_1.granted != 0 { stat(STAT_REVOKED); }
        (*d).slice.state.__bindgen_anon_1.granted = 0;
    }
    (*d).ids = *ids;
    stat(STAT_IDS);
    #[cfg(CONFIG_TRACEPOINTS)]
    rust_rseq_trace_update_if_enabled(t);
    if cfg!(CONFIG_GENERIC_IRQ_ENTRY) && !rust_rseq_debug_enabled() && (*event_bits(d)).user_irq == 0 { return true; }
    if csaddr == 0 { return true; }
    update_user_cs(t, regs, csaddr as c_ulong)
}

unsafe fn handle_cs(t: *mut task_struct, regs: *mut pt_regs) -> bool {
    let mut csaddr = 0u64;
    if !rust_rseq_read_csaddr((*rust_rseq_data(t)).usrptr, &mut csaddr) { return false; }
    csaddr == 0 || update_user_cs(t, regs, csaddr as c_ulong)
}

#[no_mangle]
pub unsafe extern "C" fn __rseq_handle_slowpath(regs: *mut pt_regs) {
    if regs.is_null() { return; }
    let t = rust_rseq_current();
    let d = rust_rseq_data(t);
    if rust_rseq_task_flags(t) & PF_EXITING != 0 { return; }
    stat(STAT_SLOWPATH);
    let mut mask: rseq_event = zeroed();
    mask.__bindgen_anon_1.__bindgen_anon_1.has_rseq = RSEQ_HAS_RSEQ_VERSION_MASK as u8;
    mask.__bindgen_anon_1.__bindgen_anon_1.__bindgen_anon_1.__bindgen_anon_1.user_irq = 1;
    let mut ids: rseq_ids = zeroed();
    rust_rseq_irq_disable();
    let pending = (*event_bits(d)).sched_switch != 0;
    (*d).event.__bindgen_anon_1.all &= mask.__bindgen_anon_1.all;
    ids.__bindgen_anon_1.__bindgen_anon_1.cpu_id = rust_rseq_task_cpu(t);
    ids.__bindgen_anon_1.__bindgen_anon_1.mm_cid = rust_rseq_task_mm_cid(t);
    rust_rseq_irq_enable();
    if !pending { return; }
    ids.node_id = rust_rseq_cpu_to_node(ids.__bindgen_anon_1.__bindgen_anon_1.cpu_id);
    if !update_usr(t, regs, &ids) {
        clear_error(d);
        force_sig(SIGSEGV as c_int);
    }
}

#[no_mangle]
pub unsafe extern "C" fn __rseq_signal_deliver(sig: c_int, regs: *mut pt_regs) {
    let t = rust_rseq_current();
    let d = rust_rseq_data(t);
    stat(STAT_SIGNAL);
    if !handle_cs(t, regs) { clear_error(d); force_sigsegv(sig); }
    if !v2(d) { rust_rseq_force_update(); }
}
#[no_mangle]
pub unsafe extern "C" fn __rseq_debug_syscall_return(regs: *mut pt_regs) {
    let t = rust_rseq_current();
    let d = rust_rseq_data(t);
    if (*event(d)).has_rseq == 0 { return; }
    let mut csaddr = 0u64;
    if rust_rseq_get_u64(addr_of!((*(*d).usrptr).rseq_cs), &mut csaddr) == 0 {
        if csaddr == 0 { return; }
        if csaddr < rust_rseq_task_size() as u64 && rseq_debug_update_user_cs(t, regs, csaddr as c_ulong) { return; }
    }
    force_sig(SIGSEGV as c_int);
}
#[cfg(CONFIG_DEBUG_RSEQ)]
#[no_mangle]
pub unsafe extern "C" fn rseq_syscall(regs: *mut pt_regs) { __rseq_debug_syscall_return(regs); }

unsafe fn reset_ids(d: *mut rseq_data) -> bool {
    let fields = [store32(offset_of!(rseq, cpu_id_start), 0),
        store32(offset_of!(rseq, cpu_id), RSEQ_CPU_ID_UNINITIALIZED as u32),
        store32(offset_of!(rseq, node_id), 0), store32(offset_of!(rseq, mm_cid), 0)];
    if write_fields((*d).usrptr, &fields, true) { return true; }
    force_sig(SIGSEGV as c_int);
    false
}
unsafe fn register(p: *mut rseq, len: u32, flags: c_int, sig: u32) -> c_long {
    if !rust_rseq_access_ok(p.cast(), len) { return -(EFAULT as c_long); }
    let version = if cfg!(CONFIG_GENERIC_IRQ_ENTRY) && len > ORIG_RSEQ_SIZE { 2 } else { 1 };
    let mut rflags = 0;
    if cfg!(CONFIG_RSEQ_SLICE_EXTENSION) && version > 1 && rust_rseq_slice_enabled() {
        rflags |= RSEQ_CS_FLAG_SLICE_EXT_AVAILABLE;
        if flags & RSEQ_FLAG_SLICE_EXT_DEFAULT_ON as c_int != 0 { rflags |= RSEQ_CS_FLAG_SLICE_EXT_ENABLED; }
    }
    let fields = [store64(offset_of!(rseq, rseq_cs), 0), store32(offset_of!(rseq, flags), rflags),
        store32(offset_of!(rseq, cpu_id_start), RSEQ_CPU_ID_UNINITIALIZED as u32),
        store32(offset_of!(rseq, cpu_id), RSEQ_CPU_ID_UNINITIALIZED as u32),
        store32(offset_of!(rseq, node_id), 0), store32(offset_of!(rseq, mm_cid), 0),
        store32(offset_of!(rseq, slice_ctrl), 0)];
    let count = if cfg!(CONFIG_RSEQ_SLICE_EXTENSION) && version > 1 { 7 } else { 6 };
    if !write_fields(p, &fields[..count], false) { return -(EFAULT as c_long); }
    let d = rust_rseq_data(rust_rseq_current());
    (*d).usrptr = p;
    (*d).len = len;
    (*d).sig = sig;
    #[cfg(CONFIG_RSEQ_SLICE_EXTENSION)]
    { (*d).slice.state.__bindgen_anon_1.enabled = (rflags & RSEQ_CS_FLAG_SLICE_EXT_ENABLED != 0) as u8; }
    (*event(d)).has_rseq = version;
    rust_rseq_force_update();
    0
}
unsafe fn unregister(p: *mut rseq, len: u32, flags: c_int, sig: u32) -> c_long {
    let t = rust_rseq_current();
    let d = rust_rseq_data(t);
    if flags & !(RSEQ_FLAG_UNREGISTER as c_int) != 0 || (*d).usrptr != p || (*d).usrptr.is_null() || (*d).len != len {
        return -(EINVAL as c_long);
    }
    if (*d).sig != sig { return -(EPERM as c_long); }
    if !reset_ids(d) { return -(EFAULT as c_long); }
    rust_rseq_reset(t);
    0
}
#[no_mangle]
pub unsafe extern "C" fn rust_sys_rseq(p: *mut rseq, len: u32, flags: c_int, sig: u32) -> c_long {
    if flags & RSEQ_FLAG_UNREGISTER as c_int != 0 { return unregister(p, len, flags, sig); }
    if flags & !(RSEQ_FLAG_SLICE_EXT_DEFAULT_ON as c_int) != 0 { return -(EINVAL as c_long); }
    let d = rust_rseq_data(rust_rseq_current());
    if !(*d).usrptr.is_null() {
        if (*d).usrptr != p || (*d).len != len { return -(EINVAL as c_long); }
        if (*d).sig != sig { return -(EPERM as c_long); }
        return -(EBUSY as c_long);
    }
    if len < ORIG_RSEQ_SIZE { return -(EINVAL as c_long); }
    if len == ORIG_RSEQ_SIZE {
        if p as usize & (ORIG_RSEQ_SIZE as usize - 1) != 0 { return -(EINVAL as c_long); }
    } else if p as usize & (rust_rseq_alloc_align() as usize).wrapping_sub(1) != 0 || len < offset_of!(rseq, end) as u32 {
        return -(EINVAL as c_long);
    }
    register(p, len, flags, sig)
}

#[cfg(CONFIG_RSEQ_STATS)]
unsafe extern "C" fn stats_show(m: *mut seq_file, _: *mut c_void) -> c_int {
    let mut sum: rseq_stats = zeroed();
    let mut cpu = rust_rseq_next_cpu(u32::MAX);
    while cpu < rust_rseq_cpu_limit() {
        let mut value: rseq_stats = zeroed();
        rust_rseq_read_stats(cpu, &mut value);
        macro_rules! add { ($field:ident) => { sum.$field = sum.$field.wrapping_add(value.$field); } }
        add!(exit); add!(signal); add!(slowpath); add!(fastpath); add!(ids); add!(cs); add!(clear); add!(fixup);
        if cfg!(CONFIG_RSEQ_SLICE_EXTENSION) {
            add!(s_granted); add!(s_expired); add!(s_revoked); add!(s_yielded); add!(s_aborted);
        }
        cpu = rust_rseq_next_cpu(cpu);
    }
    seq_printf(m, c"exit:   %16lu\n".as_ptr().cast(), sum.exit);
    seq_printf(m, c"signal: %16lu\n".as_ptr().cast(), sum.signal);
    seq_printf(m, c"slowp:  %16lu\n".as_ptr().cast(), sum.slowpath);
    seq_printf(m, c"fastp:  %16lu\n".as_ptr().cast(), sum.fastpath);
    seq_printf(m, c"ids:    %16lu\n".as_ptr().cast(), sum.ids);
    seq_printf(m, c"cs:     %16lu\n".as_ptr().cast(), sum.cs);
    seq_printf(m, c"clear:  %16lu\n".as_ptr().cast(), sum.clear);
    seq_printf(m, c"fixup:  %16lu\n".as_ptr().cast(), sum.fixup);
    if cfg!(CONFIG_RSEQ_SLICE_EXTENSION) {
        seq_printf(m, c"sgrant: %16lu\n".as_ptr().cast(), sum.s_granted);
        seq_printf(m, c"sexpir: %16lu\n".as_ptr().cast(), sum.s_expired);
        seq_printf(m, c"srevok: %16lu\n".as_ptr().cast(), sum.s_revoked);
        seq_printf(m, c"syield: %16lu\n".as_ptr().cast(), sum.s_yielded);
        seq_printf(m, c"sabort: %16lu\n".as_ptr().cast(), sum.s_aborted);
    }
    0
}
#[cfg(CONFIG_RSEQ_STATS)]
unsafe extern "C" fn stats_open(i: *mut inode, f: *mut file) -> c_int { single_open(f, Some(stats_show), rust_rseq_inode_private(i)) }
unsafe extern "C" fn debug_show(m: *mut seq_file, _: *mut c_void) -> c_int {
    seq_printf(m, c"%d\n".as_ptr().cast(), rust_rseq_debug_enabled() as c_int);
    0
}
unsafe extern "C" fn debug_write(_: *mut file, buf: *const c_char, count: usize, _: *mut loff_t) -> isize {
    let mut on = false;
    if kstrtobool_from_user(buf, count, &mut on) != 0 { return -(EINVAL as isize); }
    rust_rseq_control_debug(on);
    count as isize
}
unsafe extern "C" fn debug_open(i: *mut inode, f: *mut file) -> c_int { single_open(f, Some(debug_show), rust_rseq_inode_private(i)) }
// Like the C const tables, these immutable operations reside in read-only
// storage. The raw pointers in file_operations do not confer mutability.
#[repr(transparent)]
struct FileOps(file_operations);
unsafe impl Sync for FileOps {}
static DEBUG_OPS: FileOps = FileOps(file_operations {
    open: Some(debug_open), read: Some(seq_read), write: Some(debug_write), llseek: Some(seq_lseek),
    release: Some(single_release), ..unsafe { zeroed() }
});
#[cfg(CONFIG_RSEQ_STATS)]
static STAT_OPS: FileOps = FileOps(file_operations {
    open: Some(stats_open), read: Some(seq_read), llseek: Some(seq_lseek), release: Some(single_release), ..unsafe { zeroed() }
});
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn rust_rseq_debugfs_init() -> c_int {
    let root = rust_rseq_debugfs_dir(c"rseq".as_ptr().cast());
    rust_rseq_debugfs_file(c"debug".as_ptr().cast(), 0o644, root, addr_of!(DEBUG_OPS.0));
    #[cfg(CONFIG_RSEQ_STATS)]
    rust_rseq_debugfs_file(c"stats".as_ptr().cast(), 0o444, root, addr_of!(STAT_OPS.0));
    #[cfg(CONFIG_RSEQ_SLICE_EXTENSION)]
    rust_rseq_debugfs_file(c"slice_ext_nsec".as_ptr().cast(), 0o644, root, addr_of!(SLICE_OPS.0));
    0
}

#[cfg(CONFIG_RSEQ_SLICE_EXTENSION)]
const SLICE_MIN: u32 = 5 * NSEC_PER_USEC as u32;
#[cfg(CONFIG_RSEQ_SLICE_EXTENSION)]
const SLICE_MAX: u32 = 50 * NSEC_PER_USEC as u32;
#[cfg(CONFIG_RSEQ_SLICE_EXTENSION)]
#[no_mangle]
#[link_section = ".data..read_mostly"]
pub static mut rseq_slice_ext_nsecs: u32 = SLICE_MIN;

#[cfg(CONFIG_RSEQ_SLICE_EXTENSION)]
#[no_mangle]
pub unsafe extern "C" fn rust_rseq_slice_expired(timer: *mut hrtimer) -> hrtimer_restart {
    let st = timer.cast::<u8>().sub(offset_of!(slice_timer, timer)).cast::<slice_timer>();
    let curr = rust_rseq_current();
    if (*st).cookie == curr.cast() && (*rust_rseq_data(curr)).slice.state.__bindgen_anon_1.granted != 0 {
        stat(STAT_EXPIRED);
        rust_rseq_need_resched();
    }
    HRTIMER_NORESTART
}
#[cfg(CONFIG_RSEQ_SLICE_EXTENSION)]
#[no_mangle]
pub unsafe extern "C" fn __rseq_arm_slice_extension_timer() -> bool {
    let st = rust_rseq_this_timer();
    let curr = rust_rseq_current();
    let d = rust_rseq_data(curr);
    rust_rseq_lockdep_irqs_disabled();
    if (*d).slice.expires < ktime_get_mono_fast_ns() {
        rust_rseq_need_resched();
        return true;
    }
    (*st).cookie = curr.cast();
    rust_rseq_timer_start(addr_of_mut!((*st).timer), (*d).slice.expires as ktime_t);
    rust_rseq_set_syscall_work(curr);
    false
}
#[cfg(CONFIG_RSEQ_SLICE_EXTENSION)]
#[no_mangle]
pub unsafe extern "C" fn rseq_syscall_enter_work(syscall: c_long) {
    let curr = rust_rseq_current();
    let d = rust_rseq_data(curr);
    let mut ctrl: rseq_slice_ctrl = zeroed();
    ctrl.__bindgen_anon_1.__bindgen_anon_1.granted = (*d).slice.state.__bindgen_anon_1.granted;
    rust_rseq_clear_syscall_work(curr);
    if rust_rseq_debug_enabled() {
        let mut value = 0;
        if rust_rseq_get_u32(addr_of!((*(*d).usrptr).slice_ctrl.__bindgen_anon_1.all), &mut value) != 0 ||
           value != ctrl.__bindgen_anon_1.all { force_sig(SIGSEGV as c_int); }
    }
    if ctrl.__bindgen_anon_1.__bindgen_anon_1.granted == 0 { return; }
    rust_rseq_preempt_disable();
    let st = rust_rseq_this_timer();
    if (*st).cookie == curr.cast() { hrtimer_try_to_cancel(addr_of_mut!((*st).timer)); }
    if (*event_bits(d)).sched_switch == 0 {
        rust_rseq_irq_disable();
        rust_rseq_need_resched();
        rust_rseq_irq_enable();
        if syscall == RUST_NR_RSEQ_SLICE_YIELD as c_long {
            stat(STAT_YIELDED);
            (*d).slice.yielded = 1;
        } else { stat(STAT_ABORTED); }
    }
    rust_rseq_preempt_enable();
    rust_rseq_cond_resched();
    (*d).slice.state.__bindgen_anon_1.granted = 0;
    if rust_rseq_put_u32(addr_of_mut!((*(*d).usrptr).slice_ctrl.__bindgen_anon_1.all), 0) != 0 { force_sig(SIGSEGV as c_int); }
}
#[cfg(CONFIG_RSEQ_SLICE_EXTENSION)]
#[no_mangle]
pub unsafe extern "C" fn rseq_slice_extension_prctl(arg2: c_ulong, arg3: c_ulong) -> c_int {
    let d = rust_rseq_data(rust_rseq_current());
    if arg2 == PR_RSEQ_SLICE_EXTENSION_GET as c_ulong {
        if arg3 != 0 { return -(EINVAL as c_int); }
        return if (*d).slice.state.__bindgen_anon_1.enabled != 0 { PR_RSEQ_SLICE_EXT_ENABLE as c_int } else { 0 };
    }
    if arg2 != PR_RSEQ_SLICE_EXTENSION_SET as c_ulong || arg3 & !(PR_RSEQ_SLICE_EXT_ENABLE as c_ulong) != 0 { return -(EINVAL as c_int); }
    if !rust_rseq_slice_enabled() { return -(ENOTSUPP as c_int); }
    if (*d).usrptr.is_null() { return -(ENXIO as c_int); }
    if !v2(d) { return -(ENOTSUPP as c_int); }
    let enable = arg3 & PR_RSEQ_SLICE_EXT_ENABLE as c_ulong != 0;
    if enable == ((*d).slice.state.__bindgen_anon_1.enabled != 0) { return 0; }
    let mut flags = 0;
    if rust_rseq_get_u32(addr_of!((*(*d).usrptr).flags), &mut flags) == 0 {
        let mut valid = RSEQ_CS_FLAG_SLICE_EXT_AVAILABLE;
        if (*d).slice.state.__bindgen_anon_1.enabled != 0 { valid |= RSEQ_CS_FLAG_SLICE_EXT_ENABLED; }
        if flags & valid == valid {
            flags &= !RSEQ_CS_FLAG_SLICE_EXT_ENABLED;
            flags |= RSEQ_CS_FLAG_SLICE_EXT_AVAILABLE;
            if enable { flags |= RSEQ_CS_FLAG_SLICE_EXT_ENABLED; }
            if rust_rseq_put_u32(addr_of_mut!((*(*d).usrptr).flags), flags) == 0 {
                (*d).slice.state.__bindgen_anon_1.enabled = enable as u8;
                return 0;
            }
        }
    }
    force_sig(SIGSEGV as c_int);
    -(EFAULT as c_int)
}
#[cfg(CONFIG_RSEQ_SLICE_EXTENSION)]
#[no_mangle]
pub unsafe extern "C" fn rust_sys_rseq_slice_yield() -> c_long {
    let d = rust_rseq_data(rust_rseq_current());
    let yielded = ((*d).slice.yielded != 0) as c_long;
    (*d).slice.yielded = 0;
    yielded
}
#[cfg(CONFIG_RSEQ_SLICE_EXTENSION)]
unsafe extern "C" fn slice_show(m: *mut seq_file, _: *mut c_void) -> c_int {
    seq_printf(m, c"%d\n".as_ptr().cast(), read_volatile(addr_of!(rseq_slice_ext_nsecs)) as c_int);
    0
}
#[cfg(CONFIG_RSEQ_SLICE_EXTENSION)]
unsafe extern "C" fn slice_write(_: *mut file, buf: *const c_char, count: usize, _: *mut loff_t) -> isize {
    let mut nsecs = 0;
    if kstrtouint_from_user(buf, count, 10, &mut nsecs) != 0 { return -(EINVAL as isize); }
    if nsecs < SLICE_MIN || nsecs > SLICE_MAX { return -(ERANGE as isize); }
    write_volatile(addr_of_mut!(rseq_slice_ext_nsecs), nsecs);
    count as isize
}
#[cfg(CONFIG_RSEQ_SLICE_EXTENSION)]
unsafe extern "C" fn slice_open(i: *mut inode, f: *mut file) -> c_int { single_open(f, Some(slice_show), rust_rseq_inode_private(i)) }
#[cfg(CONFIG_RSEQ_SLICE_EXTENSION)]
static SLICE_OPS: FileOps = FileOps(file_operations {
    open: Some(slice_open), read: Some(seq_read), write: Some(slice_write), llseek: Some(seq_lseek),
    release: Some(single_release), ..unsafe { zeroed() }
});
#[cfg(CONFIG_RSEQ_SLICE_EXTENSION)]
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn rust_rseq_slice_cmdline(s: *mut c_char) -> c_int {
    let mut on = false;
    if kstrtobool(s, &mut on) != 0 { return 0; }
    if !on { rust_rseq_slice_disable(); }
    1
}
#[cfg(CONFIG_RSEQ_SLICE_EXTENSION)]
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn rust_rseq_slice_init() -> c_int {
    let mut cpu = rust_rseq_next_timer_cpu(u32::MAX);
    while cpu < rust_rseq_cpu_limit() {
        let st = rust_rseq_cpu_timer(cpu);
        rust_rseq_timer_setup(addr_of_mut!((*st).timer), CLOCK_MONOTONIC as c_int, HRTIMER_MODE_REL_PINNED_HARD);
        cpu = rust_rseq_next_timer_cpu(cpu);
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
