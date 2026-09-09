/*
 * Copyright 2005, Red Hat, Inc., Ingo Molnar
 * Released under the General Public License (GPL).
 *
 * This file contains the spinlock/rwlock implementations for
 * DEBUG_SPINLOCK.
 *
 * Linux kernel dependencies are supplied by other translation units.
 */

pub unsafe fn __raw_spin_lock_init(
    lock: *mut raw_spinlock_t,
    name: *const core::ffi::c_char,
    key: *mut lock_class_key,
    inner: i16,
) {
    // CONFIG_DEBUG_LOCK_ALLOC: preserve the lockdep initialization path.
    debug_check_no_locks_freed(lock.cast(), core::mem::size_of::<raw_spinlock_t>());
    lockdep_init_map_wait(&mut (*lock).dep_map, name, key, 0, inner);
    (*lock).raw_lock = __ARCH_SPIN_LOCK_UNLOCKED;
    (*lock).magic = SPINLOCK_MAGIC;
    (*lock).owner = SPINLOCK_OWNER_INIT;
    (*lock).owner_cpu = -1;
}

pub unsafe fn __rwlock_init(
    lock: *mut rwlock_t,
    name: *const core::ffi::c_char,
    key: *mut lock_class_key,
) {
    // !CONFIG_PREEMPT_RT; CONFIG_DEBUG_LOCK_ALLOC preserves the lockdep path.
    debug_check_no_locks_freed(lock.cast(), core::mem::size_of::<rwlock_t>());
    lockdep_init_map_wait(&mut (*lock).dep_map, name, key, 0, LD_WAIT_CONFIG);
    (*lock).raw_lock = __ARCH_RW_LOCK_UNLOCKED;
    (*lock).magic = RWLOCK_MAGIC;
    (*lock).owner = SPINLOCK_OWNER_INIT;
    (*lock).owner_cpu = -1;
}

unsafe fn spin_dump(lock: *mut raw_spinlock_t, msg: *const core::ffi::c_char) {
    let mut owner: *mut task_struct = core::ptr::read_volatile(&(*lock).owner);
    if owner == SPINLOCK_OWNER_INIT { owner = core::ptr::null_mut(); }
    printk(KERN_EMERG, c"BUG: spinlock %s on CPU#%d, %s/%d\n", msg,
        raw_smp_processor_id(), (*current).comm.as_ptr(), task_pid_nr(current));
    printk(KERN_EMERG, c" lock: %pS, .magic: %08x, .owner: %s/%d, .owner_cpu: %d\n",
        lock, core::ptr::read_volatile(&(*lock).magic),
        if !owner.is_null() { (*owner).comm.as_ptr() } else { c"<none>".as_ptr() },
        if !owner.is_null() { task_pid_nr(owner) } else { -1 },
        core::ptr::read_volatile(&(*lock).owner_cpu));
    dump_stack();
}

unsafe fn spin_bug(lock: *mut raw_spinlock_t, msg: *const core::ffi::c_char) {
    if !debug_locks_off() { return; }
    spin_dump(lock, msg);
}

macro_rules! SPIN_BUG_ON { ($cond:expr, $lock:expr, $msg:expr) => { if unlikely($cond) { spin_bug($lock, $msg); } }; }

unsafe fn debug_spin_lock_before(lock: *mut raw_spinlock_t) {
    SPIN_BUG_ON!(core::ptr::read_volatile(&(*lock).magic) != SPINLOCK_MAGIC, lock, c"bad magic".as_ptr());
    SPIN_BUG_ON!(core::ptr::read_volatile(&(*lock).owner) == current, lock, c"recursion".as_ptr());
    SPIN_BUG_ON!(core::ptr::read_volatile(&(*lock).owner_cpu) == raw_smp_processor_id(), lock, c"cpu recursion".as_ptr());
}

unsafe fn debug_spin_lock_after(lock: *mut raw_spinlock_t) {
    core::ptr::write_volatile(&mut (*lock).owner_cpu, raw_smp_processor_id());
    core::ptr::write_volatile(&mut (*lock).owner, current);
}

unsafe fn debug_spin_unlock(lock: *mut raw_spinlock_t) {
    SPIN_BUG_ON!((*lock).magic != SPINLOCK_MAGIC, lock, c"bad magic".as_ptr());
    SPIN_BUG_ON!(!raw_spin_is_locked(lock), lock, c"already unlocked".as_ptr());
    SPIN_BUG_ON!((*lock).owner != current, lock, c"wrong owner".as_ptr());
    SPIN_BUG_ON!((*lock).owner_cpu != raw_smp_processor_id(), lock, c"wrong CPU".as_ptr());
    core::ptr::write_volatile(&mut (*lock).owner, SPINLOCK_OWNER_INIT);
    core::ptr::write_volatile(&mut (*lock).owner_cpu, -1);
}

