// SPDX-License-Identifier: GPL-2.0
/* Converted from tools/testing/selftests/bpf/verifier/int_ptr.c */

// C dependencies removed from executable Rust:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include "bpf_misc.h"

unsafe extern "C" {
    fn bpf_strtoul();
}

// SEC("socket")
// __description("arg pointer to long uninitialized")
// __success
#[unsafe(naked)]
pub unsafe extern "C" fn arg_ptr_to_long_uninitialized() {
    core::arch::naked_asm!(
        r#"
	/* bpf_strtoul arg1 (buf) */
	r7 = r10;
	r7 += -8;
	r0 = 0x00303036;
	*(u64*)(r7 + 0) = r0;
	r1 = r7;
	/* bpf_strtoul arg2 (buf_len) */
	r2 = 4;
	/* bpf_strtoul arg3 (flags) */
	r3 = 0;
	/* bpf_strtoul arg4 (res) */
	r7 += -8;
	r4 = r7;
	/* bpf_strtoul() */
	call {bpf_strtoul};
	r0 = 1;
	exit;
"#,
        bpf_strtoul = sym bpf_strtoul,
    );
}

// SEC("socket")
// __description("arg pointer to long half-uninitialized")
// __success
// __retval(0)
#[unsafe(naked)]
pub unsafe extern "C" fn ptr_to_long_half_uninitialized() {
    core::arch::naked_asm!(
        r#"
	/* bpf_strtoul arg1 (buf) */
	r7 = r10;
	r7 += -8;
	r0 = 0x00303036;
	*(u64*)(r7 + 0) = r0;
	r1 = r7;
	/* bpf_strtoul arg2 (buf_len) */
	r2 = 4;
	/* bpf_strtoul arg3 (flags) */
	r3 = 0;
	/* bpf_strtoul arg4 (res) */
	r7 += -8;
	*(u32*)(r7 + 0) = r0;
	r4 = r7;
	/* bpf_strtoul() */
	call {bpf_strtoul};
	r0 = 0;
	exit;
"#,
        bpf_strtoul = sym bpf_strtoul,
    );
}

// SEC("cgroup/sysctl")
// __description("arg pointer to long misaligned")
// __failure __msg("misaligned stack access off -20+0 size 8")
#[unsafe(naked)]
pub unsafe extern "C" fn arg_ptr_to_long_misaligned() {
    core::arch::naked_asm!(
        r#"
	/* bpf_strtoul arg1 (buf) */
	r7 = r10;
	r7 += -8;
	r0 = 0x00303036;
	*(u64*)(r7 + 0) = r0;
	r1 = r7;
	/* bpf_strtoul arg2 (buf_len) */
	r2 = 4;
	/* bpf_strtoul arg3 (flags) */
	r3 = 0;
	/* bpf_strtoul arg4 (res) */
	r7 += -12;
	r0 = 0;
	*(u32*)(r7 + 0) = r0;
	*(u64*)(r7 + 4) = r0;
	r4 = r7;
	/* bpf_strtoul() */
	call {bpf_strtoul};
	r0 = 1;
	exit;
"#,
        bpf_strtoul = sym bpf_strtoul,
    );
}

// SEC("cgroup/sysctl")
// __description("arg pointer to long size < sizeof(long)")
// __failure __msg("invalid write to stack R4 off=-4 size=8")
#[unsafe(naked)]
pub unsafe extern "C" fn to_long_size_sizeof_long() {
    core::arch::naked_asm!(
        r#"
	/* bpf_strtoul arg1 (buf) */
	r7 = r10;
	r7 += -16;
	r0 = 0x00303036;
	*(u64*)(r7 + 0) = r0;
	r1 = r7;
	/* bpf_strtoul arg2 (buf_len) */
	r2 = 4;
	/* bpf_strtoul arg3 (flags) */
	r3 = 0;
	/* bpf_strtoul arg4 (res) */
	r7 += 12;
	*(u32*)(r7 + 0) = r0;
	r4 = r7;
	/* bpf_strtoul() */
	call {bpf_strtoul};
	r0 = 1;
	exit;
"#,
        bpf_strtoul = sym bpf_strtoul,
    );
}

// SEC("cgroup/sysctl")
// __description("arg pointer to long initialized")
// __success
#[unsafe(naked)]
pub unsafe extern "C" fn arg_ptr_to_long_initialized() {
    core::arch::naked_asm!(
        r#"
	/* bpf_strtoul arg1 (buf) */
	r7 = r10;
	r7 += -8;
	r0 = 0x00303036;
	*(u64*)(r7 + 0) = r0;
	r1 = r7;
	/* bpf_strtoul arg2 (buf_len) */
	r2 = 4;
	/* bpf_strtoul arg3 (flags) */
	r3 = 0;
	/* bpf_strtoul arg4 (res) */
	r7 += -8;
	*(u64*)(r7 + 0) = r0;
	r4 = r7;
	/* bpf_strtoul() */
	call {bpf_strtoul};
	r0 = 1;
	exit;
"#,
        bpf_strtoul = sym bpf_strtoul,
    );
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";
