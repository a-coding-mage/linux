// SPDX-License-Identifier: GPL-2.0
// F15 DSQ kfunc owners, ext.c:9311-9337,9349-9406,9427-9513,9583-9884.
// Authority: 126a30fae3bba11420ec2fcbde51a0a01bab1b5b. Extends retained ext.rs.
// Native shells/primitives are explicit unqualified C runtime, not Rust owners.
compile_error!("SOURCE ONLY HOLD: sched_ext DSQ kfunc ABI, shared access and protection qualification incomplete");

use super::*;
use core::ptr;
use kernel::ffi::c_int;

/// Legacy local reenqueue, restricted to the original cpu_release context.
///
/// # Safety
/// The exact native kfunc shell holds guard(rcu). The callback caller already
/// pins its CPU and owns its rq lock; this body does not acquire an rq lock.
#[export_name = "lupos_scx_core_kq_reenqueue_local_body"]
pub unsafe extern "C" fn scx_bpf_reenqueue_local(aux: *const bpf_prog_aux) -> u32 {
    unsafe {
        let sch = lupos_scx_core_kq_prog_sched(aux);
        if lupos_scx_core_kq_unlikely_reenqueue_no_sched(sch) {
            return 0;
        }
        let rq = lupos_scx_core_kq_cpu_rq(lupos_scx_core_kq_processor_id());
        lupos_scx_core_kq_assert_reenqueue_rq(rq);
        reenq_local(sch, rq, SCX_REENQ_ANY as u64)
    }
}

/// Allocate and initialize before acquiring RCU, then publish or unwind.
///
/// # Safety
/// Called in the original sleepable GFP_KERNEL context with live implicit aux.
/// The fresh native DSQ allocation stays exclusively owned until insertion.
#[export_name = "lupos_scx_core_kq_create_dsq_body"]
pub unsafe extern "C" fn scx_bpf_create_dsq(
    dsq_id: u64, node: i32, aux: *const bpf_prog_aux,
) -> i32 {
    unsafe {
        if lupos_scx_core_kq_unlikely_bad_node(node) {
            return -(EINVAL as i32);
        }
        if lupos_scx_core_kq_unlikely_create_builtin(dsq_id) {
            return -(EINVAL as i32);
        }
        let dsq = lupos_scx_core_kq_alloc_dsq(node);
        if dsq.is_null() {
            return -(ENOMEM as i32);
        }
        // scx_init_dsq may sleep. Its NULL owner is filled only under RCU.
        let ret = scx_init_dsq(dsq, dsq_id, ptr::null_mut());
        if ret != 0 {
            lupos_scx_core_kq_free_dsq(dsq);
            return ret;
        }
        let ret = lupos_scx_core_kq_create_rcu(dsq, aux);
        if ret != 0 {
            exit_dsq(dsq);
            lupos_scx_core_kq_free_dsq(dsq);
        }
        ret
    }
}

/// RCU-delimited continuation of create_dsq, with no additional allocation.
///
/// # Safety
/// The native envelope holds the original explicit rcu_read_lock. dsq is a
/// fully initialized private allocation, not yet visible in any scheduler hash.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_core_kq_create_rcu_body(
    dsq: *mut scx_dispatch_q, aux: *const bpf_prog_aux,
) -> i32 {
    unsafe {
        let sch = lupos_scx_core_kq_prog_sched(aux);
        if !sch.is_null() {
            (*dsq).sched = sch;
            lupos_scx_core_kq_hash_insert(sch, dsq)
        } else {
            -(ENODEV as i32)
        }
    }
}

