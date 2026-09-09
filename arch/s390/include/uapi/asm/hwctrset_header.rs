/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Copyright IBM Corp. 2021
 * Interface implementation for communication with the CPU Measurement
 * counter facility device driver.
 *
 * Author(s): Thomas Richter <tmricht@linux.ibm.com>
 *
 * Define for ioctl() commands to communicate with the CPU Measurement
 * counter facility device driver.
 */

pub const S390_HWCTR_DEVICE: &str = "hwctr";
pub const S390_HWCTR_START_VERSION: u64 = 1;

#[repr(C)]
pub struct s390_ctrset_start {
    /* Version of interface */
    pub version: u64,
    /* # of bytes required */
    pub data_bytes: u64,
    /* Length of CPU mask in bytes */
    pub cpumask_len: u64,
    /* Pointer to CPU mask */
    pub cpumask: *mut u64,
    /* Bit mask of counter sets to get */
    pub counter_sets: u64,
}

#[repr(C)]
pub struct s390_ctrset_setdata {
    /* Counter set number */
    pub set: u32,
    /* # of counters stored in cv[] */
    pub no_cnts: u32,
    /* Counter values (variable length) */
    pub cv: [u64; 0],
}

#[repr(C)]
pub struct s390_ctrset_cpudata {
    /* CPU number */
    pub cpu_nr: u32,
    /* # of counters sets in data[] */
    pub no_sets: u32,
    pub data: [s390_ctrset_setdata; 0],
}

#[repr(C)]
pub struct s390_ctrset_read {
    /* Total # of CPUs data taken from */
    pub no_cpus: u64,
    pub data: [s390_ctrset_cpudata; 0],
}

/* Random magic # for ioctls */
pub const S390_HWCTR_MAGIC: u8 = b'C';

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
    (dir << IOC_DIRSHIFT)
        | (ty << IOC_TYPESHIFT)
        | (nr << IOC_NRSHIFT)
        | (size << IOC_SIZESHIFT)
}

const fn iowr<T>(ty: u32, nr: u32) -> u32 {
    ioc(IOC_READ | IOC_WRITE, ty, nr, core::mem::size_of::<T>() as u32)
}

const fn io(ty: u32, nr: u32) -> u32 {
    ioc(0, ty, nr, 0)
}

pub const S390_HWCTR_START: u32 = iowr::<s390_ctrset_start>(S390_HWCTR_MAGIC as u32, 1);
pub const S390_HWCTR_STOP: u32 = io(S390_HWCTR_MAGIC as u32, 2);
pub const S390_HWCTR_READ: u32 = iowr::<s390_ctrset_read>(S390_HWCTR_MAGIC as u32, 3);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
