/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding architecture headers:
// asm/barrier.h, asm/ldcw.h, asm/processor.h, asm/spinlock_types.h

pub unsafe fn arch_spin_val_check(lock_val: i32) {
    // CONFIG_LIGHTWEIGHT_SPINLOCK_CHECK is a build-time configuration.
    // When enabled, the original PA-RISC inline assembly is:
    //   andcm,= lock_val, __ARCH_SPIN_LOCK_UNLOCKED_VAL, %r0
    //   .word SPINLOCK_BREAK_INSN
    // This architecture-specific instruction sequence has no portable Rust
    // equivalent and is preserved here as the required external dependency.
    if IS_ENABLED(CONFIG_LIGHTWEIGHT_SPINLOCK_CHECK) {
        let _ = lock_val;
        // TODO: emit the PA-RISC `andcm` / `SPINLOCK_BREAK_INSN` sequence.
    }
}

pub unsafe fn arch_spin_is_locked(x: *mut arch_spinlock_t) -> i32 {
    let a: *mut core::ffi::c_uint = __ldcw_align(x);
    let lock_val: i32 = core::ptr::read_volatile(a) as i32;
    arch_spin_val_check(lock_val);
    (lock_val == 0) as i32
}

pub unsafe fn arch_spin_lock(x: *mut arch_spinlock_t) {
    let a: *mut core::ffi::c_uint = __ldcw_align(x);
    loop {
        let lock_val_old: i32 = __ldcw(a) as i32;
        arch_spin_val_check(lock_val_old);
        if lock_val_old != 0 {
            return; // got lock
        }

        // wait until we should try to get lock again
        while core::ptr::read_volatile(a) == 0 {
            continue;
        }
    }
}

pub unsafe fn arch_spin_unlock(x: *mut arch_spinlock_t) {
    let a: *mut core::ffi::c_uint = __ldcw_align(x);
    // Release with ordered store. The original PA-RISC instruction is:
    //   stw,ma __ARCH_SPIN_LOCK_UNLOCKED_VAL,0(a)
    // TODO: preserve the architecture-specific ordered-store instruction.
    core::ptr::write_volatile(a, __ARCH_SPIN_LOCK_UNLOCKED_VAL as core::ffi::c_uint);
}

pub unsafe fn arch_spin_trylock(x: *mut arch_spinlock_t) -> i32 {
    let a: *mut core::ffi::c_uint = __ldcw_align(x);
    let lock_val: i32 = __ldcw(a) as i32;
    arch_spin_val_check(lock_val);
    (lock_val != 0) as i32
}

/*
 * Read-write spinlocks, allowing multiple readers but only one writer.
 * Unfair locking as Writers could be starved indefinitely by Reader(s)
 *
 * The spinlock itself is contained in @counter and access to it is
 * serialized with @lock_mutex.
 */

/* 1 - lock taken successfully */
pub unsafe fn arch_read_trylock(rw: *mut arch_rwlock_t) -> i32 {
    let mut ret = 0;
    let mut flags: core::ffi::c_ulong = 0;

    local_irq_save(&mut flags);
    arch_spin_lock(core::ptr::addr_of_mut!((*rw).lock_mutex));

    /* zero means writer holds the lock exclusively, deny Reader.
     * Otherwise grant lock to first/subseq reader */
    if (*rw).counter > 0 {
        (*rw).counter -= 1;
        ret = 1;
    }

    arch_spin_unlock(core::ptr::addr_of_mut!((*rw).lock_mutex));
    local_irq_restore(flags);
    ret
}

/* 1 - lock taken successfully */
pub unsafe fn arch_write_trylock(rw: *mut arch_rwlock_t) -> i32 {
    let mut ret = 0;
    let mut flags: core::ffi::c_ulong = 0;

    local_irq_save(&mut flags);
    arch_spin_lock(core::ptr::addr_of_mut!((*rw).lock_mutex));

    /* If reader(s) hold lock (lock < __ARCH_RW_LOCK_UNLOCKED__),
     * deny writer. Otherwise if unlocked grant to writer
     * Hence the claim that Linux rwlocks are unfair to writers.
     * (can be starved for an indefinite time by readers). */
    if (*rw).counter == __ARCH_RW_LOCK_UNLOCKED__ {
        (*rw).counter = 0;
        ret = 1;
    }
    arch_spin_unlock(core::ptr::addr_of_mut!((*rw).lock_mutex));
    local_irq_restore(flags);
    ret
}

pub unsafe fn arch_read_lock(rw: *mut arch_rwlock_t) {
    while arch_read_trylock(rw) == 0 {
        cpu_relax();
    }
}

pub unsafe fn arch_write_lock(rw: *mut arch_rwlock_t) {
    while arch_write_trylock(rw) == 0 {
        cpu_relax();
    }
}

pub unsafe fn arch_read_unlock(rw: *mut arch_rwlock_t) {
    let mut flags: core::ffi::c_ulong = 0;
    local_irq_save(&mut flags);
    arch_spin_lock(core::ptr::addr_of_mut!((*rw).lock_mutex));
    (*rw).counter += 1;
    arch_spin_unlock(core::ptr::addr_of_mut!((*rw).lock_mutex));
    local_irq_restore(flags);
}

pub unsafe fn arch_write_unlock(rw: *mut arch_rwlock_t) {
    let mut flags: core::ffi::c_ulong = 0;
    local_irq_save(&mut flags);
    arch_spin_lock(core::ptr::addr_of_mut!((*rw).lock_mutex));
    (*rw).counter = __ARCH_RW_LOCK_UNLOCKED__;
    arch_spin_unlock(core::ptr::addr_of_mut!((*rw).lock_mutex));
    local_irq_restore(flags);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
