/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Queued spinlock defines
 *
 * This file contains macro definitions and functions shared between different
 * qspinlock slow path implementations.
 */

// Dependencies supplied by the surrounding kernel translation unit:
// asm-generic/percpu.h, linux/percpu-defs.h, asm-generic/qspinlock.h,
// and asm-generic/mcs_spinlock.h.

pub const _Q_MAX_NODES: usize = 4;

/* The build may override this value. */
pub const _Q_PENDING_LOOPS: usize = 1;

#[repr(C)]
pub struct qnode {
    pub mcs: mcs_spinlock,
    #[cfg(CONFIG_PARAVIRT_SPINLOCKS)]
    pub reserved: [c_long; 2],
}

/*
 * We must be able to distinguish between no-tail and the tail at 0:0,
 * therefore increment the cpu number by one.
 */
#[inline]
pub unsafe fn encode_tail(cpu: c_int, idx: c_int) -> u32 {
    let mut tail: u32;

    tail = ((cpu + 1) as u32) << _Q_TAIL_CPU_OFFSET;
    tail |= (idx as u32) << _Q_TAIL_IDX_OFFSET; /* assume < 4 */

    tail
}

#[inline]
pub unsafe fn decode_tail(
    tail: u32,
    qnodes: *mut qnode,
) -> *mut mcs_spinlock {
    let cpu = ((tail >> _Q_TAIL_CPU_OFFSET) as c_int) - 1;
    let idx = ((tail & _Q_TAIL_IDX_MASK) >> _Q_TAIL_IDX_OFFSET) as usize;

    per_cpu_ptr(unsafe { &mut (*qnodes.add(idx)).mcs }, cpu)
}

#[inline]
pub unsafe fn grab_mcs_node(base: *mut mcs_spinlock, idx: c_int) -> *mut mcs_spinlock {
    &mut (*(base as *mut qnode).add(idx as usize)).mcs
}

pub const _Q_LOCKED_PENDING_MASK: u32 = _Q_LOCKED_MASK | _Q_PENDING_MASK;

#[cfg(_Q_PENDING_BITS == 8)]
#[inline(always)]
pub unsafe fn clear_pending(lock: *mut qspinlock) {
    WRITE_ONCE(&mut (*lock).pending, 0);
}

#[cfg(_Q_PENDING_BITS == 8)]
#[inline(always)]
pub unsafe fn clear_pending_set_locked(lock: *mut qspinlock) {
    WRITE_ONCE(&mut (*lock).locked_pending, _Q_LOCKED_VAL);
}

#[cfg(_Q_PENDING_BITS == 8)]
#[inline(always)]
pub unsafe fn xchg_tail(lock: *mut qspinlock, tail: u32) -> u32 {
    (xchg_relaxed(&mut (*lock).tail, tail >> _Q_TAIL_OFFSET) as u32)
        << _Q_TAIL_OFFSET
}

#[cfg(not(_Q_PENDING_BITS == 8))]
#[inline(always)]
pub unsafe fn clear_pending(lock: *mut qspinlock) {
    atomic_andnot(_Q_PENDING_VAL, &mut (*lock).val);
}

#[cfg(not(_Q_PENDING_BITS == 8))]
#[inline(always)]
pub unsafe fn clear_pending_set_locked(lock: *mut qspinlock) {
    atomic_add(-(_Q_PENDING_VAL as i32) + (_Q_LOCKED_VAL as i32), &mut (*lock).val);
}

#[cfg(not(_Q_PENDING_BITS == 8))]
#[inline(always)]
pub unsafe fn xchg_tail(lock: *mut qspinlock, tail: u32) -> u32 {
    let mut old: u32 = atomic_read(&(*lock).val);
    let new: u32;

    loop {
        new = (old & _Q_LOCKED_PENDING_MASK) | tail;
        if atomic_try_cmpxchg_relaxed(&mut (*lock).val, &mut old, new) {
            break;
        }
    }

    old
}

#[inline(always)]
pub unsafe fn queued_fetch_set_pending_acquire(lock: *mut qspinlock) -> u32 {
    atomic_fetch_or_acquire(_Q_PENDING_VAL, &mut (*lock).val)
}

#[inline(always)]
pub unsafe fn set_locked(lock: *mut qspinlock) {
    WRITE_ONCE(&mut (*lock).locked, _Q_LOCKED_VAL);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
