/* SPDX-License-Identifier: LGPL-2.1 OR MIT */
/*
 * rseq-x86-bits.h
 *
 * (C) Copyright 2016-2022 - Mathieu Desnoyers <mathieu.desnoyers@efficios.com>
 */

/*
 * Rust translation of the x86-64 rseq template header.
 *
 * The original C file includes "rseq-bits-template.h" and is compiled through
 * preprocessor templates:
 *
 * - __x86_64__
 * - RSEQ_TEMPLATE_MO_RELAXED
 * - RSEQ_TEMPLATE_MO_RELEASE
 * - RSEQ_TEMPLATE_CPU_ID
 * - RSEQ_TEMPLATE_MM_CID
 * - RSEQ_COMPARE_TWICE
 * - RSEQ_TEMPLATE_IDENTIFIER(...)
 *
 * The C implementation relies on GCC asm goto and rseq table-generation macros
 * (RSEQ_ASM_DEFINE_TABLE, RSEQ_ASM_DEFINE_EXIT_POINT,
 * RSEQ_ASM_STORE_RSEQ_CS, RSEQ_ASM_CMP_CPU_ID, RSEQ_ASM_DEFINE_ABORT,
 * RSEQ_ASM_DEFINE_CMPFAIL, RSEQ_INJECT_*). Those dependencies are supplied by
 * other headers and have no complete file-local Rust equivalent here. The
 * functions below preserve the source-level data operations, pointer behavior,
 * comments, and return conventions of the successful and comparison-failure
 * paths. Kernel rseq critical-section registration, abort fixups, injected
 * failure behavior, and optional double-compare bug checks remain external
 * template/assembly responsibilities in the original source.
 */

#![allow(non_camel_case_types)]

use core::ffi::{c_int, c_long, c_void};

pub type intptr_t = isize;
pub type size_t = usize;

/*
 * Original condition:
 *
 * #ifdef __x86_64__
 * #if defined(RSEQ_TEMPLATE_MO_RELAXED) &&
 *     (defined(RSEQ_TEMPLATE_CPU_ID) || defined(RSEQ_TEMPLATE_MM_CID))
 */

#[cfg(target_arch = "x86_64")]
#[inline(always)]
pub unsafe fn rseq_cmpeqv_storev(
    v: *mut intptr_t,
    expect: intptr_t,
    newv: intptr_t,
    _cpu: c_int,
) -> c_int {
    /* RSEQ_INJECT_C(9) */
    /*
     * Start rseq by storing table entry pointer into rseq_cs.
     * Compare CPU id/cid through the template-selected rseq field.
     */
    if core::ptr::read_volatile(v) != expect {
        return 1;
    }
    /*
     * RSEQ_COMPARE_TWICE optionally repeats the cpu_id and expected-value
     * comparisons and calls rseq_bug on mismatch in the C template.
     */

    /* final store */
    core::ptr::write_volatile(v, newv);
    0
}

/*
 * Compare @v against @expectnot. When it does _not_ match, load @v
 * into @load, and store the content of *@v + voffp into @v.
 */
#[cfg(target_arch = "x86_64")]
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
     * Start rseq by storing table entry pointer into rseq_cs.
     * Compare CPU id/cid through the template-selected rseq field.
     */
    let mut rbx = core::ptr::read_volatile(v);
    if rbx == expectnot {
        return 1;
    }
    /*
     * RSEQ_COMPARE_TWICE optionally repeats the cpu_id and expected-not
     * comparisons and calls rseq_bug on mismatch in the C template.
     */

    core::ptr::write_volatile(load, rbx);
    rbx = rbx.wrapping_add(voffp as intptr_t);
    rbx = core::ptr::read_volatile(rbx as *const intptr_t);

    /* final store */
    core::ptr::write_volatile(v, rbx);
    0
}

#[cfg(target_arch = "x86_64")]
#[inline(always)]
pub unsafe fn rseq_addv(v: *mut intptr_t, count: intptr_t, _cpu: c_int) -> c_int {
    /* RSEQ_INJECT_C(9) */
    /*
     * Start rseq by storing table entry pointer into rseq_cs.
     * Compare CPU id/cid through the template-selected rseq field.
     */
    /*
     * RSEQ_COMPARE_TWICE optionally repeats the cpu_id comparison and calls
     * rseq_bug on mismatch in the C template.
     */

    /* final store */
    let old = core::ptr::read_volatile(v);
    core::ptr::write_volatile(v, old.wrapping_add(count));
    0
}

