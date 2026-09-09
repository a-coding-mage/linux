// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the surrounding kernel translation are intentionally external.

#[repr(C)]
pub struct optimistic_spin_node {
    pub next: *mut optimistic_spin_node,
    pub prev: *mut optimistic_spin_node,
    pub locked: i32, // 1 if lock acquired
    pub cpu: i32, // encoded CPU # + 1 value
}

#[repr(C)]
pub struct optimistic_spin_queue {
    pub tail: atomic_t,
}

#[repr(C)]
pub struct atomic_t {
    pub counter: i32,
}

extern "C" {
    static mut osq_node: optimistic_spin_node;
    fn smp_processor_id() -> i32;
    fn atomic_read(v: *const atomic_t) -> i32;
    fn atomic_cmpxchg_acquire(v: *mut atomic_t, old: i32, new: i32) -> i32;
    fn atomic_xchg(v: *mut atomic_t, new: i32) -> i32;
    fn atomic_try_cmpxchg_release(v: *mut atomic_t, old: *mut i32, new: i32) -> bool;
    fn cpu_relax();
    fn need_resched() -> bool;
    fn vcpu_is_preempted(cpu: i32) -> bool;
    fn smp_wmb();
    fn smp_load_acquire(v: *const i32) -> i32;
    fn cmpxchg<T>(ptr: *mut T, old: T, new: T) -> T;
    fn xchg<T>(ptr: *mut T, new: T) -> T;
    fn smp_cond_load_relaxed(v: *const i32, condition: bool) -> bool;
}

// We use the value 0 to represent "no CPU", thus the encoded value
// will be the CPU number incremented by 1.
#[inline]
unsafe fn encode_cpu(cpu_nr: i32) -> i32 {
    cpu_nr.wrapping_add(1)
}

#[inline]
unsafe fn node_cpu(node: *mut optimistic_spin_node) -> i32 {
    (*node).cpu.wrapping_sub(1)
}

#[inline]
unsafe fn decode_cpu(encoded_cpu_val: i32) -> *mut optimistic_spin_node {
    let cpu_nr = encoded_cpu_val.wrapping_sub(1);
    // Equivalent to per_cpu_ptr(&osq_node, cpu_nr).
    per_cpu_ptr(&raw mut osq_node, cpu_nr)
}

extern "C" {
    fn per_cpu_ptr(node: *mut optimistic_spin_node, cpu: i32) -> *mut optimistic_spin_node;
}

#[inline]
unsafe fn osq_wait_next(
    lock: *mut optimistic_spin_queue,
    node: *mut optimistic_spin_node,
    old_cpu: i32,
) -> *mut optimistic_spin_node {
    let curr = encode_cpu(smp_processor_id());

    loop {
        if atomic_read(&(*lock).tail) == curr
            && atomic_cmpxchg_acquire(&mut (*lock).tail, curr, old_cpu) == curr
        {
            return core::ptr::null_mut();
        }

        if !(*node).next.is_null() {
            let next = xchg(&mut (*node).next, core::ptr::null_mut());
            if !next.is_null() {
                return next;
            }
        }

        cpu_relax();
    }
}

pub unsafe fn osq_lock(lock: *mut optimistic_spin_queue) -> bool {
    let node = per_cpu_ptr(&raw mut osq_node, smp_processor_id());
    let mut prev: *mut optimistic_spin_node;
    let mut next: *mut optimistic_spin_node;
    let curr = encode_cpu(smp_processor_id());
    let old;

    (*node).locked = 0;
    (*node).next = core::ptr::null_mut();
    (*node).cpu = curr;

    old = atomic_xchg(&mut (*lock).tail, curr);
    if old == OSQ_UNLOCKED_VAL {
        return true;
    }

    prev = decode_cpu(old);
    (*node).prev = prev;
    smp_wmb();
    (*prev).next = node;

    if smp_cond_load_relaxed(&(*node).locked, (*node).locked != 0
        || need_resched() || vcpu_is_preempted(node_cpu((*node).prev)))
    {
        return true;
    }

    loop {
        if (*prev).next == node && cmpxchg(&mut (*prev).next, node, core::ptr::null_mut()) == node {
            break;
        }

        if smp_load_acquire(&(*node).locked) != 0 {
            return true;
        }

        cpu_relax();
        prev = (*node).prev;
    }

    next = osq_wait_next(lock, node, (*prev).cpu);
    if next.is_null() {
        return false;
    }

    (*next).prev = prev;
    (*prev).next = next;
    false
}

pub unsafe fn osq_unlock(lock: *mut optimistic_spin_queue) {
    let mut curr = encode_cpu(smp_processor_id());

    if atomic_try_cmpxchg_release(&mut (*lock).tail, &mut curr, OSQ_UNLOCKED_VAL) {
        return;
    }

    let node = per_cpu_ptr(&raw mut osq_node, smp_processor_id());
    let mut next = xchg(&mut (*node).next, core::ptr::null_mut());
    if !next.is_null() {
        (*next).locked = 1;
        return;
    }

    next = osq_wait_next(lock, node, OSQ_UNLOCKED_VAL);
    if !next.is_null() {
        (*next).locked = 1;
    }
}

// Supplied by the kernel lock implementation.
extern "C" {
    static OSQ_UNLOCKED_VAL: i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
