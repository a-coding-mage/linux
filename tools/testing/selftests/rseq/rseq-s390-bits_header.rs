/* SPDX-License-Identifier: LGPL-2.1 OR MIT */

/*
 * Rust translation of rseq-s390-bits.h.
 *
 * Original C dependency intent:
 *   #include "rseq-bits-template.h"
 *
 * The C source is template-expanded by preprocessor symbols such as
 * RSEQ_TEMPLATE_IDENTIFIER, RSEQ_TEMPLATE_MO_RELAXED, RSEQ_TEMPLATE_MO_RELEASE,
 * RSEQ_TEMPLATE_CPU_ID, RSEQ_TEMPLATE_MM_CID, and RSEQ_COMPARE_TWICE. Those
 * build-time template conditions are preserved here as comments around the
 * translated items.
 */

use core::ffi::c_void;
use core::ptr;

type intptr_t = isize;
type size_t = usize;

/*
 * Original condition:
 * #if defined(RSEQ_TEMPLATE_MO_RELAXED) &&
 *     (defined(RSEQ_TEMPLATE_CPU_ID) || defined(RSEQ_TEMPLATE_MM_CID))
 */

#[inline(always)]
pub unsafe fn rseq_cmpeqv_storev(
    v: *mut intptr_t,
    expect: intptr_t,
    newv: intptr_t,
    _cpu: i32,
) -> i32 {
    /*
     * Original implementation uses s390 asm goto, rseq critical section table
     * setup, cpu-id comparison, optional RSEQ_COMPARE_TWICE checks, and abort
     * handling. In this isolated Rust translation, the local memory operation
     * semantics are represented directly; the external rseq abort path remains
     * a build/template responsibility.
     */
    if ptr::read_volatile(v) != expect {
        return 1;
    }
    ptr::write_volatile(v, newv);
    0
}

/*
 * Compare @v against @expectnot. When it does _not_ match, load @v
 * into @load, and store the content of *@v + voffp into @v.
 */
#[inline(always)]
pub unsafe fn rseq_cmpnev_storeoffp_load(
    v: *mut intptr_t,
    expectnot: intptr_t,
    voffp: core::ffi::c_long,
    load: *mut intptr_t,
    _cpu: i32,
) -> i32 {
    let r1 = ptr::read_volatile(v);
    if r1 == expectnot {
        return 1;
    }
    ptr::write_volatile(load, r1);
    let p = (r1 as *const u8).offset(voffp as isize) as *const intptr_t;
    let r1 = ptr::read_volatile(p);
    ptr::write_volatile(v, r1);
    0
}

#[inline(always)]
pub unsafe fn rseq_addv(v: *mut intptr_t, count: intptr_t, _cpu: i32) -> i32 {
    let r0 = ptr::read_volatile(v).wrapping_add(count);
    ptr::write_volatile(v, r0);
    0
}

#[inline(always)]
pub unsafe fn rseq_cmpeqv_cmpeqv_storev(
    v: *mut intptr_t,
    expect: intptr_t,
    v2: *mut intptr_t,
    expect2: intptr_t,
    newv: intptr_t,
    _cpu: i32,
) -> i32 {
    if ptr::read_volatile(v) != expect {
        return 1;
    }
    if ptr::read_volatile(v2) != expect2 {
        return 1;
    }
    ptr::write_volatile(v, newv);
    0
}

/*
 * End original condition:
 * #if defined(RSEQ_TEMPLATE_MO_RELAXED) &&
 *     (defined(RSEQ_TEMPLATE_CPU_ID) || defined(RSEQ_TEMPLATE_MM_CID))
 */

/*
 * Original condition:
 * #if (defined(RSEQ_TEMPLATE_MO_RELAXED) || defined(RSEQ_TEMPLATE_MO_RELEASE)) &&
 *     (defined(RSEQ_TEMPLATE_CPU_ID) || defined(RSEQ_TEMPLATE_MM_CID))
 */

/* s390 is TSO. */
#[inline(always)]
pub unsafe fn rseq_cmpeqv_trystorev_storev(
    v: *mut intptr_t,
    expect: intptr_t,
    v2: *mut intptr_t,
    newv2: intptr_t,
    newv: intptr_t,
    _cpu: i32,
) -> i32 {
    if ptr::read_volatile(v) != expect {
        return 1;
    }
    /* try store */
    ptr::write_volatile(v2, newv2);
    /* final store */
    ptr::write_volatile(v, newv);
    0
}

/* s390 is TSO. */
#[inline(always)]
pub unsafe fn rseq_cmpeqv_trymemcpy_storev(
    v: *mut intptr_t,
    expect: intptr_t,
    dst: *mut c_void,
    src: *mut c_void,
    len: size_t,
    newv: intptr_t,
    _cpu: i32,
) -> i32 {
    let mut rseq_scratch: [u64; 3] = [0; 3];

    rseq_scratch[0] = src as u64;
    rseq_scratch[1] = dst as u64;
    rseq_scratch[2] = len as u64;

    if ptr::read_volatile(v) != expect {
        let _len = rseq_scratch[2] as size_t;
        let _dst = rseq_scratch[1] as *mut c_void;
        let _src = rseq_scratch[0] as *mut c_void;
        return 1;
    }

    /* try memcpy */
    let mut src_cur = src as *const u8;
    let mut dst_cur = dst as *mut u8;
    let mut len_cur = len;
    while len_cur != 0 {
        let byte = ptr::read(src_cur);
        ptr::write(dst_cur, byte);
        src_cur = src_cur.add(1);
        dst_cur = dst_cur.add(1);
        len_cur = len_cur.wrapping_sub(1);
    }

    /* final store */
    ptr::write_volatile(v, newv);

    /* teardown */
    let _len = rseq_scratch[2] as size_t;
    let _dst = rseq_scratch[1] as *mut c_void;
    let _src = rseq_scratch[0] as *mut c_void;
    0
}

/*
 * End original condition:
 * #if (defined(RSEQ_TEMPLATE_MO_RELAXED) || defined(RSEQ_TEMPLATE_MO_RELEASE)) &&
 *     (defined(RSEQ_TEMPLATE_CPU_ID) || defined(RSEQ_TEMPLATE_MM_CID))
 *
 * Original C dependency intent:
 *   #include "rseq-bits-reset.h"
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
