// SPDX-License-Identifier: GPL-2.0-only

// The C source includes <linux/kbuild.h>, <nvhe/memory.h>, and <nvhe/pkvm.h>.
// Their Rust equivalents provide the DEFINE! macro and the hyp_page,
// pkvm_hyp_vm, and pkvm_hyp_vcpu types referenced below.

pub fn main() -> i32 {
	DEFINE!(STRUCT_HYP_PAGE_SIZE, core::mem::size_of::<hyp_page>());
	DEFINE!(PKVM_HYP_VM_SIZE, core::mem::size_of::<pkvm_hyp_vm>());
	DEFINE!(PKVM_HYP_VCPU_SIZE, core::mem::size_of::<pkvm_hyp_vcpu>());
	0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
