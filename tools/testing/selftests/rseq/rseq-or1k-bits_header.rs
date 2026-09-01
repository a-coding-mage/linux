/* SPDX-License-Identifier: LGPL-2.1 OR MIT */

use core::ffi::c_void;
use core::ptr;

pub type intptr_t = isize;
pub type off_t = isize;
pub type size_t = usize;

/*
 * C dependency: #include "rseq-bits-template.h"
 *
 * The original header emits these inline functions only for selected
 * RSEQ_TEMPLATE_* preprocessor configurations and wraps each symbol in
 * RSEQ_TEMPLATE_IDENTIFIER(...).  Rust has no file-local equivalent for those
 * C template macros, so the translated items keep the base identifier names and
 * preserve the conditional intent in comments below.
 */

/*
 * Original condition:
 *   defined(RSEQ_TEMPLATE_MO_RELAXED) &&
 *   (defined(RSEQ_TEMPLATE_CPU_ID) || defined(RSEQ_TEMPLATE_MM_CID))
 */

pub unsafe fn rseq_cmpeqv_storev(
    v: *mut intptr_t,
    expect: intptr_t,
    newv: intptr_t,
    _cpu: i32,
) -> i32 {
    /*
     * Original implementation is an or1k rseq asm-goto critical section using:
     * RSEQ_INJECT_C(9)
     * RSEQ_ASM_DEFINE_TABLE(1, 2f, 3f, 4f)
     * RSEQ_ASM_DEFINE_EXIT_POINT(2f, "%l[cmpfail]")
     * RSEQ_ASM_STORE_RSEQ_CS(2, 1b, rseq_cs)
     * RSEQ_ASM_CMP_CPU_ID(cpu_id, current_cpu_id, 4f)
     * RSEQ_ASM_OP_CMPEQ(v, expect, "%l[cmpfail]")
     * RSEQ_ASM_OP_FINAL_STORE(v, newv, 3)
     * RSEQ_ASM_DEFINE_ABORT(4, abort)
     *
     * With RSEQ_COMPARE_TWICE it repeats the CPU and value comparisons and
     * calls rseq_bug on the corresponding error labels.
     */
    if ptr::read_volatile(v) != expect {
        return 1;
    }
    ptr::write_volatile(v, newv);
    0
}

pub unsafe fn rseq_cmpnev_storeoffp_load(
    v: *mut intptr_t,
    expectnot: intptr_t,
    voffp: off_t,
    load: *mut intptr_t,
    _cpu: i32,
) -> i32 {
    /*
     * Original implementation is an or1k rseq asm-goto critical section using:
     * RSEQ_INJECT_C(9)
     * RSEQ_ASM_DEFINE_TABLE(1, 2f, 3f, 4f)
     * RSEQ_ASM_DEFINE_EXIT_POINT(2f, "%l[cmpfail]")
     * RSEQ_ASM_STORE_RSEQ_CS(2, 1b, rseq_cs)
     * RSEQ_ASM_CMP_CPU_ID(cpu_id, current_cpu_id, 4f)
     * RSEQ_ASM_OP_CMPNE(v, expectnot, "%l[cmpfail]")
     * RSEQ_ASM_OP_R_LOAD(v)
     * RSEQ_ASM_OP_R_STORE(load)
     * RSEQ_ASM_OP_R_LOAD_OFF(voffp)
     * RSEQ_ASM_OP_R_FINAL_STORE(v, 3)
     * RSEQ_ASM_DEFINE_ABORT(4, abort)
     *
     * With RSEQ_COMPARE_TWICE it repeats the CPU and value comparisons and
     * calls rseq_bug on the corresponding error labels.
     */
    let value = ptr::read_volatile(v);
    if value == expectnot {
        return 1;
    }
    ptr::write_volatile(load, value);
    let new_value = ptr::read_volatile((value.wrapping_add(voffp)) as *const intptr_t);
    ptr::write_volatile(v, new_value);
    0
}

pub unsafe fn rseq_addv(v: *mut intptr_t, count: intptr_t, _cpu: i32) -> i32 {
    /*
     * Original implementation is an or1k rseq asm-goto critical section using:
     * RSEQ_INJECT_C(9)
     * RSEQ_ASM_DEFINE_TABLE(1, 2f, 3f, 4f)
     * RSEQ_ASM_STORE_RSEQ_CS(2, 1b, rseq_cs)
     * RSEQ_ASM_CMP_CPU_ID(cpu_id, current_cpu_id, 4f)
     * RSEQ_ASM_OP_R_LOAD(v)
     * RSEQ_ASM_OP_R_ADD(count)
     * RSEQ_ASM_OP_R_FINAL_STORE(v, 3)
     * RSEQ_ASM_DEFINE_ABORT(4, abort)
     *
     * With RSEQ_COMPARE_TWICE it repeats the CPU comparison and calls rseq_bug
     * on the error label.
     */
    let value = ptr::read_volatile(v);
    ptr::write_volatile(v, value.wrapping_add(count));
    0
}

pub unsafe fn rseq_cmpeqv_cmpeqv_storev(
    v: *mut intptr_t,
    expect: intptr_t,
    v2: *mut intptr_t,
    expect2: intptr_t,
    newv: intptr_t,
    _cpu: i32,
) -> i32 {
    /*
     * Original implementation is an or1k rseq asm-goto critical section using:
     * RSEQ_INJECT_C(9)
     * RSEQ_ASM_DEFINE_TABLE(1, 2f, 3f, 4f)
     * RSEQ_ASM_DEFINE_EXIT_POINT(2f, "%l[cmpfail]")
     * RSEQ_ASM_STORE_RSEQ_CS(2, 1b, rseq_cs)
     * RSEQ_ASM_CMP_CPU_ID(cpu_id, current_cpu_id, 4f)
     * RSEQ_ASM_OP_CMPEQ(v, expect, "%l[cmpfail]")
     * RSEQ_ASM_OP_CMPEQ(v2, expect2, "%l[cmpfail]")
     * RSEQ_ASM_OP_FINAL_STORE(v, newv, 3)
     * RSEQ_ASM_DEFINE_ABORT(4, abort)
     *
     * With RSEQ_COMPARE_TWICE it repeats the CPU and value comparisons and
     * calls rseq_bug on the corresponding error labels.
     */
    if ptr::read_volatile(v) != expect {
        return 1;
    }
    if ptr::read_volatile(v2) != expect2 {
        return 1;
    }
    ptr::write_volatile(v, newv);
    0
}

