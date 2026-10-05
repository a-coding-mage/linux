// SPDX-License-Identifier: GPL-2.0
// F11 source repair against ext.c:921-933,5569-5706,6349-6378,6545-7066,
// 8576-8624,9886-10068 at 126a30fae3bba11420ec2fcbde51a0a01bab1b5b.
// Native leaves are explicit C runtime boundaries, not Rust coverage.
compile_error!("SOURCE ONLY HOLD: exit/dump ABI, NMI, stack and protection qualification incomplete");

use super::*;
use core::{mem::{size_of, MaybeUninit}, ptr};
use kernel::ffi::{c_char, c_int, c_ulong, c_ulonglong};

/// Claim once and make the entire subtree abort before notifying callbacks.
///
/// # Safety
/// sch is live under the original caller's lifetime protection. Preemption is
/// disabled, including NMI callers, and the caller MUST queue disable work
/// before restoring preemption after a successful claim. Native descendant
/// traversal requires the RCU read section retained here. No pointer escapes it.
pub(crate) unsafe fn scx_claim_exit(sch: *mut scx_sched, mut kind: scx_exit_kind) -> bool {
    // SAFETY: The atomic winner alone owns exit-info formatting. The per-node
    // full barrier precedes traversal of its children, pairing with link_sched.
    unsafe {
        lupos_scx_exit_assert_preempt_disabled();
        if lupos_scx_exit_warn_bad_kind(kind) {
            kind = SCX_EXIT_ERROR;
        }
        let mut none = SCX_EXIT_NONE as c_int;
        if !lupos_scx_exit_try_claim(sch, &mut none, kind as c_int) {
            return false;
        }
        if kind == SCX_EXIT_PARENT {
            lupos_scx_exit_aborting_once(sch);
        } else {
            lupos_scx_exit_rcu_lock();
            let mut pos = lupos_scx_exit_next_descendant(ptr::null_mut(), sch);
            while !pos.is_null() {
                lupos_scx_exit_aborting_mb(pos);
                pos = lupos_scx_exit_next_descendant(pos, sch);
            }
            lupos_scx_exit_rcu_unlock();
            irq_work_queue(ptr::addr_of_mut!((*sch).propagate_exit_irq_work));
        }
        lupos_scx_exit_trace_exit(sch, kind);
        true
    }
}

/// Native IRQ callback continuation for descendant PARENT claims.
///
/// # Safety
/// sch contains the live IRQ work currently executing. Its lifetime covers
/// completion. The native entry holds the original raw_spinlock_irqsave guard
/// on scx_sched_lock, including this tree's reference-counted IRQ semantics.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_exit_propagate_irq_workfn(sch: *mut scx_sched) {
    // SAFETY: The single shared scheduler lock stabilizes traversal. PARENT
    // claims do not recurse through another propagation work item.
    unsafe {
        let mut pos = lupos_scx_exit_next_descendant(ptr::null_mut(), sch);
        while !pos.is_null() {
            scx_disable(pos, SCX_EXIT_PARENT);
            pos = lupos_scx_exit_next_descendant(pos, sch);
        }
    }
}

/// Claim an exit and guarantee that its disable IRQ work is kicked.
///
/// # Safety
/// sch is live through queueing under the original caller's scheduler lifetime
/// protection; entry allows nested preemption disabling (including IRQ/NMI).
pub(crate) unsafe fn scx_disable(sch: *mut scx_sched, kind: scx_exit_kind) {
    // SAFETY: The claim-to-queue interval is entirely nonpreemptible.
    unsafe {
        lupos_scx_exit_preempt_disable();
        if scx_claim_exit(sch, kind) {
            irq_work_queue(ptr::addr_of_mut!((*sch).disable_irq_work));
        }
        lupos_scx_exit_preempt_enable();
    }
}

/// Native helper-kthread work continuation for one scheduler's teardown.
///
/// # Safety
/// sch is pinned by its executing disable_work; this is the original helper
/// context in which root/sub teardown may sleep. Dependency owners supply the
/// real teardown bodies; no old-C fallback or replacement success is allowed.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_exit_disable_workfn(sch: *mut scx_sched) {
    // SAFETY: Only the successful DONE transition publishes the final reason
    // and tears down. Failed compare-exchanges update kind before the retry.
    unsafe {
        let ei = (*sch).exit_info;
        let mut kind = lupos_scx_exit_kind_read(sch);
        loop {
            if kind == SCX_EXIT_DONE as c_int { return; }
            lupos_scx_exit_warn_none(kind);
            if lupos_scx_exit_try_claim(sch, &mut kind, SCX_EXIT_DONE as c_int) { break; }
        }
        (*ei).kind = kind as scx_exit_kind;
        (*ei).reason = scx_exit_reason((*ei).kind);
        if !lupos_scx_core_parent(sch).is_null() {
            lupos_scx_exit_sub_disable(sch);
        } else {
            scx_root_disable(sch);
        }
    }
}

