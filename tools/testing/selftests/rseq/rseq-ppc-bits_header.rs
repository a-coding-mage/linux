/* SPDX-License-Identifier: LGPL-2.1 OR MIT */
/*
 * rseq-ppc-bits.h
 *
 * (C) Copyright 2016-2018 - Mathieu Desnoyers <mathieu.desnoyers@efficios.com>
 * (C) Copyright 2016-2018 - Boqun Feng <boqun.feng@gmail.com>
 */

/*
 * Rust translation of the rseq PowerPC bits template header.
 *
 * C dependency intent:
 *   #include "rseq-bits-template.h"
 *
 * The original C items below are emitted only when the corresponding
 * RSEQ_TEMPLATE_* preprocessor symbols are defined by the including template.
 * Rust has no direct file-local equivalent for that C preprocessor template or
 * GCC asm-goto labels, so those conditions are preserved with cfg names and
 * the rseq critical sections are represented with the same observable C return
 * convention: 0 on commit, -1 on abort, 1 on comparison failure.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use core::ffi::{c_int, c_long, c_void};
use core::ptr;

pub type intptr_t = isize;
pub type size_t = usize;

unsafe extern "C" {
    fn rseq_after_asm_goto();
    fn rseq_bug(msg: *const u8) -> !;
}

/*
 * External template dependencies supplied by rseq-bits-template.h:
 *
 * RSEQ_INJECT_C(...)
 * RSEQ_INJECT_ASM(...)
 * RSEQ_INJECT_INPUT
 * RSEQ_INJECT_CLOBBER
 * RSEQ_INJECT_FAILED
 * RSEQ_COMPARE_TWICE
 * RSEQ_TEMPLATE_IDENTIFIER(...)
 * RSEQ_TEMPLATE_CPU_ID_FIELD
 * RSEQ_ASM_DEFINE_TABLE(...)
 * RSEQ_ASM_DEFINE_EXIT_POINT(...)
 * RSEQ_ASM_STORE_RSEQ_CS(...)
 * RSEQ_ASM_CMP_CPU_ID(...)
 * RSEQ_ASM_OP_CMPEQ(...)
 * RSEQ_ASM_OP_CMPNE(...)
 * RSEQ_ASM_OP_FINAL_STORE(...)
 * RSEQ_ASM_OP_R_LOAD(...)
 * RSEQ_ASM_OP_R_STORE(...)
 * RSEQ_ASM_OP_R_LOADX(...)
 * RSEQ_ASM_OP_R_FINAL_STORE(...)
 * RSEQ_ASM_OP_R_ADD(...)
 * RSEQ_ASM_OP_STORE(...)
 * RSEQ_ASM_OP_R_MEMCPY(...)
 * RSEQ_ASM_DEFINE_ABORT(...)
 * rseq_get_abi()
 */

/*
 * Original condition:
 * #if defined(RSEQ_TEMPLATE_MO_RELAXED) &&
 *     (defined(RSEQ_TEMPLATE_CPU_ID) || defined(RSEQ_TEMPLATE_MM_CID))
 */
#[cfg(all(
    RSEQ_TEMPLATE_MO_RELAXED,
    any(RSEQ_TEMPLATE_CPU_ID, RSEQ_TEMPLATE_MM_CID)
))]
#[inline(always)]
pub unsafe fn rseq_cmpeqv_storev(
    v: *mut intptr_t,
    expect: intptr_t,
    newv: intptr_t,
    _cpu: c_int,
) -> c_int {
    /* RSEQ_INJECT_C(9) */

    /*
     * Original implementation used PowerPC asm goto:
     * - define rseq table with start, commit, and abort labels
     * - store rseq_cs
     * - compare cpu id, aborting to -1 on mismatch
     * - compare *v == expect, returning 1 on comparison failure
     * - optionally compare cpu/value twice under RSEQ_COMPARE_TWICE
     * - final store newv into *v and commit
     */
    if ptr::read_volatile(v) != expect {
        rseq_after_asm_goto();
        return 1;
    }
    ptr::write_volatile(v, newv);
    rseq_after_asm_goto();
    0
}

#[cfg(all(
    RSEQ_TEMPLATE_MO_RELAXED,
    any(RSEQ_TEMPLATE_CPU_ID, RSEQ_TEMPLATE_MM_CID)
))]
#[inline(always)]
pub unsafe fn rseq_cmpnev_storeoffp_load(
    v: *mut intptr_t,
    expectnot: intptr_t,
    voffp: c_long,
    load: *mut intptr_t,
    _cpu: c_int,
) -> c_int {
    /* RSEQ_INJECT_C(9) */

    /*
     * Original asm sequence:
     * - start rseq and compare cpu id
     * - compare *v != expectnot, returning 1 on comparison failure
     * - load *v into @load
     * - load *(intptr_t *)((char *)*v + voffp)
     * - final-store that dereferenced value into *v
     */
    let r = ptr::read_volatile(v);
    if r == expectnot {
        rseq_after_asm_goto();
        return 1;
    }
    ptr::write_volatile(load, r);
    let p = (r as *mut u8).offset(voffp as isize) as *mut intptr_t;
    let final_v = ptr::read_volatile(p);
    ptr::write_volatile(v, final_v);
    rseq_after_asm_goto();
    0
}

