/* SPDX-License-Identifier: LGPL-2.1 OR MIT */
/*
 * Author: Paul Burton <paul.burton@mips.com>
 * (C) Copyright 2018 MIPS Tech LLC
 * (C) Copyright 2016-2022 - Mathieu Desnoyers <mathieu.desnoyers@efficios.com>
 */

/*
 * Source dependency: "rseq-bits-template.h".
 *
 * The C header emits these inline helpers only for selected template
 * expansions:
 *
 * #if defined(RSEQ_TEMPLATE_MO_RELAXED) &&
 *     (defined(RSEQ_TEMPLATE_CPU_ID) || defined(RSEQ_TEMPLATE_MM_CID))
 *
 * and:
 *
 * #if (defined(RSEQ_TEMPLATE_MO_RELAXED) || defined(RSEQ_TEMPLATE_MO_RELEASE)) &&
 *     (defined(RSEQ_TEMPLATE_CPU_ID) || defined(RSEQ_TEMPLATE_MM_CID))
 *
 * RSEQ_TEMPLATE_IDENTIFIER(...), RSEQ_INJECT_*, RSEQ_ASM_*, LONG_L, LONG_S,
 * LONG_ADDI, RSEQ_COMPARE_TWICE, and RSEQ_TEMPLATE_MO_RELEASE are supplied by
 * the surrounding rseq template machinery and MIPS asm support. Rust has no
 * direct stable file-local equivalent for GCC asm goto with these C preprocessor
 * macros, so the helpers below preserve the C control-flow result contract with
 * raw pointers and volatile accesses.
 */

pub type intptr_t = isize;
pub type uintptr_t = usize;
pub type size_t = usize;

unsafe extern "C" {
    fn rseq_bug(msg: *const core::ffi::c_char) -> !;
}

/* Start of:
 * #if defined(RSEQ_TEMPLATE_MO_RELAXED) &&
 *     (defined(RSEQ_TEMPLATE_CPU_ID) || defined(RSEQ_TEMPLATE_MM_CID))
 */

#[inline(always)]
pub unsafe fn rseq_cmpeqv_storev(
    v: *mut intptr_t,
    expect: intptr_t,
    newv: intptr_t,
    cpu: core::ffi::c_int,
) -> core::ffi::c_int {
    let _ = cpu;

    /* RSEQ_INJECT_C(9) */
    /* Start rseq by storing table entry pointer into rseq_cs. */
    /* RSEQ_ASM_CMP_CPU_ID(cpu_id, current_cpu_id, abort) */
    let oldv = unsafe { core::ptr::read_volatile(v) };
    if oldv != expect {
        return 1;
    }

    /* RSEQ_COMPARE_TWICE would re-check cpu_id and *v here and call rseq_bug
     * on mismatch.
     */

    /* final store */
    unsafe { core::ptr::write_volatile(v, newv) };
    0
}

#[inline(always)]
pub unsafe fn rseq_cmpnev_storeoffp_load(
    v: *mut intptr_t,
    expectnot: intptr_t,
    voffp: core::ffi::c_long,
    load: *mut intptr_t,
    cpu: core::ffi::c_int,
) -> core::ffi::c_int {
    let _ = cpu;

    /* RSEQ_INJECT_C(9) */
    /* Start rseq by storing table entry pointer into rseq_cs. */
    /* RSEQ_ASM_CMP_CPU_ID(cpu_id, current_cpu_id, abort) */
    let oldv = unsafe { core::ptr::read_volatile(v) };
    if oldv == expectnot {
        return 1;
    }

    /* RSEQ_COMPARE_TWICE would re-check cpu_id and *v here and call rseq_bug
     * on mismatch.
     */

    unsafe { core::ptr::write_volatile(load, oldv) };
    let ptr = oldv.wrapping_add(voffp as intptr_t) as *const intptr_t;
    let new_value = unsafe { core::ptr::read_volatile(ptr) };

    /* final store */
    unsafe { core::ptr::write_volatile(v, new_value) };
    0
}

#[inline(always)]
pub unsafe fn rseq_addv(
    v: *mut intptr_t,
    count: intptr_t,
    cpu: core::ffi::c_int,
) -> core::ffi::c_int {
    let _ = cpu;

    /* RSEQ_INJECT_C(9) */
    /* Start rseq by storing table entry pointer into rseq_cs. */
    /* RSEQ_ASM_CMP_CPU_ID(cpu_id, current_cpu_id, abort) */
    /* RSEQ_COMPARE_TWICE would re-check cpu_id here and call rseq_bug on
     * mismatch.
     */
    let oldv = unsafe { core::ptr::read_volatile(v) };
    let new_value = oldv.wrapping_add(count);

    /* final store */
    unsafe { core::ptr::write_volatile(v, new_value) };
    0
}

