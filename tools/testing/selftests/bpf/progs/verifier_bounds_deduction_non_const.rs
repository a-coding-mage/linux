// SPDX-License-Identifier: GPL-2.0

// C dependencies translated as external expectations:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include "bpf_misc.h"

#![no_std]

use core::arch::naked_asm;

unsafe extern "C" {
    fn bpf_ktime_get_ns() -> u64;
}

// SEC("socket")
// __description("check deducing bounds from non-const, jmp64, <non_const> == <const>, 1")
// __success __retval(0)
#[unsafe(link_section = "socket")]
#[naked]
pub unsafe extern "C" fn deducing_bounds_from_non_const_1() {
    naked_asm!(
        "call {bpf_ktime_get_ns}",
        "if r0 < 3 goto l0_1",
        "r2 = 2",
        "if r0 == r2 goto l1_1",
        "l0_1:",
        "r0 = 0",
        "exit",
        "l1_1:",
        "r0 -= r1",
        "exit",
        bpf_ktime_get_ns = sym bpf_ktime_get_ns,
    );
}

// SEC("socket")
// __description("check deducing bounds from non-const, jmp64, <non_const> == <const>, 2")
// __success __retval(0)
#[unsafe(link_section = "socket")]
#[naked]
pub unsafe extern "C" fn deducing_bounds_from_non_const_2() {
    naked_asm!(
        "call {bpf_ktime_get_ns}",
        "if r0 > 3 goto l0_2",
        "r2 = 4",
        "if r0 == r2 goto l1_2",
        "l0_2:",
        "r0 = 0",
        "exit",
        "l1_2:",
        "r0 -= r1",
        "exit",
        bpf_ktime_get_ns = sym bpf_ktime_get_ns,
    );
}

// SEC("socket")
// __description("check deducing bounds from non-const, jmp64, <non_const> != <const>, 1")
// __success __retval(0)
#[unsafe(link_section = "socket")]
#[naked]
pub unsafe extern "C" fn deducing_bounds_from_non_const_3() {
    naked_asm!(
        "call {bpf_ktime_get_ns}",
        "if r0 < 3 goto l0_3",
        "r2 = 2",
        "if r0 != r2 goto l0_3",
        "goto l1_3",
        "l0_3:",
        "r0 = 0",
        "exit",
        "l1_3:",
        "r0 -= r1",
        "exit",
        bpf_ktime_get_ns = sym bpf_ktime_get_ns,
    );
}

// SEC("socket")
// __description("check deducing bounds from non-const, jmp64, <non_const> != <const>, 2")
// __success __retval(0)
#[unsafe(link_section = "socket")]
#[naked]
pub unsafe extern "C" fn deducing_bounds_from_non_const_4() {
    naked_asm!(
        "call {bpf_ktime_get_ns}",
        "if r0 > 3 goto l0_4",
        "r2 = 4",
        "if r0 != r2 goto l0_4",
        "goto l1_4",
        "l0_4:",
        "r0 = 0",
        "exit",
        "l1_4:",
        "r0 -= r1",
        "exit",
        bpf_ktime_get_ns = sym bpf_ktime_get_ns,
    );
}

// SEC("socket")
// __description("check deducing bounds from non-const, jmp32, <non_const> == <const>, 1")
// __success __retval(0)
#[unsafe(link_section = "socket")]
#[naked]
pub unsafe extern "C" fn deducing_bounds_from_non_const_5() {
    naked_asm!(
        "call {bpf_ktime_get_ns}",
        "if w0 < 4 goto l0_5",
        "w2 = 3",
        "if w0 == w2 goto l1_5",
        "l0_5:",
        "r0 = 0",
        "exit",
        "l1_5:",
        "r0 -= r1",
        "exit",
        bpf_ktime_get_ns = sym bpf_ktime_get_ns,
    );
}

