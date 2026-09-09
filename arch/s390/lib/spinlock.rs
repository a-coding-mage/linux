// SPDX-License-Identifier: GPL-2.0
/*
 *    Out of line spinlock code.
 *
 *    Copyright IBM Corp. 2004, 2006
 *    Author(s): Martin Schwidefsky (schwidefsky@de.ibm.com)
 */

use core::ptr;

pub static mut spin_retry: i32 = -1;

unsafe extern "C" {
    fn simple_strtoul(s: *mut i8, endp: *mut *mut i8, base: i32) -> u64;
    fn memset(s: *mut core::ffi::c_void, c: i32, n: usize) -> *mut core::ffi::c_void;
    fn per_cpu_ptr<T>(ptr: *mut T, cpu: i32) -> *mut T;
    fn this_cpu_ptr<T>(ptr: *mut T) -> *mut T;
    fn get_lowcore() -> *mut Lowcore;
    fn spinlock_lockval() -> i32;
    fn arch_vcpu_is_preempted(cpu: i32) -> bool;
    fn smp_yield_cpu(cpu: i32);
    fn machine_is_lpar() -> bool;
    fn test_cpu_flag(flag: i32) -> bool;
    fn in_interrupt() -> bool;
    fn barrier();
    fn trace_contention_begin(lock: *mut arch_spinlock_t, flags: i32);
    fn trace_contention_end(lock: *mut arch_spinlock_t, ret: i32);
    fn arch_spin_lock(lock: *mut arch_spinlock_t);
    fn arch_spin_unlock(lock: *mut arch_spinlock_t);
    fn register_sysctl_init(name: *const i8, table: *const ctl_table);
    fn proc_dointvec();
}

#[repr(C)]
pub struct Lowcore { pub spinlock_index: i32 }
#[repr(C)]
pub struct arch_spinlock_t { pub lock: i32 }
#[repr(C)]
pub struct arch_rwlock_t { pub cnts: i32, pub wait: arch_spinlock_t }
#[repr(C)]
pub struct ctl_table {
    pub procname: *const i8,
    pub data: *mut i32,
    pub maxlen: usize,
    pub mode: u32,
    pub proc_handler: unsafe extern "C" fn(),
}

const CIF_DEDICATED_CPU: i32 = 0;
const LCB_F_SPIN: i32 = 1;

unsafe fn spin_retry_init() -> i32 {
    if spin_retry < 0 { spin_retry = 1000; }
    0
}

unsafe fn spin_retry_setup(str_: *mut i8) -> i32 {
    spin_retry = simple_strtoul(str_, &mut str_, 0) as i32;
    1
}

static mut S390_SPIN_SYSCTL_TABLE: [ctl_table; 2] = [
    ctl_table { procname: b"spin_retry\0".as_ptr() as *const i8, data: core::ptr::addr_of_mut!(spin_retry), maxlen: core::mem::size_of::<i32>(), mode: 0o644, proc_handler: proc_dointvec },
    ctl_table { procname: ptr::null(), data: ptr::null_mut(), maxlen: 0, mode: 0, proc_handler: proc_dointvec },
];

unsafe fn init_s390_spin_sysctls() -> i32 {
    register_sysctl_init(b"kernel\0".as_ptr() as *const i8, S390_SPIN_SYSCTL_TABLE.as_ptr());
    0
}

#[repr(C, align(32))]
pub struct spin_wait {
    pub next: *mut spin_wait,
    pub prev: *mut spin_wait,
    pub node_id: i32,
}

static mut SPIN_WAIT: [spin_wait; 4] = [spin_wait { next: ptr::null_mut(), prev: ptr::null_mut(), node_id: 0 }; 4];

