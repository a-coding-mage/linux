/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * This is a module to test the HMM (Heterogeneous Memory Management) API
 * of the kernel. It allows a userspace program to expose its entire address
 * space through the HMM test module device file.
 */

/*
 * Structure to pass to the HMM test driver to mimic a device accessing
 * system memory and ZONE_DEVICE private memory through device page tables.
 *
 * @addr: (in) user address the device will read/write
 * @ptr: (in) user address where device data is copied to/from
 * @npages: (in) number of pages to read/write
 * @cpages: (out) number of pages copied
 * @faults: (out) number of device page faults seen
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hmm_dmirror_cmd {
    pub addr: u64,
    pub ptr: u64,
    pub npages: u64,
    pub cpages: u64,
    pub faults: u64,
}

/* Linux _IOWR encoding, corresponding to <linux/ioctl.h>. */
const fn _iowr<T>(ty: u8, nr: u8) -> u64 {
    ((3u64 << 30)
        | ((core::mem::size_of::<T>() as u64) << 16)
        | ((ty as u64) << 8)
        | nr as u64)
}

/* Expose the address space of the calling process through hmm device file */
pub const HMM_DMIRROR_READ: u64 = _iowr::<hmm_dmirror_cmd>(b'H', 0x00);
pub const HMM_DMIRROR_WRITE: u64 = _iowr::<hmm_dmirror_cmd>(b'H', 0x01);
pub const HMM_DMIRROR_MIGRATE_TO_DEV: u64 = _iowr::<hmm_dmirror_cmd>(b'H', 0x02);
pub const HMM_DMIRROR_MIGRATE_TO_SYS: u64 = _iowr::<hmm_dmirror_cmd>(b'H', 0x03);
pub const HMM_DMIRROR_SNAPSHOT: u64 = _iowr::<hmm_dmirror_cmd>(b'H', 0x04);
pub const HMM_DMIRROR_EXCLUSIVE: u64 = _iowr::<hmm_dmirror_cmd>(b'H', 0x05);
pub const HMM_DMIRROR_CHECK_EXCLUSIVE: u64 = _iowr::<hmm_dmirror_cmd>(b'H', 0x06);
pub const HMM_DMIRROR_RELEASE: u64 = _iowr::<hmm_dmirror_cmd>(b'H', 0x07);
pub const HMM_DMIRROR_FLAGS: u64 = _iowr::<hmm_dmirror_cmd>(b'H', 0x08);
pub const HMM_DMIRROR_READ_UNLOCKED: u64 = _iowr::<hmm_dmirror_cmd>(b'H', 0x09);

pub const HMM_DMIRROR_FLAG_FAIL_ALLOC: u64 = 1u64 << 0;

/* Values returned in hmm_dmirror_cmd.ptr for HMM_DMIRROR_SNAPSHOT. */
#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hmm_dmirror_prot {
    HMM_DMIRROR_PROT_ERROR = 0xFF,
    HMM_DMIRROR_PROT_NONE = 0x00,
    HMM_DMIRROR_PROT_READ = 0x01,
    HMM_DMIRROR_PROT_WRITE = 0x02,
    HMM_DMIRROR_PROT_PMD = 0x04,
    HMM_DMIRROR_PROT_PUD = 0x08,
    HMM_DMIRROR_PROT_ZERO = 0x10,
    HMM_DMIRROR_PROT_DEV_PRIVATE_LOCAL = 0x20,
    HMM_DMIRROR_PROT_DEV_PRIVATE_REMOTE = 0x30,
    HMM_DMIRROR_PROT_DEV_COHERENT_LOCAL = 0x40,
    HMM_DMIRROR_PROT_DEV_COHERENT_REMOTE = 0x50,
}

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hmm_dmirror_memory_type {
    /* 0 is reserved to catch uninitialized type fields */
    HMM_DMIRROR_MEMORY_DEVICE_PRIVATE = 1,
    HMM_DMIRROR_MEMORY_DEVICE_COHERENT,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