// SEC("socket")
// __description("check deducing bounds from non-const, jmp32, <non_const> == <const>, 2")
// __success __retval(0)
#[unsafe(link_section = "socket")]
#[naked]
pub unsafe extern "C" fn deducing_bounds_from_non_const_6() {
    naked_asm!(
        "call {bpf_ktime_get_ns}",
        "if w0 > 4 goto l0_6",
        "w2 = 5",
        "if w0 == w2 goto l1_6",
        "l0_6:",
        "r0 = 0",
        "exit",
        "l1_6:",
        "r0 -= r1",
        "exit",
        bpf_ktime_get_ns = sym bpf_ktime_get_ns,
    );
}

// SEC("socket")
// __description("check deducing bounds from non-const, jmp32, <non_const> != <const>, 1")
// __success __retval(0)
#[unsafe(link_section = "socket")]
#[naked]
pub unsafe extern "C" fn deducing_bounds_from_non_const_7() {
    naked_asm!(
        "call {bpf_ktime_get_ns}",
        "if w0 < 3 goto l0_7",
        "w2 = 2",
        "if w0 != w2 goto l0_7",
        "goto l1_7",
        "l0_7:",
        "r0 = 0",
        "exit",
        "l1_7:",
        "r0 -= r1",
        "exit",
        bpf_ktime_get_ns = sym bpf_ktime_get_ns,
    );
}

// SEC("socket")
// __description("check deducing bounds from non-const, jmp32, <non_const> != <const>, 2")
// __success __retval(0)
#[unsafe(link_section = "socket")]
#[naked]
pub unsafe extern "C" fn deducing_bounds_from_non_const_8() {
    naked_asm!(
        "call {bpf_ktime_get_ns}",
        "if w0 > 3 goto l0_8",
        "w2 = 4",
        "if w0 != w2 goto l0_8",
        "goto l1_8",
        "l0_8:",
        "r0 = 0",
        "exit",
        "l1_8:",
        "r0 -= r1",
        "exit",
        bpf_ktime_get_ns = sym bpf_ktime_get_ns,
    );
}

// SEC("socket")
// __description("check deducing bounds from non-const, jmp64, <const> > <non_const>, 1")
// __success __retval(0)
#[unsafe(link_section = "socket")]
#[naked]
pub unsafe extern "C" fn deducing_bounds_from_non_const_9() {
    naked_asm!(
        "call {bpf_ktime_get_ns}",
        "r2 = 0",
        "if r2 > r0 goto l0_9",
        "r0 = 0",
        "exit",
        "l0_9:",
        "r0 -= r1",
        "exit",
        bpf_ktime_get_ns = sym bpf_ktime_get_ns,
    );
}

// SEC("socket")
// __description("check deducing bounds from non-const, jmp64, <const> > <non_const>, 2")
// __success __retval(0)
#[unsafe(link_section = "socket")]
#[naked]
pub unsafe extern "C" fn deducing_bounds_from_non_const_10() {
    naked_asm!(
        "call {bpf_ktime_get_ns}",
        "if r0 < 4 goto l0_10",
        "r2 = 4",
        "if r2 > r0 goto l1_10",
        "l0_10:",
        "r0 = 0",
        "exit",
        "l1_10:",
        "r0 -= r1",
        "exit",
        bpf_ktime_get_ns = sym bpf_ktime_get_ns,
    );
}

// SEC("socket")
// __description("check deducing bounds from non-const, jmp64, <const> >= <non_const>")
// __success __retval(0)
#[unsafe(link_section = "socket")]
#[naked]
pub unsafe extern "C" fn deducing_bounds_from_non_const_11() {
    naked_asm!(
        "call {bpf_ktime_get_ns}",
        "if r0 < 4 goto l0_11",
        "r2 = 3",
        "if r2 >= r0 goto l1_11",
        "l0_11:",
        "r0 = 0",
        "exit",
        "l1_11:",
        "r0 -= r1",
        "exit",
        bpf_ktime_get_ns = sym bpf_ktime_get_ns,
    );
}

// SEC("socket")
// __description("check deducing bounds from non-const, jmp64, <const> < <non_const>")
// __success __retval(0)
#[unsafe(link_section = "socket")]
#[naked]
pub unsafe extern "C" fn deducing_bounds_from_non_const_12() {
    naked_asm!(
        "call {bpf_ktime_get_ns}",
        "if r0 > 4 goto l0_12",
        "r2 = 4",
        "if r2 < r0 goto l1_12",
        "l0_12:",
        "r0 = 0",
        "exit",
        "l1_12:",
        "r0 -= r1",
        "exit",
        bpf_ktime_get_ns = sym bpf_ktime_get_ns,
    );
}

