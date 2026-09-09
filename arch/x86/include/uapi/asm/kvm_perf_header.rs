/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependencies supplied by the corresponding asm headers:
// <asm/svm.h>, <asm/vmx.h>, <asm/kvm.h>

pub const DECODE_STR_LEN: usize = 20;

pub const VCPU_ID: &str = "vcpu_id";

pub const KVM_ENTRY_TRACE: &str = "kvm:kvm_entry";
pub const KVM_EXIT_TRACE: &str = "kvm:kvm_exit";
pub const KVM_EXIT_REASON: &str = "exit_reason";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
