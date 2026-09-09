/* SPDX-License-Identifier: GPL-2.0 */

// C header guard: __ASM_CSKY_ATOMIC_H
// CONFIG_SMP selects the SMP implementation; the non-SMP implementation is
// supplied by asm-generic/atomic.h.

#[cfg(feature = "CONFIG_SMP")]
use core::arch::asm;

#[cfg(feature = "CONFIG_SMP")]
#[inline(always)]
pub unsafe fn __atomic_acquire_fence() {
    // __bar_brarw();
    asm!("bar brarw", options(nostack));
}

#[cfg(feature = "CONFIG_SMP")]
#[inline(always)]
pub unsafe fn __atomic_release_fence() {
    // __bar_brwaw();
    asm!("bar brwaw", options(nostack));
}

#[cfg(feature = "CONFIG_SMP")]
#[inline(always)]
pub unsafe fn arch_atomic_read(v: *const atomic_t) -> i32 {
    core::ptr::read_volatile(core::ptr::addr_of!((*v).counter))
}

#[cfg(feature = "CONFIG_SMP")]
#[inline(always)]
pub unsafe fn arch_atomic_set(v: *mut atomic_t, i: i32) {
    core::ptr::write_volatile(core::ptr::addr_of_mut!((*v).counter), i);
}

#[cfg(feature = "CONFIG_SMP")]
#[inline(always)]
pub unsafe fn arch_atomic_add(i: i32, v: *mut atomic_t) {
    let mut tmp: usize;
    asm!(
        "1: ldex.w {tmp}, ({counter})",
        "add {tmp}, {i}",
        "stex.w {tmp}, ({counter})",
        "bez {tmp}, 1b",
        tmp = out(reg) tmp, i = in(reg) i, counter = in(reg) core::ptr::addr_of_mut!((*v).counter),
        options(nostack)
    );
}

#[cfg(feature = "CONFIG_SMP")]
#[inline(always)]
pub unsafe fn arch_atomic_sub(i: i32, v: *mut atomic_t) {
    let mut tmp: usize;
    asm!(
        "1: ldex.w {tmp}, ({counter})", "sub {tmp}, {i}",
        "stex.w {tmp}, ({counter})", "bez {tmp}, 1b",
        tmp = out(reg) tmp, i = in(reg) i, counter = in(reg) core::ptr::addr_of_mut!((*v).counter), options(nostack)
    );
}

#[cfg(feature = "CONFIG_SMP")]
#[inline(always)]
pub unsafe fn arch_atomic_and(i: i32, v: *mut atomic_t) {
    let mut tmp: usize;
    asm!("1: ldex.w {tmp}, ({counter})", "and {tmp}, {i}", "stex.w {tmp}, ({counter})", "bez {tmp}, 1b", tmp = out(reg) tmp, i = in(reg) i, counter = in(reg) core::ptr::addr_of_mut!((*v).counter), options(nostack));
}

#[cfg(feature = "CONFIG_SMP")]
#[inline(always)]
pub unsafe fn arch_atomic_or(i: i32, v: *mut atomic_t) {
    let mut tmp: usize;
    asm!("1: ldex.w {tmp}, ({counter})", "or {tmp}, {i}", "stex.w {tmp}, ({counter})", "bez {tmp}, 1b", tmp = out(reg) tmp, i = in(reg) i, counter = in(reg) core::ptr::addr_of_mut!((*v).counter), options(nostack));
}

#[cfg(feature = "CONFIG_SMP")]
#[inline(always)]
pub unsafe fn arch_atomic_xor(i: i32, v: *mut atomic_t) {
    let mut tmp: usize;
    asm!("1: ldex.w {tmp}, ({counter})", "xor {tmp}, {i}", "stex.w {tmp}, ({counter})", "bez {tmp}, 1b", tmp = out(reg) tmp, i = in(reg) i, counter = in(reg) core::ptr::addr_of_mut!((*v).counter), options(nostack));
}

#[cfg(feature = "CONFIG_SMP")]
#[inline(always)]
pub unsafe fn arch_atomic_fetch_add_relaxed(i: i32, v: *mut atomic_t) -> i32 {
    let mut tmp: i32;
    let mut ret: i32;
    asm!("1: ldex.w {tmp}, ({counter})", "mov {ret}, {tmp}", "add {tmp}, {i}", "stex.w {tmp}, ({counter})", "bez {tmp}, 1b", tmp = out(reg) tmp, ret = out(reg) ret, i = in(reg) i, counter = in(reg) core::ptr::addr_of_mut!((*v).counter), options(nostack));
    ret
}

#[cfg(feature = "CONFIG_SMP")]
#[inline(always)]
pub unsafe fn arch_atomic_fetch_sub_relaxed(i: i32, v: *mut atomic_t) -> i32 {
    let mut tmp: i32;
    let mut ret: i32;
    asm!("1: ldex.w {tmp}, ({counter})", "mov {ret}, {tmp}", "sub {tmp}, {i}", "stex.w {tmp}, ({counter})", "bez {tmp}, 1b", tmp = out(reg) tmp, ret = out(reg) ret, i = in(reg) i, counter = in(reg) core::ptr::addr_of_mut!((*v).counter), options(nostack));
    ret
}

#[cfg(feature = "CONFIG_SMP")]
#[inline(always)]
pub unsafe fn arch_atomic_add_return_relaxed(i: i32, v: *mut atomic_t) -> i32 { arch_atomic_fetch_add_relaxed(i, v).wrapping_add(i) }