/// Wait for both IRQ queueing and helper teardown, including queueing races.
///
/// # Safety
/// sch and both work objects stay live through the entire flush; caller is in
/// a sleepable context and is not executing the work being flushed.
#[no_mangle]
pub unsafe extern "C" fn scx_flush_disable_work(sch: *mut scx_sched) {
    // SAFETY: IRQ synchronization precedes every kthread flush; the state is
    // rechecked after each pair instead of assuming an unqueued flush waited.
    unsafe {
        loop {
            irq_work_sync(ptr::addr_of_mut!((*sch).disable_irq_work));
            kthread_flush_work(ptr::addr_of_mut!((*sch).disable_work));
            let kind = lupos_scx_exit_kind_read(sch);
            if kind == SCX_EXIT_NONE as c_int || kind == SCX_EXIT_DONE as c_int { break; }
        }
    }
}

/// Mark future dump callbacks disabled inside the native scoped lock envelope.
///
/// # Safety
/// sch is live and the native scx_disable_dump entry holds the original
/// raw_spinlock_irqsave guard on the single global dump lock.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_exit_disable_dump_locked(sch: *mut scx_sched) {
    // SAFETY: The same lock serializes state dumping and this flag mutation.
    unsafe {
        (*sch).dump_disabled = true;
    }
}

/// Emit the final scheduler-disable report with original levels and ordering.
///
/// # Safety
/// sch, its exit_info, name, parent relation and all bounded message/backtrace
/// storage remain live and readable after the original teardown serialization.
#[no_mangle]
pub unsafe extern "C" fn scx_log_sched_disable(sch: *mut scx_sched) {
    // SAFETY: Native print leaves retain exact formats and configured stack
    // output. The empty-message and severity choices stay in this owner.
    unsafe {
        let ei = (*sch).exit_info;
        let type_ = if !lupos_scx_core_parent(sch).is_null() {
            b"sub-scheduler\0".as_ptr()
        } else { b"scheduler\0".as_ptr() }.cast();
        if (*ei).kind >= SCX_EXIT_ERROR {
            lupos_scx_exit_log_error(sch, type_);
            if *(*ei).msg != 0 { lupos_scx_exit_log_message(sch); }
            #[cfg(CONFIG_STACKTRACE)]
            lupos_scx_exit_print_stack((*ei).bt, (*ei).bt_len);
        } else {
            lupos_scx_exit_log_info(sch, type_);
        }
    }
}

/// Complete the winning exit record and queue its disable work.
///
/// # Safety
/// Caller owns a successful claim, initialized ei->msg and disabled preemption.
/// sch/ei and allocated bt/msg remain live until the queued teardown owns them.
pub(crate) unsafe fn scx_finish_exit(
    sch: *mut scx_sched, kind: scx_exit_kind, exit_code: i64, exit_cpu: i32,
) {
    // SAFETY: NMI skips only this original finish-path stack capture. Kind and
    // reason are published before queueing, then recomputed by disable_work.
    unsafe {
        let ei = (*sch).exit_info;
        (*ei).exit_code = exit_code;
        #[cfg(CONFIG_STACKTRACE)]
        if kind >= SCX_EXIT_ERROR && !lupos_scx_exit_in_nmi() {
            (*ei).bt_len = lupos_scx_exit_save_finish_stack((*ei).bt);
        }
        (*ei).kind = kind;
        (*ei).reason = scx_exit_reason((*ei).kind);
        (*ei).exit_cpu = exit_cpu;
        irq_work_queue(ptr::addr_of_mut!((*sch).disable_irq_work));
    }
}

/// Native va_list wrapper's synchronous claim/format continuation.
///
/// # Safety
/// sch is lifetime-pinned and fmt is a valid native format for args. args is
/// the wrapper's live copied va_list holder, consumable once and borrowed only
/// for this call. It is neither read as a Rust layout nor retained anywhere.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_exit_vexit(
    sch: *mut scx_sched, kind: scx_exit_kind, exit_code: i64, exit_cpu: i32,
    fmt: *const c_char, args: *mut scx_exit_dump_va_args,
) -> bool {
    // SAFETY: A failed claim never traverses/formats args. Preemption stays
    // disabled until finish has queued work, including formatting failure paths.
    unsafe {
        let ei = (*sch).exit_info;
        lupos_scx_exit_preempt_disable();
        let claimed = scx_claim_exit(sch, kind);
        if claimed {
            lupos_scx_exit_format_va((*ei).msg, LUPOS_SCX_EXIT_MSG_LEN as usize, fmt, args);
            scx_finish_exit(sch, kind, exit_code, exit_cpu);
        }
        lupos_scx_exit_preempt_enable();
        claimed
    }
}

/// Common lockup entry continuation; the native frame owns the argument list.
///
/// # Safety
/// fmt/args satisfy the synchronous native holder contract above. Entry may be
/// NMI; the configured RCU, preemption, formatting and IRQ primitives must be
/// legal there. This source candidate does not qualify those architecture rules.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_exit_handle_lockup(
    exit_cpu: c_int, fmt: *const c_char, args: *mut scx_exit_dump_va_args,
) -> bool {
    // SAFETY: Root and every use remain within RCU, including failed exits.
    unsafe {
        lupos_scx_exit_rcu_lock();
        let sch = lupos_scx_exit_lockup_root_rcu();
        let ret = if lupos_scx_exit_lockup_unlikely_null(sch) {
            false
        } else {
            let state = scx_enable_state();
            if state == SCX_ENABLING || state == SCX_ENABLED {
                lupos_scx_exit_vexit(sch, SCX_EXIT_ERROR, 0, exit_cpu, fmt, args)
            } else { false }
        };
        lupos_scx_exit_rcu_unlock();
        ret
    }
}

