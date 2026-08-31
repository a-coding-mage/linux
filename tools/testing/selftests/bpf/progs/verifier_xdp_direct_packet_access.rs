// SPDX-License-Identifier: GPL-2.0
/* Converted from tools/testing/selftests/bpf/verifier/xdp_direct_packet_access.c */

// C includes translated as dependency intent:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include "bpf_misc.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![feature(asm_experimental_arch)]
#![feature(naked_functions)]

use core::arch::asm;

// Offsets supplied by the translated equivalents of linux/bpf.h and bpf_misc.h.
extern "C" {
    static xdp_md_data: u32;
    static xdp_md_data_end: u32;
    static xdp_md_data_meta: u32;
}

// SEC("xdp")
// __description("XDP pkt read, pkt_end mangling, bad access 1")
// __failure __msg("R3 pointer arithmetic on pkt_end")
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn end_mangling_bad_access_1() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data]);
	r3 = *(u32*)(r1 + %[xdp_md_data_end]);
	r1 = r2;
	r1 += 8;
	r3 += 8;
	if r1 > r3 goto l0_%=;
	r0 = *(u64*)(r1 - 8);
l0_%=:	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_end = sym xdp_md_data_end,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("XDP pkt read, pkt_end mangling, bad access 2")
// __failure __msg("R3 pointer arithmetic on pkt_end")
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn end_mangling_bad_access_2() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data]);
	r3 = *(u32*)(r1 + %[xdp_md_data_end]);
	r1 = r2;
	r1 += 8;
	r3 -= 8;
	if r1 > r3 goto l0_%=;
	r0 = *(u64*)(r1 - 8);
l0_%=:	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_end = sym xdp_md_data_end,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("XDP pkt read, pkt_data' > pkt_end, corner case, good access")
// __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn end_corner_case_good_access_1() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data]);
	r3 = *(u32*)(r1 + %[xdp_md_data_end]);
	r1 = r2;
	r1 += 8;
	if r1 > r3 goto l0_%=;
	r0 = *(u64*)(r1 - 8);
l0_%=:	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_end = sym xdp_md_data_end,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("XDP pkt read, pkt_data' > pkt_end, bad access 1")
// __failure __msg("R1 {{min|max}} value is outside of the allowed memory range")
// __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pkt_end_bad_access_1_1() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data]);
	r3 = *(u32*)(r1 + %[xdp_md_data_end]);
	r1 = r2;
	r1 += 8;
	if r1 > r3 goto l0_%=;
	r0 = *(u64*)(r1 - 4);
l0_%=:	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_end = sym xdp_md_data_end,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("XDP pkt read, pkt_data' > pkt_end, bad access 2")
// __failure __msg("R1 offset is outside of the packet")
// __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pkt_end_bad_access_2_1() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data]);
	r3 = *(u32*)(r1 + %[xdp_md_data_end]);
	r1 = r2;
	r1 += 8;
	if r1 > r3 goto l0_%=;
l0_%=:	r0 = *(u64*)(r1 - 8);
	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_end = sym xdp_md_data_end,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("XDP pkt read, pkt_data' > pkt_end, corner case +1, good access")
// __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn corner_case_1_good_access_1() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data]);
	r3 = *(u32*)(r1 + %[xdp_md_data_end]);
	r1 = r2;
	r1 += 9;
	if r1 > r3 goto l0_%=;
	r0 = *(u64*)(r1 - 9);
l0_%=:	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_end = sym xdp_md_data_end,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("XDP pkt read, pkt_data' > pkt_end, corner case -1, bad access")
// __failure __msg("R1 {{min|max}} value is outside of the allowed memory range")
// __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn corner_case_1_bad_access_1() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data]);
	r3 = *(u32*)(r1 + %[xdp_md_data_end]);
	r1 = r2;
	r1 += 7;
	if r1 > r3 goto l0_%=;
	r0 = *(u64*)(r1 - 7);
l0_%=:	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_end = sym xdp_md_data_end,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("XDP pkt read, pkt_end > pkt_data', good access")
// __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn end_pkt_data_good_access_1() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data]);
	r3 = *(u32*)(r1 + %[xdp_md_data_end]);
	r1 = r2;
	r1 += 8;
	if r3 > r1 goto l0_%=;
	goto l1_%=;
l0_%=:	r0 = *(u32*)(r1 - 5);
l1_%=:	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_end = sym xdp_md_data_end,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("XDP pkt read, pkt_end > pkt_data', corner case -1, bad access")
// __failure __msg("R1 {{min|max}} value is outside of the allowed memory range")
// __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn corner_case_1_bad_access_2() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data]);
	r3 = *(u32*)(r1 + %[xdp_md_data_end]);
	r1 = r2;
	r1 += 6;
	if r3 > r1 goto l0_%=;
	goto l1_%=;
