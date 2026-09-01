// SPDX-License-Identifier: GPL-2.0

// C includes translated as dependency intent:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include "bpf_misc.h"

use core::arch::naked_asm;

#[unsafe(link_section = "netfilter")]
#[unsafe(no_mangle)]
// __description("bpf_exit with invalid return code. test1")
// __failure __msg("R0 is not a known value")
#[naked]
pub unsafe extern "C" fn with_invalid_return_code_test1() {
    naked_asm!(
        "r0 = *(u64*)(r1 + 0);
         exit;"
    );
}

#[unsafe(link_section = "netfilter")]
#[unsafe(no_mangle)]
// __description("bpf_exit with valid return code. test2")
// __success
#[naked]
pub unsafe extern "C" fn with_valid_return_code_test2() {
    naked_asm!(
        "r0 = 0;
         exit;"
    );
}

#[unsafe(link_section = "netfilter")]
#[unsafe(no_mangle)]
// __description("bpf_exit with valid return code. test3")
// __success
#[naked]
pub unsafe extern "C" fn with_valid_return_code_test3() {
    naked_asm!(
        "r0 = 1;
         exit;"
    );
}

#[unsafe(link_section = "netfilter")]
#[unsafe(no_mangle)]
// __description("bpf_exit with invalid return code. test4")
// __failure __msg("R0 has smin=2 smax=2 should have been in [0, 1]")
#[naked]
pub unsafe extern "C" fn with_invalid_return_code_test4() {
    naked_asm!(
        "r0 = 2;
         exit;"
    );
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
