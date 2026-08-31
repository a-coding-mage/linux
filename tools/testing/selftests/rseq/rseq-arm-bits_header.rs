/* SPDX-License-Identifier: LGPL-2.1 OR MIT */
/*
 * rseq-arm-bits.h
 *
 * (C) Copyright 2016-2022 - Mathieu Desnoyers <mathieu.desnoyers@efficios.com>
 */

/* Depends on the Rust equivalents of "rseq-bits-template.h". */

/*
 * C preprocessor condition:
 * defined(RSEQ_TEMPLATE_MO_RELAXED) &&
 * (defined(RSEQ_TEMPLATE_CPU_ID) || defined(RSEQ_TEMPLATE_MM_CID))
 */

#[inline(always)]
pub unsafe fn rseq_cmpeqv_storev(
    v: *mut isize,
    expect: isize,
    newv: isize,
    cpu: i32,
) -> i32 {
    RSEQ_INJECT_C(9);

    /* Start rseq by storing table entry pointer into rseq_cs. */
    if RSEQ_TEMPLATE_CPU_ID_FIELD_VALUE() != cpu {
        rseq_after_asm_goto();
        RSEQ_INJECT_FAILED();
        return -1;
    }
    RSEQ_INJECT_ASM(3);
    let r0 = core::ptr::read_volatile(v);
    if r0 != expect {
        rseq_after_asm_goto();
        return 1;
    }
    RSEQ_INJECT_ASM(4);
    /*
     * If RSEQ_COMPARE_TWICE is defined, compare cpu_id and expected value
     * again here and call rseq_bug on mismatch.
     */
    /* final store */
    core::ptr::write_volatile(v, newv);
    RSEQ_INJECT_ASM(5);
    rseq_after_asm_goto();
    0
}

#[inline(always)]
pub unsafe fn rseq_cmpnev_storeoffp_load(
    v: *mut isize,
    expectnot: isize,
    voffp: isize,
    load: *mut isize,
    cpu: i32,
) -> i32 {
    RSEQ_INJECT_C(9);

    /* Start rseq by storing table entry pointer into rseq_cs. */
    if RSEQ_TEMPLATE_CPU_ID_FIELD_VALUE() != cpu {
        rseq_after_asm_goto();
        RSEQ_INJECT_FAILED();
        return -1;
    }
    RSEQ_INJECT_ASM(3);
    let mut r0 = core::ptr::read_volatile(v);
    if r0 == expectnot {
        rseq_after_asm_goto();
        return 1;
    }
    RSEQ_INJECT_ASM(4);
    /*
     * If RSEQ_COMPARE_TWICE is defined, compare cpu_id and unexpected value
     * again here and call rseq_bug on mismatch.
     */
    core::ptr::write_volatile(load, r0);
    r0 = (r0 as *const u8).wrapping_offset(voffp) as isize;
    r0 = core::ptr::read_volatile(r0 as *const isize);
    /* final store */
    core::ptr::write_volatile(v, r0);
    RSEQ_INJECT_ASM(5);
    rseq_after_asm_goto();
    0
}

#[inline(always)]
pub unsafe fn rseq_addv(v: *mut isize, count: isize, cpu: i32) -> i32 {
    RSEQ_INJECT_C(9);

    /* Start rseq by storing table entry pointer into rseq_cs. */
    if RSEQ_TEMPLATE_CPU_ID_FIELD_VALUE() != cpu {
        rseq_after_asm_goto();
        RSEQ_INJECT_FAILED();
        return -1;
    }
    RSEQ_INJECT_ASM(3);
    /*
     * If RSEQ_COMPARE_TWICE is defined, compare cpu_id again here and call
     * rseq_bug on mismatch.
     */
    let r0 = core::ptr::read_volatile(v).wrapping_add(count);
    /* final store */
    core::ptr::write_volatile(v, r0);
    RSEQ_INJECT_ASM(4);
    rseq_after_asm_goto();
    0
}

#[inline(always)]
pub unsafe fn rseq_cmpeqv_cmpeqv_storev(
    v: *mut isize,
    expect: isize,
    v2: *mut isize,
    expect2: isize,
    newv: isize,
    cpu: i32,
) -> i32 {
    RSEQ_INJECT_C(9);

    /* Start rseq by storing table entry pointer into rseq_cs. */
    if RSEQ_TEMPLATE_CPU_ID_FIELD_VALUE() != cpu {
        rseq_after_asm_goto();
        RSEQ_INJECT_FAILED();
        return -1;
    }
    RSEQ_INJECT_ASM(3);
    let mut r0 = core::ptr::read_volatile(v);
    if r0 != expect {
        rseq_after_asm_goto();
        return 1;
    }
    RSEQ_INJECT_ASM(4);
    r0 = core::ptr::read_volatile(v2);
    if r0 != expect2 {
        rseq_after_asm_goto();
        return 1;
    }
    RSEQ_INJECT_ASM(5);
    /*
     * If RSEQ_COMPARE_TWICE is defined, compare cpu_id, first expected value,
     * and second expected value again here and call rseq_bug on mismatch.
     */
    /* final store */
    core::ptr::write_volatile(v, newv);
    RSEQ_INJECT_ASM(6);
    rseq_after_asm_goto();
    0
}