const _Q_LOCK_CPU_OFFSET: i32 = 0;
const _Q_LOCK_STEAL_OFFSET: i32 = 16;
const _Q_TAIL_IDX_OFFSET: i32 = 18;
const _Q_TAIL_CPU_OFFSET: i32 = 20;
const _Q_LOCK_CPU_MASK: i32 = 0x0000ffff;
const _Q_LOCK_STEAL_ADD: i32 = 0x00010000;
const _Q_LOCK_STEAL_MASK: i32 = 0x00030000;
const _Q_TAIL_IDX_MASK: i32 = 0x000c0000u32 as i32;
const _Q_TAIL_CPU_MASK: i32 = 0xfff00000u32 as i32;
const _Q_LOCK_MASK: i32 = _Q_LOCK_CPU_MASK | _Q_LOCK_STEAL_MASK;
const _Q_TAIL_MASK: i32 = _Q_TAIL_IDX_MASK | _Q_TAIL_CPU_MASK;

pub unsafe fn arch_spin_lock_setup(cpu: i32) {
    let node = per_cpu_ptr(SPIN_WAIT.as_mut_ptr(), cpu);
    for ix in 0..4 {
        memset(node.add(ix), 0, core::mem::size_of::<spin_wait>());
        (*node.add(ix)).node_id = ((cpu + 1) << _Q_TAIL_CPU_OFFSET) + ((ix as i32) << _Q_TAIL_IDX_OFFSET);
    }
}

unsafe fn arch_load_niai4(lock: *mut i32) -> i32 { ptr::read_volatile(lock) }

unsafe fn arch_try_cmpxchg_niai8(lock: *mut i32, old: i32, new: i32) -> bool {
    match (*lock).cmpxchg(old, new) { Ok(_) => true, Err(_) => false }
}

unsafe fn arch_spin_decode_tail(lock: i32) -> *mut spin_wait {
    let ix = ((lock & _Q_TAIL_IDX_MASK) >> _Q_TAIL_IDX_OFFSET) as usize;
    let cpu = (lock & _Q_TAIL_CPU_MASK) >> _Q_TAIL_CPU_OFFSET;
    per_cpu_ptr(SPIN_WAIT.as_mut_ptr().add(ix), cpu - 1)
}

unsafe fn arch_spin_yield_target(lock: i32, mut node: *mut spin_wait) -> i32 {
    if lock & _Q_LOCK_CPU_MASK != 0 { return lock & _Q_LOCK_CPU_MASK; }
    if node.is_null() || (*node).prev.is_null() { return 0; }
    while !(*node).prev.is_null() { node = (*node).prev; }
    (*node).node_id >> _Q_TAIL_CPU_OFFSET
}

unsafe fn arch_spin_lock_queued(lp: *mut arch_spinlock_t) {
    let ix = (*get_lowcore()).spinlock_index as usize; (*get_lowcore()).spinlock_index += 1; barrier();
    let lockval = spinlock_lockval(); let node = this_cpu_ptr(SPIN_WAIT.as_mut_ptr().add(ix));
    (*node).prev = ptr::null_mut(); (*node).next = ptr::null_mut(); let node_id = (*node).node_id;
    let mut old = ptr::read_volatile(&(*lp).lock);
    loop {
        let new;
        if old & _Q_LOCK_CPU_MASK == 0 && old & _Q_LOCK_STEAL_MASK != _Q_LOCK_STEAL_MASK {
            new = (if old != 0 { old.wrapping_add(_Q_LOCK_STEAL_ADD) } else { 0 }) | lockval;
            if arch_try_cmpxchg_niai8(&mut (*lp).lock, old, new) { break; } continue;
        }
        new = node_id | (old & _Q_LOCK_MASK);
        if arch_try_cmpxchg_niai8(&mut (*lp).lock, old, new) { break; }
        old = ptr::read_volatile(&(*lp).lock);
    }
    let tail_id = old & _Q_TAIL_MASK;
    if tail_id != 0 { (*node).prev = arch_spin_decode_tail(tail_id); (*(*node).prev).next = node; }
    let owner = arch_spin_yield_target(old, node);
    if owner != 0 && arch_vcpu_is_preempted(owner - 1) { smp_yield_cpu(owner - 1); }
    if tail_id != 0 { let mut count = spin_retry; while !(*node).prev.is_null() { if count >= 0 { count -= 1; continue; } count = spin_retry; } }
    let mut count = spin_retry;
    loop {
        old = ptr::read_volatile(&(*lp).lock); let owner = old & _Q_LOCK_CPU_MASK;
        if owner == 0 { let tail_id = old & _Q_TAIL_MASK; let new = (if tail_id != node_id { tail_id } else { 0 }) | lockval; if arch_try_cmpxchg_niai8(&mut (*lp).lock, old, new) { break; } continue; }
        if count >= 0 { count -= 1; continue; } count = spin_retry;
        if !machine_is_lpar() || arch_vcpu_is_preempted(owner - 1) { smp_yield_cpu(owner - 1); }
    }
    if node_id != 0 && (old & _Q_TAIL_MASK) != node_id { let next; loop { next = (*node).next; if !next.is_null() { break; } } (*next).prev = ptr::null_mut(); }
    (*get_lowcore()).spinlock_index -= 1;
}

