/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Diag ioctls and its associated structures definitions.
 *
 * Copyright IBM Corp. 2024
 */

/* Dependency intent: linux/types.h supplies __u64 and size_t. */

pub const DIAG_MAGIC_STR: u8 = b'D';

#[repr(C)]
pub struct diag324_pib {
    pub address: u64,
    pub sequence: u64,
}

#[repr(C)]
pub struct diag310_memtop {
    pub address: u64,
    pub nesting_lvl: u64,
}

/* Diag ioctl definitions. */
/* Dependency intent: _IOC/_IOWR/_IOR use the Linux ioctl encoding. */
const IOC_NRBITS: usize = 8;
const IOC_TYPEBITS: usize = 8;
const IOC_SIZEBITS: usize = 14;
const IOC_NRSHIFT: usize = 0;
const IOC_TYPESHIFT: usize = IOC_NRSHIFT + IOC_NRBITS;
const IOC_SIZESHIFT: usize = IOC_TYPESHIFT + IOC_TYPEBITS;
const IOC_DIRSHIFT: usize = IOC_SIZESHIFT + IOC_SIZEBITS;
const IOC_NONE: usize = 0;
const IOC_WRITE: usize = 1;
const IOC_READ: usize = 2;

const fn ioc(dir: usize, type_: usize, nr: usize, size: usize) -> usize {
    (dir << IOC_DIRSHIFT)
        | (type_ << IOC_TYPESHIFT)
        | (nr << IOC_NRSHIFT)
        | (size << IOC_SIZESHIFT)
}

const fn iowr<T>(type_: usize, nr: usize) -> usize {
    ioc(IOC_READ | IOC_WRITE, type_, nr, core::mem::size_of::<T>())
}

const fn ior<T>(type_: usize, nr: usize) -> usize {
    ioc(IOC_READ, type_, nr, core::mem::size_of::<T>())
}

pub const DIAG324_GET_PIBBUF: usize = iowr::<diag324_pib>(DIAG_MAGIC_STR as usize, 0x77);
pub const DIAG324_GET_PIBLEN: usize = ior::<usize>(DIAG_MAGIC_STR as usize, 0x78);
pub const DIAG310_GET_STRIDE: usize = ior::<usize>(DIAG_MAGIC_STR as usize, 0x79);
pub const DIAG310_GET_MEMTOPLEN: usize = iowr::<usize>(DIAG_MAGIC_STR as usize, 0x7a);
pub const DIAG310_GET_MEMTOPBUF: usize = iowr::<diag310_memtop>(DIAG_MAGIC_STR as usize, 0x7b);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