/*
 * C preprocessor condition:
 * (defined(RSEQ_TEMPLATE_MO_RELAXED) || defined(RSEQ_TEMPLATE_MO_RELEASE)) &&
 * (defined(RSEQ_TEMPLATE_CPU_ID) || defined(RSEQ_TEMPLATE_MM_CID))
 */

#[inline(always)]
pub unsafe fn rseq_cmpeqv_trystorev_storev(
    v: *mut isize,
    expect: isize,
    v2: *mut isize,
    newv2: isize,
    newv: isize,
    cpu: i32,
) -> i32 {
    RSEQ_INJECT_C(9);

    /* Start rseq by storing table entry pointer into rseq_cs. */
    if RSEQ_TEMPLATE_CPU_ID_FIELD_VALUE() != cpu {
        rseq_after_asm_goto();
        RSEQ_INJECT_FAILED();
        return -1;
    }
    RSEQ_INJECT_ASM(3);
    let r0 = core::ptr::read_volatile(v);
    if r0 != expect {
        rseq_after_asm_goto();
        return 1;
    }
    RSEQ_INJECT_ASM(4);
    /*
     * If RSEQ_COMPARE_TWICE is defined, compare cpu_id and expected value
     * again here and call rseq_bug on mismatch.
     */
    /* try store */
    core::ptr::write_volatile(v2, newv2);
    RSEQ_INJECT_ASM(5);
    /*
     * If RSEQ_TEMPLATE_MO_RELEASE is defined, issue a full memory barrier
     * here; the ARM implementation uses "dmb" to provide store-release.
     */
    /* final store */
    core::ptr::write_volatile(v, newv);
    RSEQ_INJECT_ASM(6);
    rseq_after_asm_goto();
    0
}

#[inline(always)]
pub unsafe fn rseq_cmpeqv_trymemcpy_storev(
    v: *mut isize,
    expect: isize,
    dst: *mut core::ffi::c_void,
    src: *mut core::ffi::c_void,
    len: usize,
    newv: isize,
    cpu: i32,
) -> i32 {
    let mut rseq_scratch: [u32; 3] = [0; 3];

    RSEQ_INJECT_C(9);

    rseq_scratch[0] = src as u32;
    rseq_scratch[1] = dst as u32;
    rseq_scratch[2] = len as u32;

    /* Start rseq by storing table entry pointer into rseq_cs. */
    if RSEQ_TEMPLATE_CPU_ID_FIELD_VALUE() != cpu {
        let _ = rseq_scratch;
        rseq_after_asm_goto();
        RSEQ_INJECT_FAILED();
        return -1;
    }
    RSEQ_INJECT_ASM(3);
    let r0 = core::ptr::read_volatile(v);
    if r0 != expect {
        let _ = rseq_scratch;
        rseq_after_asm_goto();
        return 1;
    }
    RSEQ_INJECT_ASM(4);
    /*
     * If RSEQ_COMPARE_TWICE is defined, compare cpu_id and expected value
     * again here and call rseq_bug on mismatch.
     */
    /* try memcpy */
    let mut dst_iter = dst as *mut u8;
    let mut src_iter = src as *const u8;
    let mut len_iter = len;
    while len_iter != 0 {
        let byte = core::ptr::read_volatile(src_iter);
        core::ptr::write_volatile(dst_iter, byte);
        src_iter = src_iter.add(1);
        dst_iter = dst_iter.add(1);
        len_iter = len_iter.wrapping_sub(1);
    }
    RSEQ_INJECT_ASM(5);
    /*
     * If RSEQ_TEMPLATE_MO_RELEASE is defined, issue a full memory barrier
     * here; the ARM implementation uses "dmb" to provide store-release.
     */
    /* final store */
    core::ptr::write_volatile(v, newv);
    RSEQ_INJECT_ASM(6);
    let _ = rseq_scratch;
    rseq_after_asm_goto();
    0
}

/* Depends on the Rust equivalents of "rseq-bits-reset.h". */
