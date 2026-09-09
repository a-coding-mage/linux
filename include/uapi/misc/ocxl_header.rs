/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/* Copyright 2017 IBM Corp. */

// Equivalent declarations for linux/types.h and linux/ioctl.h are supplied by
// the containing translation unit.

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ocxl_event_type {
    OCXL_AFU_EVENT_XSL_FAULT_ERROR = 0,
}

pub const OCXL_KERNEL_EVENT_FLAG_LAST: u16 = 0x0001; /* This is the last event pending */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocxl_kernel_event_header {
    pub type_: u16,
    pub flags: u16,
    pub reserved: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocxl_kernel_event_xsl_fault_error {
    pub addr: u64,
    pub dsisr: u64,
    pub count: u64,
    pub reserved: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocxl_ioctl_attach {
    pub amr: u64,
    pub reserved1: u64,
    pub reserved2: u64,
    pub reserved3: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocxl_ioctl_metadata {
    pub version: u16, /* struct version, always backwards compatible */

    /* Version 0 fields */
    pub afu_version_major: u8,
    pub afu_version_minor: u8,
    pub pasid: u32, /* PASID assigned to the current context */
    pub pp_mmio_size: u64, /* Per PASID MMIO size */
    pub global_mmio_size: u64,

    /* End version 0 fields */
    pub reserved: [u64; 13], /* Total of 16*u64 */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocxl_ioctl_p9_wait {
    pub thread_id: u16, /* The thread ID required to wake this thread */
    pub reserved1: u16,
    pub reserved2: u32,
    pub reserved3: [u64; 3],
}

pub const OCXL_IOCTL_FEATURES_FLAGS0_P9_WAIT: u8 = 0x01;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocxl_ioctl_features {
    pub flags: [u64; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocxl_ioctl_irq_fd {
    pub irq_offset: u64,
    pub eventfd: i32,
    pub reserved: u32,
}

/* ioctl numbers */
pub const OCXL_MAGIC: u32 = 0xCA;

// ioctl encoding follows the Linux _IOC/_IOW/_IOR definitions.
const IOC_NRBITS: u32 = 8;
const IOC_TYPEBITS: u32 = 8;
const IOC_SIZEBITS: u32 = 14;
const IOC_NRSHIFT: u32 = 0;
const IOC_TYPESHIFT: u32 = IOC_NRSHIFT + IOC_NRBITS;
const IOC_SIZESHIFT: u32 = IOC_TYPESHIFT + IOC_TYPEBITS;
const IOC_DIRSHIFT: u32 = IOC_SIZESHIFT + IOC_SIZEBITS;
const IOC_WRITE: u32 = 1;
const IOC_READ: u32 = 2;

const fn ioc(dir: u32, ty: u32, nr: u32, size: usize) -> u32 {
    (dir << IOC_DIRSHIFT)
        | (ty << IOC_TYPESHIFT)
        | (nr << IOC_NRSHIFT)
        | ((size as u32) << IOC_SIZESHIFT)
}

pub const OCXL_IOCTL_ATTACH: u32 = ioc(IOC_WRITE, OCXL_MAGIC, 0x10, core::mem::size_of::<ocxl_ioctl_attach>());
pub const OCXL_IOCTL_IRQ_ALLOC: u32 = ioc(IOC_READ, OCXL_MAGIC, 0x11, core::mem::size_of::<u64>());
pub const OCXL_IOCTL_IRQ_FREE: u32 = ioc(IOC_WRITE, OCXL_MAGIC, 0x12, core::mem::size_of::<u64>());
pub const OCXL_IOCTL_IRQ_SET_FD: u32 = ioc(IOC_WRITE, OCXL_MAGIC, 0x13, core::mem::size_of::<ocxl_ioctl_irq_fd>());
pub const OCXL_IOCTL_GET_METADATA: u32 = ioc(IOC_READ, OCXL_MAGIC, 0x14, core::mem::size_of::<ocxl_ioctl_metadata>());
pub const OCXL_IOCTL_ENABLE_P9_WAIT: u32 = ioc(IOC_READ, OCXL_MAGIC, 0x15, core::mem::size_of::<ocxl_ioctl_p9_wait>());
pub const OCXL_IOCTL_GET_FEATURES: u32 = ioc(IOC_READ, OCXL_MAGIC, 0x16, core::mem::size_of::<ocxl_ioctl_features>());

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
