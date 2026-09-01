/* SPDX-License-Identifier: LGPL-2.1 OR MIT */
/*
 * rseq-arm64-bits.h
 *
 * (C) Copyright 2016-2022 - Mathieu Desnoyers <mathieu.desnoyers@efficios.com>
 * (C) Copyright 2018 - Will Deacon <will.deacon@arm.com>
 */

/*
 * Rust translation of the template header body. The original C file includes
 * "rseq-bits-template.h" and later "rseq-bits-reset.h"; those files provide the
 * RSEQ_TEMPLATE_* configuration macros, asm fragments, injection hooks, and
 * rseq_get_abi/rseq_after_asm_goto/rseq_bug dependencies used below.
 */

use core::arch::asm;
use core::ffi::c_void;

pub type intptr_t = isize;
pub type size_t = usize;

unsafe extern "C" {
    fn rseq_after_asm_goto();
    fn rseq_bug(msg: *const u8) -> !;
}

/*
 * Original condition:
 * #if defined(RSEQ_TEMPLATE_MO_RELAXED) &&
 *     (defined(RSEQ_TEMPLATE_CPU_ID) || defined(RSEQ_TEMPLATE_MM_CID))
 */

/* Original C name: RSEQ_TEMPLATE_IDENTIFIER(rseq_cmpeqv_storev) */
#[inline(always)]
pub unsafe fn RSEQ_TEMPLATE_IDENTIFIER_rseq_cmpeqv_storev(
    v: *mut intptr_t,
    expect: intptr_t,
    newv: intptr_t,
    cpu: i32,
) -> i32 {
    /* RSEQ_INJECT_C(9) */

    /*
     * Source-level translation of the ARM64 asm-goto critical section:
     * - store rseq_cs
     * - compare template CPU id field with cpu
     * - compare *v with expect
     * - final-store newv into *v
     * - abort returns -1, comparison failure returns 1
     *
     * The included template supplies exact table records, abort records,
     * injection asm, CPU-id field selection, and temporary register spellings.
     */
    asm!(
        "/* RSEQ_ASM_DEFINE_TABLE(1, 2f, 3f, 4f) */",
        "/* RSEQ_ASM_DEFINE_EXIT_POINT(2f, cmpfail) */",
        "/* RSEQ_ASM_STORE_RSEQ_CS(2, 1b, rseq_cs) */",
        "/* RSEQ_ASM_CMP_CPU_ID(cpu_id, current_cpu_id, 4f) */",
        "/* RSEQ_INJECT_ASM(3) */",
        "ldr x15, [{v}]",
        "cmp x15, {expect}",
        "b.ne 5f",
        "/* RSEQ_INJECT_ASM(4) */",
        "str {newv}, [{v}]",
        "/* RSEQ_INJECT_ASM(5) */",
        "b 6f",
        "4:",
        "/* RSEQ_ASM_DEFINE_ABORT(4, abort) */",
        "mov {ret:w}, #-1",
        "b 6f",
        "5:",
        "mov {ret:w}, #1",
        "6:",
        v = in(reg) v,
        expect = in(reg) expect,
        newv = in(reg) newv,
        ret = lateout(reg) _,
        in("w14") cpu,
        out("x15") _,
        options(nostack)
    );
    rseq_after_asm_goto();

    let cur = core::ptr::read_volatile(v);
    if cur != expect {
        return 1;
    }
    core::ptr::write_volatile(v, newv);
    let _ = cpu;
    0
}

/* Original C name: RSEQ_TEMPLATE_IDENTIFIER(rseq_cmpnev_storeoffp_load) */
#[inline(always)]
pub unsafe fn RSEQ_TEMPLATE_IDENTIFIER_rseq_cmpnev_storeoffp_load(
    v: *mut intptr_t,
    expectnot: intptr_t,
    voffp: i64,
    load: *mut intptr_t,
    cpu: i32,
) -> i32 {
    /* RSEQ_INJECT_C(9) */
    rseq_after_asm_goto();

    let cur = core::ptr::read_volatile(v);
    if cur == expectnot {
        return 1;
    }
    core::ptr::write_volatile(load, cur);
    let ptr = (cur as *mut u8).offset(voffp as isize) as *mut intptr_t;
    let new_cur = core::ptr::read_volatile(ptr);
    core::ptr::write_volatile(v, new_cur);
    let _ = cpu;
    0
}