/// Apply a task slice directly under the applicable rq lock, or stash OOB.
///
/// # Safety
/// The native kfunc RCU guard pins scheduler association and verifier-supplied
/// task lifetime. Native plain slice access retains the permitted race with a
/// user-DSQ/BPF dispatch commit; RCU is not writer exclusion. The shared-access
/// boundary is transitional native runtime, with no new atomic/lock semantics.
#[export_name = "lupos_scx_core_kq_task_set_slice_body"]
pub unsafe extern "C" fn scx_bpf_task_set_slice(
    p: *mut task_struct, slice: u64, aux: *const bpf_prog_aux,
) -> bool {
    unsafe {
        let sch = lupos_scx_core_kq_prog_sched(aux);
        if lupos_scx_core_kq_unlikely_slice_unauthorized(sch, p) {
            return false;
        }
        let locked_rq = lupos_scx_core_locked_rq();
        if locked_rq.is_null()
            || (lupos_scx_core_kq_runnable_cpu_read_once(p)
                != lupos_scx_core_kq_cpu_of(locked_rq)
                && !lupos_scx_core_kq_task_current(locked_rq, p))
        {
            set_task_slice_oob(sch, p, slice);
            return true;
        }
        if slice > scx_shared_slice_read(ptr::addr_of!((*p).scx))
            && lupos_scx_core_kq_unlikely_slice_missing_base(sch, locked_rq)
        {
            lupos_scx_core_kq_event_slice_denied(sch);
            return true;
        }
        if lupos_scx_core_kq_unlikely_slice_protected(!scx_set_task_slice(p, slice)) {
            lupos_scx_core_kq_event_slice_denied(sch);
        }
        true
    }
}

/// Preserve the authorized plain virtual-time write, without adding locking.
///
/// # Safety
/// Native RCU pins the scheduler/task lifetime and the verifier supplies p.
/// The original scheduler is responsible for synchronization of BPF-owned and
/// user-DSQ tasks; the plain native shared-access boundary is not a race proof.
#[export_name = "lupos_scx_core_kq_task_set_dsq_vtime_body"]
pub unsafe extern "C" fn scx_bpf_task_set_dsq_vtime(
    p: *mut task_struct, vtime: u64, aux: *const bpf_prog_aux,
) -> bool {
    unsafe {
        let sch = lupos_scx_core_kq_prog_sched(aux);
        if lupos_scx_core_kq_unlikely_vtime_unauthorized(sch, p) {
            return false;
        }
        scx_shared_vtime_write(ptr::addr_of_mut!((*p).scx), vtime);
        true
    }
}

/// CPU-addressed kick wrapper; the F07 owner keeps actual deferred delivery.
///
/// # Safety
/// The native kfunc guard pins sch under RCU. Arguments can be invalid; validate
/// CPU before calling F07, which retains NMI, IRQ and capability handling.
#[export_name = "lupos_scx_core_kq_kick_cpu_body"]
pub unsafe extern "C" fn scx_bpf_kick_cpu(cpu: i32, flags: u64, aux: *const bpf_prog_aux) {
    unsafe {
        let sch = lupos_scx_core_kq_prog_sched(aux);
        if lupos_scx_core_kq_likely_kick_sched(sch) && scx_cpu_valid(sch, cpu, ptr::null()) {
            scx_kick_cpu(sch, cpu, flags);
        }
    }
}

/// CID-addressed kick wrapper with the original CID error path.
///
/// # Safety
/// The native RCU guard pins sch and CID mapping. F07 owns custody/capability
/// checks when deferred delivery occurs, rather than this request wrapper.
#[export_name = "lupos_scx_core_kq_kick_cid_body"]
pub unsafe extern "C" fn scx_bpf_kick_cid(cid: i32, flags: u64, aux: *const bpf_prog_aux) {
    unsafe {
        let sch = lupos_scx_core_kq_prog_sched(aux);
        if lupos_scx_core_kq_unlikely_kick_cid_no_sched(sch) {
            return;
        }
        let cpu = lupos_scx_core_kq_cid_to_cpu(sch, cid);
        if cpu < 0 {
            return;
        }
        scx_kick_cpu(sch, cpu, flags);
    }
}