l0_%=:	r0 = *(u64*)(r1 - 6);
l1_%=:	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_end = sym xdp_md_data_end,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("XDP pkt read, pkt_end > pkt_data', bad access 2")
// __failure __msg("R1 offset is outside of the packet")
// __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pkt_data_bad_access_2_1() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data]);
	r3 = *(u32*)(r1 + %[xdp_md_data_end]);
	r1 = r2;
	r1 += 8;
	if r3 > r1 goto l0_%=;
	r0 = *(u64*)(r1 - 8);
l0_%=:	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_end = sym xdp_md_data_end,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("XDP pkt read, pkt_end > pkt_data', corner case, good access")
// __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn data_corner_case_good_access_1() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data]);
	r3 = *(u32*)(r1 + %[xdp_md_data_end]);
	r1 = r2;
	r1 += 7;
	if r3 > r1 goto l0_%=;
	goto l1_%=;
l0_%=:	r0 = *(u64*)(r1 - 7);
l1_%=:	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_end = sym xdp_md_data_end,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("XDP pkt read, pkt_end > pkt_data', corner case +1, good access")
// __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn corner_case_1_good_access_2() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data]);
	r3 = *(u32*)(r1 + %[xdp_md_data_end]);
	r1 = r2;
	r1 += 8;
	if r3 > r1 goto l0_%=;
	goto l1_%=;
l0_%=:	r0 = *(u64*)(r1 - 8);
l1_%=:	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_end = sym xdp_md_data_end,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("XDP pkt read, pkt_data' < pkt_end, good access")
// __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn data_pkt_end_good_access_1() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data]);
	r3 = *(u32*)(r1 + %[xdp_md_data_end]);
	r1 = r2;
	r1 += 8;
	if r1 < r3 goto l0_%=;
	goto l1_%=;
l0_%=:	r0 = *(u32*)(r1 - 5);
l1_%=:	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_end = sym xdp_md_data_end,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("XDP pkt read, pkt_data' < pkt_end, corner case -1, bad access")
// __failure __msg("R1 {{min|max}} value is outside of the allowed memory range")
// __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn corner_case_1_bad_access_3() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data]);
	r3 = *(u32*)(r1 + %[xdp_md_data_end]);
	r1 = r2;
	r1 += 6;
	if r1 < r3 goto l0_%=;
	goto l1_%=;
l0_%=:	r0 = *(u64*)(r1 - 6);
l1_%=:	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_end = sym xdp_md_data_end,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("XDP pkt read, pkt_data' < pkt_end, bad access 2")
// __failure __msg("R1 offset is outside of the packet")
// __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pkt_end_bad_access_2_2() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data]);
	r3 = *(u32*)(r1 + %[xdp_md_data_end]);
	r1 = r2;
	r1 += 8;
	if r1 < r3 goto l0_%=;
	r0 = *(u64*)(r1 - 8);
l0_%=:	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_end = sym xdp_md_data_end,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("XDP pkt read, pkt_data' < pkt_end, corner case, good access")
// __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn end_corner_case_good_access_2() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data]);
	r3 = *(u32*)(r1 + %[xdp_md_data_end]);
	r1 = r2;
	r1 += 7;
	if r1 < r3 goto l0_%=;
	goto l1_%=;
l0_%=:	r0 = *(u64*)(r1 - 7);
l1_%=:	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_end = sym xdp_md_data_end,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("XDP pkt read, pkt_data' < pkt_end, corner case +1, good access")
// __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn corner_case_1_good_access_3() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data]);
	r3 = *(u32*)(r1 + %[xdp_md_data_end]);
	r1 = r2;
	r1 += 8;
	if r1 < r3 goto l0_%=;
	goto l1_%=;
l0_%=:	r0 = *(u64*)(r1 - 8);
l1_%=:	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_end = sym xdp_md_data_end,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("XDP pkt read, pkt_end < pkt_data', corner case, good access")
// __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn data_corner_case_good_access_2() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data]);
	r3 = *(u32*)(r1 + %[xdp_md_data_end]);
	r1 = r2;
	r1 += 8;
	if r3 < r1 goto l0_%=;
	r0 = *(u64*)(r1 - 8);
l0_%=:	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_end = sym xdp_md_data_end,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("XDP pkt read, pkt_end < pkt_data', bad access 1")
// __failure __msg("R1 {{min|max}} value is outside of the allowed memory range")
// __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pkt_data_bad_access_1_1() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data]);
	r3 = *(u32*)(r1 + %[xdp_md_data_end]);
	r1 = r2;
	r1 += 8;
	if r3 < r1 goto l0_%=;
	r0 = *(u64*)(r1 - 4);
l0_%=:	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_end = sym xdp_md_data_end,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("XDP pkt read, pkt_end < pkt_data', bad access 2")
// __failure __msg("R1 offset is outside of the packet")
// __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pkt_data_bad_access_2_2() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data]);
	r3 = *(u32*)(r1 + %[xdp_md_data_end]);
	r1 = r2;
	r1 += 8;
	if r3 < r1 goto l0_%=;