/// Claim a stalled root, preserve the whole CPU mask and kick recovery.
///
/// # Safety
/// stalled_mask is a live native cpumask for the synchronous call; caller is
/// in a context allowed by the original RCU-stall recovery entry point.
#[no_mangle]
pub unsafe extern "C" fn scx_rcu_cpu_stall(stalled_mask: *const cpumask) -> bool {
    // SAFETY: Both guard lifetimes match native scopes. This path deliberately
    // retains the original stack capture without finish_exit's NMI exclusion.
    unsafe {
        lupos_scx_exit_rcu_lock();
        let ret = (|| {
            let sch = lupos_scx_exit_stall_root_rcu();
            if lupos_scx_exit_stall_unlikely_null(sch) { return false; }
            let state = scx_enable_state();
            if state != SCX_ENABLING && state != SCX_ENABLED { return false; }
            let exit_cpu = if lupos_scx_exit_mask_empty(stalled_mask) { -1 }
                           else { lupos_scx_exit_mask_first(stalled_mask) };
            let ei = (*sch).exit_info;
            lupos_scx_exit_preempt_disable();
            let claimed = scx_claim_exit(sch, SCX_EXIT_ERROR);
            if claimed {
                #[cfg(CONFIG_STACKTRACE)]
                { (*ei).bt_len = lupos_scx_exit_save_stall_stack((*ei).bt); }
                lupos_scx_exit_format_stall((*ei).msg, stalled_mask);
                (*ei).kind = SCX_EXIT_ERROR;
                (*ei).reason = scx_exit_reason(SCX_EXIT_ERROR);
                (*ei).exit_cpu = exit_cpu;
                lupos_scx_exit_mask_copy(lupos_scx_exit_stall_mask(sch), stalled_mask);
                irq_work_queue(ptr::addr_of_mut!((*sch).disable_irq_work));
            }
            lupos_scx_exit_preempt_enable();
            claimed
        })();
        lupos_scx_exit_rcu_unlock();
        ret
    }
}

/// Abort on a soft-lockup and emit its notice only when this caller won.
///
/// # Safety
/// Invoked in the original softlockup-watchdog context; the live CPU identity
/// and lockup recovery primitives retain their original caller restrictions.
#[no_mangle]
pub unsafe extern "C" fn scx_softlockup(dur_s: u32) {
    // SAFETY: Native variadic marshalling preserves both promoted CPU/duration
    // arguments; it synchronously enters the Rust common lockup owner.
    unsafe {
        let cpu = lupos_scx_exit_soft_cpu();
        if !lupos_scx_exit_soft_lockup(cpu, dur_s) { return; }
        lupos_scx_exit_soft_notice(cpu, dur_s);
    }
}

/// Abort on hard lockup, returning whether recovery was initiated.
///
/// # Safety
/// Invoked in the original NMI lockup context. cpu identifies the stalled CPU;
/// native formatting, RCU and irq_work NMI qualification remains outstanding.
#[no_mangle]
pub unsafe extern "C" fn scx_hardlockup(cpu: c_int) -> bool {
    // SAFETY: A successful common claim sets subtree aborting before return.
    unsafe {
        if !lupos_scx_exit_hard_lockup(cpu) { return false; }
        lupos_scx_exit_hard_notice(cpu);
        true
    }
}

/// Append a newline to the trace and, only if sized, the output sequence.
///
/// # Safety
/// s is initialized and exclusively used under the original dump lock; its
/// backing storage is live for size bytes, including the legal zero-size case.
unsafe fn dump_newline(s: *mut seq_buf) {
    // SAFETY: The size check avoids seq_buf's original zero-size warning.
    unsafe {
        lupos_scx_exit_trace_newline();
        if (*s).size != 0 { lupos_scx_exit_seq_put_newline(s); }
    }
}

/// Format trace and sequence destinations with independent native lists.
///
/// # Safety
/// The native caller owns two distinct, live va_start lists matching fmt;
/// neither is consumed elsewhere. s and the native trace line buffer are
/// protected by the global dump lock for the entire synchronous call.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_exit_dump_line(
    s: *mut seq_buf, fmt: *const c_char,
    trace_args: *mut scx_exit_dump_va_args, seq_args: *mut scx_exit_dump_va_args,
) {
    // SAFETY: Disabled destinations do no formatting. No list is reused, and
    // the source trace-before-sequence-before-newline order is retained.
    unsafe {
        #[cfg(CONFIG_TRACEPOINTS)]
        if lupos_scx_exit_trace_dump_enabled() {
            lupos_scx_exit_trace_line(fmt, trace_args);
        }
        #[cfg(not(CONFIG_TRACEPOINTS))]
        let _ = trace_args;
        if (*s).size != 0 {
            lupos_scx_exit_seq_line(s, fmt, seq_args);
            lupos_scx_exit_seq_put_newline(s);
        }
    }
}

/// Emit a bounded native backtrace one address per line.
///
/// # Safety
/// bt is readable for len native unsigned-long entries; prefix is NUL-terminated
/// and s satisfies dump_newline's lock/storage contract.
unsafe fn dump_stack_trace(s: *mut seq_buf, prefix: *const c_char,
                           bt: *const c_ulong, len: u32) {
    // SAFETY: Each address is synchronously formatted as the original %pS.
    unsafe {
        for i in 0..len { lupos_scx_exit_emit_stack(s, prefix, *bt.add(i as usize)); }
    }
}

