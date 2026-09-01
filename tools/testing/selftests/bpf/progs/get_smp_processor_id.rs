// SPDX-License-Identifier: GPL-2.0

// Dependencies from the original C source:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include "bpf_misc.h"

#![allow(non_upper_case_globals)]

type __u64 = u64;

unsafe extern "C" {
    static bpf_get_smp_processor_id: i32;
}

#[unsafe(no_mangle)]
pub static mut cpu_nr_result: __u64 = 0;

#[unsafe(no_mangle)]
#[unsafe(link_section = "raw_tp")]
pub unsafe extern "C" fn call_bpf_get_smp_processor_id() {
    let mut r0: __u64 = (-1i32) as __u64;

    unsafe {
        core::arch::asm!(
            "call {helper}",
            helper = sym bpf_get_smp_processor_id,
            inout("r0") r0,
            lateout("r1") _,
            lateout("r2") _,
            lateout("r3") _,
            lateout("r4") _,
            lateout("r5") _,
            options(nostack),
        );
        cpu_nr_result = r0;
    }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "license")]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
