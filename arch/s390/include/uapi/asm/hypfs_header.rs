/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Structures for hypfs interface
 *
 * Copyright IBM Corp. 2013
 *
 * Author: Martin Schwidefsky <schwidefsky@de.ibm.com>
 */

/* C header guard: _ASM_HYPFS_H */
/* Dependency: Linux __u* types are represented by Rust unsigned integer types. */

/*
 * IOCTL for binary interface /sys/kernel/debug/diag_304
 */
#[repr(C, packed)]
pub struct hypfs_diag304 {
    pub args: [u32; 2],
    pub data: u64,
    pub rc: u64,
}

pub const HYPFS_IOCTL_MAGIC: u32 = 0x10;

/*
 * _IOWR(HYPFS_IOCTL_MAGIC, 0x20, struct hypfs_diag304)
 * The ioctl encoding macro is supplied by the consuming interface.
 */
pub const HYPFS_DIAG304: u32 = _IOWR!(HYPFS_IOCTL_MAGIC, 0x20, hypfs_diag304);

/*
 * Structures for binary interface /sys/kernel/debug/diag_0c
 */
#[repr(C)]
pub struct hypfs_diag0c_hdr {
    pub len: u64,              /* Length of diag0c buffer without header */
    pub version: u16,          /* Version of header */
    pub reserved1: [i8; 6],    /* Reserved */
    pub tod_ext: [i8; 16],     /* TOD clock for diag0c */
    pub count: u64,            /* Number of entries (CPUs) in diag0c array */
    pub reserved2: [i8; 24],   /* Reserved */
}

#[repr(C)]
pub struct hypfs_diag0c_entry {
    pub date: [i8; 8],         /* MM/DD/YY in EBCDIC */
    pub time: [i8; 8],         /* HH:MM:SS in EBCDIC */
    pub virtcpu: u64,          /* Virtual time consumed by the virt CPU (us) */
    pub totalproc: u64,        /* Total of virtual and simulation time (us) */
    pub cpu: u32,              /* Linux logical CPU number */
    pub reserved: u32,         /* Align to 8 byte */
}

#[repr(C)]
pub struct hypfs_diag0c_data {
    pub hdr: hypfs_diag0c_hdr,       /* 64 byte header */
    pub entry: [hypfs_diag0c_entry; 0], /* diag0c entry array */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
