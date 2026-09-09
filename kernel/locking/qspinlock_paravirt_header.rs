/* SPDX-License-Identifier: GPL-2.0 */
// Original header guard and required includes are intentionally omitted.
// This file depends on the corresponding kernel declarations.

pub const _Q_SLOW_VAL: u32 = 3u32 << _Q_LOCKED_OFFSET;
pub const PV_PREV_CHECK_MASK: i32 = 0xff;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vcpu_state {
    VCPU_RUNNING = 0,
    VCPU_HALTED,
    VCPU_HASHED,
}

#[repr(C)]
pub struct pv_node {
    pub mcs: mcs_spinlock,
    pub cpu: i32,
    pub state: u8,
}

#[repr(C)]
pub struct pv_hash_entry {
    pub lock: *mut qspinlock,
    pub node: *mut pv_node,
}

pub const PV_HE_PER_LINE: usize = SMP_CACHE_BYTES / core::mem::size_of::<pv_hash_entry>();
pub const PV_HE_MIN: usize = PAGE_SIZE / core::mem::size_of::<pv_hash_entry>();

static mut pv_lock_hash: *mut pv_hash_entry = core::ptr::null_mut();
static mut pv_lock_hash_bits: u32 = 0;

#[macro_export]
macro_rules! queued_spin_trylock {
    ($l:expr) => { pv_hybrid_queued_unfair_trylock($l) };
}

pub unsafe fn pv_hybrid_queued_unfair_trylock(lock: *mut qspinlock) -> bool {
    loop {
        let val = atomic_read(&(*lock).val);
        let mut old: u8 = 0;
        if (val & !_Q_LOCKED_PENDING_MASK) == 0
            && try_cmpxchg_acquire(&mut (*lock).locked, &mut old, _Q_LOCKED_VAL)
        {
            lockevent_inc(pv_lock_stealing);
            return true;
        }
        if (val & _Q_TAIL_MASK) == 0 || (val & _Q_PENDING_MASK) != 0 {
            break;
        }
        cpu_relax();
    }
    false
}

// When _Q_PENDING_BITS == 8, use the byte pending field implementation.
#[cfg(_Q_PENDING_BITS_8)]
pub unsafe fn set_pending(lock: *mut qspinlock) {
    WRITE_ONCE((*lock).pending, 1);
}

#[cfg(_Q_PENDING_BITS_8)]
pub unsafe fn trylock_clear_pending(lock: *mut qspinlock) -> bool {
    let mut old: u16 = _Q_PENDING_VAL;
    !READ_ONCE((*lock).locked) && try_cmpxchg_acquire(&mut (*lock).locked_pending, &mut old, _Q_LOCKED_VAL)
}

// Else branch: _Q_PENDING_BITS != 8.
#[cfg(not(_Q_PENDING_BITS_8))]
pub unsafe fn set_pending(lock: *mut qspinlock) {
    atomic_or(_Q_PENDING_VAL, &mut (*lock).val);
}

#[cfg(not(_Q_PENDING_BITS_8))]
pub unsafe fn trylock_clear_pending(lock: *mut qspinlock) -> bool {
    let mut old = atomic_read(&(*lock).val);
    loop {
        if (old & _Q_LOCKED_MASK) != 0 {
            return false;
        }
        let new = (old & !_Q_PENDING_MASK) | _Q_LOCKED_VAL;
        if !atomic_try_cmpxchg_acquire(&mut (*lock).val, &mut old, new) {
            return true;
        }
    }
}

pub unsafe fn __pv_init_lock_hash() {
    let mut pv_hash_size = ALIGN(4 * num_possible_cpus(), PV_HE_PER_LINE);
    if pv_hash_size < PV_HE_MIN {
        pv_hash_size = PV_HE_MIN;
    }
    pv_lock_hash = alloc_large_system_hash(
        "PV qspinlock".as_ptr() as *const i8,
        core::mem::size_of::<pv_hash_entry>(), pv_hash_size, 0,
        HASH_EARLY | HASH_ZERO, &mut pv_lock_hash_bits, core::ptr::null_mut(),
        pv_hash_size, pv_hash_size,
    ) as *mut pv_hash_entry;
}

pub unsafe fn pv_hash(lock: *mut qspinlock, node: *mut pv_node) -> *mut *mut qspinlock {
    let hash = hash_ptr(lock, pv_lock_hash_bits);
    let start = hash & !(PV_HE_PER_LINE - 1);
    let mask = (1usize << pv_lock_hash_bits) - 1;
    for offset in 0..(1usize << pv_lock_hash_bits) {
        let he = pv_lock_hash.add((start + offset) & mask);
        let mut old: *mut qspinlock = core::ptr::null_mut();
        if try_cmpxchg(&mut (*he).lock, &mut old, lock) {
            WRITE_ONCE((*he).node, node);
            lockevent_pv_hop((offset + 1) as i32);
            return &mut (*he).lock;
        }
    }
    BUG();
}