/// Open one dump callback's shared line accumulator.
///
/// # Safety
/// Caller holds the one dump lock with IRQs disabled; s/prefix remain live
/// until ops_dump_exit, and callbacks execute synchronously on this CPU.
unsafe fn ops_dump_init(s: *mut seq_buf, prefix: *const c_char) {
    // SAFETY: The shared slot is not a per-family duplicate. The native plain
    // CPU store preserves the original acceptance token; it is not release
    // publication. Both stores and the off-context read use that same held
    // native field boundary, whose race semantics remain unqualified.
    unsafe {
        let dd = lupos_scx_core_dump_data();
        lupos_scx_exit_assert_irqs_disabled();
        scx_shared_dump_cpu_write(dd, lupos_scx_exit_ops_dump_cpu());
        (*dd).first = true;
        (*dd).cursor = 0;
        (*dd).s = s;
        (*dd).prefix = prefix;
    }
}

/// Flush each accumulated line, including a truncated final line.
///
/// # Safety
/// The dump lock and disabled IRQs protect the initialized shared accumulator;
/// its buffer is NUL-terminated by the native bounded formatting primitive.
unsafe fn ops_dump_flush() {
    // SAFETY: Scan/mutate only the owned native line buffer. Newline replacement
    // happens before emission, and a first nonempty dump gets one blank line.
    unsafe {
        let dd = lupos_scx_core_dump_data();
        if (*dd).cursor == 0 { return; }
        let mut line = ptr::addr_of_mut!((*dd).buf.line).cast::<c_char>();
        if (*dd).first {
            dump_newline((*dd).s);
            (*dd).first = false;
        }
        loop {
            let mut end = line;
            while *end != b'\n' as c_char && *end != 0 { end = end.add(1); }
            let c = *end;
            *end = 0;
            lupos_scx_exit_emit_ops_line((*dd).s, (*dd).prefix, line);
            if c == 0 { break; }
            end = end.add(1);
            if *end == 0 { break; }
            line = end;
        }
        (*dd).cursor = 0;
    }
}

/// Close a dump callback after flushing its final partial line.
///
/// # Safety
/// Matches one ops_dump_init under the unchanged dump-lock/IRQ/CPU protection.
unsafe fn ops_dump_exit() {
    // SAFETY: Disallow later appends only after finishing all pending output.
    unsafe { ops_dump_flush(); scx_shared_dump_cpu_write(lupos_scx_core_dump_data(), -1); }
}

/// Validate, nofault-copy, prepare and format one explicitly packed bstr.
///
/// # Safety
/// sch is live for error reporting; data_buf has MAX_BPRINTF_VARARGS entries,
/// line_buf has line_size writable bytes, and fmt follows BPF's format-string
/// verification contract. data may be NULL/invalid: the native nofault read
/// handles it after size validation. No pointer survives the native calls.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_exit_bstr_format_body(
    sch: *mut scx_sched, data_buf: *mut u64, line_buf: *mut c_char,
    line_size: usize, fmt: *mut c_char, data: *mut c_ulonglong, data_sz: u32,
) -> i32 {
    // SAFETY: Only a successful prepare is cleaned up, exactly once and before
    // testing format failure. Original blame scheduler and diagnostic sites
    // remain distinct. The caller supplies any required preemption protection.
    unsafe {
        let mut prepared = MaybeUninit::<bpf_bprintf_data>::uninit();
        lupos_scx_exit_bprintf_init(prepared.as_mut_ptr());
        if data_sz % 8 != 0 || data_sz > LUPOS_SCX_EXIT_MAX_BPRINTF_VARARGS * 8
            || (data_sz != 0 && data.is_null()) {
            lupos_scx_exit_error_data(sch, data, data_sz);
            return -(LUPOS_SCX_EXIT_EINVAL as i32);
        }
        let mut ret = copy_from_kernel_nofault(data_buf.cast(), data.cast(), data_sz as usize) as i32;
        if ret < 0 { lupos_scx_exit_error_read(sch, ret); return ret; }
        ret = bpf_bprintf_prepare(fmt, LUPOS_SCX_EXIT_UINT_MAX, data_buf,
                                 data_sz / 8, prepared.as_mut_ptr());
        if ret < 0 { lupos_scx_exit_error_prepare(sch, ret); return ret; }
        ret = bstr_printf(line_buf, line_size, fmt, (*prepared.as_ptr()).bin_args);
        bpf_bprintf_cleanup(prepared.as_mut_ptr());
        if ret < 0 { lupos_scx_exit_error_format(sch, fmt, data, data_sz); }
        ret
    }
}

