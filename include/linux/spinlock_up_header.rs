//! UP-debug version of spinlocks.
//!
//! Portions Copyright 2005, Red Hat, Inc., Ingo Molnar.
//! Released under the General Public License (GPL).
//!
//! In the debug case, 1 means unlocked, 0 means locked. The values are
//! inverted to catch initialization bugs.
//!
//! No atomicity anywhere, we are on UP. However, compiler barriers are still
//! needed so the compiler does not move potentially faulting instructions
//! (notably user accesses) into the locked sequence, resulting in non-atomic
//! execution.

// C dependencies: asm/processor.h (cpu_relax) and asm/barrier.h (barrier).
// The containing spinlock header must provide `arch_spinlock_t`.

#[cfg(feature = "CONFIG_DEBUG_SPINLOCK")]
#[inline]
pub unsafe fn arch_spin_is_locked(lock: *const arch_spinlock_t) -> bool {
    (*lock).slock == 0
}

#[cfg(feature = "CONFIG_DEBUG_SPINLOCK")]
#[inline]
pub unsafe fn arch_spin_lock(lock: *mut arch_spinlock_t) {
    (*lock).slock = 0;
    barrier();
}

#[cfg(feature = "CONFIG_DEBUG_SPINLOCK")]
#[inline]
pub unsafe fn arch_spin_trylock(lock: *mut arch_spinlock_t) -> bool {
    let oldval: i8 = (*lock).slock;

    (*lock).slock = 0;
    barrier();

    oldval > 0
}

#[cfg(feature = "CONFIG_DEBUG_SPINLOCK")]
#[inline]
pub unsafe fn arch_spin_unlock(lock: *mut arch_spinlock_t) {
    barrier();
    (*lock).slock = 1;
}

#[cfg(not(feature = "CONFIG_DEBUG_SPINLOCK"))]
#[inline]
pub unsafe fn arch_spin_is_locked(lock: *const arch_spinlock_t) -> i32 {
    let _ = lock;
    0
}

#[cfg(not(feature = "CONFIG_DEBUG_SPINLOCK"))]
#[inline]
pub unsafe fn arch_spin_lock(lock: *mut arch_spinlock_t) {
    barrier();
    let _ = lock;
}

#[cfg(not(feature = "CONFIG_DEBUG_SPINLOCK"))]
#[inline]
pub unsafe fn arch_spin_unlock(lock: *mut arch_spinlock_t) {
    barrier();
    let _ = lock;
}

#[cfg(not(feature = "CONFIG_DEBUG_SPINLOCK"))]
#[inline]
pub unsafe fn arch_spin_trylock(lock: *mut arch_spinlock_t) -> i32 {
    barrier();
    let _ = lock;
    1
}

#[inline]
pub unsafe fn arch_spin_is_contended(lock: *const arch_spinlock_t) -> i32 {
    let _ = lock;
    0
}

// Read-write spinlocks. No debug version.
#[inline]
pub unsafe fn arch_read_lock(lock: *mut arch_spinlock_t) {
    barrier();
    let _ = lock;
}

#[inline]
pub unsafe fn arch_write_lock(lock: *mut arch_spinlock_t) {
    barrier();
    let _ = lock;
}

#[inline]
pub unsafe fn arch_read_trylock(lock: *mut arch_spinlock_t) -> i32 {
    barrier();
    let _ = lock;
    1
}

#[inline]
pub unsafe fn arch_write_trylock(lock: *mut arch_spinlock_t) -> i32 {
    barrier();
    let _ = lock;
    1
}

#[inline]
pub unsafe fn arch_read_unlock(lock: *mut arch_spinlock_t) {
    barrier();
    let _ = lock;
}

#[inline]
pub unsafe fn arch_write_unlock(lock: *mut arch_spinlock_t) {
    barrier();
    let _ = lock;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