/// Count tasks in a local/local-on/user DSQ using the original READ_ONCE sites.
///
/// # Safety
/// The native wrapper disables preemption before association lookup and enables
/// it after every return, retaining the original RCU reader lifetime protocol.
#[export_name = "lupos_scx_core_kq_dsq_nr_queued_body"]
pub unsafe extern "C" fn scx_bpf_dsq_nr_queued(dsq_id: u64, aux: *const bpf_prog_aux) -> i32 {
    unsafe {
        let sch = lupos_scx_core_kq_prog_sched(aux);
        if lupos_scx_core_kq_unlikely_count_no_sched(sch) {
            return -(ENODEV as i32);
        }
        if dsq_id == SCX_DSQ_LOCAL as u64 {
            let locked_rq = lupos_scx_core_locked_rq();
            let rq = if locked_rq.is_null() { lupos_scx_core_kq_this_rq() } else { locked_rq };
            return lupos_scx_core_kq_local_nr_read_once(rq) as i32;
        } else if dsq_id & SCX_DSQ_LOCAL_ON as u64 == SCX_DSQ_LOCAL_ON as u64 {
            // Preserve native u64-to-s32 parameter conversion before CID/CPU lookup.
            let cpu = lupos_scx_core_kq_cpu_ret(sch, (dsq_id & SCX_DSQ_LOCAL_CPU_MASK as u64) as i32);
            if scx_cpu_valid(sch, cpu, ptr::null()) {
                return lupos_scx_core_kq_local_on_nr_read_once(cpu) as i32;
            }
        } else {
            let dsq = find_user_dsq(sch, dsq_id);
            if !dsq.is_null() {
                return lupos_scx_core_kq_user_nr_read_once(dsq) as i32;
            }
        }
        -(ENOENT as i32)
    }
}

/// Resolve the calling scheduler and hand custom DSQ destruction to F09.
///
/// # Safety
/// The native kfunc's RCU guard spans lookup and nested F09 destruction. Caller
/// must obey the original empty/no-further-dispatch contract for this DSQ.
#[export_name = "lupos_scx_core_kq_destroy_dsq_body"]
pub unsafe extern "C" fn scx_bpf_destroy_dsq(dsq_id: u64, aux: *const bpf_prog_aux) {
    unsafe {
        let sch = lupos_scx_core_kq_prog_sched(aux);
        if !sch.is_null() {
            destroy_dsq(sch, dsq_id);
        }
    }
}

/// Initialize the real native iterator, leaving dsq NULL on every early error.
///
/// # Safety
/// The native shell verifies the original size, alignment and flag overlap,
/// then performs the exact native it-to-kit conversion. The verifier provides
/// writable iterator storage and the original RCU-protected iterator lifetime;
/// its stable address is required by INIT_DSQ_LIST_CURSOR's self-linked node.
#[export_name = "lupos_scx_core_kq_iter_new_body"]
pub unsafe extern "C" fn bpf_iter_scx_dsq_new(
    kit: *mut bpf_iter_scx_dsq_kern, dsq_id: u64, flags: u64,
    aux: *const bpf_prog_aux,
) -> c_int {
    unsafe {
        (*kit).dsq = ptr::null_mut();
        let sch = lupos_scx_core_kq_prog_sched(aux);
        if lupos_scx_core_kq_unlikely_iter_no_sched(sch) {
            return -(ENODEV as c_int);
        }
        if lupos_scx_core_kq_iter_bad_flags(flags) {
            return -(EINVAL as c_int);
        }
        (*kit).dsq = find_user_dsq(sch, dsq_id);
        if (*kit).dsq.is_null() {
            return -(ENOENT as c_int);
        }
        lupos_scx_core_kq_init_cursor(kit, flags);
        0
    }
}

/// Advance the original iterator without acquiring a task reference.
///
/// # Safety
/// kit is the verifier's live initialized iterator, including failed new().
/// Caller keeps the original RCU lifetime through iteration and destroy. Each
/// returned task is borrowed under that lifetime, never get_task_struct-owned.
#[export_name = "lupos_scx_core_kq_iter_next_body"]
pub unsafe extern "C" fn bpf_iter_scx_dsq_next(kit: *mut bpf_iter_scx_dsq_kern) -> *mut task_struct {
    unsafe {
        if (*kit).dsq.is_null() {
            return ptr::null_mut();
        }
        lupos_scx_core_kq_iter_next_guard(kit)
    }
}

/// DSQ-locked continuation, delegating cursor policy only to the F01 owner.
///
/// # Safety
/// Native guard(raw_spinlock_irqsave) holds kit->dsq->lock, and the original
/// RCU/verifier lifetime pins the iterator and DSQ throughout this call.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_core_kq_iter_next_locked_body(
    kit: *mut bpf_iter_scx_dsq_kern,
) -> *mut task_struct {
    unsafe { nldsq_cursor_next_task(ptr::addr_of_mut!((*kit).cursor), (*kit).dsq) }
}