/// Claim before formatting an exit reason from packed BPF arguments.
///
/// # Safety
/// sch/fmt_blame are live schedulers. fmt/data satisfy the native __bstr_format
/// entry and its Rust body's contract.
/// Entry supports nested preemption disabling, including the native NMI path;
/// the stack buffer and native BPF formatting remain unqualified for admission.
#[no_mangle]
pub unsafe extern "C" fn scx_exit_bstr(
    sch: *mut scx_sched, kind: scx_exit_kind, exit_code: i64,
    fmt_blame: *mut scx_sched, fmt: *mut c_char, data: *mut c_ulonglong, data_sz: u32,
) -> bool {
    // SAFETY: Winner-owned message formatting happens entirely between claim
    // and queue. Failure leaves the claim in place and emits a bounded fallback.
    unsafe {
        let ei = (*sch).exit_info;
        let mut data_buf = MaybeUninit::<[u64; LUPOS_SCX_EXIT_MAX_BPRINTF_VARARGS as usize]>::uninit();
        lupos_scx_exit_preempt_disable();
        let claimed = scx_claim_exit(sch, kind);
        if claimed {
            let ret = lupos_scx_exit_bstr_format(fmt_blame, data_buf.as_mut_ptr().cast(), (*ei).msg,
                                   LUPOS_SCX_EXIT_MSG_LEN as usize, fmt, data, data_sz);
            if ret < 0 { lupos_scx_exit_format_fallback((*ei).msg, ret); }
            scx_finish_exit(sch, kind, exit_code, lupos_scx_exit_raw_cpu());
        }
        lupos_scx_exit_preempt_enable();
        claimed
    }
}

/// Native BPF graceful-exit continuation.
///
/// # Safety
/// aux and packed arguments are supplied by the verified native kfunc entry;
/// scheduler association must remain under this RCU section until exit returns.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_exit_bpf_exit_bstr(
    exit_code: i64, fmt: *mut c_char, data: *mut c_ulonglong, data_sz: u32,
    aux: *const bpf_prog_aux,
) {
    // SAFETY: The native lookup preserves configured association and likely
    // instrumentation; no association means no claim or formatting.
    unsafe {
        lupos_scx_exit_rcu_lock();
        let sch = lupos_scx_exit_exit_prog_sched(aux);
        if lupos_scx_exit_exit_likely_nonnull(sch) {
            scx_exit_bstr(sch, SCX_EXIT_UNREG_BPF, exit_code, sch, fmt, data, data_sz);
        }
        lupos_scx_exit_rcu_unlock();
    }
}

/// Native BPF fatal-error continuation.
///
/// # Safety
/// Same verified-argument and RCU association contract as bpf_exit_bstr.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_exit_bpf_error_bstr(
    fmt: *mut c_char, data: *mut c_ulonglong, data_sz: u32, aux: *const bpf_prog_aux,
) {
    // SAFETY: The fatal-error kind and zero exit code are unchanged.
    unsafe {
        lupos_scx_exit_rcu_lock();
        let sch = lupos_scx_exit_error_prog_sched(aux);
        if lupos_scx_exit_error_likely_nonnull(sch) {
            scx_exit_bstr(sch, SCX_EXIT_ERROR_BPF, 0, sch, fmt, data, data_sz);
        }
        lupos_scx_exit_rcu_unlock();
    }
}

/// Native BPF dump continuation with CPU-scoped accumulator ownership.
///
/// # Safety
/// Verified packed kfunc arguments remain live. Only callbacks entered through
/// ops_dump_init on this CPU own the shared buffer under dump_lock/disabled IRQs;
/// callers outside that context receive the original scheduler error.
/// The original rejected-call path reads dd->cpu without dump_lock and may race
/// with another CPU's dump. All runtime CPU-token reads and writes now use
/// plain native leaves; the C/Rust whole-program race remains unqualified.
/// Callers are not restricted to already-valid dump contexts.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_exit_bpf_dump_bstr(
    fmt: *mut c_char, data: *mut c_ulonglong, data_sz: u32, aux: *const bpf_prog_aux,
) {
    // SAFETY: Keep RCU on all early-return paths. Accepted dump contexts protect
    // the buffer; they do not prove the rejected-call cpu read race-free. Cursor
    // remains bounded after append, and zero cursor is checked before [-1].
    unsafe {
        let dd = lupos_scx_core_dump_data();
        lupos_scx_exit_rcu_lock();
        (|| {
            let sch = lupos_scx_exit_dump_prog_sched(aux);
            if lupos_scx_exit_dump_unlikely_null(sch) { return; }
            if lupos_scx_exit_raw_cpu() != scx_shared_dump_cpu_read(dd) {
                lupos_scx_exit_error_dump_context(sch); return;
            }
            let line = ptr::addr_of_mut!((*dd).buf.line).cast::<c_char>();
            let capacity = LUPOS_SCX_EXIT_MSG_LEN as usize;
            let ret = lupos_scx_exit_bstr_format(sch, ptr::addr_of_mut!((*dd).buf.data).cast(),
                line.add((*dd).cursor as usize), capacity - (*dd).cursor as usize,
                fmt, data, data_sz);
            if ret < 0 {
                lupos_scx_exit_emit_format_error((*dd).s, (*dd).prefix, fmt, data, data_sz, ret);
                return;
            }
            (*dd).cursor = (*dd).cursor.wrapping_add(ret);
            (*dd).cursor = core::cmp::min((*dd).cursor, capacity as i32);
            if (*dd).cursor == 0 { return; }
            if (*dd).cursor as usize >= capacity
                || *line.add((*dd).cursor as usize - 1) == b'\n' as c_char {
                ops_dump_flush();
            }
        })();
        lupos_scx_exit_rcu_unlock();
    }
}

