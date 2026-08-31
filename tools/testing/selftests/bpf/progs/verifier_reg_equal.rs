// SPDX-License-Identifier: GPL-2.0

// C includes translated as dependency intent:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include "bpf_misc.h"

unsafe extern "C" {
    fn bpf_ktime_get_ns() -> u64;
}

// SEC("socket")
// __description("check w reg equal if r reg upper32 bits 0")
// __success
// __naked
#[unsafe(no_mangle)]
pub unsafe extern "C" fn subreg_equality_1() {
    core::arch::asm!(
        r#"
	call {bpf_ktime_get_ns};
	*(u64 *)(r10 - 8) = r0;
	r2 = *(u32 *)(r10 - 8);
	/* At this point upper 4-bytes of r2 are 0,
	 * thus insn w3 = w2 should propagate reg id,
	 * and w2 < 9 comparison would also propagate
	 * the range for r3.
	 */
	w3 = w2;
	if w2 < 9 goto 0f;
	exit;
0:	if r3 < 9 goto 1f;
	/* r1 read is illegal at this point */
	r0 -= r1;
1:	exit;
"#,
        bpf_ktime_get_ns = sym bpf_ktime_get_ns,
        options(noreturn)
    );
}

// SEC("socket")
// __description("check w reg not equal if r reg upper32 bits not 0")
// __failure
// __msg("R1 !read_ok")
// __naked
#[unsafe(no_mangle)]
pub unsafe extern "C" fn subreg_equality_2() {
    core::arch::asm!(
        r#"
	call {bpf_ktime_get_ns};
	r2 = r0;
	/* Upper 4-bytes of r2 may not be 0, thus insn
	 * w3 = w2 should not propagate reg id,	and
	 * w2 < 9 comparison should not propagate
	 * the range for r3 either.
	 */
	w3 = w2;
	if w2 < 9 goto 0f;
	exit;
0:	if r3 < 9 goto 1f;
	/* r1 read is illegal at this point */
	r0 -= r1;
1:	exit;
"#,
        bpf_ktime_get_ns = sym bpf_ktime_get_ns,
        options(noreturn)
    );
}

// char _license[] SEC("license") = "GPL";
#[unsafe(no_mangle)]
#[unsafe(link_section = "license")]
pub static _license: [u8; 4] = *b"GPL\0";