l0_%=:	r0 = *(u64*)(r1 - 8);
	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_end = sym xdp_md_data_end,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("XDP pkt read, pkt_end < pkt_data', corner case +1, good access")
// __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn corner_case_1_good_access_4() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data]);
	r3 = *(u32*)(r1 + %[xdp_md_data_end]);
	r1 = r2;
	r1 += 9;
	if r3 < r1 goto l0_%=;
	r0 = *(u64*)(r1 - 9);
l0_%=:	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_end = sym xdp_md_data_end,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("XDP pkt read, pkt_end < pkt_data', corner case -1, bad access")
// __failure __msg("R1 {{min|max}} value is outside of the allowed memory range")
// __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn corner_case_1_bad_access_4() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data]);
	r3 = *(u32*)(r1 + %[xdp_md_data_end]);
	r1 = r2;
	r1 += 7;
	if r3 < r1 goto l0_%=;
	r0 = *(u64*)(r1 - 7);
l0_%=:	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_end = sym xdp_md_data_end,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("XDP pkt read, pkt_data' >= pkt_end, good access")
// __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn data_pkt_end_good_access_2() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data]);
	r3 = *(u32*)(r1 + %[xdp_md_data_end]);
	r1 = r2;
	r1 += 8;
	if r1 >= r3 goto l0_%=;
	r0 = *(u32*)(r1 - 5);
l0_%=:	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_end = sym xdp_md_data_end,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("XDP pkt read, pkt_data' >= pkt_end, corner case -1, bad access")
// __failure __msg("R1 {{min|max}} value is outside of the allowed memory range")
// __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn corner_case_1_bad_access_5() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data]);
	r3 = *(u32*)(r1 + %[xdp_md_data_end]);
	r1 = r2;
	r1 += 6;
	if r1 >= r3 goto l0_%=;
	r0 = *(u64*)(r1 - 6);
l0_%=:	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_end = sym xdp_md_data_end,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("XDP pkt read, pkt_data' >= pkt_end, bad access 2")
// __failure __msg("R1 offset is outside of the packet")
// __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pkt_end_bad_access_2_3() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data]);
	r3 = *(u32*)(r1 + %[xdp_md_data_end]);
	r1 = r2;
	r1 += 8;
	if r1 >= r3 goto l0_%=;
l0_%=:	r0 = *(u32*)(r1 - 5);
	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_end = sym xdp_md_data_end,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("XDP pkt read, pkt_data' >= pkt_end, corner case, good access")
// __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn end_corner_case_good_access_3() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data]);
	r3 = *(u32*)(r1 + %[xdp_md_data_end]);
	r1 = r2;
	r1 += 7;
	if r1 >= r3 goto l0_%=;
	r0 = *(u64*)(r1 - 7);
l0_%=:	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_end = sym xdp_md_data_end,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("XDP pkt read, pkt_data' >= pkt_end, corner case +1, good access")
// __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn corner_case_1_good_access_5() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data]);
	r3 = *(u32*)(r1 + %[xdp_md_data_end]);
	r1 = r2;
	r1 += 8;
	if r1 >= r3 goto l0_%=;
	r0 = *(u64*)(r1 - 8);
l0_%=:	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_end = sym xdp_md_data_end,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("XDP pkt read, pkt_end >= pkt_data', corner case, good access")
// __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn data_corner_case_good_access_3() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data]);
	r3 = *(u32*)(r1 + %[xdp_md_data_end]);
	r1 = r2;
	r1 += 8;
	if r3 >= r1 goto l0_%=;
	goto l1_%=;
l0_%=:	r0 = *(u64*)(r1 - 8);
l1_%=:	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_end = sym xdp_md_data_end,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("XDP pkt read, pkt_end >= pkt_data', bad access 1")
// __failure __msg("R1 {{min|max}} value is outside of the allowed memory range")
// __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pkt_data_bad_access_1_2() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data]);
	r3 = *(u32*)(r1 + %[xdp_md_data_end]);
	r1 = r2;
	r1 += 8;
	if r3 >= r1 goto l0_%=;
	goto l1_%=;
l0_%=:	r0 = *(u64*)(r1 - 4);
l1_%=:	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_end = sym xdp_md_data_end,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("XDP pkt read, pkt_end >= pkt_data', bad access 2")
// __failure __msg("R1 offset is outside of the packet")
// __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pkt_data_bad_access_2_3() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data]);
	r3 = *(u32*)(r1 + %[xdp_md_data_end]);
	r1 = r2;
	r1 += 8;
	if r3 >= r1 goto l0_%=;
	r0 = *(u64*)(r1 - 8);
l0_%=:	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_end = sym xdp_md_data_end,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("XDP pkt read, pkt_end >= pkt_data', corner case +1, good access")
// __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn corner_case_1_good_access_6() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data]);
	r3 = *(u32*)(r1 + %[xdp_md_data_end]);
	r1 = r2;
	r1 += 9;
	if r3 >= r1 goto l0_%=;
	goto l1_%=;