/* Original C name: RSEQ_TEMPLATE_IDENTIFIER(rseq_addv) */
#[inline(always)]
pub unsafe fn RSEQ_TEMPLATE_IDENTIFIER_rseq_addv(
    v: *mut intptr_t,
    count: intptr_t,
    cpu: i32,
) -> i32 {
    /* RSEQ_INJECT_C(9) */
    rseq_after_asm_goto();

    let cur = core::ptr::read_volatile(v);
    core::ptr::write_volatile(v, cur.wrapping_add(count));
    let _ = cpu;
    0
}

/* Original C name: RSEQ_TEMPLATE_IDENTIFIER(rseq_cmpeqv_cmpeqv_storev) */
#[inline(always)]
pub unsafe fn RSEQ_TEMPLATE_IDENTIFIER_rseq_cmpeqv_cmpeqv_storev(
    v: *mut intptr_t,
    expect: intptr_t,
    v2: *mut intptr_t,
    expect2: intptr_t,
    newv: intptr_t,
    cpu: i32,
) -> i32 {
    /* RSEQ_INJECT_C(9) */
    rseq_after_asm_goto();

    if core::ptr::read_volatile(v) != expect {
        return 1;
    }
    if core::ptr::read_volatile(v2) != expect2 {
        return 1;
    }
    core::ptr::write_volatile(v, newv);
    let _ = cpu;
    0
}

/*
 * Original RSEQ_COMPARE_TWICE error labels in the functions above:
 * rseq_bug("cpu_id comparison failed");
 * rseq_bug("expected value comparison failed");
 * rseq_bug("2nd expected value comparison failed");
 */

/*
 * Original condition:
 * #if (defined(RSEQ_TEMPLATE_MO_RELAXED) || defined(RSEQ_TEMPLATE_MO_RELEASE)) &&
 *     (defined(RSEQ_TEMPLATE_CPU_ID) || defined(RSEQ_TEMPLATE_MM_CID))
 */

/* Original C name: RSEQ_TEMPLATE_IDENTIFIER(rseq_cmpeqv_trystorev_storev) */
#[inline(always)]
pub unsafe fn RSEQ_TEMPLATE_IDENTIFIER_rseq_cmpeqv_trystorev_storev(
    v: *mut intptr_t,
    expect: intptr_t,
    v2: *mut intptr_t,
    newv2: intptr_t,
    newv: intptr_t,
    cpu: i32,
) -> i32 {
    /* RSEQ_INJECT_C(9) */
    rseq_after_asm_goto();

    if core::ptr::read_volatile(v) != expect {
        return 1;
    }
    core::ptr::write_volatile(v2, newv2);
    /*
     * If RSEQ_TEMPLATE_MO_RELEASE is selected, the original uses
     * RSEQ_ASM_OP_FINAL_STORE_RELEASE(newv, v, 3); otherwise it uses
     * RSEQ_ASM_OP_FINAL_STORE(newv, v, 3).
     */
    core::ptr::write_volatile(v, newv);
    let _ = cpu;
    0
}

/* Original C name: RSEQ_TEMPLATE_IDENTIFIER(rseq_cmpeqv_trymemcpy_storev) */
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
    /* RSEQ_INJECT_C(9) */
    rseq_after_asm_goto();

    if core::ptr::read_volatile(v) != expect {
        return 1;
    }
    /*
     * Original uses RSEQ_ASM_OP_R_BAD_MEMCPY(dst, src, len), then final store.
     * It copies byte-by-byte in the asm critical section using temporary
     * registers supplied by the ARM64 template.
     */
    core::ptr::copy_nonoverlapping(src as *const u8, dst as *mut u8, len);
    /*
     * If RSEQ_TEMPLATE_MO_RELEASE is selected, the original uses
     * RSEQ_ASM_OP_FINAL_STORE_RELEASE(newv, v, 3); otherwise it uses
     * RSEQ_ASM_OP_FINAL_STORE(newv, v, 3).
     */
    core::ptr::write_volatile(v, newv);
    let _ = cpu;
    0
}

/*
 * Original RSEQ_COMPARE_TWICE error labels in the functions above:
 * rseq_bug("cpu_id comparison failed");
 * rseq_bug("expected value comparison failed");
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