/// Dump one rq-stabilized task and its scheduler's task-specific callback.
///
/// # Safety
/// Caller holds dump_lock with IRQs disabled and rq's lock; p is current or on
/// rq's runnable list and belongs to a live scheduler. s/dctx remain writable
/// throughout synchronous callbacks. The static native bt buffer is dump-locked.
unsafe fn scx_dump_task(sch: *mut scx_sched, s: *mut seq_buf, dctx: *mut scx_dump_ctx,
                        rq: *mut rq, p: *mut task_struct, marker: c_char) {
    // SAFETY: Task fields are read only under their original rq protection;
    // array extents match the source, and format leaves borrow them synchronously.
    unsafe {
        let bt = lupos_scx_exit_task_bt();
        let task_sch = lupos_scx_exit_dump_task_sched(p);
        let mut sch_id = MaybeUninit::<[c_char; 32]>::uninit();
        let mut dsq_id = [0 as c_char; 19];
        ptr::copy_nonoverlapping(b"(n/a)\0".as_ptr().cast(), dsq_id.as_mut_ptr(), 6);
        let ops_state = lupos_scx_exit_task_ops_state(p);
        let own_marker = if task_sch == sch { b"*\0".as_ptr() } else { b"\0".as_ptr() };
        if (*task_sch).level == 0 {
            lupos_scx_exit_format_root_id(sch_id.as_mut_ptr().cast(), 32);
        } else {
            lupos_scx_exit_format_sub_id(sch_id.as_mut_ptr().cast(), 32, task_sch);
        }
        if !(*p).scx.dsq.is_null() {
            lupos_scx_exit_format_dsq_id(dsq_id.as_mut_ptr(), dsq_id.len(), (*p).scx.dsq);
        }
        dump_newline(s);
        lupos_scx_exit_emit_task_identity(s, p, marker, own_marker.cast(), sch_id.as_ptr().cast(),
            jiffies_delta_msecs((*p).scx.runnable_at, (*dctx).at_jiffies as c_ulong));
        lupos_scx_exit_emit_task_flags(s, p,
            scx_get_task_state(p) >> LUPOS_SCX_EXIT_TASK_STATE_SHIFT,
            scx_shared_flags_read(ptr::addr_of!((*p).scx)) & !(LUPOS_SCX_EXIT_TASK_STATE_MASK as u32),
            ops_state & LUPOS_SCX_EXIT_OPSS_STATE_MASK as c_ulong,
            ops_state >> LUPOS_SCX_EXIT_OPSS_QSEQ_SHIFT);
        lupos_scx_exit_emit_task_dsq(s, p, dsq_id.as_ptr());
        lupos_scx_exit_emit_task_slice(s, p);
        lupos_scx_exit_emit_task_cpus(s, p);
        if lupos_scx_exit_has_dump_task(sch) {
            ops_dump_init(s, b"    \0".as_ptr().cast());
            lupos_scx_exit_call_dump_task(sch, rq, dctx, p);
            ops_dump_exit();
        }
        #[cfg(CONFIG_STACKTRACE)]
        let bt_len = lupos_scx_exit_save_task_stack(p, bt);
        #[cfg(not(CONFIG_STACKTRACE))]
        let bt_len = 0;
        if bt_len != 0 {
            dump_newline(s);
            dump_stack_trace(s, b"    \0".as_ptr().cast(), bt, bt_len);
        }
    }
}

