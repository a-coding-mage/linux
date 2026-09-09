/* SPDX-License-Identifier: GPL-2.0-or-later */

/*
 * Simple spin lock operations.
 *
 * Copyright (C) 2001-2004 Paul Mackerras <paulus@au.ibm.com>, IBM
 * Copyright (C) 2001 Anton Blanchard <anton@au.ibm.com>, IBM
 * Copyright (C) 2002 Dave Engebretsen <engebret@us.ibm.com>, IBM
 *	Rework to support virtual processors
 *
 * Type of int is used as a full 64b word is not necessary.
 *
 * (the type definitions are in asm/simple_spinlock_types.h)
 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/irqflags.h, linux/kcsan-checks.h, asm/paravirt.h, asm/paca.h,
// asm/synch.h, and asm/ppc-opcode.h.

#[cfg(target_arch = "powerpc64")]
#[cfg(target_endian = "big")]
#[inline(always)]
unsafe fn lock_token() -> u32 {
    *(core::ptr::addr_of!((*get_paca()).lock_token) as *const u32)
}

#[cfg(target_arch = "powerpc64")]
#[cfg(target_endian = "little")]
#[inline(always)]
unsafe fn lock_token() -> u32 {
    *(core::ptr::addr_of!((*get_paca()).paca_index) as *const u32)
}

#[cfg(not(target_arch = "powerpc64"))]
const LOCK_TOKEN: u32 = 1;

#[inline(always)]
fn arch_spin_value_unlocked(lock: arch_spinlock_t) -> i32 {
    (lock.slock == 0) as i32
}

#[inline]
unsafe fn arch_spin_is_locked(lock: *mut arch_spinlock_t) -> i32 {
    (!((core::ptr::read_volatile(lock)).slock == 0)) as i32
}

/* This returns the old value in the lock; success is indicated by 0. */
#[inline]
unsafe fn __arch_spin_trylock(lock: *mut arch_spinlock_t) -> usize {
    let mut tmp: usize;
    #[cfg(target_arch = "powerpc64")]
    let token = lock_token();
    #[cfg(not(target_arch = "powerpc64"))]
    let token = LOCK_TOKEN;
    let eh: u32 = cfg!(target_arch = "powerpc64") as u32;
    core::arch::asm!(
        "1: lwarx {tmp},0,{addr},{eh}",
        "cmpwi 0,{tmp},0",
        "bne- 2f",
        "stwcx. {token},0,{addr}",
        "bne- 1b",
        "2:",
        tmp = lateout(reg) tmp,
        token = in(reg) token,
        addr = in(reg) core::ptr::addr_of_mut!((*lock).slock),
        eh = const eh,
        options(nostack)
    );
    tmp
}

#[inline]
unsafe fn arch_spin_trylock(lock: *mut arch_spinlock_t) -> i32 {
    (__arch_spin_trylock(lock) == 0) as i32
}

#[cfg(target_arch = "powerpc64")]
const WRLOCK_TOKEN: u32 = 0; // LOCK_TOKEN; supplied as the negative lock token.
#[cfg(not(target_arch = "powerpc64"))]
const WRLOCK_TOKEN: i32 = -1;

#[inline]
unsafe fn __arch_read_trylock(rw: *mut arch_rwlock_t) -> isize {
    let mut tmp: isize;
    let eh: u32 = cfg!(target_arch = "powerpc64") as u32;
    core::arch::asm!(
        "1: lwarx {tmp},0,{addr},{eh}",
        "addic. {tmp},{tmp},1",
        "ble- 2f",
        "stwcx. {tmp},0,{addr}",
        "bne- 1b",
        "2:",
        tmp = lateout(reg) tmp,
        addr = in(reg) core::ptr::addr_of_mut!((*rw).lock),
        eh = const eh,
        options(nostack)
    );
    tmp
}

#[inline]
unsafe fn __arch_write_trylock(rw: *mut arch_rwlock_t) -> isize {
    let mut tmp: isize;
    #[cfg(target_arch = "powerpc64")]
    let token = lock_token();
    #[cfg(not(target_arch = "powerpc64"))]
    let token = WRLOCK_TOKEN;
    let eh: u32 = cfg!(target_arch = "powerpc64") as u32;
    core::arch::asm!(
        "1: lwarx {tmp},0,{addr},{eh}",
        "cmpwi 0,{tmp},0",
        "bne- 2f",
        "stwcx. {token},0,{addr}",
        "bne- 1b",
        "2:",
        tmp = lateout(reg) tmp,
        token = in(reg) token,
        addr = in(reg) core::ptr::addr_of_mut!((*rw).lock),
        eh = const eh,
        options(nostack)
    );
    tmp
}

#[cfg(feature = "CONFIG_PPC_SPLPAR")]
extern "C" { fn splpar_spin_yield(lock: *mut arch_spinlock_t); fn splpar_rw_yield(lock: *mut arch_rwlock_t); }
#[cfg(not(feature = "CONFIG_PPC_SPLPAR"))]
#[inline] unsafe fn splpar_spin_yield(_: *mut arch_spinlock_t) {}
#[cfg(not(feature = "CONFIG_PPC_SPLPAR"))]
#[inline] unsafe fn splpar_rw_yield(_: *mut arch_rwlock_t) {}

#[inline] unsafe fn spin_yield(lock: *mut arch_spinlock_t) { if is_shared_processor() { splpar_spin_yield(lock) } else { barrier() } }
#[inline] unsafe fn rw_yield(lock: *mut arch_rwlock_t) { if is_shared_processor() { splpar_rw_yield(lock) } else { barrier() } }

#[inline] unsafe fn arch_spin_lock(lock: *mut arch_spinlock_t) {
    loop { if __arch_spin_trylock(lock) == 0 { break; } loop { HMT_low(); if is_shared_processor() { splpar_spin_yield(lock); } if (*lock).slock == 0 { break; } } HMT_medium(); }
}

#[inline] unsafe fn arch_spin_unlock(lock: *mut arch_spinlock_t) {
    kcsan_mb();
    core::arch::asm!("# arch_spin_unlock", options(nostack));
    (*lock).slock = 0;
}

#[inline] unsafe fn arch_read_lock(rw: *mut arch_rwlock_t) {
    loop { if __arch_read_trylock(rw) > 0 { break; } loop { HMT_low(); if is_shared_processor() { splpar_rw_yield(rw); } if (*rw).lock >= 0 { break; } } HMT_medium(); }
}

#[inline] unsafe fn arch_write_lock(rw: *mut arch_rwlock_t) {
    loop { if __arch_write_trylock(rw) == 0 { break; } loop { HMT_low(); if is_shared_processor() { splpar_rw_yield(rw); } if (*rw).lock == 0 { break; } } HMT_medium(); }
}

#[inline] unsafe fn arch_read_trylock(rw: *mut arch_rwlock_t) -> i32 { (__arch_read_trylock(rw) > 0) as i32 }
#[inline] unsafe fn arch_write_trylock(rw: *mut arch_rwlock_t) -> i32 { (__arch_write_trylock(rw) == 0) as i32 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
