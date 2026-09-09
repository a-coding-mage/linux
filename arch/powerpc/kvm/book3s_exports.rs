// SPDX-License-Identifier: GPL-2.0-only
/*
 *
 * Copyright SUSE Linux Products GmbH 2009
 *
 * Authors: Alexander Graf <agraf@suse.de>
 */

// The C source includes <linux/export.h>, <asm/kvm_ppc.h>, and
// <asm/kvm_book3s.h>. Their declarations are supplied by other files.

#[cfg(CONFIG_KVM_BOOK3S_HV_POSSIBLE)]
extern "C" {
    // Equivalent of EXPORT_SYMBOL_GPL(kvmppc_hv_entry_trampoline).
    pub static kvmppc_hv_entry_trampoline: u8;
}

#[cfg(CONFIG_KVM_BOOK3S_PR_POSSIBLE)]
extern "C" {
    // Equivalent of EXPORT_SYMBOL_GPL(kvmppc_entry_trampoline).
    pub static kvmppc_entry_trampoline: u8;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
