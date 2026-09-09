/* SPDX-License-Identifier: GPL-2.0 */

/*
 * 'Generic' ticket-lock implementation.
 *
 * This is a low-level translation of the C header.  The atomic and memory
 * ordering primitives, as well as `arch_spinlock_t`, are supplied externally.
 */

#[inline(always)]
pub unsafe fn ticket_spin_lock(lock: *mut arch_spinlock_t) {
    let val: u32 = atomic_fetch_add(1u32 << 16, unsafe { &mut (*lock).val });
    let ticket: u16 = (val >> 16) as u16;

    if ticket == val as u16 {
        return;
    }

    /*
     * atomic_cond_read_acquire() is RCpc; the full fence upgrades the
     * otherwise-RCpc acquire path to the required ordering.
     */
    atomic_cond_read_acquire(unsafe { &(*lock).val }, ticket == (VAL as u16));
    smp_mb();
}

#[inline(always)]
pub unsafe fn ticket_spin_trylock(lock: *mut arch_spinlock_t) -> bool {
    let mut old: u32 = atomic_read(unsafe { &(*lock).val });

    if (old >> 16) != (old & 0xffff) {
        return false;
    }

    // SC, for RCsc
    atomic_try_cmpxchg(unsafe { &mut (*lock).val }, &mut old, old.wrapping_add(1u32 << 16))
}

#[inline(always)]
pub unsafe fn ticket_spin_unlock(lock: *mut arch_spinlock_t) {
    // CONFIG_CPU_BIG_ENDIAN is a build-time configuration supplied externally.
    let offset = if cfg!(target_endian = "big") { 1usize } else { 0usize };
    let ptr: *mut u16 = (lock as *mut u16).add(offset);
    let val: u32 = atomic_read(unsafe { &(*lock).val });

    smp_store_release(ptr, (val as u16).wrapping_add(1));
}

#[inline(always)]
pub unsafe fn ticket_spin_value_unlocked(lock: arch_spinlock_t) -> i32 {
    let val: u32 = lock.val.counter;

    ((val >> 16) == (val & 0xffff)) as i32
}

#[inline(always)]
pub unsafe fn ticket_spin_is_locked(lock: *mut arch_spinlock_t) -> i32 {
    let val: arch_spinlock_t = READ_ONCE(unsafe { &*lock });

    (!ticket_spin_value_unlocked(val)).into()
}

#[inline(always)]
pub unsafe fn ticket_spin_is_contended(lock: *mut arch_spinlock_t) -> i32 {
    let val: u32 = atomic_read(unsafe { &(*lock).val });
    let difference = ((val >> 16).wrapping_sub(val & 0xffff)) as i16;

    (difference > 1).into()
}

/* Remapping architecture-specific spinlock functions to ticket functions. */
#[macro_export]
macro_rules! arch_spin_is_locked { ($l:expr) => { $crate::ticket_spin_is_locked($l) }; }
#[macro_export]
macro_rules! arch_spin_is_contended { ($l:expr) => { $crate::ticket_spin_is_contended($l) }; }
#[macro_export]
macro_rules! arch_spin_value_unlocked { ($l:expr) => { $crate::ticket_spin_value_unlocked($l) }; }
#[macro_export]
macro_rules! arch_spin_lock { ($l:expr) => { $crate::ticket_spin_lock($l) }; }
#[macro_export]
macro_rules! arch_spin_trylock { ($l:expr) => { $crate::ticket_spin_trylock($l) }; }
#[macro_export]
macro_rules! arch_spin_unlock { ($l:expr) => { $crate::ticket_spin_unlock($l) }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