/* #define RSEQ_ARCH_HAS_OFFSET_DEREF_ADDV */
pub const RSEQ_ARCH_HAS_OFFSET_DEREF_ADDV: bool = true;

/*
 *   pval = *(ptr+off)
 *  *pval += inc;
 */
#[cfg(target_arch = "x86_64")]
#[inline(always)]
pub unsafe fn rseq_offset_deref_addv(
    ptr: *mut intptr_t,
    off: c_long,
    inc: intptr_t,
    _cpu: c_int,
) -> c_int {
    /* RSEQ_INJECT_C(9) */
    /*
     * Start rseq by storing table entry pointer into rseq_cs.
     * Compare CPU id/cid through the template-selected rseq field.
     */
    /*
     * RSEQ_COMPARE_TWICE optionally repeats the cpu_id comparison and calls
     * rseq_bug on mismatch in the C template.
     */

    /* get p+v */
    let rbx = (ptr as *mut u8).offset(off as isize) as *mut intptr_t;
    /* get pv */
    let rcx = core::ptr::read_volatile(rbx) as *mut intptr_t;
    /* *pv += inc */
    let old = core::ptr::read_volatile(rcx);
    core::ptr::write_volatile(rcx, old.wrapping_add(inc));
    0
}

#[cfg(target_arch = "x86_64")]
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
     * Start rseq by storing table entry pointer into rseq_cs.
     * Compare CPU id/cid through the template-selected rseq field.
     */
    if core::ptr::read_volatile(v) != expect {
        return 1;
    }
    if core::ptr::read_volatile(v2) != expect2 {
        return 1;
    }
    /*
     * RSEQ_COMPARE_TWICE optionally repeats the cpu_id and both expected-value
     * comparisons and calls rseq_bug on mismatch in the C template.
     */

    /* final store */
    core::ptr::write_volatile(v, newv);
    0
}

/*
 * #endif
 *
 * Original condition:
 *
 * #if (defined(RSEQ_TEMPLATE_MO_RELAXED) || defined(RSEQ_TEMPLATE_MO_RELEASE)) &&
 *     (defined(RSEQ_TEMPLATE_CPU_ID) || defined(RSEQ_TEMPLATE_MM_CID))
 */

#[cfg(target_arch = "x86_64")]
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
     * Start rseq by storing table entry pointer into rseq_cs.
     * Compare CPU id/cid through the template-selected rseq field.
     */
    if core::ptr::read_volatile(v) != expect {
        return 1;
    }
    /*
     * RSEQ_COMPARE_TWICE optionally repeats the cpu_id and expected-value
     * comparisons and calls rseq_bug on mismatch in the C template.
     */

    /* try store */
    core::ptr::write_volatile(v2, newv2);

    /* final store */
    core::ptr::write_volatile(v, newv);
    0
}

#[cfg(target_arch = "x86_64")]
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
    let mut rseq_scratch: [u64; 3] = [0; 3];

    /* RSEQ_INJECT_C(9) */
    rseq_scratch[0] = src as u64;
    rseq_scratch[1] = dst as u64;
    rseq_scratch[2] = len as u64;

    /*
     * Start rseq by storing table entry pointer into rseq_cs.
     * Compare CPU id/cid through the template-selected rseq field.
     */
    if core::ptr::read_volatile(v) != expect {
        let _ = rseq_scratch;
        return 1;
    }
    /*
     * RSEQ_COMPARE_TWICE optionally repeats the cpu_id and expected-value
     * comparisons and calls rseq_bug on mismatch in the C template.
     */

    /* try memcpy */
    let mut dst_cur = dst as *mut u8;
    let mut src_cur = src as *const u8;
    let mut len_cur = len;
    if len_cur != 0 {
        loop {
            let al = core::ptr::read_volatile(src_cur);
            core::ptr::write_volatile(dst_cur, al);
            src_cur = src_cur.add(1);
            dst_cur = dst_cur.add(1);
            len_cur -= 1;
            if len_cur == 0 {
                break;
            }
        }
    }

    /* final store */
    core::ptr::write_volatile(v, newv);

    /* teardown restores the asm input registers in the original C template. */
    let _ = rseq_scratch;
    0
}

/*
 * #endif
 * #endif
 */