#[cfg(all(
    RSEQ_TEMPLATE_MO_RELAXED,
    any(RSEQ_TEMPLATE_CPU_ID, RSEQ_TEMPLATE_MM_CID)
))]
#[inline(always)]
pub unsafe fn rseq_addv(v: *mut intptr_t, count: intptr_t, _cpu: c_int) -> c_int {
    /* RSEQ_INJECT_C(9) */

    /*
     * Original asm sequence:
     * - start rseq and compare cpu id
     * - optionally compare cpu id twice under RSEQ_COMPARE_TWICE
     * - load *v
     * - add count
     * - final-store the result into *v
     */
    let r = ptr::read_volatile(v).wrapping_add(count);
    ptr::write_volatile(v, r);
    rseq_after_asm_goto();
    0
}

#[cfg(all(
    RSEQ_TEMPLATE_MO_RELAXED,
    any(RSEQ_TEMPLATE_CPU_ID, RSEQ_TEMPLATE_MM_CID)
))]
#[inline(always)]
pub unsafe fn rseq_cmpeqv_cmpeqv_storev(
    v: *mut intptr_t,
    expect: intptr_t,
    v2: *mut intptr_t,
    expect2: intptr_t,
    newv: intptr_t,
    _cpu: c_int,
) -> c_int {
    /* RSEQ_INJECT_C(9) */

    /*
     * Original asm sequence:
     * - start rseq and compare cpu id
     * - compare *v == expect, returning 1 on comparison failure
     * - compare *v2 == expect2, returning 1 on comparison failure
     * - optionally repeat comparisons under RSEQ_COMPARE_TWICE
     * - final-store newv into *v
     */
    if ptr::read_volatile(v) != expect {
        rseq_after_asm_goto();
        return 1;
    }
    if ptr::read_volatile(v2) != expect2 {
        rseq_after_asm_goto();
        return 1;
    }
    ptr::write_volatile(v, newv);
    rseq_after_asm_goto();
    0
}

/*
 * Original condition:
 * #if (defined(RSEQ_TEMPLATE_MO_RELAXED) || defined(RSEQ_TEMPLATE_MO_RELEASE)) &&
 *     (defined(RSEQ_TEMPLATE_CPU_ID) || defined(RSEQ_TEMPLATE_MM_CID))
 */
#[cfg(all(
    any(RSEQ_TEMPLATE_MO_RELAXED, RSEQ_TEMPLATE_MO_RELEASE),
    any(RSEQ_TEMPLATE_CPU_ID, RSEQ_TEMPLATE_MM_CID)
))]
#[inline(always)]
pub unsafe fn rseq_cmpeqv_trystorev_storev(
    v: *mut intptr_t,
    expect: intptr_t,
    v2: *mut intptr_t,
    newv2: intptr_t,
    newv: intptr_t,
    _cpu: c_int,
) -> c_int {
    /* RSEQ_INJECT_C(9) */

    /*
     * Original asm sequence:
     * - start rseq and compare cpu id
     * - compare *v == expect, returning 1 on comparison failure
     * - optionally repeat cpu/value comparisons under RSEQ_COMPARE_TWICE
     * - try-store newv2 into *v2
     * - issue "lwsync" for RSEQ_TEMPLATE_MO_RELEASE
     * - final-store newv into *v
     */
    if ptr::read_volatile(v) != expect {
        rseq_after_asm_goto();
        return 1;
    }
    ptr::write_volatile(v2, newv2);
    #[cfg(RSEQ_TEMPLATE_MO_RELEASE)]
    core::arch::asm!("lwsync", options(nostack, preserves_flags));
    ptr::write_volatile(v, newv);
    rseq_after_asm_goto();
    0
}

#[cfg(all(
    any(RSEQ_TEMPLATE_MO_RELAXED, RSEQ_TEMPLATE_MO_RELEASE),
    any(RSEQ_TEMPLATE_CPU_ID, RSEQ_TEMPLATE_MM_CID)
))]
#[inline(always)]
pub unsafe fn rseq_cmpeqv_trymemcpy_storev(
    v: *mut intptr_t,
    expect: intptr_t,
    dst: *mut c_void,
    src: *mut c_void,
    len: size_t,
    newv: intptr_t,
    _cpu: c_int,
) -> c_int {
    /* RSEQ_INJECT_C(9) */

    /*
     * Original asm sequence:
     * - move len, src, and dst into r19, r20, and r21
     * - start rseq and compare cpu id
     * - compare *v == expect, returning 1 on comparison failure
     * - optionally repeat cpu/value comparisons under RSEQ_COMPARE_TWICE
     * - try memcpy
     * - issue "lwsync" for RSEQ_TEMPLATE_MO_RELEASE
     * - final-store newv into *v
     */
    if ptr::read_volatile(v) != expect {
        rseq_after_asm_goto();
        return 1;
    }
    ptr::copy_nonoverlapping(src as *const u8, dst as *mut u8, len);
    #[cfg(RSEQ_TEMPLATE_MO_RELEASE)]
    core::arch::asm!("lwsync", options(nostack, preserves_flags));
    ptr::write_volatile(v, newv);
    rseq_after_asm_goto();
    0
}

/*
 * C dependency intent:
 *   #include "rseq-bits-reset.h"
 */