l0_%=:	r0 = *(u64*)(r1 - 9);
l1_%=:	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_end = sym xdp_md_data_end,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("XDP pkt read, pkt_end >= pkt_data', corner case -1, bad access")
// __failure __msg("R1 {{min|max}} value is outside of the allowed memory range")
// __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn corner_case_1_bad_access_6() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data]);
	r3 = *(u32*)(r1 + %[xdp_md_data_end]);
	r1 = r2;
	r1 += 7;
	if r3 >= r1 goto l0_%=;
	goto l1_%=;
l0_%=:	r0 = *(u64*)(r1 - 7);
l1_%=:	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_end = sym xdp_md_data_end,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("XDP pkt read, pkt_data' <= pkt_end, corner case, good access")
// __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn end_corner_case_good_access_4() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data]);
	r3 = *(u32*)(r1 + %[xdp_md_data_end]);
	r1 = r2;
	r1 += 8;
	if r1 <= r3 goto l0_%=;
	goto l1_%=;
l0_%=:	r0 = *(u64*)(r1 - 8);
l1_%=:	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_end = sym xdp_md_data_end,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("XDP pkt read, pkt_data' <= pkt_end, bad access 1")
// __failure __msg("R1 {{min|max}} value is outside of the allowed memory range")
// __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pkt_end_bad_access_1_2() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data]);
	r3 = *(u32*)(r1 + %[xdp_md_data_end]);
	r1 = r2;
	r1 += 8;
	if r1 <= r3 goto l0_%=;
	goto l1_%=;
l0_%=:	r0 = *(u64*)(r1 - 4);
l1_%=:	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_end = sym xdp_md_data_end,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("XDP pkt read, pkt_data' <= pkt_end, bad access 2")
// __failure __msg("R1 offset is outside of the packet")
// __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pkt_end_bad_access_2_4() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data]);
	r3 = *(u32*)(r1 + %[xdp_md_data_end]);
	r1 = r2;
	r1 += 8;
	if r1 <= r3 goto l0_%=;
	r0 = *(u64*)(r1 - 8);
l0_%=:	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_end = sym xdp_md_data_end,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("XDP pkt read, pkt_data' <= pkt_end, corner case +1, good access")
// __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn corner_case_1_good_access_7() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data]);
	r3 = *(u32*)(r1 + %[xdp_md_data_end]);
	r1 = r2;
	r1 += 9;
	if r1 <= r3 goto l0_%=;
	goto l1_%=;
l0_%=:	r0 = *(u64*)(r1 - 9);
l1_%=:	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_end = sym xdp_md_data_end,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("XDP pkt read, pkt_data' <= pkt_end, corner case -1, bad access")
// __failure __msg("R1 {{min|max}} value is outside of the allowed memory range")
// __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn corner_case_1_bad_access_7() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data]);
	r3 = *(u32*)(r1 + %[xdp_md_data_end]);
	r1 = r2;
	r1 += 7;
	if r1 <= r3 goto l0_%=;
	goto l1_%=;
l0_%=:	r0 = *(u64*)(r1 - 7);
l1_%=:	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_end = sym xdp_md_data_end,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("XDP pkt read, pkt_end <= pkt_data', good access")
// __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn end_pkt_data_good_access_2() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data]);
	r3 = *(u32*)(r1 + %[xdp_md_data_end]);
	r1 = r2;
	r1 += 8;
	if r3 <= r1 goto l0_%=;
	r0 = *(u32*)(r1 - 5);
l0_%=:	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_end = sym xdp_md_data_end,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("XDP pkt read, pkt_end <= pkt_data', corner case -1, bad access")
// __failure __msg("R1 {{min|max}} value is outside of the allowed memory range")
// __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn corner_case_1_bad_access_8() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data]);
	r3 = *(u32*)(r1 + %[xdp_md_data_end]);
	r1 = r2;
	r1 += 6;
	if r3 <= r1 goto l0_%=;
	r0 = *(u64*)(r1 - 6);
l0_%=:	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_end = sym xdp_md_data_end,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("XDP pkt read, pkt_end <= pkt_data', bad access 2")
// __failure __msg("R1 offset is outside of the packet")
// __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pkt_data_bad_access_2_4() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data]);
	r3 = *(u32*)(r1 + %[xdp_md_data_end]);
	r1 = r2;
	r1 += 8;
	if r3 <= r1 goto l0_%=;
l0_%=:	r0 = *(u32*)(r1 - 5);
	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_end = sym xdp_md_data_end,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("XDP pkt read, pkt_end <= pkt_data', corner case, good access")
// __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn data_corner_case_good_access_4() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data]);
	r3 = *(u32*)(r1 + %[xdp_md_data_end]);
	r1 = r2;
	r1 += 7;
	if r3 <= r1 goto l0_%=;
	r0 = *(u64*)(r1 - 7);
