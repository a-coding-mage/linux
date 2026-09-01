// SPDX-License-Identifier: GPL-2.0

// C dependencies:
// #include <vmlinux.h>
// #include <bpf/bpf_helpers.h>
// #include "bpf_misc.h"

#[unsafe(link_section = "iter/bpf_map_elem")]
// __description("PTR_TO_BUF: reject negative const offset")
// __failure
// __msg("invalid negative rdwr buffer offset")
// __naked
pub unsafe extern "C" fn ptr_to_buf_reject_negative_const_offset() {
    core::arch::asm!(
        "r0 = 0",
        "r2 = *(u64 *)(r1 + {value_off})",
        "if r2 == 0 goto 0f",
        "r2 += -8",
        "r0 = *(u64 *)(r2 + 0)",
        "0:",
        "exit",
        value_off = const core::mem::offset_of!(bpf_iter__bpf_map_elem, value),
        options(noreturn)
    );
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