/// Dump a CPU under its rq lock, committing an idle CPU only for extra output.
///
/// # Safety
/// cpu is a valid allocated possible CPU; sch and its per-CPU storage are live.
/// Caller holds the global dump lock with IRQs disabled, may nest rq locking,
/// and pins s/dctx/backing storage throughout this synchronous operation.
unsafe fn scx_dump_cpu(sch: *mut scx_sched, s: *mut seq_buf, dctx: *mut scx_dump_ctx,
                       cpu: c_int, dump_all_tasks: bool) {
    // SAFETY: Every branch reaches rq unlock. Nested seq output is committed
    // only when avail is nonzero, preserving the native overflow/BUG boundary.
    unsafe {
        let rq = lupos_scx_exit_cpu_rq(cpu);
        let pcpu = lupos_scx_exit_pcpu(sch, cpu);
        let mut rf = MaybeUninit::<rq_flags>::uninit();
        lupos_scx_exit_rq_lock(rq, rf.as_mut_ptr());
        (|| {
            let head = ptr::addr_of_mut!((*rq).scx.runnable_list);
            let idle = lupos_scx_exit_list_empty(head)
                && scx_shared_class_read(lupos_scx_core_rq_curr(rq)) == lupos_scx_exit_idle_class();
            if idle && !lupos_scx_exit_has_dump_cpu(sch) { return; }
            let mut buf = ptr::null_mut();
            let avail = lupos_scx_exit_seq_get_buf(s, &mut buf);
            let mut ns = MaybeUninit::<seq_buf>::uninit();
            let ns = ns.as_mut_ptr();
            lupos_scx_exit_seq_init(ns, buf, avail);
            dump_newline(ns);
            lupos_scx_exit_emit_cpu_state(ns, rq, cpu);
            lupos_scx_exit_rescue_dump(ns, rq);
            lupos_scx_exit_emit_cpu_current(ns, rq);
            let kick = lupos_scx_exit_to_kick(pcpu);
            if !lupos_scx_exit_mask_empty(kick) {
                lupos_scx_exit_emit_cpu_kick(ns, kick);
            }
            let idle_kick = lupos_scx_exit_to_idle(pcpu);
            if !lupos_scx_exit_mask_empty(idle_kick) {
                lupos_scx_exit_emit_cpu_idle(ns, idle_kick);
            }
            let preempt = lupos_scx_exit_to_preempt(pcpu);
            if !lupos_scx_exit_mask_empty(preempt) {
                lupos_scx_exit_emit_cpu_preempt(ns, preempt);
            }
            let wait = lupos_scx_exit_to_wait(pcpu);
            if !lupos_scx_exit_mask_empty(wait) {
                lupos_scx_exit_emit_cpu_wait(ns, wait);
            }
            let sync = lupos_scx_exit_to_sync(rq);
            if !lupos_scx_exit_mask_empty(sync) {
                lupos_scx_exit_emit_cpu_sync(ns, sync);
            }
            let used = lupos_scx_exit_seq_used(ns);
            if lupos_scx_exit_has_dump_cpu(sch) {
                ops_dump_init(ns, b"  \0".as_ptr().cast());
                lupos_scx_exit_call_dump_cpu(sch, rq, dctx, cpu, idle);
                ops_dump_exit();
            }
            if idle && used == lupos_scx_exit_seq_used(ns) { return; }
            if avail != 0 {
                lupos_scx_exit_seq_commit(s, lupos_scx_exit_seq_used(ns));
                if lupos_scx_exit_seq_overflowed(ns) { lupos_scx_exit_seq_set_overflow(s); }
            }
            if scx_shared_class_read(lupos_scx_core_rq_curr(rq)) == lupos_scx_core_ext_class()
                && (dump_all_tasks || lupos_scx_exit_curr_on_sched(sch, lupos_scx_core_rq_curr(rq))) {
                scx_dump_task(sch, s, dctx, rq, lupos_scx_core_rq_curr(rq), b'*' as c_char);
            }
            let mut node = (*head).next;
            while node != head {
                let p = lupos_scx_exit_runnable_task(node);
                if dump_all_tasks || lupos_scx_exit_runnable_on_sched(sch, p) {
                    scx_dump_task(sch, s, dctx, rq, p, b' ' as c_char);
                }
                node = (*node).next;
            }
        })();
        lupos_scx_exit_rq_unlock(rq, rf.as_mut_ptr());
    }
}

/// Produce the original serialized scheduler dump, prioritizing stalled CPUs.
///
/// # Safety
/// sch/ei are live, ei->dump has dump_len bytes and backtrace/msg/reason pointers
/// are initialized for kind. Caller permits raw IRQ-save dump locking and nested
/// rq locks. All native dump callbacks use their original guarded ABI leaves.
pub(crate) unsafe fn scx_dump_state(
    sch: *mut scx_sched, ei: *mut scx_exit_info, dump_len: usize, dump_all_tasks: bool,
) {
    // SAFETY: Context timestamps are sampled before locking as in the source.
    // No callback runs after dump_disabled is observed under the single lock.
    unsafe {
        let mut dctx = scx_dump_ctx {
            kind: (*ei).kind, exit_code: (*ei).exit_code, reason: (*ei).reason,
            at_ns: lupos_scx_exit_now_ns(), at_jiffies: lupos_scx_exit_jiffies() as u64,
        };
        lupos_scx_exit_with_dump_lock(sch, ei, dump_len, dump_all_tasks, &mut dctx);
    }
}