l0_%=:	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_end = sym xdp_md_data_end,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("XDP pkt read, pkt_end <= pkt_data', corner case +1, good access")
// __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn corner_case_1_good_access_8() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data]);
	r3 = *(u32*)(r1 + %[xdp_md_data_end]);
	r1 = r2;
	r1 += 8;
	if r3 <= r1 goto l0_%=;
	r0 = *(u64*)(r1 - 8);
l0_%=:	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_end = sym xdp_md_data_end,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("XDP pkt read, pkt_meta' > pkt_data, corner case, good access")
// __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn data_corner_case_good_access_5() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data_meta]);
	r3 = *(u32*)(r1 + %[xdp_md_data]);
	r1 = r2;
	r1 += 8;
	if r1 > r3 goto l0_%=;
	r0 = *(u64*)(r1 - 8);
l0_%=:	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_meta = sym xdp_md_data_meta,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("XDP pkt read, pkt_meta' > pkt_data, bad access 1")
// __failure __msg("R1 {{min|max}} value is outside of the allowed memory range")
// __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pkt_data_bad_access_1_3() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data_meta]);
	r3 = *(u32*)(r1 + %[xdp_md_data]);
	r1 = r2;
	r1 += 8;
	if r1 > r3 goto l0_%=;
	r0 = *(u64*)(r1 - 4);
l0_%=:	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_meta = sym xdp_md_data_meta,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("XDP pkt read, pkt_meta' > pkt_data, bad access 2")
// __failure __msg("R1 {{min|max}} value is outside of the allowed memory range")
// __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pkt_data_bad_access_2_5() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data_meta]);
	r3 = *(u32*)(r1 + %[xdp_md_data]);
	r1 = r2;
	r1 += 8;
	if r1 > r3 goto l0_%=;
l0_%=:	r0 = *(u64*)(r1 - 8);
	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_meta = sym xdp_md_data_meta,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("XDP pkt read, pkt_meta' > pkt_data, corner case +1, good access")
// __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn corner_case_1_good_access_9() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data_meta]);
	r3 = *(u32*)(r1 + %[xdp_md_data]);
	r1 = r2;
	r1 += 9;
	if r1 > r3 goto l0_%=;
	r0 = *(u64*)(r1 - 9);
l0_%=:	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_meta = sym xdp_md_data_meta,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("XDP pkt read, pkt_meta' > pkt_data, corner case -1, bad access")
// __failure __msg("R1 {{min|max}} value is outside of the allowed memory range")
// __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn corner_case_1_bad_access_9() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data_meta]);
	r3 = *(u32*)(r1 + %[xdp_md_data]);
	r1 = r2;
	r1 += 7;
	if r1 > r3 goto l0_%=;
	r0 = *(u64*)(r1 - 7);
l0_%=:	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_meta = sym xdp_md_data_meta,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("XDP pkt read, pkt_data > pkt_meta', good access")
// __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn data_pkt_meta_good_access_1() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data_meta]);
	r3 = *(u32*)(r1 + %[xdp_md_data]);
	r1 = r2;
	r1 += 8;
	if r3 > r1 goto l0_%=;
	goto l1_%=;
l0_%=:	r0 = *(u32*)(r1 - 5);
l1_%=:	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_meta = sym xdp_md_data_meta,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("XDP pkt read, pkt_data > pkt_meta', corner case -1, bad access")
// __failure __msg("R1 {{min|max}} value is outside of the allowed memory range")
// __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn corner_case_1_bad_access_10() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data_meta]);
	r3 = *(u32*)(r1 + %[xdp_md_data]);
	r1 = r2;
	r1 += 6;
	if r3 > r1 goto l0_%=;
	goto l1_%=;
l0_%=:	r0 = *(u64*)(r1 - 6);
l1_%=:	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_meta = sym xdp_md_data_meta,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("XDP pkt read, pkt_data > pkt_meta', bad access 2")
// __failure __msg("R1 {{min|max}} value is outside of the allowed memory range")
// __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pkt_meta_bad_access_2_1() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data_meta]);
	r3 = *(u32*)(r1 + %[xdp_md_data]);
	r1 = r2;
	r1 += 8;
	if r3 > r1 goto l0_%=;
	r0 = *(u64*)(r1 - 8);
l0_%=:	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_meta = sym xdp_md_data_meta,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("XDP pkt read, pkt_data > pkt_meta', corner case, good access")
// __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn meta_corner_case_good_access_1() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data_meta]);
	r3 = *(u32*)(r1 + %[xdp_md_data]);
	r1 = r2;
	r1 += 7;
	if r3 > r1 goto l0_%=;
	goto l1_%=;
l0_%=:	r0 = *(u64*)(r1 - 7);
l1_%=:	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_meta = sym xdp_md_data_meta,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("XDP pkt read, pkt_data > pkt_meta', corner case +1, good access")
// __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn corner_case_1_good_access_10() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data_meta]);
	r3 = *(u32*)(r1 + %[xdp_md_data]);
	r1 = r2;
	r1 += 8;
	if r3 > r1 goto l0_%=;
	goto l1_%=;
