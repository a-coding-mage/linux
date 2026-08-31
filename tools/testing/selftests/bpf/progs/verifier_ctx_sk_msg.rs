// SPDX-License-Identifier: GPL-2.0
/* Converted from tools/testing/selftests/bpf/verifier/ctx_sk_msg.c */

// C dependencies removed from executable Rust:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include "bpf_misc.h"

use core::arch::asm;
use core::mem::offset_of;

// The verifier harness macros from bpf_misc.h are preserved as comments on the
// translated functions: __description, __success, __failure, __msg, and __flag.

#[link_section = "sk_msg"]
// __description("valid access family in SK_MSG")
// __success
#[no_mangle]
pub unsafe extern "C" fn access_family_in_sk_msg() {
    asm!(
        "r0 = *(u32*)(r1 + {sk_msg_md_family})",
        "exit",
        sk_msg_md_family = const offset_of!(sk_msg_md, family),
        options(noreturn)
    );
}

#[link_section = "sk_msg"]
// __description("valid access remote_ip4 in SK_MSG")
// __success
#[no_mangle]
pub unsafe extern "C" fn remote_ip4_in_sk_msg() {
    asm!(
        "r0 = *(u32*)(r1 + {sk_msg_md_remote_ip4})",
        "exit",
        sk_msg_md_remote_ip4 = const offset_of!(sk_msg_md, remote_ip4),
        options(noreturn)
    );
}

#[link_section = "sk_msg"]
// __description("valid access local_ip4 in SK_MSG")
// __success
#[no_mangle]
pub unsafe extern "C" fn local_ip4_in_sk_msg() {
    asm!(
        "r0 = *(u32*)(r1 + {sk_msg_md_local_ip4})",
        "exit",
        sk_msg_md_local_ip4 = const offset_of!(sk_msg_md, local_ip4),
        options(noreturn)
    );
}

#[link_section = "sk_msg"]
// __description("valid access remote_port in SK_MSG")
// __success
#[no_mangle]
pub unsafe extern "C" fn remote_port_in_sk_msg() {
    asm!(
        "r0 = *(u32*)(r1 + {sk_msg_md_remote_port})",
        "exit",
        sk_msg_md_remote_port = const offset_of!(sk_msg_md, remote_port),
        options(noreturn)
    );
}

#[link_section = "sk_msg"]
// __description("valid access local_port in SK_MSG")
// __success
#[no_mangle]
pub unsafe extern "C" fn local_port_in_sk_msg() {
    asm!(
        "r0 = *(u32*)(r1 + {sk_msg_md_local_port})",
        "exit",
        sk_msg_md_local_port = const offset_of!(sk_msg_md, local_port),
        options(noreturn)
    );
}

#[link_section = "sk_skb"]
// __description("valid access remote_ip6 in SK_MSG")
// __success
#[no_mangle]
pub unsafe extern "C" fn remote_ip6_in_sk_msg() {
    asm!(
        "r0 = *(u32*)(r1 + {sk_msg_md_remote_ip6_0})",
        "r0 = *(u32*)(r1 + {sk_msg_md_remote_ip6_1})",
        "r0 = *(u32*)(r1 + {sk_msg_md_remote_ip6_2})",
        "r0 = *(u32*)(r1 + {sk_msg_md_remote_ip6_3})",
        "exit",
        sk_msg_md_remote_ip6_0 = const offset_of!(sk_msg_md, remote_ip6[0]),
        sk_msg_md_remote_ip6_1 = const offset_of!(sk_msg_md, remote_ip6[1]),
        sk_msg_md_remote_ip6_2 = const offset_of!(sk_msg_md, remote_ip6[2]),
        sk_msg_md_remote_ip6_3 = const offset_of!(sk_msg_md, remote_ip6[3]),
        options(noreturn)
    );
}

#[link_section = "sk_skb"]
// __description("valid access local_ip6 in SK_MSG")
// __success
#[no_mangle]
pub unsafe extern "C" fn local_ip6_in_sk_msg() {
    asm!(
        "r0 = *(u32*)(r1 + {sk_msg_md_local_ip6_0})",
        "r0 = *(u32*)(r1 + {sk_msg_md_local_ip6_1})",
        "r0 = *(u32*)(r1 + {sk_msg_md_local_ip6_2})",
        "r0 = *(u32*)(r1 + {sk_msg_md_local_ip6_3})",
        "exit",
        sk_msg_md_local_ip6_0 = const offset_of!(sk_msg_md, local_ip6[0]),
        sk_msg_md_local_ip6_1 = const offset_of!(sk_msg_md, local_ip6[1]),
        sk_msg_md_local_ip6_2 = const offset_of!(sk_msg_md, local_ip6[2]),
        sk_msg_md_local_ip6_3 = const offset_of!(sk_msg_md, local_ip6[3]),
        options(noreturn)
    );
}