unsafe fn arch_spin_lock_classic(lp: *mut arch_spinlock_t) {
    let lockval = spinlock_lockval(); let owner = arch_spin_yield_target(ptr::read_volatile(&(*lp).lock), ptr::null_mut());
    if owner != 0 && arch_vcpu_is_preempted(owner - 1) { smp_yield_cpu(owner - 1); }
    let mut count = spin_retry;
    loop { let old = arch_load_niai4(&mut (*lp).lock); let owner = old & _Q_LOCK_CPU_MASK; if owner == 0 { let new = (old & _Q_TAIL_MASK) | lockval; if arch_try_cmpxchg_niai8(&mut (*lp).lock, old, new) { return; } continue; } if count >= 0 { count -= 1; continue; } count = spin_retry; if !machine_is_lpar() || arch_vcpu_is_preempted(owner - 1) { smp_yield_cpu(owner - 1); } }
}

pub unsafe fn arch_spin_lock_wait(lp: *mut arch_spinlock_t) { trace_contention_begin(lp, LCB_F_SPIN); if test_cpu_flag(CIF_DEDICATED_CPU) { arch_spin_lock_queued(lp); } else { arch_spin_lock_classic(lp); } trace_contention_end(lp, 0); }

pub unsafe fn arch_spin_trylock_retry(lp: *mut arch_spinlock_t) -> i32 { let cpu = spinlock_lockval(); let mut count = spin_retry; while count > 0 { let owner = ptr::read_volatile(&(*lp).lock); if owner == 0 && arch_try_cmpxchg_niai8(&mut (*lp).lock, owner, cpu) { return 1; } count -= 1; } 0 }

pub unsafe fn arch_read_lock_wait(rw: *mut arch_rwlock_t) { if in_interrupt() { while ptr::read_volatile(&(*rw).cnts) & 0x10000 != 0 { barrier(); } return; } (*rw).cnts -= 1; arch_spin_lock(&mut (*rw).wait); (*rw).cnts += 1; while ptr::read_volatile(&(*rw).cnts) & 0x10000 != 0 { barrier(); } arch_spin_unlock(&mut (*rw).wait); }

pub unsafe fn arch_write_lock_wait(rw: *mut arch_rwlock_t) { (*rw).cnts += 0x20000; arch_spin_lock(&mut (*rw).wait); loop { let old = ptr::read_volatile(&(*rw).cnts); if old & 0x1ffff == 0 && arch_try_cmpxchg_niai8(&mut (*rw).cnts, old, old | 0x10000) { break; } barrier(); } arch_spin_unlock(&mut (*rw).wait); }

pub unsafe fn arch_spin_relax(lp: *mut arch_spinlock_t) { let cpu = ptr::read_volatile(&(*lp).lock) & _Q_LOCK_CPU_MASK; if cpu == 0 || (machine_is_lpar() && !arch_vcpu_is_preempted(cpu - 1)) { return; } smp_yield_cpu(cpu - 1); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