l0_%=:	r0 = *(u64*)(r1 - 8);
l1_%=:	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_meta = sym xdp_md_data_meta,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("XDP pkt read, pkt_meta' < pkt_data, good access")
// __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn meta_pkt_data_good_access_1() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data_meta]);
	r3 = *(u32*)(r1 + %[xdp_md_data]);
	r1 = r2;
	r1 += 8;
	if r1 < r3 goto l0_%=;
	goto l1_%=;
l0_%=:	r0 = *(u32*)(r1 - 5);
l1_%=:	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_meta = sym xdp_md_data_meta,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("XDP pkt read, pkt_meta' < pkt_data, corner case -1, bad access")
// __failure __msg("R1 {{min|max}} value is outside of the allowed memory range")
// __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn corner_case_1_bad_access_11() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data_meta]);
	r3 = *(u32*)(r1 + %[xdp_md_data]);
	r1 = r2;
	r1 += 6;
	if r1 < r3 goto l0_%=;
	goto l1_%=;
l0_%=:	r0 = *(u64*)(r1 - 6);
l1_%=:	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_meta = sym xdp_md_data_meta,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("XDP pkt read, pkt_meta' < pkt_data, bad access 2")
// __failure __msg("R1 {{min|max}} value is outside of the allowed memory range")
// __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pkt_data_bad_access_2_6() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data_meta]);
	r3 = *(u32*)(r1 + %[xdp_md_data]);
	r1 = r2;
	r1 += 8;
	if r1 < r3 goto l0_%=;
	r0 = *(u64*)(r1 - 8);
l0_%=:	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_meta = sym xdp_md_data_meta,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("XDP pkt read, pkt_meta' < pkt_data, corner case, good access")
// __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn data_corner_case_good_access_6() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data_meta]);
	r3 = *(u32*)(r1 + %[xdp_md_data]);
	r1 = r2;
	r1 += 7;
	if r1 < r3 goto l0_%=;
	goto l1_%=;
l0_%=:	r0 = *(u64*)(r1 - 7);
l1_%=:	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_meta = sym xdp_md_data_meta,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("XDP pkt read, pkt_meta' < pkt_data, corner case +1, good access")
// __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn corner_case_1_good_access_11() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data_meta]);
	r3 = *(u32*)(r1 + %[xdp_md_data]);
	r1 = r2;
	r1 += 8;
	if r1 < r3 goto l0_%=;
	goto l1_%=;
l0_%=:	r0 = *(u64*)(r1 - 8);
l1_%=:	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_meta = sym xdp_md_data_meta,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("XDP pkt read, pkt_data < pkt_meta', corner case, good access")
// __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn meta_corner_case_good_access_2() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data_meta]);
	r3 = *(u32*)(r1 + %[xdp_md_data]);
	r1 = r2;
	r1 += 8;
	if r3 < r1 goto l0_%=;
	r0 = *(u64*)(r1 - 8);
l0_%=:	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_meta = sym xdp_md_data_meta,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("XDP pkt read, pkt_data < pkt_meta', bad access 1")
// __failure __msg("R1 {{min|max}} value is outside of the allowed memory range")
// __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pkt_meta_bad_access_1_1() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data_meta]);
	r3 = *(u32*)(r1 + %[xdp_md_data]);
	r1 = r2;
	r1 += 8;
	if r3 < r1 goto l0_%=;
	r0 = *(u64*)(r1 - 4);
l0_%=:	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_meta = sym xdp_md_data_meta,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("XDP pkt read, pkt_data < pkt_meta', bad access 2")
// __failure __msg("R1 {{min|max}} value is outside of the allowed memory range")
// __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pkt_meta_bad_access_2_2() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data_meta]);
	r3 = *(u32*)(r1 + %[xdp_md_data]);
	r1 = r2;
	r1 += 8;
	if r3 < r1 goto l0_%=;
l0_%=:	r0 = *(u64*)(r1 - 8);
	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_meta = sym xdp_md_data_meta,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("XDP pkt read, pkt_data < pkt_meta', corner case +1, good access")
// __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn corner_case_1_good_access_12() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data_meta]);
	r3 = *(u32*)(r1 + %[xdp_md_data]);
	r1 = r2;
	r1 += 9;
	if r3 < r1 goto l0_%=;
	r0 = *(u64*)(r1 - 9);
l0_%=:	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_meta = sym xdp_md_data_meta,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("XDP pkt read, pkt_data < pkt_meta', corner case -1, bad access")
// __failure __msg("R1 {{min|max}} value is outside of the allowed memory range")
// __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn corner_case_1_bad_access_12() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data_meta]);
	r3 = *(u32*)(r1 + %[xdp_md_data]);
	r1 = r2;
	r1 += 7;
	if r3 < r1 goto l0_%=;
	r0 = *(u64*)(r1 - 7);