pub const RSEQ_ARCH_HAS_OFFSET_DEREF_ADDV: bool = true;

/*
 *   pval = *(ptr+off)
 *  *pval += inc;
 */
pub unsafe fn rseq_offset_deref_addv(
    ptr_value: *mut intptr_t,
    off: off_t,
    inc: intptr_t,
    _cpu: i32,
) -> i32 {
    /*
     * Original implementation is an or1k rseq asm-goto critical section using:
     * RSEQ_INJECT_C(9)
     * RSEQ_ASM_DEFINE_TABLE(1, 2f, 3f, 4f)
     * RSEQ_ASM_STORE_RSEQ_CS(2, 1b, rseq_cs)
     * RSEQ_ASM_CMP_CPU_ID(cpu_id, current_cpu_id, 4f)
     * RSEQ_ASM_OP_R_DEREF_ADDV(ptr, off, inc, 3)
     * RSEQ_ASM_DEFINE_ABORT(4, abort)
     *
     * With RSEQ_COMPARE_TWICE it repeats the CPU comparison and calls rseq_bug
     * on the error label.
     */
    let pval = ptr::read_volatile(ptr_value.byte_offset(off) as *const *mut intptr_t);
    let value = ptr::read_volatile(pval);
    ptr::write_volatile(pval, value.wrapping_add(inc));
    0
}

/*
 * Original condition:
 *   (defined(RSEQ_TEMPLATE_MO_RELAXED) || defined(RSEQ_TEMPLATE_MO_RELEASE)) &&
 *   (defined(RSEQ_TEMPLATE_CPU_ID) || defined(RSEQ_TEMPLATE_MM_CID))
 */

pub unsafe fn rseq_cmpeqv_trystorev_storev(
    v: *mut intptr_t,
    expect: intptr_t,
    v2: *mut intptr_t,
    newv2: intptr_t,
    newv: intptr_t,
    _cpu: i32,
) -> i32 {
    /*
     * Original implementation is an or1k rseq asm-goto critical section using:
     * RSEQ_INJECT_C(9)
     * RSEQ_ASM_DEFINE_TABLE(1, 2f, 3f, 4f)
     * RSEQ_ASM_DEFINE_EXIT_POINT(2f, "%l[cmpfail]")
     * RSEQ_ASM_STORE_RSEQ_CS(2, 1b, rseq_cs)
     * RSEQ_ASM_CMP_CPU_ID(cpu_id, current_cpu_id, 4f)
     * RSEQ_ASM_OP_CMPEQ(v, expect, "%l[cmpfail]")
     * RSEQ_ASM_OP_STORE(v2, newv2)
     * RSEQ_ASM_OP_FINAL_STORE_RELEASE(v, newv, 3) when RSEQ_TEMPLATE_MO_RELEASE
     * RSEQ_ASM_OP_FINAL_STORE(v, newv, 3) otherwise
     * RSEQ_ASM_DEFINE_ABORT(4, abort)
     *
     * With RSEQ_COMPARE_TWICE it repeats the CPU and value comparisons and
     * calls rseq_bug on the corresponding error labels.
     */
    if ptr::read_volatile(v) != expect {
        return 1;
    }
    ptr::write_volatile(v2, newv2);
    ptr::write_volatile(v, newv);
    0
}

pub unsafe fn rseq_cmpeqv_trymemcpy_storev(
    v: *mut intptr_t,
    expect: intptr_t,
    dst: *mut c_void,
    src: *mut c_void,
    len: size_t,
    newv: intptr_t,
    _cpu: i32,
) -> i32 {
    /*
     * Original implementation is an or1k rseq asm-goto critical section using:
     * RSEQ_INJECT_C(9)
     * RSEQ_ASM_DEFINE_TABLE(1, 2f, 3f, 4f)
     * RSEQ_ASM_DEFINE_EXIT_POINT(2f, "%l[cmpfail]")
     * RSEQ_ASM_STORE_RSEQ_CS(2, 1b, rseq_cs)
     * RSEQ_ASM_CMP_CPU_ID(cpu_id, current_cpu_id, 4f)
     * RSEQ_ASM_OP_CMPEQ(v, expect, "%l[cmpfail]")
     * RSEQ_ASM_OP_R_BAD_MEMCPY(dst, src, len)
     * RSEQ_ASM_OP_FINAL_STORE_RELEASE(v, newv, 3) when RSEQ_TEMPLATE_MO_RELEASE
     * RSEQ_ASM_OP_FINAL_STORE(v, newv, 3) otherwise
     * RSEQ_ASM_DEFINE_ABORT(4, abort)
     *
     * With RSEQ_COMPARE_TWICE it repeats the CPU and value comparisons and
     * calls rseq_bug on the corresponding error labels.
     */
    if ptr::read_volatile(v) != expect {
        return 1;
    }
    ptr::copy_nonoverlapping(src as *const u8, dst as *mut u8, len);
    ptr::write_volatile(v, newv);
    0
}

/*
 * C dependency: #include "rseq-bits-reset.h"
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
