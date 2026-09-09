/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* Translated from sparc/include/uapi/asm/openpromio.h. */

/*
 * SunOS and Solaris /dev/openprom definitions. The ioctl values
 * were chosen to be exactly equal to the SunOS equivalents.
 */

#[repr(C)]
pub struct openpromio {
    /* Actual size of the oprom_array. */
    pub oprom_size: core::ffi::c_uint,
    /* Holds property names and values. */
    pub oprom_array: [core::ffi::c_char; 0],
}

/* Maximum size of oprom_array. */
pub const OPROMMAXPARAM: u32 = 4096;

pub const OPROMGETOPT: u32 = 0x20004F01;
pub const OPROMSETOPT: u32 = 0x20004F02;
pub const OPROMNXTOPT: u32 = 0x20004F03;
pub const OPROMSETOPT2: u32 = 0x20004F04;
pub const OPROMNEXT: u32 = 0x20004F05;
pub const OPROMCHILD: u32 = 0x20004F06;
pub const OPROMGETPROP: u32 = 0x20004F07;
pub const OPROMNXTPROP: u32 = 0x20004F08;
pub const OPROMU2P: u32 = 0x20004F09;
pub const OPROMGETCONS: u32 = 0x20004F0A;
pub const OPROMGETFBNAME: u32 = 0x20004F0B;
pub const OPROMGETBOOTARGS: u32 = 0x20004F0C;

/* Linux extensions. Arguments in oprom_array: */
/* int node - Sets current node */
pub const OPROMSETCUR: u32 = 0x20004FF0;
/* int pci_bus, pci_devfn - Sets current node to PCI device's node */
pub const OPROMPCI2NODE: u32 = 0x20004FF1;
/* char path[] - Set current node from fully qualified PROM path */
pub const OPROMPATH2NODE: u32 = 0x20004FF2;

/* Return values from OPROMGETCONS: */
pub const OPROMCONS_NOT_WSCONS: u32 = 0;
/* stdin device is kbd */
pub const OPROMCONS_STDIN_IS_KBD: u32 = 0x1;
/* stdout is a framebuffer */
pub const OPROMCONS_STDOUT_IS_FB: u32 = 0x2;
/* supports openboot */
pub const OPROMCONS_OPENPROM: u32 = 0x4;

/* NetBSD/OpenBSD /dev/openprom definitions. */

#[repr(C)]
pub struct opiocdesc {
    /* PROM Node ID (value-result) */
    pub op_nodeid: core::ffi::c_int,
    /* Length of op_name. */
    pub op_namelen: core::ffi::c_int,
    /* Pointer to the property name. */
    pub op_name: *mut core::ffi::c_char,
    /* Length of op_buf (value-result) */
    pub op_buflen: core::ffi::c_int,
    /* Pointer to buffer. */
    pub op_buf: *mut core::ffi::c_char,
}

/* __user annotations are kernel-only and have no Rust representation. */

const IOC_NRBITS: u32 = 8;
const IOC_TYPEBITS: u32 = 8;
const IOC_SIZEBITS: u32 = 14;
const IOC_NRSHIFT: u32 = 0;
const IOC_TYPESHIFT: u32 = IOC_NRSHIFT + IOC_NRBITS;
const IOC_SIZESHIFT: u32 = IOC_TYPESHIFT + IOC_TYPEBITS;
const IOC_DIRSHIFT: u32 = IOC_SIZESHIFT + IOC_SIZEBITS;
const IOC_WRITE: u32 = 1;
const IOC_READ: u32 = 2;

const fn ioc(dir: u32, ty: u32, nr: u32, size: u32) -> u32 {
    (dir << IOC_DIRSHIFT) | (ty << IOC_TYPESHIFT) | (nr << IOC_NRSHIFT) | (size << IOC_SIZESHIFT)
}

const fn iowr<T>(ty: u32, nr: u32) -> u32 {
    ioc(IOC_READ | IOC_WRITE, ty, nr, core::mem::size_of::<T>() as u32)
}

const fn iow<T>(ty: u32, nr: u32) -> u32 {
    ioc(IOC_WRITE, ty, nr, core::mem::size_of::<T>() as u32)
}

const fn ior<T>(ty: u32, nr: u32) -> u32 {
    ioc(IOC_READ, ty, nr, core::mem::size_of::<T>() as u32)
}

pub const OPIOCGET: u32 = iowr::<opiocdesc>(b'O' as u32, 1);
pub const OPIOCSET: u32 = iow::<opiocdesc>(b'O' as u32, 2);
pub const OPIOCNEXTPROP: u32 = iowr::<opiocdesc>(b'O' as u32, 3);
pub const OPIOCGETOPTNODE: u32 = ior::<core::ffi::c_int>(b'O' as u32, 4);
pub const OPIOCGETNEXT: u32 = iowr::<core::ffi::c_int>(b'O' as u32, 5);
pub const OPIOCGETCHILD: u32 = iowr::<core::ffi::c_int>(b'O' as u32, 6);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
