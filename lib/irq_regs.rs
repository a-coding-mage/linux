// SPDX-License-Identifier: GPL-2.0-or-later
/* saved per-CPU IRQ register pointer
 *
 * Copyright (C) 2006 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// The Linux kernel headers supplying `pt_regs` and per-CPU storage are
// external dependencies of this translation.

#[cfg(not(ARCH_HAS_OWN_IRQ_REGS))]
#[no_mangle]
pub static mut __irq_regs: *mut pt_regs = core::ptr::null_mut();


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