// SEC("socket")
// __description("check deducing bounds from non-const, jmp64, <const> <= <non_const>")
// __success __retval(0)
#[unsafe(link_section = "socket")]
#[naked]
pub unsafe extern "C" fn deducing_bounds_from_non_const_13() {
    naked_asm!(
        "call {bpf_ktime_get_ns}",
        "if r0 >= 4 goto l0_13",
        "r2 = 4",
        "if r2 <= r0 goto l1_13",
        "l0_13:",
        "r0 = 0",
        "exit",
        "l1_13:",
        "r0 -= r1",
        "exit",
        bpf_ktime_get_ns = sym bpf_ktime_get_ns,
    );
}

// SEC("socket")
// __description("check deducing bounds from non-const, jmp64, <const> == <non_const>")
// __success __retval(0)
#[unsafe(link_section = "socket")]
#[naked]
pub unsafe extern "C" fn deducing_bounds_from_non_const_14() {
    naked_asm!(
        "call {bpf_ktime_get_ns}",
        "if r0 < 3 goto l0_14",
        "r2 = 2",
        "if r2 == r0 goto l1_14",
        "l0_14:",
        "r0 = 0",
        "exit",
        "l1_14:",
        "r0 -= r1",
        "exit",
        bpf_ktime_get_ns = sym bpf_ktime_get_ns,
    );
}

// SEC("socket")
// __description("check deducing bounds from non-const, jmp64, <const> s> <non_const>")
// __success __retval(0)
#[unsafe(link_section = "socket")]
#[naked]
pub unsafe extern "C" fn deducing_bounds_from_non_const_15() {
    naked_asm!(
        "call {bpf_ktime_get_ns}",
        "if r0 s< 4 goto l0_15",
        "r2 = 4",
        "if r2 s> r0 goto l1_15",
        "l0_15:",
        "r0 = 0",
        "exit",
        "l1_15:",
        "r0 -= r1",
        "exit",
        bpf_ktime_get_ns = sym bpf_ktime_get_ns,
    );
}

// SEC("socket")
// __description("check deducing bounds from non-const, jmp64, <const> s>= <non_const>")
// __success __retval(0)
#[unsafe(link_section = "socket")]
#[naked]
pub unsafe extern "C" fn deducing_bounds_from_non_const_16() {
    naked_asm!(
        "call {bpf_ktime_get_ns}",
        "if r0 s< 4 goto l0_16",
        "r2 = 3",
        "if r2 s>= r0 goto l1_16",
        "l0_16:",
        "r0 = 0",
        "exit",
        "l1_16:",
        "r0 -= r1",
        "exit",
        bpf_ktime_get_ns = sym bpf_ktime_get_ns,
    );
}

// SEC("socket")
// __description("check deducing bounds from non-const, jmp64, <const> s< <non_const>")
// __success __retval(0)
#[unsafe(link_section = "socket")]
#[naked]
pub unsafe extern "C" fn deducing_bounds_from_non_const_17() {
    naked_asm!(
        "call {bpf_ktime_get_ns}",
        "if r0 s> 4 goto l0_17",
        "r2 = 4",
        "if r2 s< r0 goto l1_17",
        "l0_17:",
        "r0 = 0",
        "exit",
        "l1_17:",
        "r0 -= r1",
        "exit",
        bpf_ktime_get_ns = sym bpf_ktime_get_ns,
    );
}

// SEC("socket")
// __description("check deducing bounds from non-const, jmp64, <const> s<= <non_const>")
// __success __retval(0)
#[unsafe(link_section = "socket")]
#[naked]
pub unsafe extern "C" fn deducing_bounds_from_non_const_18() {
    naked_asm!(
        "call {bpf_ktime_get_ns}",
        "if r0 s> 4 goto l0_18",
        "r2 = 5",
        "if r2 s<= r0 goto l1_18",
        "l0_18:",
        "r0 = 0",
        "exit",
        "l1_18:",
        "r0 -= r1",
        "exit",
        bpf_ktime_get_ns = sym bpf_ktime_get_ns,
    );
}

