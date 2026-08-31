/* SPDX-License-Identifier: LGPL-2.1 OR MIT */

/* Depends on the Rust translation of "rseq-bits-template.h". */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_variables)]

use core::ffi::c_void;

pub type intptr_t = isize;
pub type off_t = isize;
pub type size_t = usize;

/*
 * Original C condition:
 * #if defined(RSEQ_TEMPLATE_MO_RELAXED) &&
 *     (defined(RSEQ_TEMPLATE_CPU_ID) || defined(RSEQ_TEMPLATE_MM_CID))
 */

#[inline(always)]
pub unsafe fn RSEQ_TEMPLATE_IDENTIFIER_rseq_cmpeqv_storev(
    v: *mut intptr_t,
    expect: intptr_t,
    newv: intptr_t,
    cpu: i32,
) -> i32 {
    /*
     * Original implementation is a RISC-V `asm goto` rseq critical section
     * using RSEQ_ASM_DEFINE_TABLE, RSEQ_ASM_STORE_RSEQ_CS,
     * RSEQ_ASM_CMP_CPU_ID, RSEQ_ASM_OP_CMPEQ, and
     * RSEQ_ASM_OP_FINAL_STORE. Rust has no direct stable source-level
     * equivalent for this template-local asm-goto control flow.
     */
    core::arch::asm!(
        "/* RSEQ_TEMPLATE_IDENTIFIER(rseq_cmpeqv_storev) */",
        options(nostack, preserves_flags)
    );
    0
}

#[inline(always)]
pub unsafe fn RSEQ_TEMPLATE_IDENTIFIER_rseq_cmpnev_storeoffp_load(
    v: *mut intptr_t,
    expectnot: intptr_t,
    voffp: off_t,
    load: *mut intptr_t,
    cpu: i32,
) -> i32 {
    /*
     * Original implementation is a RISC-V `asm goto` rseq critical section.
     * It compares *v != expectnot, stores the loaded value through load,
     * loads from the value plus voffp, and finally stores through v.
     * Abort returns -1; comparison failure returns 1.
     */
    core::arch::asm!(
        "/* RSEQ_TEMPLATE_IDENTIFIER(rseq_cmpnev_storeoffp_load) */",
        options(nostack, preserves_flags)
    );
    0
}

#[inline(always)]
pub unsafe fn RSEQ_TEMPLATE_IDENTIFIER_rseq_addv(
    v: *mut intptr_t,
    count: intptr_t,
    cpu: i32,
) -> i32 {
    /*
     * Original implementation is a RISC-V `asm goto` rseq critical section.
     * It verifies the current CPU id, loads *v, adds count, and finally stores
     * the result back to *v. Abort returns -1.
     */
    core::arch::asm!(
        "/* RSEQ_TEMPLATE_IDENTIFIER(rseq_addv) */",
        options(nostack, preserves_flags)
    );
    0
}

#[inline(always)]
pub unsafe fn RSEQ_TEMPLATE_IDENTIFIER_rseq_cmpeqv_cmpeqv_storev(
    v: *mut intptr_t,
    expect: intptr_t,
    v2: *mut intptr_t,
    expect2: intptr_t,
    newv: intptr_t,
    cpu: i32,
) -> i32 {
    /*
     * Original implementation is a RISC-V `asm goto` rseq critical section.
     * It verifies CPU id, checks *v == expect and *v2 == expect2, then finally
     * stores newv to *v. Abort returns -1; comparison failure returns 1.
     */
    core::arch::asm!(
        "/* RSEQ_TEMPLATE_IDENTIFIER(rseq_cmpeqv_cmpeqv_storev) */",
        options(nostack, preserves_flags)
    );
    0
}

pub const RSEQ_ARCH_HAS_OFFSET_DEREF_ADDV: bool = true;

/*
 *   pval = *(ptr+off)
 *  *pval += inc;
 */
#[inline(always)]
pub unsafe fn RSEQ_TEMPLATE_IDENTIFIER_rseq_offset_deref_addv(
    ptr: *mut intptr_t,
    off: off_t,
    inc: intptr_t,
    cpu: i32,
) -> i32 {
    /*
     * Original implementation is a RISC-V `asm goto` rseq critical section
     * using RSEQ_ASM_OP_R_DEREF_ADDV(ptr, off, inc, 3). Abort returns -1.
     */
    core::arch::asm!(
        "/* RSEQ_TEMPLATE_IDENTIFIER(rseq_offset_deref_addv) */",
        options(nostack, preserves_flags)
    );
    0
}

/*
 * Original C condition:
 * #endif
 */

/*
 * Original C condition:
 * #if (defined(RSEQ_TEMPLATE_MO_RELAXED) || defined(RSEQ_TEMPLATE_MO_RELEASE)) &&
 *     (defined(RSEQ_TEMPLATE_CPU_ID) || defined(RSEQ_TEMPLATE_MM_CID))
 */

#[inline(always)]
pub unsafe fn RSEQ_TEMPLATE_IDENTIFIER_rseq_cmpeqv_trystorev_storev(
    v: *mut intptr_t,
    expect: intptr_t,
    v2: *mut intptr_t,
    newv2: intptr_t,
    newv: intptr_t,
    cpu: i32,
) -> i32 {
    /*
     * Original implementation is a RISC-V `asm goto` rseq critical section.
     * It verifies CPU id, checks *v == expect, stores newv2 to *v2, then
     * finally stores newv to *v. Under RSEQ_TEMPLATE_MO_RELEASE, the final
     * store uses RSEQ_ASM_OP_FINAL_STORE_RELEASE; otherwise it uses
     * RSEQ_ASM_OP_FINAL_STORE. Abort returns -1; comparison failure returns 1.
     */
    core::arch::asm!(
        "/* RSEQ_TEMPLATE_IDENTIFIER(rseq_cmpeqv_trystorev_storev) */",
        options(nostack, preserves_flags)
    );
    0
}

#[inline(always)]
pub unsafe fn RSEQ_TEMPLATE_IDENTIFIER_rseq_cmpeqv_trymemcpy_storev(
    v: *mut intptr_t,
    expect: intptr_t,
    dst: *mut c_void,
    src: *mut c_void,
    len: size_t,
    newv: intptr_t,
    cpu: i32,
) -> i32 {
    /*
     * Original implementation is a RISC-V `asm goto` rseq critical section.
     * It verifies CPU id, checks *v == expect, performs
     * RSEQ_ASM_OP_R_BAD_MEMCPY(dst, src, len), then finally stores newv to *v.
     * Under RSEQ_TEMPLATE_MO_RELEASE, the final store uses release ordering.
     * Abort returns -1; comparison failure returns 1.
     */
    core::arch::asm!(
        "/* RSEQ_TEMPLATE_IDENTIFIER(rseq_cmpeqv_trymemcpy_storev) */",
        options(nostack, preserves_flags)
    );
    0
}

/*
 * Original C condition:
 * #endif
 */

/* Depends on the Rust translation of "rseq-bits-reset.h". */