l0_%=:	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_meta = sym xdp_md_data_meta,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("XDP pkt read, pkt_meta' >= pkt_data, good access")
// __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn meta_pkt_data_good_access_2() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data_meta]);
	r3 = *(u32*)(r1 + %[xdp_md_data]);
	r1 = r2;
	r1 += 8;
	if r1 >= r3 goto l0_%=;
	r0 = *(u32*)(r1 - 5);
l0_%=:	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_meta = sym xdp_md_data_meta,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("XDP pkt read, pkt_meta' >= pkt_data, corner case -1, bad access")
// __failure __msg("R1 {{min|max}} value is outside of the allowed memory range")
// __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn corner_case_1_bad_access_13() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data_meta]);
	r3 = *(u32*)(r1 + %[xdp_md_data]);
	r1 = r2;
	r1 += 6;
	if r1 >= r3 goto l0_%=;
	r0 = *(u64*)(r1 - 6);
l0_%=:	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_meta = sym xdp_md_data_meta,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("XDP pkt read, pkt_meta' >= pkt_data, bad access 2")
// __failure __msg("R1 {{min|max}} value is outside of the allowed memory range")
// __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pkt_data_bad_access_2_7() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data_meta]);
	r3 = *(u32*)(r1 + %[xdp_md_data]);
	r1 = r2;
	r1 += 8;
	if r1 >= r3 goto l0_%=;
l0_%=:	r0 = *(u32*)(r1 - 5);
	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_meta = sym xdp_md_data_meta,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("XDP pkt read, pkt_meta' >= pkt_data, corner case, good access")
// __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn data_corner_case_good_access_7() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data_meta]);
	r3 = *(u32*)(r1 + %[xdp_md_data]);
	r1 = r2;
	r1 += 7;
	if r1 >= r3 goto l0_%=;
	r0 = *(u64*)(r1 - 7);
l0_%=:	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_meta = sym xdp_md_data_meta,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("XDP pkt read, pkt_meta' >= pkt_data, corner case +1, good access")
// __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn corner_case_1_good_access_13() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data_meta]);
	r3 = *(u32*)(r1 + %[xdp_md_data]);
	r1 = r2;
	r1 += 8;
	if r1 >= r3 goto l0_%=;
	r0 = *(u64*)(r1 - 8);
l0_%=:	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_meta = sym xdp_md_data_meta,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("XDP pkt read, pkt_data >= pkt_meta', corner case, good access")
// __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn meta_corner_case_good_access_3() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data_meta]);
	r3 = *(u32*)(r1 + %[xdp_md_data]);
	r1 = r2;
	r1 += 8;
	if r3 >= r1 goto l0_%=;
	goto l1_%=;
l0_%=:	r0 = *(u64*)(r1 - 8);
l1_%=:	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_meta = sym xdp_md_data_meta,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("XDP pkt read, pkt_data >= pkt_meta', bad access 1")
// __failure __msg("R1 {{min|max}} value is outside of the allowed memory range")
// __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pkt_meta_bad_access_1_2() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data_meta]);
	r3 = *(u32*)(r1 + %[xdp_md_data]);
	r1 = r2;
	r1 += 8;
	if r3 >= r1 goto l0_%=;
	goto l1_%=;
l0_%=:	r0 = *(u64*)(r1 - 4);
l1_%=:	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_meta = sym xdp_md_data_meta,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("XDP pkt read, pkt_data >= pkt_meta', bad access 2")
// __failure __msg("R1 {{min|max}} value is outside of the allowed memory range")
// __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pkt_meta_bad_access_2_3() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data_meta]);
	r3 = *(u32*)(r1 + %[xdp_md_data]);
	r1 = r2;
	r1 += 8;
	if r3 >= r1 goto l0_%=;
	r0 = *(u64*)(r1 - 8);
l0_%=:	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_meta = sym xdp_md_data_meta,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("XDP pkt read, pkt_data >= pkt_meta', corner case +1, good access")
// __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn corner_case_1_good_access_14() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data_meta]);
	r3 = *(u32*)(r1 + %[xdp_md_data]);
	r1 = r2;
	r1 += 9;
	if r3 >= r1 goto l0_%=;
	goto l1_%=;
l0_%=:	r0 = *(u64*)(r1 - 9);
l1_%=:	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_meta = sym xdp_md_data_meta,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("XDP pkt read, pkt_data >= pkt_meta', corner case -1, bad access")
// __failure __msg("R1 {{min|max}} value is outside of the allowed memory range")
// __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn corner_case_1_bad_access_14() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data_meta]);
	r3 = *(u32*)(r1 + %[xdp_md_data]);
	r1 = r2;
	r1 += 7;
	if r3 >= r1 goto l0_%=;
	goto l1_%=;
l0_%=:	r0 = *(u64*)(r1 - 7);
l1_%=:	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_meta = sym xdp_md_data_meta,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("XDP pkt read, pkt_meta' <= pkt_data, corner case, good access")
// __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn data_corner_case_good_access_8() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data_meta]);
	r3 = *(u32*)(r1 + %[xdp_md_data]);
	r1 = r2;
	r1 += 8;
	if r1 <= r3 goto l0_%=;
	goto l1_%=;
