// SPDX-License-Identifier: GPL-2.0-only
// Dependencies in the C source:
// #include <stddef.h>
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include "bpf_misc.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use core::arch::asm;

pub const BPF_F_TEST_STATE_FREQ: u32 = 1;

#[repr(C)]
pub struct __sk_buff {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn bpf_get_prandom_u32() -> u32;
}

#[inline(never)]
pub unsafe extern "C" fn foo(v: *mut u32) -> i32 {
    if !v.is_null() {
        unsafe {
            *v = bpf_get_prandom_u32();
        }
    }

    0
}

// SEC("cgroup_skb/ingress")
// __failure __msg("At program exit the register R0 has ")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn global_func15(skb: *mut __sk_buff) -> i32 {
    let mut v: u32 = 1;

    unsafe {
        foo(&mut v as *mut u32);
    }

    v as i32
}

// SEC("cgroup_skb/ingress")
// __log_level(2) __flag(BPF_F_TEST_STATE_FREQ)
// __failure
// check that fallthrough code path marks r0 as precise
// __msg("mark_precise: frame0: regs=r0 stack= before 2: (b7) r0 = 1")
// check that branch code path marks r0 as precise
// __msg("mark_precise: frame0: regs=r0 stack= before 0: (85) call bpf_get_prandom_u32#7")
// __msg("At program exit the register R0 has ")
// __naked
#[unsafe(no_mangle)]
pub unsafe extern "C" fn global_func15_tricky_pruning() -> i32 {
    unsafe {
        asm!(
            "call {bpf_get_prandom_u32}",
            "if r0 s> 1000 goto 1f",
            "r0 = 1",
            "1:",
            "goto +0",
            // checkpoint
            // cgroup_skb/ingress program is expected to return [0, 1]
            // values, so branch above makes sure that in a fallthrough
            // case we have a valid 1 stored in R0 register, but in
            // a branch case we assign some random value to R0.  So if
            // there is something wrong with precision tracking for R0 at
            // program exit, we might erroneously prune branch case,
            // because R0 in fallthrough case is imprecise (and thus any
            // value is valid from POV of verifier is_state_equal() logic)
            "exit",
            bpf_get_prandom_u32 = sym bpf_get_prandom_u32,
            options(noreturn)
        );
    }
}
