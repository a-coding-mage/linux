// SPDX-License-Identifier: GPL-2.0-only
/*
 * VFIO ZPCI devices support
 *
 * Copyright (C) IBM Corp. 2022.  All rights reserved.
 *	Author(s): Pierre Morel <pmorel@linux.ibm.com>
 */

// C dependencies:
// #include <linux/kvm_host.h>
// #include <linux/export.h>

// `struct zpci_kvm_hook` is supplied by the kernel KVM headers.
#[repr(C)]
pub struct zpci_kvm_hook {
    _private: [u8; 0],
}

#[no_mangle]
pub static mut zpci_kvm_hook: zpci_kvm_hook = zpci_kvm_hook { _private: [] };

// C: EXPORT_SYMBOL_GPL(zpci_kvm_hook);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