/// Remove the iterator cursor if linked, then invalidate the iterator.
///
/// # Safety
/// kit has its original verifier/RCU lifetime and is not concurrently operated
/// on. The first list_empty is intentionally before taking dsq->lock; the
/// explicit native lock/IRQ pair protects removal exactly as the source does.
#[export_name = "lupos_scx_core_kq_iter_destroy_body"]
pub unsafe extern "C" fn bpf_iter_scx_dsq_destroy(kit: *mut bpf_iter_scx_dsq_kern) {
    unsafe {
        if (*kit).dsq.is_null() {
            return;
        }
        if !lupos_scx_core_kq_cursor_empty(kit) {
            lupos_scx_core_kq_cursor_unlink_locked(kit);
        }
        (*kit).dsq = ptr::null_mut();
    }
}

/// Borrow the first user-DSQ task through the original RCU dereference.
///
/// # Safety
/// Original KF_RCU_PROTECTED context pins scheduler/DSQ/task storage. No guard
/// or reference is added here; the returned pointer is a changing snapshot.
#[export_name = "lupos_scx_core_kq_dsq_peek_body"]
pub unsafe extern "C" fn scx_bpf_dsq_peek(dsq_id: u64, aux: *const bpf_prog_aux) -> *mut task_struct {
    unsafe {
        let sch = lupos_scx_core_kq_prog_sched(aux);
        if lupos_scx_core_kq_unlikely_peek_no_sched(sch) {
            return ptr::null_mut();
        }
        if lupos_scx_core_kq_unlikely_peek_builtin(dsq_id) {
            lupos_scx_core_kq_error_peek_builtin(sch, dsq_id);
            return ptr::null_mut();
        }
        let dsq = find_user_dsq(sch, dsq_id);
        if lupos_scx_core_kq_unlikely_peek_missing(dsq) {
            lupos_scx_core_kq_error_peek_missing(sch, dsq_id);
            return ptr::null_mut();
        }
        lupos_scx_core_kq_first_task_rcu(dsq)
    }
}

/// Validate flags and resolve the target before scheduling asynchronous reenq.
///
/// # Safety
/// The native shell snapshots scx_locked_rq before guard(preempt), exactly as
/// the original, then pins CPU and scheduler/DSQ lifetime for this whole body.
/// locked_rq is that original snapshot; no lock is invented for DSQ-id reads.
#[export_name = "lupos_scx_core_kq_dsq_reenq_body"]
pub unsafe extern "C" fn scx_bpf_dsq_reenq(
    dsq_id: u64, mut reenq_flags: u64, aux: *const bpf_prog_aux,
    locked_rq: *mut rq,
) {
    unsafe {
        let sch = lupos_scx_core_kq_prog_sched(aux);
        if lupos_scx_core_kq_unlikely_reenq_no_sched(sch) {
            return;
        }
        if lupos_scx_core_kq_unlikely_reenq_flags(reenq_flags) {
            lupos_scx_core_kq_error_reenq_flags(sch, reenq_flags);
            return;
        }
        if reenq_flags & __SCX_REENQ_FILTER_MASK as u64 == 0 {
            reenq_flags |= SCX_REENQ_ANY as u64;
        }
        let rq = if locked_rq.is_null() { lupos_scx_core_kq_this_rq() } else { locked_rq };
        let dsq = find_dsq_for_dispatch(sch, rq, dsq_id, lupos_scx_core_kq_processor_id());
        schedule_dsq_reenq(sch, dsq, reenq_flags, locked_rq);
    }
}

/// The v2 local wrapper retains its call to the original public reenq shell.
///
/// # Safety
/// aux is the implicit verifier-supplied association. The target native shell
/// performs its original locked-rq snapshot and preempt guard at call entry.
#[export_name = "lupos_scx_core_kq_reenqueue_local_v2_body"]
pub unsafe extern "C" fn scx_bpf_reenqueue_local___v2(aux: *const bpf_prog_aux) {
    unsafe { lupos_scx_core_kq_call_dsq_reenq(SCX_DSQ_LOCAL as u64, 0, aux); }
}
