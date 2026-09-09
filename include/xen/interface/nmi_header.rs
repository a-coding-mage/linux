/* SPDX-License-Identifier: MIT */
/******************************************************************************
 * nmi.h
 *
 * NMI callback registration and reason codes.
 *
 * Copyright (c) 2005, Keir Fraser <keir@xensource.com>
 */

//! NMI callback registration and reason codes.

use core::ffi::c_ulong;

/*
 * NMI reason codes:
 * Currently these are x86-specific, stored in arch_shared_info.nmi_reason.
 */
/* I/O-check error reported via ISA port 0x61, bit 6. */
pub const _XEN_NMIREASON_io_error: u32 = 0;
pub const XEN_NMIREASON_io_error: c_ulong = 1u64 << _XEN_NMIREASON_io_error;

/* PCI SERR reported via ISA port 0x61, bit 7. */
pub const _XEN_NMIREASON_pci_serr: u32 = 1;
pub const XEN_NMIREASON_pci_serr: c_ulong = 1u64 << _XEN_NMIREASON_pci_serr;

/* Unknown hardware-generated NMI. */
pub const _XEN_NMIREASON_unknown: u32 = 2;
pub const XEN_NMIREASON_unknown: c_ulong = 1u64 << _XEN_NMIREASON_unknown;

/*
 * long nmi_op(unsigned int cmd, void *arg)
 * NB. All ops return zero on success, else a negative error code.
 */

/*
 * Register NMI callback for this (calling) VCPU. Currently this only makes
 * sense for domain 0, vcpu 0. All other callers will be returned EINVAL.
 * arg == pointer to xennmi_callback structure.
 */
pub const XENNMI_register_callback: u32 = 0;

#[repr(C)]
pub struct xennmi_callback {
    pub handler_address: c_ulong,
    pub pad: c_ulong,
}

/* Translation of DEFINE_GUEST_HANDLE_STRUCT(xennmi_callback). */
pub type xen_guest_handle_xennmi_callback = *mut xennmi_callback;

/*
 * Deregister NMI callback for this (calling) VCPU.
 * arg == NULL.
 */
pub const XENNMI_unregister_callback: u32 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
