// SPDX-License-Identifier: GPL-2.0
/* Converted from tools/testing/selftests/bpf/verifier/div_overflow.c */

/* Original C dependencies:
 * #include <linux/bpf.h>
 * #include <bpf/bpf_helpers.h>
 * #include <limits.h>
 * #include "bpf_misc.h"
 */

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use core::arch::asm;

const INT_MIN_: i32 = i32::MIN;
const LLONG_MIN_: i64 = i64::MIN;
const _INT_MIN: i32 = i32::MIN;

/* Just make sure that JITs used udiv/umod as otherwise we get
 * an exception from INT_MIN/-1 overflow similarly as with div
 * by zero.
 */

#[link_section = "tc"]
#[no_mangle]
/* __description("DIV32 overflow, check 1") */
/* __success __retval(0) */
pub unsafe extern "C" fn div32_overflow_check_1() {
    asm!(
        "w1 = -1",
        "w0 = {int_min}",
        "w0 /= w1",
        "exit",
        int_min = const INT_MIN_,
        options(noreturn)
    );
}

#[link_section = "tc"]
#[no_mangle]
/* __description("DIV32 overflow, check 2") */
/* __success __retval(0) */
pub unsafe extern "C" fn div32_overflow_check_2() {
    asm!(
        "w0 = {int_min}",
        "w0 /= -1",
        "exit",
        int_min = const INT_MIN_,
        options(noreturn)
    );
}

#[link_section = "tc"]
#[no_mangle]
/* __description("DIV64 overflow, check 1") */
/* __success __retval(0) */
pub unsafe extern "C" fn div64_overflow_check_1() {
    asm!(
        "r1 = -1",
        "r2 = {llong_min} ll",
        "r2 /= r1",
        "w0 = 0",
        "if r0 == r2 goto 1f",
        "w0 = 1",
        "1:",
        "exit",
        llong_min = const LLONG_MIN_,
        options(noreturn)
    );
}

#[link_section = "tc"]
#[no_mangle]
/* __description("DIV64 overflow, check 2") */
/* __success __retval(0) */
pub unsafe extern "C" fn div64_overflow_check_2() {
    asm!(
        "r1 = {llong_min} ll",
        "r1 /= -1",
        "w0 = 0",
        "if r0 == r1 goto 1f",
        "w0 = 1",
        "1:",
        "exit",
        llong_min = const LLONG_MIN_,
        options(noreturn)
    );
}

#[link_section = "tc"]
#[no_mangle]
/* __description("MOD32 overflow, check 1") */
/* __success __retval(_INT_MIN) */
pub unsafe extern "C" fn mod32_overflow_check_1() {
    asm!(
        "w1 = -1",
        "w0 = {int_min}",
        "w0 %= w1",
        "exit",
        int_min = const INT_MIN_,
        options(noreturn)
    );
}

#[link_section = "tc"]
#[no_mangle]
/* __description("MOD32 overflow, check 2") */
/* __success __retval(_INT_MIN) */
pub unsafe extern "C" fn mod32_overflow_check_2() {
    asm!(
        "w0 = {int_min}",
        "w0 %= -1",
        "exit",
        int_min = const INT_MIN_,
        options(noreturn)
    );
}

#[link_section = "tc"]
#[no_mangle]
/* __description("MOD64 overflow, check 1") */
/* __success __retval(1) */
pub unsafe extern "C" fn mod64_overflow_check_1() {
    asm!(
        "r1 = -1",
        "r2 = {llong_min} ll",
        "r3 = r2",
        "r2 %= r1",
        "w0 = 0",
        "if r3 != r2 goto 1f",
        "w0 = 1",
        "1:",
        "exit",
        llong_min = const LLONG_MIN_,
        options(noreturn)
    );
}

#[link_section = "tc"]
#[no_mangle]
/* __description("MOD64 overflow, check 2") */
/* __success __retval(1) */
pub unsafe extern "C" fn mod64_overflow_check_2() {
    asm!(
        "r2 = {llong_min} ll",
        "r3 = r2",
        "r2 %= -1",
        "w0 = 0",
        "if r3 != r2 goto 1f",
        "w0 = 1",
        "1:",
        "exit",
        llong_min = const LLONG_MIN_,
        options(noreturn)
    );
}

#[link_section = "license"]
#[no_mangle]
pub static _license: [u8; 4] = *b"GPL\0";
