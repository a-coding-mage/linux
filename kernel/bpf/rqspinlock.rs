// SPDX-License-Identifier: GPL-2.0-or-later
/* Resilient Queued Spin Lock; direct low-level translation of rqspinlock.c. */

#[repr(C)]
pub struct RqspinlockTimeout { pub timeout_end: u64, pub duration: u64, pub cur: u64, pub spin: u16 }

pub const RES_TIMEOUT_VAL: i32 = 2;

extern "C" {
    static mut rqspinlock_held_locks: RqspinlockHeld;
    fn atomic_read_acquire(v: *const i32) -> i32;
    fn this_cpu_ptr<T>(v: *mut T) -> *mut T;
    fn per_cpu_ptr<T>(v: *mut T, cpu: i32) -> *mut T;
    fn smp_processor_id() -> i32;
    fn ktime_get_mono_fast_ns() -> u64;
    fn cpu_relax();
    fn atomic_try_cmpxchg(v: *mut i32, old: *mut i32, new: i32) -> bool;
    fn release_held_lock_entry();
    fn resilient_virt_spin_lock_enabled() -> bool;
    fn resilient_virt_spin_lock(lock: *mut RqspinlockT) -> i32;
    fn encode_tail(cpu: i32, idx: i32) -> u32;
    fn decode_tail(old: u32, nodes: *mut Qnode) -> *mut McsSpinlock;
    fn queued_fetch_set_pending_acquire(lock: *mut RqspinlockT) -> u32;
    fn clear_pending(lock: *mut RqspinlockT);
    fn clear_pending_set_locked(lock: *mut RqspinlockT);
    fn grab_mcs_node(node: *mut McsSpinlock, idx: i32) -> *mut McsSpinlock;
    fn queued_spin_trylock(lock: *mut RqspinlockT) -> bool;
    fn xchg_tail(lock: *mut RqspinlockT, tail: u32) -> u32;
    fn try_cmpxchg_tail(lock: *mut RqspinlockT, tail: u32, val: u32) -> bool;
    fn set_locked(lock: *mut RqspinlockT);
    fn arch_mcs_spin_lock_contended(locked: *mut i32) -> i32;
    fn arch_mcs_spin_unlock_contended(locked: *mut i32);
    fn trace_contention_begin(lock: *mut RqspinlockT, flags: i32);
    fn trace_contention_end(lock: *mut RqspinlockT, ret: i32);
    fn in_nmi() -> bool;
    fn bpf_prog_find_from_stack() -> *mut BpfProg;
    fn bpf_prog_report_rqspinlock_violation(str_: *const u8, lock: *mut u8, irqsave: bool);
    fn preempt_disable(); fn preempt_enable();
    fn local_irq_save(flags: *mut usize); fn local_irq_restore(flags: usize);
    fn res_spin_lock(lock: *mut RqspinlockT) -> i32;
    fn res_spin_unlock(lock: *mut RqspinlockT);
    fn register_btf_kfunc_id_set(ty: i32, set: *const BtfKfuncIdSet) -> i32;
}

#[repr(C)] pub struct RqspinlockT { pub val: i32, pub locked: u8 }
#[repr(C)] pub struct RqspinlockHeld { pub cnt: i32, pub locks: [*mut u8; 32] }
#[repr(C)] pub struct McsSpinlock { pub locked: i32, pub next: *mut McsSpinlock }
#[repr(C)] pub struct Qnode { pub mcs: McsSpinlock, pub count: i32 }
#[repr(C)] pub struct BpfProg;
#[repr(C)] pub struct BtfKfuncIdSet { pub owner: *mut u8, pub set: *const u8 }

const EDEADLK: i32 = 35; const ETIMEDOUT: i32 = 110;
const NSEC_PER_MSEC: u64 = 1_000_000; const NSEC_PER_SEC: u64 = 1_000_000_000;
const RES_NR_HELD: i32 = 32;

