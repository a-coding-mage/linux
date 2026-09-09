/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding kernel/Alpha headers.

/*
 * Simple spin lock operations.  There are two variants, one clears IRQ's
 * on the local processor, one does not.
 *
 * We make no fairness assumptions. They have a cost.
 */

#[inline]
pub unsafe fn arch_spin_is_locked(x: *const arch_spinlock_t) -> bool {
    (*x).lock != 0
}

#[inline]
pub fn arch_spin_value_unlocked(lock: arch_spinlock_t) -> i32 {
    (lock.lock == 0) as i32
}

#[inline]
pub unsafe fn arch_spin_unlock(lock: *mut arch_spinlock_t) {
    mb();
    (*lock).lock = 0;
}

#[inline]
pub unsafe fn arch_spin_lock(lock: *mut arch_spinlock_t) {
    let mut tmp: libc::c_long;
    core::arch::asm!(
        "1: ldl_l {0}, {1}", "bne {0}, 2f", "lda {0}, 1", "stl_c {0}, {1}",
        "beq {0}, 2f", "mb", ".subsection 2", "2: ldl {0}, {1}",
        "bne {0}, 2b", "br 1b", ".previous",
        inout(reg) tmp, inout(reg) (*lock).lock, options(nostack, preserves_flags));
}

#[inline]
pub unsafe fn arch_spin_trylock(lock: *mut arch_spinlock_t) -> i32 {
    (!test_and_set_bit(0, &mut (*lock).lock)) as i32
}

#[inline]
pub unsafe fn arch_read_lock(lock: *mut arch_rwlock_t) {
    let mut regx: libc::c_long;
    core::arch::asm!(
        "1: ldl_l {1}, {0}", "blbs {1}, 6f", "subl {1}, 2, {1}",
        "stl_c {1}, {0}", "beq {1}, 6f", "mb", ".subsection 2",
        "6: ldl {1}, {0}", "blbs {1}, 6b", "br 1b", ".previous",
        inout(reg) (*lock).lock, inout(reg) regx, options(nostack, preserves_flags));
}

#[inline]
pub unsafe fn arch_write_lock(lock: *mut arch_rwlock_t) {
    let mut regx: libc::c_long;
    core::arch::asm!(
        "1: ldl_l {1}, {0}", "bne {1}, 6f", "lda {1}, 1", "stl_c {1}, {0}",
        "beq {1}, 6f", "mb", ".subsection 2", "6: ldl {1}, {0}",
        "bne {1}, 6b", "br 1b", ".previous",
        inout(reg) (*lock).lock, inout(reg) regx, options(nostack, preserves_flags));
}

#[inline]
pub unsafe fn arch_read_trylock(lock: *mut arch_rwlock_t) -> i32 {
    let mut regx: libc::c_long;
    let mut success: libc::c_int;
    core::arch::asm!(
        "1: ldl_l {1}, {0}", "lda {2}, 0", "blbs {1}, 2f", "subl {1}, 2, {2}",
        "stl_c {2}, {0}", "beq {2}, 6f", "2: mb", ".subsection 2",
        "6: br 1b", ".previous",
        inout(reg) (*lock).lock, inout(reg) regx, inout(reg) success,
        options(nostack, preserves_flags));
    success
}

#[inline]
pub unsafe fn arch_write_trylock(lock: *mut arch_rwlock_t) -> i32 {
    let mut regx: libc::c_long;
    let mut success: libc::c_int;
    core::arch::asm!(
        "1: ldl_l {1}, {0}", "lda {2}, 0", "bne {1}, 2f", "lda {2}, 1",
        "stl_c {2}, {0}", "beq {2}, 6f", "2: mb", ".subsection 2",
        "6: br 1b", ".previous",
        inout(reg) (*lock).lock, inout(reg) regx, inout(reg) success,
        options(nostack, preserves_flags));
    success
}

#[inline]
pub unsafe fn arch_read_unlock(lock: *mut arch_rwlock_t) {
    let mut regx: libc::c_long;
    core::arch::asm!(
        "mb", "1: ldl_l {1}, {0}", "addl {1}, 2, {1}", "stl_c {1}, {0}",
        "beq {1}, 6f", ".subsection 2", "6: br 1b", ".previous",
        inout(reg) (*lock).lock, inout(reg) regx, options(nostack, preserves_flags));
}

#[inline]
pub unsafe fn arch_write_unlock(lock: *mut arch_rwlock_t) {
    mb();
    (*lock).lock = 0;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