#[cfg(feature = "CONFIG_SMP")]
#[inline(always)]
pub unsafe fn arch_atomic_sub_return_relaxed(i: i32, v: *mut atomic_t) -> i32 { arch_atomic_fetch_sub_relaxed(i, v).wrapping_sub(i) }

#[cfg(feature = "CONFIG_SMP")]
#[inline(always)]
pub unsafe fn arch_atomic_fetch_and_relaxed(i: i32, v: *mut atomic_t) -> i32 {
    let mut tmp: i32; let mut ret: i32;
    asm!("1: ldex.w {tmp}, ({counter})", "mov {ret}, {tmp}", "and {tmp}, {i}", "stex.w {tmp}, ({counter})", "bez {tmp}, 1b", tmp = out(reg) tmp, ret = out(reg) ret, i = in(reg) i, counter = in(reg) core::ptr::addr_of_mut!((*v).counter), options(nostack)); ret
}

#[cfg(feature = "CONFIG_SMP")]
#[inline(always)]
pub unsafe fn arch_atomic_fetch_or_relaxed(i: i32, v: *mut atomic_t) -> i32 {
    let mut tmp: i32; let mut ret: i32;
    asm!("1: ldex.w {tmp}, ({counter})", "mov {ret}, {tmp}", "or {tmp}, {i}", "stex.w {tmp}, ({counter})", "bez {tmp}, 1b", tmp = out(reg) tmp, ret = out(reg) ret, i = in(reg) i, counter = in(reg) core::ptr::addr_of_mut!((*v).counter), options(nostack)); ret
}

#[cfg(feature = "CONFIG_SMP")]
#[inline(always)]
pub unsafe fn arch_atomic_fetch_xor_relaxed(i: i32, v: *mut atomic_t) -> i32 {
    let mut tmp: i32; let mut ret: i32;
    asm!("1: ldex.w {tmp}, ({counter})", "mov {ret}, {tmp}", "xor {tmp}, {i}", "stex.w {tmp}, ({counter})", "bez {tmp}, 1b", tmp = out(reg) tmp, ret = out(reg) ret, i = in(reg) i, counter = in(reg) core::ptr::addr_of_mut!((*v).counter), options(nostack)); ret
}

#[cfg(feature = "CONFIG_SMP")]
#[inline(always)]
pub unsafe fn arch_atomic_fetch_add_unless(v: *mut atomic_t, a: i32, u: i32) -> i32 {
    let mut prev: i32; let mut tmp: i32;
    asm!("1: ldex.w {prev}, ({counter})", "cmpne {prev}, {u}", "bf 2f", "mov {tmp}, {prev}", "add {tmp}, {a}", "stex.w {tmp}, ({counter})", "bez {tmp}, 1b", "2:", prev = out(reg) prev, tmp = out(reg) tmp, a = in(reg) a, u = in(reg) u, counter = in(reg) core::ptr::addr_of_mut!((*v).counter), options(nostack)); prev
}

#[cfg(feature = "CONFIG_SMP")]
#[inline(always)]
pub unsafe fn arch_atomic_inc_unless_negative(v: *mut atomic_t) -> bool {
    let mut rc: i32; let mut tmp: i32;
    asm!("1: ldex.w {tmp}, ({counter})", "movi {rc}, 0", "blz {tmp}, 2f", "movi {rc}, 1", "addi {tmp}, 1", "stex.w {tmp}, ({counter})", "bez {tmp}, 1b", "2:", tmp = out(reg) tmp, rc = out(reg) rc, counter = in(reg) core::ptr::addr_of_mut!((*v).counter), options(nostack)); rc != 0
}

#[cfg(feature = "CONFIG_SMP")]
#[inline(always)]
pub unsafe fn arch_atomic_dec_unless_positive(v: *mut atomic_t) -> bool {
    let mut rc: i32; let mut tmp: i32;
    asm!("1: ldex.w {tmp}, ({counter})", "movi {rc}, 0", "bhz {tmp}, 2f", "movi {rc}, 1", "subi {tmp}, 1", "stex.w {tmp}, ({counter})", "bez {tmp}, 1b", "2:", tmp = out(reg) tmp, rc = out(reg) rc, counter = in(reg) core::ptr::addr_of_mut!((*v).counter), options(nostack)); rc != 0
}

#[cfg(feature = "CONFIG_SMP")]
#[inline(always)]
pub unsafe fn arch_atomic_dec_if_positive(v: *mut atomic_t) -> i32 {
    let mut dec: i32; let mut tmp: i32;
    asm!("1: ldex.w {dec}, ({counter})", "subi {tmp}, {dec}, 1", "blz {tmp}, 2f", "stex.w {tmp}, ({counter})", "bez {tmp}, 1b", "2:", dec = out(reg) dec, tmp = out(reg) tmp, counter = in(reg) core::ptr::addr_of_mut!((*v).counter), options(nostack)); dec.wrapping_sub(1)
}

// The remaining C header operations use the same Csky ldex.w/stex.w retry
// loops and RELEASE_FENCE/FULL_FENCE macros. Their external atomic_t type and
// fence definitions are supplied by the surrounding translation unit.

#[cfg(not(feature = "CONFIG_SMP"))]
// Equivalent to: #include <asm-generic/atomic.h>
pub mod generic_atomic {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
