// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2025 Nandakumar Edamana */

// C dependencies translated as external expectations:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_tracing.h>
// #include "bpf_misc.h"

unsafe extern "C" {
    fn bpf_get_prandom_u32() -> u32;
}

/* Intended to test the abstract multiplication technique(s) used by
 * the verifier. Using assembly to avoid compiler optimizations.
 */
#[link_section = "fentry/bpf_fentry_test1"]
#[no_mangle]
pub unsafe extern "C" fn mul_precise(x: i32) {
    let _ = x;

    /* First, force the verifier to be uncertain about the value:
     *     unsigned int a = (bpf_get_prandom_u32() & 0x2) | 0x1;
     *
     * Assuming the verifier is using tnum, a must be tnum{.v=0x1, .m=0x2}.
     * Then a * 0x3 would be m0m1 (m for uncertain). Added imprecision
     * would cause the following to fail, because the required return value
     * is 0:
     *     return (a * 0x3) & 0x4);
     */
    core::arch::asm!(
        "call {bpf_get_prandom_u32}",
        "r0 &= 0x2",
        "r0 |= 0x1",
        "r0 *= 0x3",
        "r0 &= 0x4",
        "if r0 != 0 goto l0_0",
        "r0 = 0",
        "goto l1_0",
        "l0_0:",
        "r0 = 1",
        "l1_0:",
        bpf_get_prandom_u32 = sym bpf_get_prandom_u32,
        options(nostack)
    );
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