// SEC("socket")
// __description("check deducing bounds from non-const, jmp64, <const> != <non_const>")
// __success __retval(0)
#[unsafe(link_section = "socket")]
#[naked]
pub unsafe extern "C" fn deducing_bounds_from_non_const_19() {
    naked_asm!(
        "call {bpf_ktime_get_ns}",
        "if r0 < 3 goto l0_19",
        "r2 = 2",
        "if r2 != r0 goto l0_19",
        "goto l1_19",
        "l0_19:",
        "r0 = 0",
        "exit",
        "l1_19:",
        "r0 -= r1",
        "exit",
        bpf_ktime_get_ns = sym bpf_ktime_get_ns,
    );
}

// SEC("socket")
// __description("check deducing bounds from non-const, jmp32, <const> > <non_const>, 1")
// __success __retval(0)
#[unsafe(link_section = "socket")]
#[naked]
pub unsafe extern "C" fn deducing_bounds_from_non_const_20() {
    naked_asm!(
        "call {bpf_ktime_get_ns}",
        "w2 = 0",
        "if w2 > w0 goto l0_20",
        "r0 = 0",
        "exit",
        "l0_20:",
        "r0 -= r1",
        "exit",
        bpf_ktime_get_ns = sym bpf_ktime_get_ns,
    );
}

// SEC("socket")
// __description("check deducing bounds from non-const, jmp32, <const> > <non_const>, 2")
// __success __retval(0)
#[unsafe(link_section = "socket")]
#[naked]
pub unsafe extern "C" fn deducing_bounds_from_non_const_21() {
    naked_asm!(
        "call {bpf_ktime_get_ns}",
        "if w0 < 4 goto l0_21",
        "w2 = 4",
        "if w2 > w0 goto l1_21",
        "l0_21:",
        "r0 = 0",
        "exit",
        "l1_21:",
        "r0 -= r1",
        "exit",
        bpf_ktime_get_ns = sym bpf_ktime_get_ns,
    );
}

// SEC("socket")
// __description("check deducing bounds from non-const, jmp32, <const> >= <non_const>")
// __success __retval(0)
#[unsafe(link_section = "socket")]
#[naked]
pub unsafe extern "C" fn deducing_bounds_from_non_const_22() {
    naked_asm!(
        "call {bpf_ktime_get_ns}",
        "if w0 < 4 goto l0_22",
        "w2 = 3",
        "if w2 >= w0 goto l1_22",
        "l0_22:",
        "r0 = 0",
        "exit",
        "l1_22:",
        "r0 -= r1",
        "exit",
        bpf_ktime_get_ns = sym bpf_ktime_get_ns,
    );
}

// SEC("socket")
// __description("check deducing bounds from non-const, jmp32, <const> < <non_const>")
// __success __retval(0)
#[unsafe(link_section = "socket")]
#[naked]
pub unsafe extern "C" fn deducing_bounds_from_non_const_23() {
    naked_asm!(
        "call {bpf_ktime_get_ns}",
        "if w0 > 4 goto l0_23",
        "w2 = 4",
        "if w2 < w0 goto l1_23",
        "l0_23:",
        "r0 = 0",
        "exit",
        "l1_23:",
        "r0 -= r1",
        "exit",
        bpf_ktime_get_ns = sym bpf_ktime_get_ns,
    );
}

// SEC("socket")
// __description("check deducing bounds from non-const, jmp32, <const> <= <non_const>")
// __success __retval(0)
#[unsafe(link_section = "socket")]
#[naked]
pub unsafe extern "C" fn deducing_bounds_from_non_const_24() {
    naked_asm!(
        "call {bpf_ktime_get_ns}",
        "if w0 >= 4 goto l0_24",
        "w2 = 4",
        "if w2 <= w0 goto l1_24",
        "l0_24:",
        "r0 = 0",
        "exit",
        "l1_24:",
        "r0 -= r1",
        "exit",
        bpf_ktime_get_ns = sym bpf_ktime_get_ns,
    );
}

