/* SPDX-License-Identifier: GPL-2.0 */
// Translated from the m68k atomic operations header.

/*
 * Atomic operations that C can't guarantee us. Useful for
 * resource counting etc..
 */
/* We do not have SMP m68k systems, so we don't have to deal with that. */

#[inline(always)]
pub unsafe fn arch_atomic_read(v: *const atomic_t) -> i32 {
    core::ptr::read_volatile(core::ptr::addr_of!((*v).counter))
}

#[inline(always)]
pub unsafe fn arch_atomic_set(v: *mut atomic_t, i: i32) {
    core::ptr::write_volatile(core::ptr::addr_of_mut!((*v).counter), i)
}

// The original ASM_DI constraint is "d" for ColdFire and "di" otherwise.

#[cfg(feature = "CONFIG_RMW_INSNS")]
#[inline(always)]
pub unsafe fn arch_atomic_add(i: i32, v: *mut atomic_t) {
    core::arch::asm!("addl {i}, [{v}]", i = in(reg) i, v = in(reg) v, options(nostack));
}
#[cfg(not(feature = "CONFIG_RMW_INSNS"))]
#[inline(always)]
pub unsafe fn arch_atomic_add(i: i32, v: *mut atomic_t) {
    let mut flags: usize;
    local_irq_save(&mut flags);
    (*v).counter = (*v).counter.wrapping_add(i);
    local_irq_restore(flags);
}

#[cfg(feature = "CONFIG_RMW_INSNS")]
#[inline(always)]
pub unsafe fn arch_atomic_sub(i: i32, v: *mut atomic_t) {
    core::arch::asm!("subl {i}, [{v}]", i = in(reg) i, v = in(reg) v, options(nostack));
}
#[cfg(not(feature = "CONFIG_RMW_INSNS"))]
#[inline(always)]
pub unsafe fn arch_atomic_sub(i: i32, v: *mut atomic_t) {
    let mut flags: usize;
    local_irq_save(&mut flags);
    (*v).counter = (*v).counter.wrapping_sub(i);
    local_irq_restore(flags);
}

#[cfg(feature = "CONFIG_RMW_INSNS")]
#[inline(always)]
pub unsafe fn arch_atomic_add_return(i: i32, v: *mut atomic_t) -> i32 {
    let mut t: i32;
    let mut tmp: i32;
    core::arch::asm!("1: movel {old}, {t}; addl {i}, {t}; casl {old}, {t}, [{v}]; jne 1b",
        old = inout(reg) arch_atomic_read(v) => tmp, t = lateout(reg) t,
        i = in(reg) i, v = in(reg) v, options(nostack));
    t
}
#[cfg(feature = "CONFIG_RMW_INSNS")]
#[inline(always)]
pub unsafe fn arch_atomic_sub_return(i: i32, v: *mut atomic_t) -> i32 {
    let old = arch_atomic_read(v); let value = old.wrapping_sub(i); arch_atomic_set(v, value); value
}
#[cfg(not(feature = "CONFIG_RMW_INSNS"))]
#[inline(always)]
pub unsafe fn arch_atomic_add_return(i: i32, v: *mut atomic_t) -> i32 {
    let mut flags: usize; local_irq_save(&mut flags);
    let t = (*v).counter.wrapping_add(i); (*v).counter = t;
    local_irq_restore(flags); t
}

#[cfg(feature = "CONFIG_RMW_INSNS")]
#[inline(always)]
pub unsafe fn arch_atomic_sub_return(i: i32, v: *mut atomic_t) -> i32 {
    let mut flags: usize; local_irq_save(&mut flags);
    let t = (*v).counter.wrapping_sub(i); (*v).counter = t;
    local_irq_restore(flags); t
}
#[cfg(not(feature = "CONFIG_RMW_INSNS"))]
#[inline(always)]
pub unsafe fn arch_atomic_sub_return(i: i32, v: *mut atomic_t) -> i32 { arch_atomic_sub_return_fallback(i, v) }