#[link_section = "sk_msg"]
// __description("valid access size in SK_MSG")
// __success
#[no_mangle]
pub unsafe extern "C" fn access_size_in_sk_msg() {
    asm!(
        "r0 = *(u32*)(r1 + {sk_msg_md_size})",
        "exit",
        sk_msg_md_size = const offset_of!(sk_msg_md, size),
        options(noreturn)
    );
}

#[link_section = "sk_msg"]
// __description("invalid 64B read of size in SK_MSG")
// __failure
// __msg("invalid bpf_context access")
// __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn of_size_in_sk_msg() {
    asm!(
        "r2 = *(u64*)(r1 + {sk_msg_md_size})",
        "exit",
        sk_msg_md_size = const offset_of!(sk_msg_md, size),
        options(noreturn)
    );
}

#[link_section = "sk_msg"]
// __description("invalid read past end of SK_MSG")
// __failure
// __msg("invalid bpf_context access")
#[no_mangle]
pub unsafe extern "C" fn past_end_of_sk_msg() {
    asm!(
        "r2 = *(u32*)(r1 + {__imm_0})",
        "exit",
        __imm_0 = const offset_of!(sk_msg_md, size) + 4,
        options(noreturn)
    );
}

#[link_section = "sk_msg"]
// __description("invalid read offset in SK_MSG")
// __failure
// __msg("invalid bpf_context access")
// __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn read_offset_in_sk_msg() {
    asm!(
        "r2 = *(u32*)(r1 + {__imm_0})",
        "exit",
        __imm_0 = const offset_of!(sk_msg_md, family) + 1,
        options(noreturn)
    );
}

#[link_section = "sk_msg"]
// __description("direct packet read for SK_MSG")
// __success
#[no_mangle]
pub unsafe extern "C" fn packet_read_for_sk_msg() {
    asm!(
        "r2 = *(u64*)(r1 + {sk_msg_md_data})",
        "r3 = *(u64*)(r1 + {sk_msg_md_data_end})",
        "r0 = r2",
        "r0 += 8",
        "if r0 > r3 goto 0f",
        "r0 = *(u8*)(r2 + 0)",
        "0:",
        "r0 = 0",
        "exit",
        sk_msg_md_data = const offset_of!(sk_msg_md, data),
        sk_msg_md_data_end = const offset_of!(sk_msg_md, data_end),
        options(noreturn)
    );
}

#[link_section = "sk_msg"]
// __description("direct packet write for SK_MSG")
// __success
#[no_mangle]
pub unsafe extern "C" fn packet_write_for_sk_msg() {
    asm!(
        "r2 = *(u64*)(r1 + {sk_msg_md_data})",
        "r3 = *(u64*)(r1 + {sk_msg_md_data_end})",
        "r0 = r2",
        "r0 += 8",
        "if r0 > r3 goto 0f",
        "*(u8*)(r2 + 0) = r2",
        "0:",
        "r0 = 0",
        "exit",
        sk_msg_md_data = const offset_of!(sk_msg_md, data),
        sk_msg_md_data_end = const offset_of!(sk_msg_md, data_end),
        options(noreturn)
    );
}

#[link_section = "sk_msg"]
// __description("overlapping checks for direct packet access SK_MSG")
// __success
#[no_mangle]
pub unsafe extern "C" fn direct_packet_access_sk_msg() {
    asm!(
        "r2 = *(u64*)(r1 + {sk_msg_md_data})",
        "r3 = *(u64*)(r1 + {sk_msg_md_data_end})",
        "r0 = r2",
        "r0 += 8",
        "if r0 > r3 goto 0f",
        "r1 = r2",
        "r1 += 6",
        "if r1 > r3 goto 0f",
        "r0 = *(u16*)(r2 + 6)",
        "0:",
        "r0 = 0",
        "exit",
        sk_msg_md_data = const offset_of!(sk_msg_md, data),
        sk_msg_md_data_end = const offset_of!(sk_msg_md, data_end),
        options(noreturn)
    );
}

#[link_section = "license"]
#[no_mangle]
pub static _license: [u8; 4] = *b"GPL\0";