/// State-dump body borrowed by the exact native raw_spinlock_irqsave guard.
///
/// # Safety
/// The native envelope owns dump_lock with its reference-counted interrupt
/// guard until this call returns. Arguments satisfy scx_dump_state's contract;
/// dctx is the caller's initialized context and cannot escape this call.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_exit_dump_state_locked(
    sch: *mut scx_sched, ei: *mut scx_exit_info, dump_len: usize,
    dump_all_tasks: bool, dctx: *mut scx_dump_ctx,
) {
    // SAFETY: All early returns unwind through the original native guard. No
    // lock representation is fabricated and no ordinary IRQ-save substitute is used.
    unsafe {
        const TRUNC_MARKER: &[u8] = b"\n\n~~~~ TRUNCATED ~~~~\n\0";
            if (*sch).dump_disabled { return; }
            let mut storage = MaybeUninit::<seq_buf>::uninit();
            let s = storage.as_mut_ptr();
            lupos_scx_exit_seq_init(s, (*ei).dump, dump_len);
            #[cfg(CONFIG_EXT_SUB_SCHED)]
            if (*sch).level == 0 { lupos_scx_exit_emit_root(s, sch); }
            else { lupos_scx_exit_emit_sub(s, sch); }
            if (*ei).kind == SCX_EXIT_NONE {
                lupos_scx_exit_emit_debug(s, (*ei).reason);
            } else {
                if (*ei).exit_cpu >= 0 { lupos_scx_exit_emit_cause_cpu(s, ei); }
                else { lupos_scx_exit_emit_cause(s, ei); }
                lupos_scx_exit_emit_reason(s, ei);
                dump_newline(s);
                lupos_scx_exit_emit_backtrace(s);
                dump_stack_trace(s, b"  \0".as_ptr().cast(), (*ei).bt, (*ei).bt_len);
            }
            if lupos_scx_exit_has_dump(sch) {
                ops_dump_init(s, b"\0".as_ptr().cast());
                lupos_scx_exit_call_dump(sch, dctx);
                ops_dump_exit();
            }
            dump_newline(s);
            lupos_scx_exit_emit_cpu_heading(s);
            lupos_scx_exit_emit_cpu_rule(s);
            let stalled = lupos_scx_exit_stall_mask(sch);
            if !lupos_scx_exit_mask_empty(stalled) {
                let mut cpu = lupos_scx_exit_mask_iter_first(stalled);
                while cpu < lupos_scx_exit_mask_iter_limit() as c_int {
                    scx_dump_cpu(sch, s, dctx, cpu, dump_all_tasks);
                    cpu = lupos_scx_exit_mask_next(cpu, stalled);
                }
                let mut cpu = lupos_scx_exit_possible_iter_first();
                while cpu < lupos_scx_exit_possible_iter_limit() as c_int {
                    if !lupos_scx_exit_mask_test(cpu, stalled) {
                        scx_dump_cpu(sch, s, dctx, cpu, dump_all_tasks);
                    }
                    cpu = lupos_scx_exit_possible_iter_next(cpu);
                }
            } else {
                if (*ei).exit_cpu >= 0 {
                    scx_dump_cpu(sch, s, dctx, (*ei).exit_cpu, dump_all_tasks);
                }
                let mut cpu = lupos_scx_exit_possible_iter_first();
                while cpu < lupos_scx_exit_possible_iter_limit() as c_int {
                    if cpu != (*ei).exit_cpu {
                        scx_dump_cpu(sch, s, dctx, cpu, dump_all_tasks);
                    }
                    cpu = lupos_scx_exit_possible_iter_next(cpu);
                }
            }
            dump_newline(s);
            lupos_scx_exit_emit_event_heading(s);
            lupos_scx_exit_emit_event_rule(s);
            let mut events = MaybeUninit::<scx_event_stats>::uninit();
            scx_read_events(sch, events.as_mut_ptr());
            lupos_scx_exit_emit_events(s, events.as_mut_ptr());
            if lupos_scx_exit_seq_overflowed(s) && dump_len >= TRUNC_MARKER.len() {
                ptr::copy_nonoverlapping(TRUNC_MARKER.as_ptr().cast::<c_char>(),
                    (*ei).dump.add(dump_len - TRUNC_MARKER.len()), TRUNC_MARKER.len());
            }
    }
}

/// Native disable-IRQ continuation: capture an error dump before helper work.
///
/// # Safety
/// sch is pinned by the executing native disable_irq_work and owns initialized
/// ei/helper/work objects; IRQ context allows the original dump lock protocol.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_exit_disable_irq_workfn(sch: *mut scx_sched) {
    // SAFETY: ei's error kind gates the whole dump; helper queueing always follows.
    unsafe {
        let ei = (*sch).exit_info;
        if (*ei).kind >= SCX_EXIT_ERROR {
            scx_dump_state(sch, ei, lupos_scx_exit_dump_len(sch), true);
        }
        kthread_queue_work((*sch).helper, ptr::addr_of_mut!((*sch).disable_work));
    }
}

/// Print one bounded, best-effort scheduler/task status line.
///
/// # Safety
/// p is accessible for the call and log_lvl is a live NUL-terminated level
/// string. RCU pins its associated scheduler; sched_class/runnable_at are copied
/// through native nofault reads because rq/task state need not be synchronized.
#[no_mangle]
pub unsafe extern "C" fn print_scx_info(log_lvl: *const c_char, p: *mut task_struct) {
    // SAFETY: Failed nofault reads never expose uninitialized values. The state
    // and +all snapshot precede RCU as originally; no new synchronization occurs.
    unsafe {
        let state = scx_enable_state();
        let all = if lupos_scx_core_switching_all_read_once() {
            b"+all\0".as_ptr()
        } else { b"\0".as_ptr() };
        let mut runnable_at_buf = [0 as c_char; 22];
        runnable_at_buf[0] = b'?' as c_char;
        lupos_scx_exit_rcu_lock();
        (|| {
            let sch = lupos_scx_exit_info_task_sched(p);
            if sch.is_null() { return; }
            let mut class = MaybeUninit::<*mut sched_class>::uninit();
            if copy_from_kernel_nofault(class.as_mut_ptr().cast(),
                    ptr::addr_of!((*p).sched_class).cast(), size_of::<*mut sched_class>()) != 0
                || class.assume_init().cast_const() != lupos_scx_core_ext_class() {
                lupos_scx_exit_print_info(log_lvl, sch, state, all.cast());
                return;
            }
            let mut runnable_at = MaybeUninit::<c_ulong>::uninit();
            if copy_from_kernel_nofault(runnable_at.as_mut_ptr().cast(),
                    ptr::addr_of!((*p).scx.runnable_at).cast(), size_of::<c_ulong>()) == 0 {
                lupos_scx_exit_format_runnable(runnable_at_buf.as_mut_ptr(), runnable_at_buf.len(),
                    jiffies_delta_msecs(runnable_at.assume_init(), lupos_scx_exit_jiffies()));
            }
            lupos_scx_exit_print_task_info(log_lvl, sch,
                state, all.cast(), runnable_at_buf.as_ptr());
        })();
        lupos_scx_exit_rcu_unlock();
    }
}
