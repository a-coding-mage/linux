/* SPDX-License-Identifier: GPL-2.0 */
/* spinlock.h: 32-bit Sparc spinlock support.
 *
 * Copyright (C) 1997 David S. Miller (davem@caip.rutgers.edu)
 */

/* C dependencies: asm/psr.h, asm/barrier.h, and asm/processor.h. */

#[inline]
pub unsafe fn arch_spin_is_locked(lock: *const core::ffi::c_void) -> bool {
    core::ptr::read_volatile(lock.cast::<u8>()) != 0
}

#[inline]
pub unsafe fn arch_spin_lock(lock: *mut arch_spinlock_t) {
    core::arch::asm!(
        "\n1:\n\t",
        "ldstub [{0}], %g2\n\t",
        "orcc %g2, 0x0, %g0\n\t",
        "bne,a 2f\n\t",
        " ldub [{0}], %g2\n\t",
        ".subsection 2\n",
        "2:\n\t",
        "orcc %g2, 0x0, %g0\n\t",
        "bne,a 2b\n\t",
        " ldub [{0}], %g2\n\t",
        "b,a 1b\n\t",
        ".previous\n",
        in(reg) lock,
        lateout("g2") _,
        options(nostack)
    );
}

#[inline]
pub unsafe fn arch_spin_trylock(lock: *mut arch_spinlock_t) -> bool {
    let mut result: u32;
    core::arch::asm!("ldstub [{1}], {0}", out(reg) result, in(reg) lock, options(nostack));
    result == 0
}

#[inline]
pub unsafe fn arch_spin_unlock(lock: *mut arch_spinlock_t) {
    core::arch::asm!("stb %g0, [{0}]", in(reg) lock, options(nostack));
}

/* Read-write spinlocks, allowing multiple readers but only one writer.
 *
 * The layout is: 24-bit counter in the high bits and wlock in the low byte.
 * wlock signifies that one writer is in or somebody is updating the counter.
 */
#[inline]
pub unsafe fn __arch_read_lock(rw: *mut arch_rwlock_t) {
    let lp = rw;
    core::arch::asm!(
        "mov %o7, %g4\n\t",
        "call ___rw_read_enter\n\t",
        " ldstub [%g1 + 3], %g2",
        in("g1") lp,
        lateout("g2") _, lateout("g4") _, options(nostack)
    );
}

#[inline]
pub unsafe fn arch_read_lock(lock: *mut arch_rwlock_t) {
    let mut flags: usize = 0;
    local_irq_save(&mut flags);
    __arch_read_lock(lock);
    local_irq_restore(flags);
}

#[inline]
pub unsafe fn __arch_read_unlock(rw: *mut arch_rwlock_t) {
    let lp = rw;
    core::arch::asm!(
        "mov %o7, %g4\n\t",
        "call ___rw_read_exit\n\t",
        " ldstub [%g1 + 3], %g2",
        in("g1") lp,
        lateout("g2") _, lateout("g4") _, options(nostack)
    );
}

#[inline]
pub unsafe fn arch_read_unlock(lock: *mut arch_rwlock_t) {
    let mut flags: usize = 0;
    local_irq_save(&mut flags);
    __arch_read_unlock(lock);
    local_irq_restore(flags);
}

#[inline]
pub unsafe fn arch_write_lock(rw: *mut arch_rwlock_t) {
    let lp = rw;
    core::arch::asm!(
        "mov %o7, %g4\n\t",
        "call ___rw_write_enter\n\t",
        " ldstub [%g1 + 3], %g2",
        in("g1") lp,
        lateout("g2") _, lateout("g4") _, options(nostack)
    );
    core::ptr::write_volatile(core::ptr::addr_of_mut!((*lp).lock), !0u32);
}

#[inline]
pub unsafe fn arch_write_unlock(lock: *mut arch_rwlock_t) {
    core::arch::asm!("st %g0, [{0}]", in(reg) lock, options(nostack));
}

#[inline]
pub unsafe fn arch_write_trylock(rw: *mut arch_rwlock_t) -> bool {
    let mut val: u32;
    core::arch::asm!("ldstub [{1} + 3], {0}", out(reg) val, in(reg) core::ptr::addr_of_mut!((*rw).lock), options(nostack));
    if val == 0 {
        val = core::ptr::read_volatile(core::ptr::addr_of!((*rw).lock)) & !0xff;
        if val != 0 {
            core::ptr::write_volatile((core::ptr::addr_of_mut!((*rw).lock) as *mut u8).add(3), 0);
        } else {
            core::ptr::write_volatile(core::ptr::addr_of_mut!((*rw).lock), !0u32);
        }
    }
    val == 0
}

#[inline]
pub unsafe fn __arch_read_trylock(rw: *mut arch_rwlock_t) -> i32 {
    let mut res: i32;
    core::arch::asm!(
        "mov %o7, %g4\n\t",
        "call ___rw_read_try\n\t",
        " ldstub [%g1 + 3], %g2",
        in("g1") rw,
        lateout("o0") res, lateout("g2") _, lateout("g4") _, options(nostack)
    );
    res
}

#[inline]
pub unsafe fn arch_read_trylock(lock: *mut arch_rwlock_t) -> i32 {
    let mut flags: usize = 0;
    local_irq_save(&mut flags);
    let res = __arch_read_trylock(lock);
    local_irq_restore(flags);
    res
}

/* External types and interrupt helpers are supplied by the surrounding kernel translation. */
extern "C" {
    pub fn local_irq_save(flags: *mut usize);
    pub fn local_irq_restore(flags: usize);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