#[inline(always)]
pub unsafe fn rseq_cmpeqv_cmpeqv_storev(
    v: *mut intptr_t,
    expect: intptr_t,
    v2: *mut intptr_t,
    expect2: intptr_t,
    newv: intptr_t,
    cpu: core::ffi::c_int,
) -> core::ffi::c_int {
    let _ = cpu;

    /* RSEQ_INJECT_C(9) */
    /* Start rseq by storing table entry pointer into rseq_cs. */
    /* RSEQ_ASM_CMP_CPU_ID(cpu_id, current_cpu_id, abort) */
    let oldv = unsafe { core::ptr::read_volatile(v) };
    if oldv != expect {
        return 1;
    }

    let oldv2 = unsafe { core::ptr::read_volatile(v2) };
    if oldv2 != expect2 {
        return 1;
    }

    /* RSEQ_COMPARE_TWICE would re-check cpu_id, *v, and *v2 here and call
     * rseq_bug on mismatch.
     */

    /* final store */
    unsafe { core::ptr::write_volatile(v, newv) };
    0
}

/* End of first template condition. */

/* Start of:
 * #if (defined(RSEQ_TEMPLATE_MO_RELAXED) || defined(RSEQ_TEMPLATE_MO_RELEASE)) &&
 *     (defined(RSEQ_TEMPLATE_CPU_ID) || defined(RSEQ_TEMPLATE_MM_CID))
 */

#[inline(always)]
pub unsafe fn rseq_cmpeqv_trystorev_storev(
    v: *mut intptr_t,
    expect: intptr_t,
    v2: *mut intptr_t,
    newv2: intptr_t,
    newv: intptr_t,
    cpu: core::ffi::c_int,
) -> core::ffi::c_int {
    let _ = cpu;

    /* RSEQ_INJECT_C(9) */
    /* Start rseq by storing table entry pointer into rseq_cs. */
    /* RSEQ_ASM_CMP_CPU_ID(cpu_id, current_cpu_id, abort) */
    let oldv = unsafe { core::ptr::read_volatile(v) };
    if oldv != expect {
        return 1;
    }

    /* RSEQ_COMPARE_TWICE would re-check cpu_id and *v here and call rseq_bug
     * on mismatch.
     */

    /* try store */
    unsafe { core::ptr::write_volatile(v2, newv2) };

    /* RSEQ_TEMPLATE_MO_RELEASE: sync; full sync provides store-release. */
    core::sync::atomic::fence(core::sync::atomic::Ordering::Release);

    /* final store */
    unsafe { core::ptr::write_volatile(v, newv) };
    0
}

#[inline(always)]
pub unsafe fn rseq_cmpeqv_trymemcpy_storev(
    v: *mut intptr_t,
    expect: intptr_t,
    dst: *mut core::ffi::c_void,
    src: *mut core::ffi::c_void,
    len: size_t,
    newv: intptr_t,
    cpu: core::ffi::c_int,
) -> core::ffi::c_int {
    let _ = cpu;
    let mut rseq_scratch: [uintptr_t; 3] = [0; 3];
    let mut dst_cur = dst as *mut u8;
    let mut src_cur = src as *mut u8;
    let mut len_cur = len;

    /* RSEQ_INJECT_C(9) */
    rseq_scratch[0] = src_cur as uintptr_t;
    rseq_scratch[1] = dst_cur as uintptr_t;
    rseq_scratch[2] = len_cur as uintptr_t;

    /* Start rseq by storing table entry pointer into rseq_cs. */
    /* RSEQ_ASM_CMP_CPU_ID(cpu_id, current_cpu_id, abort) */
    let oldv = unsafe { core::ptr::read_volatile(v) };
    if oldv != expect {
        len_cur = rseq_scratch[2] as size_t;
        dst_cur = rseq_scratch[1] as *mut u8;
        src_cur = rseq_scratch[0] as *mut u8;
        let _ = (len_cur, dst_cur, src_cur);
        return 1;
    }

    /* RSEQ_COMPARE_TWICE would re-check cpu_id and *v here and call rseq_bug
     * on mismatch after restoring src, dst, and len from rseq_scratch.
     */

    /* try memcpy */
    while len_cur != 0 {
        let byte = unsafe { core::ptr::read_volatile(src_cur) };
        unsafe { core::ptr::write_volatile(dst_cur, byte) };
        src_cur = unsafe { src_cur.add(1) };
        dst_cur = unsafe { dst_cur.add(1) };
        len_cur = len_cur.wrapping_sub(1);
    }

    /* RSEQ_TEMPLATE_MO_RELEASE: sync; full sync provides store-release. */
    core::sync::atomic::fence(core::sync::atomic::Ordering::Release);

    /* final store */
    unsafe { core::ptr::write_volatile(v, newv) };

    /* teardown */
    len_cur = rseq_scratch[2] as size_t;
    dst_cur = rseq_scratch[1] as *mut u8;
    src_cur = rseq_scratch[0] as *mut u8;
    let _ = (len_cur, dst_cur, src_cur);
    0
}

/* End of second template condition. */

/* Source dependency: "rseq-bits-reset.h". */
