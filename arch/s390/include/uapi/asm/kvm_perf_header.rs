/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Definitions for perf-kvm on s390
 *
 * Copyright 2014 IBM Corp.
 * Author(s): Alexander Yarygin <yarygin@linux.vnet.ibm.com>
 */

// Dependency from the original header: <asm/sie.h>

pub const DECODE_STR_LEN: usize = 40;

pub const VCPU_ID: &str = "id";

pub const KVM_ENTRY_TRACE: &str = "kvm:kvm_s390_sie_enter";
pub const KVM_EXIT_TRACE: &str = "kvm:kvm_s390_sie_exit";
pub const KVM_EXIT_REASON: &str = "icptcode";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