pub unsafe fn do_raw_spin_lock(lock: *mut raw_spinlock_t) {
    debug_spin_lock_before(lock);
    arch_spin_lock(&mut (*lock).raw_lock);
    mmiowb_spin_lock();
    debug_spin_lock_after(lock);
}

pub unsafe fn do_raw_spin_trylock(lock: *mut raw_spinlock_t) -> i32 {
    let ret = arch_spin_trylock(&mut (*lock).raw_lock);
    if ret != 0 { mmiowb_spin_lock(); debug_spin_lock_after(lock); }
    // !CONFIG_SMP: a trylock failure must not happen on UP.
    SPIN_BUG_ON!(ret == 0, lock, c"trylock failure on UP".as_ptr());
    ret
}

pub unsafe fn do_raw_spin_unlock(lock: *mut raw_spinlock_t) {
    mmiowb_spin_unlock();
    debug_spin_unlock(lock);
    arch_spin_unlock(&mut (*lock).raw_lock);
}

unsafe fn rwlock_bug(lock: *mut rwlock_t, msg: *const core::ffi::c_char) {
    if !debug_locks_off() { return; }
    printk(KERN_EMERG, c"BUG: rwlock %s on CPU#%d, %s/%d, %p\n", msg,
        raw_smp_processor_id(), (*current).comm.as_ptr(), task_pid_nr(current), lock);
    dump_stack();
}

macro_rules! RWLOCK_BUG_ON { ($cond:expr, $lock:expr, $msg:expr) => { if unlikely($cond) { rwlock_bug($lock, $msg); } }; }

pub unsafe fn do_raw_read_lock(lock: *mut rwlock_t) { RWLOCK_BUG_ON!((*lock).magic != RWLOCK_MAGIC, lock, c"bad magic".as_ptr()); arch_read_lock(&mut (*lock).raw_lock); }
pub unsafe fn do_raw_read_trylock(lock: *mut rwlock_t) -> i32 { let ret = arch_read_trylock(&mut (*lock).raw_lock); RWLOCK_BUG_ON!(ret == 0, lock, c"trylock failure on UP".as_ptr()); ret }
pub unsafe fn do_raw_read_unlock(lock: *mut rwlock_t) { RWLOCK_BUG_ON!((*lock).magic != RWLOCK_MAGIC, lock, c"bad magic".as_ptr()); arch_read_unlock(&mut (*lock).raw_lock); }

unsafe fn debug_write_lock_before(lock: *mut rwlock_t) { RWLOCK_BUG_ON!((*lock).magic != RWLOCK_MAGIC, lock, c"bad magic".as_ptr()); RWLOCK_BUG_ON!(core::ptr::read_volatile(&(*lock).owner) == current, lock, c"recursion".as_ptr()); RWLOCK_BUG_ON!(core::ptr::read_volatile(&(*lock).owner_cpu) == raw_smp_processor_id(), lock, c"cpu recursion".as_ptr()); }
unsafe fn debug_write_lock_after(lock: *mut rwlock_t) { core::ptr::write_volatile(&mut (*lock).owner_cpu, raw_smp_processor_id()); core::ptr::write_volatile(&mut (*lock).owner, current); }
unsafe fn debug_write_unlock(lock: *mut rwlock_t) { RWLOCK_BUG_ON!((*lock).magic != RWLOCK_MAGIC, lock, c"bad magic".as_ptr()); RWLOCK_BUG_ON!((*lock).owner != current, lock, c"wrong owner".as_ptr()); RWLOCK_BUG_ON!((*lock).owner_cpu != raw_smp_processor_id(), lock, c"wrong CPU".as_ptr()); core::ptr::write_volatile(&mut (*lock).owner, SPINLOCK_OWNER_INIT); core::ptr::write_volatile(&mut (*lock).owner_cpu, -1); }
pub unsafe fn do_raw_write_lock(lock: *mut rwlock_t) { debug_write_lock_before(lock); arch_write_lock(&mut (*lock).raw_lock); debug_write_lock_after(lock); }
pub unsafe fn do_raw_write_trylock(lock: *mut rwlock_t) -> i32 { let ret = arch_write_trylock(&mut (*lock).raw_lock); if ret != 0 { debug_write_lock_after(lock); } RWLOCK_BUG_ON!(ret == 0, lock, c"trylock failure on UP".as_ptr()); ret }
pub unsafe fn do_raw_write_unlock(lock: *mut rwlock_t) { debug_write_unlock(lock); arch_write_unlock(&mut (*lock).raw_lock); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
