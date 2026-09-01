// SPDX-License-Identifier: GPL-2.0

// C dependencies:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include "bpf_misc.h"

#![allow(non_upper_case_globals)]

unsafe extern "C" {
    fn bpf_ktime_get_ns() -> u64;
}

// Original C condition: #ifdef CAN_USE_GOTOL
#[cfg(CAN_USE_GOTOL)]
#[unsafe(link_section = "socket")]
#[unsafe(no_mangle)]
// __description("gotol, small_imm")
// __success __success_unpriv __retval(1)
pub unsafe extern "C" fn gotol_small_imm() {
    unsafe {
        core::arch::asm!(
            "
	call {bpf_ktime_get_ns};
	if r0 == 0 goto l0_{id};
	gotol l1_{id};
l2_{id}:
	gotol l3_{id};
l1_{id}:
	r0 = 1;
	gotol l2_{id};
l0_{id}:
	r0 = 2;
l3_{id}:
	exit;
",
            bpf_ktime_get_ns = sym bpf_ktime_get_ns,
            id = const 0,
            options(noreturn)
        );
    }
}

#[cfg(CAN_USE_GOTOL)]
#[unsafe(link_section = "socket")]
#[unsafe(no_mangle)]
// __description("gotol, large_imm")
// __success __failure_unpriv __retval(40000)
pub unsafe extern "C" fn gotol_large_imm() {
    unsafe {
        core::arch::asm!(
            "
	gotol 1f;
0:
	r0 = 0;
	.rept 40000;
	r0 += 1;
	.endr;
	exit;
1:	gotol 0b;
",
            options(noreturn)
        );
    }
}

// Original C condition: #else
#[cfg(not(CAN_USE_GOTOL))]
#[unsafe(link_section = "socket")]
#[unsafe(no_mangle)]
// __description("cpuv4 is not supported by compiler or jit, use a dummy test")
// __success
pub extern "C" fn dummy_test() -> i32 {
    0
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