l0_%=:	r0 = *(u64*)(r1 - 8);
l1_%=:	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_meta = sym xdp_md_data_meta,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("XDP pkt read, pkt_meta' <= pkt_data, bad access 1")
// __failure __msg("R1 {{min|max}} value is outside of the allowed memory range")
// __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pkt_data_bad_access_1_4() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data_meta]);
	r3 = *(u32*)(r1 + %[xdp_md_data]);
	r1 = r2;
	r1 += 8;
	if r1 <= r3 goto l0_%=;
	goto l1_%=;
l0_%=:	r0 = *(u64*)(r1 - 4);
l1_%=:	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_meta = sym xdp_md_data_meta,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("XDP pkt read, pkt_meta' <= pkt_data, bad access 2")
// __failure __msg("R1 {{min|max}} value is outside of the allowed memory range")
// __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pkt_data_bad_access_2_8() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data_meta]);
	r3 = *(u32*)(r1 + %[xdp_md_data]);
	r1 = r2;
	r1 += 8;
	if r1 <= r3 goto l0_%=;
	r0 = *(u64*)(r1 - 8);
l0_%=:	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_meta = sym xdp_md_data_meta,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("XDP pkt read, pkt_meta' <= pkt_data, corner case +1, good access")
// __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn corner_case_1_good_access_15() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data_meta]);
	r3 = *(u32*)(r1 + %[xdp_md_data]);
	r1 = r2;
	r1 += 9;
	if r1 <= r3 goto l0_%=;
	goto l1_%=;
l0_%=:	r0 = *(u64*)(r1 - 9);
l1_%=:	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_meta = sym xdp_md_data_meta,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("XDP pkt read, pkt_meta' <= pkt_data, corner case -1, bad access")
// __failure __msg("R1 {{min|max}} value is outside of the allowed memory range")
// __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn corner_case_1_bad_access_15() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data_meta]);
	r3 = *(u32*)(r1 + %[xdp_md_data]);
	r1 = r2;
	r1 += 7;
	if r1 <= r3 goto l0_%=;
	goto l1_%=;
l0_%=:	r0 = *(u64*)(r1 - 7);
l1_%=:	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_meta = sym xdp_md_data_meta,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("XDP pkt read, pkt_data <= pkt_meta', good access")
// __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn data_pkt_meta_good_access_2() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data_meta]);
	r3 = *(u32*)(r1 + %[xdp_md_data]);
	r1 = r2;
	r1 += 8;
	if r3 <= r1 goto l0_%=;
	r0 = *(u32*)(r1 - 5);
l0_%=:	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_meta = sym xdp_md_data_meta,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("XDP pkt read, pkt_data <= pkt_meta', corner case -1, bad access")
// __failure __msg("R1 {{min|max}} value is outside of the allowed memory range")
// __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn corner_case_1_bad_access_16() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data_meta]);
	r3 = *(u32*)(r1 + %[xdp_md_data]);
	r1 = r2;
	r1 += 6;
	if r3 <= r1 goto l0_%=;
	r0 = *(u64*)(r1 - 6);
l0_%=:	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_meta = sym xdp_md_data_meta,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("XDP pkt read, pkt_data <= pkt_meta', bad access 2")
// __failure __msg("R1 {{min|max}} value is outside of the allowed memory range")
// __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pkt_meta_bad_access_2_4() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data_meta]);
	r3 = *(u32*)(r1 + %[xdp_md_data]);
	r1 = r2;
	r1 += 8;
	if r3 <= r1 goto l0_%=;
l0_%=:	r0 = *(u32*)(r1 - 5);
	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_meta = sym xdp_md_data_meta,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("XDP pkt read, pkt_data <= pkt_meta', corner case, good access")
// __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn meta_corner_case_good_access_4() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data_meta]);
	r3 = *(u32*)(r1 + %[xdp_md_data]);
	r1 = r2;
	r1 += 7;
	if r3 <= r1 goto l0_%=;
	r0 = *(u64*)(r1 - 7);
l0_%=:	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_meta = sym xdp_md_data_meta,
        options(noreturn)
    );
}

// SEC("xdp")
// __description("XDP pkt read, pkt_data <= pkt_meta', corner case +1, good access")
// __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn corner_case_1_good_access_16() {
    asm!(
        r#"
	r2 = *(u32*)(r1 + %[xdp_md_data_meta]);
	r3 = *(u32*)(r1 + %[xdp_md_data]);
	r1 = r2;
	r1 += 8;
	if r3 <= r1 goto l0_%=;
	r0 = *(u64*)(r1 - 8);
l0_%=:	r0 = 0;
	exit;
"#,
        xdp_md_data = sym xdp_md_data,
        xdp_md_data_meta = sym xdp_md_data_meta,
        options(noreturn)
    );
}

// char _license[] SEC("license") = "GPL";
#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";