pub unsafe fn pv_unhash(lock: *mut qspinlock) -> *mut pv_node {
    let hash = hash_ptr(lock, pv_lock_hash_bits);
    let start = hash & !(PV_HE_PER_LINE - 1);
    let mask = (1usize << pv_lock_hash_bits) - 1;
    for offset in 0..(1usize << pv_lock_hash_bits) {
        let he = pv_lock_hash.add((start + offset) & mask);
        if READ_ONCE((*he).lock) == lock {
            let node = READ_ONCE((*he).node);
            WRITE_ONCE((*he).lock, core::ptr::null_mut());
            return node;
        }
    }
    BUG();
}

pub unsafe fn pv_wait_early(prev: *mut pv_node, loop_: i32) -> bool {
    if (loop_ & PV_PREV_CHECK_MASK) != 0 { return false; }
    READ_ONCE((*prev).state) != VCPU_RUNNING as u8
}

pub unsafe fn pv_init_node(node: *mut mcs_spinlock) {
    BUILD_BUG_ON(core::mem::size_of::<pv_node>() > core::mem::size_of::<qnode>());
    let pn = node as *mut pv_node;
    (*pn).cpu = smp_processor_id();
    (*pn).state = VCPU_RUNNING as u8;
}

pub unsafe fn pv_wait_node(node: *mut mcs_spinlock, prev: *mut mcs_spinlock) {
    let pn = node as *mut pv_node;
    let pp = prev as *mut pv_node;
    loop {
        let mut wait_early = false;
        let mut loop_ = SPIN_THRESHOLD;
        while loop_ != 0 {
            if READ_ONCE((*node).locked) { return; }
            if pv_wait_early(pp, loop_) { wait_early = true; break; }
            loop_ -= 1;
            cpu_relax();
        }
        smp_store_mb(&mut (*pn).state, VCPU_HALTED as u8);
        if !READ_ONCE((*node).locked) {
            lockevent_inc(pv_wait_node);
            lockevent_cond_inc(pv_wait_early, wait_early);
            pv_wait(&mut (*pn).state, VCPU_HALTED as u8);
        }
        cmpxchg(&mut (*pn).state, VCPU_HALTED as u8, VCPU_RUNNING as u8);
        lockevent_cond_inc(pv_spurious_wakeup, !READ_ONCE((*node).locked));
    }
}

pub unsafe fn pv_kick_node(lock: *mut qspinlock, node: *mut mcs_spinlock) {
    let pn = node as *mut pv_node;
    let mut old = VCPU_HALTED as u8;
    smp_mb__before_atomic();
    if !try_cmpxchg_relaxed(&mut (*pn).state, &mut old, VCPU_HASHED as u8) { return; }
    WRITE_ONCE((*lock).locked, _Q_SLOW_VAL as u8);
    let _ = pv_hash(lock, pn);
}

pub unsafe fn pv_wait_head_or_lock(lock: *mut qspinlock, node: *mut mcs_spinlock) -> u32 {
    let pn = node as *mut pv_node;
    let mut lp: *mut *mut qspinlock = core::ptr::null_mut();
    let mut waitcnt = 0;
    if READ_ONCE((*pn).state) == VCPU_HASHED as u8 { lp = 1 as *mut *mut qspinlock; }
    lockevent_inc(lock_slowpath);
    loop {
        WRITE_ONCE((*pn).state, VCPU_RUNNING as u8);
        set_pending(lock);
        let mut loop_ = SPIN_THRESHOLD;
        while loop_ != 0 {
            if trylock_clear_pending(lock) { break 'gotlock; }
            loop_ -= 1;
            cpu_relax();
        }
        clear_pending(lock);
        if lp.is_null() {
            lp = pv_hash(lock, pn);
            if xchg(&mut (*lock).locked, _Q_SLOW_VAL as u8) == 0 {
                WRITE_ONCE((*lock).locked, _Q_LOCKED_VAL as u8);
                WRITE_ONCE(*lp, core::ptr::null_mut());
                break;
            }
        }
        WRITE_ONCE((*pn).state, VCPU_HASHED as u8);
        lockevent_inc(pv_wait_head);
        lockevent_cond_inc(pv_wait_again, waitcnt);
        pv_wait(&mut (*lock).locked, _Q_SLOW_VAL as u8);
        waitcnt += 1;
    }
    'gotlock: (atomic_read(&(*lock).val) as u32) | _Q_LOCKED_VAL
}

pub unsafe fn __pv_queued_spin_unlock_slowpath(lock: *mut qspinlock, locked: u8) {
    if locked != _Q_SLOW_VAL as u8 {
        WARN(!debug_locks_silent, "pvqspinlock: lock 0x%lx has corrupted value 0x%x!\n", lock as usize, atomic_read(&(*lock).val));
        return;
    }
    smp_rmb();
    let node = pv_unhash(lock);
    smp_store_release(&mut (*lock).locked, 0);
    lockevent_inc(pv_kick_unlock);
    pv_kick((*node).cpu);
}

#[cfg(not(__pv_queued_spin_unlock))]
pub unsafe fn __pv_queued_spin_unlock(lock: *mut qspinlock) {
    let mut locked = _Q_LOCKED_VAL as u8;
    if try_cmpxchg_release(&mut (*lock).locked, &mut locked, 0) { return; }
    __pv_queued_spin_unlock_slowpath(lock, locked);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