#[inline(always)]
unsafe fn arch_atomic_sub_return_fallback(i: i32, v: *mut atomic_t) -> i32 {
    let mut flags: usize; local_irq_save(&mut flags);
    let t = (*v).counter.wrapping_sub(i); (*v).counter = t;
    local_irq_restore(flags); t
}

#[inline(always)]
pub unsafe fn arch_atomic_fetch_add(i: i32, v: *mut atomic_t) -> i32 {
    let old = arch_atomic_read(v); arch_atomic_add(i, v); old
}
#[inline(always)]
pub unsafe fn arch_atomic_fetch_sub(i: i32, v: *mut atomic_t) -> i32 {
    let old = arch_atomic_read(v); arch_atomic_sub(i, v); old
}

#[inline(always)] pub unsafe fn arch_atomic_and(i: i32, v: *mut atomic_t) { (*v).counter &= i; }
#[inline(always)] pub unsafe fn arch_atomic_or(i: i32, v: *mut atomic_t) { (*v).counter |= i; }
#[inline(always)] pub unsafe fn arch_atomic_xor(i: i32, v: *mut atomic_t) { (*v).counter ^= i; }
#[inline(always)] pub unsafe fn arch_atomic_fetch_and(i: i32, v: *mut atomic_t) -> i32 { let old = (*v).counter; (*v).counter &= i; old }
#[inline(always)] pub unsafe fn arch_atomic_fetch_or(i: i32, v: *mut atomic_t) -> i32 { let old = (*v).counter; (*v).counter |= i; old }
#[inline(always)] pub unsafe fn arch_atomic_fetch_xor(i: i32, v: *mut atomic_t) -> i32 { let old = (*v).counter; (*v).counter ^= i; old }

#[inline(always)] pub unsafe fn arch_atomic_inc(v: *mut atomic_t) { arch_atomic_add(1, v); }
#[inline(always)] pub unsafe fn arch_atomic_dec(v: *mut atomic_t) { arch_atomic_sub(1, v); }
#[inline(always)] pub unsafe fn arch_atomic_dec_and_test(v: *mut atomic_t) -> i32 { arch_atomic_dec(v); ((*v).counter == 0) as i32 }
#[inline(always)] pub unsafe fn arch_atomic_dec_and_test_lt(v: *mut atomic_t) -> i32 { arch_atomic_dec(v); ((*v).counter < 0) as i32 }
#[inline(always)] pub unsafe fn arch_atomic_inc_and_test(v: *mut atomic_t) -> i32 { arch_atomic_add(1, v); ((*v).counter == 0) as i32 }

#[cfg(not(feature = "CONFIG_RMW_INSNS"))]
#[inline(always)]
pub unsafe fn arch_atomic_cmpxchg(v: *mut atomic_t, old: i32, new: i32) -> i32 {
    let mut flags: usize; local_irq_save(&mut flags); let prev = arch_atomic_read(v);
    if prev == old { arch_atomic_set(v, new); } local_irq_restore(flags); prev
}
#[cfg(not(feature = "CONFIG_RMW_INSNS"))]
#[inline(always)]
pub unsafe fn arch_atomic_xchg(v: *mut atomic_t, new: i32) -> i32 {
    let mut flags: usize; local_irq_save(&mut flags); let prev = arch_atomic_read(v);
    arch_atomic_set(v, new); local_irq_restore(flags); prev
}

#[inline(always)] pub unsafe fn arch_atomic_sub_and_test(i: i32, v: *mut atomic_t) -> i32 { arch_atomic_sub(i, v); ((*v).counter == 0) as i32 }
#[inline(always)] pub unsafe fn arch_atomic_add_negative(i: i32, v: *mut atomic_t) -> i32 { arch_atomic_add(i, v); ((*v).counter < 0) as i32 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