// SEC("socket")
// __description("check deducing bounds from non-const, jmp32, <const> == <non_const>")
// __success __retval(0)
#[unsafe(link_section = "socket")]
#[naked]
pub unsafe extern "C" fn deducing_bounds_from_non_const_25() {
    naked_asm!(
        "call {bpf_ktime_get_ns}",
        "if w0 < 4 goto l0_25",
        "w2 = 3",
        "if w2 == w0 goto l1_25",
        "l0_25:",
        "r0 = 0",
        "exit",
        "l1_25:",
        "r0 -= r1",
        "exit",
        bpf_ktime_get_ns = sym bpf_ktime_get_ns,
    );
}

// SEC("socket")
// __description("check deducing bounds from non-const, jmp32, <const> s> <non_const>")
// __success __retval(0)
#[unsafe(link_section = "socket")]
#[naked]
pub unsafe extern "C" fn deducing_bounds_from_non_const_26() {
    naked_asm!(
        "call {bpf_ktime_get_ns}",
        "if w0 s< 4 goto l0_26",
        "w2 = 4",
        "if w2 s> w0 goto l1_26",
        "l0_26:",
        "r0 = 0",
        "exit",
        "l1_26:",
        "r0 -= r1",
        "exit",
        bpf_ktime_get_ns = sym bpf_ktime_get_ns,
    );
}

// SEC("socket")
// __description("check deducing bounds from non-const, jmp32, <const> s>= <non_const>")
// __success __retval(0)
#[unsafe(link_section = "socket")]
#[naked]
pub unsafe extern "C" fn deducing_bounds_from_non_const_27() {
    naked_asm!(
        "call {bpf_ktime_get_ns}",
        "if w0 s< 4 goto l0_27",
        "w2 = 3",
        "if w2 s>= w0 goto l1_27",
        "l0_27:",
        "r0 = 0",
        "exit",
        "l1_27:",
        "r0 -= r1",
        "exit",
        bpf_ktime_get_ns = sym bpf_ktime_get_ns,
    );
}

// SEC("socket")
// __description("check deducing bounds from non-const, jmp32, <const> s< <non_const>")
// __success __retval(0)
#[unsafe(link_section = "socket")]
#[naked]
pub unsafe extern "C" fn deducing_bounds_from_non_const_28() {
    naked_asm!(
        "call {bpf_ktime_get_ns}",
        "if w0 s> 4 goto l0_28",
        "w2 = 5",
        "if w2 s< w0 goto l1_28",
        "l0_28:",
        "r0 = 0",
        "exit",
        "l1_28:",
        "r0 -= r1",
        "exit",
        bpf_ktime_get_ns = sym bpf_ktime_get_ns,
    );
}

// SEC("socket")
// __description("check deducing bounds from non-const, jmp32, <const> s<= <non_const>")
// __success __retval(0)
#[unsafe(link_section = "socket")]
#[naked]
pub unsafe extern "C" fn deducing_bounds_from_non_const_29() {
    naked_asm!(
        "call {bpf_ktime_get_ns}",
        "if w0 s>= 4 goto l0_29",
        "w2 = 4",
        "if w2 s<= w0 goto l1_29",
        "l0_29:",
        "r0 = 0",
        "exit",
        "l1_29:",
        "r0 -= r1",
        "exit",
        bpf_ktime_get_ns = sym bpf_ktime_get_ns,
    );
}

// SEC("socket")
// __description("check deducing bounds from non-const, jmp32, <const> != <non_const>")
// __success __retval(0)
#[unsafe(link_section = "socket")]
#[naked]
pub unsafe extern "C" fn deducing_bounds_from_non_const_30() {
    naked_asm!(
        "call {bpf_ktime_get_ns}",
        "if w0 < 3 goto l0_30",
        "w2 = 2",
        "if w2 != w0 goto l0_30",
        "goto l1_30",
        "l0_30:",
        "r0 = 0",
        "exit",
        "l1_30:",
        "r0 -= r1",
        "exit",
        bpf_ktime_get_ns = sym bpf_ktime_get_ns,
    );
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