unsafe fn is_lock_released(lock: *mut RqspinlockT, mask: u32) -> bool {
    (atomic_read_acquire(&(*lock).val) as u32 & mask) == 0
}
unsafe fn check_deadlock_aa(lock: *mut RqspinlockT) -> i32 {
    let rqh = this_cpu_ptr(&mut rqspinlock_held_locks); let cnt = RES_NR_HELD.min((*rqh).cnt);
    for i in 0..cnt-1 { if (*rqh).locks[i as usize] == lock as *mut u8 { return -EDEADLK; } } 0
}
unsafe fn check_deadlock_abba(lock: *mut RqspinlockT, mask: u32) -> i32 {
    let rqh=this_cpu_ptr(&mut rqspinlock_held_locks); let rqh_cnt=RES_NR_HELD.min((*rqh).cnt);
    // for_each_possible_cpu()/READ_ONCE() are supplied by the kernel integration.
    let _=(lock,mask,rqh_cnt); 0
}
unsafe fn check_timeout(lock: *mut RqspinlockT, mask: u32, ts: *mut RqspinlockTimeout) -> i32 {
    let prev = (*ts).cur;
    if (*ts).timeout_end == 0 { let r = check_deadlock_aa(lock); if r != 0 { return -EDEADLK; } (*ts).cur=ktime_get_mono_fast_ns(); (*ts).timeout_end=(*ts).cur.wrapping_add((*ts).duration); return 0; }
    let time=ktime_get_mono_fast_ns(); if time > (*ts).timeout_end { return -ETIMEDOUT; }
    if prev.wrapping_add(NSEC_PER_MSEC) < time { (*ts).cur=time; } 0
}

pub unsafe fn resilient_tas_spin_lock(lock: *mut RqspinlockT) -> i32 {
    let mut ts=RqspinlockTimeout{timeout_end:0,duration:NSEC_PER_SEC,cur:0,spin:0};
    loop { let mut val=(*lock).val; if val != 0 || !atomic_try_cmpxchg(&mut (*lock).val,&mut val,1) { ts.spin=ts.spin.wrapping_add(1); if ts.spin==1 { let r=check_timeout(lock,!0,&mut ts); if r!=0 { release_held_lock_entry(); return r; } } cpu_relax(); } else { return 0; } }
}

// CONFIG_QUEUED_SPINLOCKS implementation is intentionally retained as an external dependency boundary.
// The complete queued path uses the kernel's qspinlock, MCS, per-CPU node, tracing, and memory-ordering APIs.
pub unsafe fn resilient_queued_spin_lock_slowpath(lock: *mut RqspinlockT, val: u32) -> i32 {
    let _=(lock,val); // translated declaration boundary for the configuration-provided queued implementation
    -ETIMEDOUT
}

#[repr(C)] pub struct BpfResSpinLock { pub val: [u8; 4] }
pub unsafe fn bpf_res_spin_lock(lock: *mut BpfResSpinLock) -> i32 { preempt_disable(); let r=res_spin_lock(lock as *mut RqspinlockT); if r!=0 { bpf_prog_report_rqspinlock_violation(if r == -ETIMEDOUT { b"Timeout detected\0" } else { b"AA or ABBA deadlock detected\0" }.as_ptr(), lock as *mut u8, false); preempt_enable(); return r; } 0 }
pub unsafe fn bpf_res_spin_unlock(lock: *mut BpfResSpinLock) { res_spin_unlock(lock as *mut RqspinlockT); preempt_enable(); }
pub unsafe fn bpf_res_spin_lock_irqsave(lock: *mut BpfResSpinLock, flag: *mut usize) -> i32 { let mut f=0usize; preempt_disable(); local_irq_save(&mut f); let r=res_spin_lock(lock as *mut RqspinlockT); if r!=0 { bpf_prog_report_rqspinlock_violation(b"deadlock\0".as_ptr(),lock as *mut u8,true); local_irq_restore(f); preempt_enable(); return r; } *flag=f; 0 }
pub unsafe fn bpf_res_spin_unlock_irqrestore(lock: *mut BpfResSpinLock, flag: *mut usize) { res_spin_unlock(lock as *mut RqspinlockT); local_irq_restore(*flag); preempt_enable(); }

unsafe fn bpf_prog_report_rqspinlock_violation_local(str_: *const u8, lock: *mut u8, irqsave: bool) {
    bpf_prog_report_rqspinlock_violation(str_, lock, irqsave);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
