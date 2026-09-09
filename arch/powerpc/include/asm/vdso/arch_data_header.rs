/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2002 Peter Bergner <bergner@vnet.ibm.com>, IBM
 * Copyright (C) 2005 Benjamin Herrenschmidy <benh@kernel.crashing.org>,
 *                    IBM Corp.
 */

// linux/unistd.h and linux/types.h provide NR_syscalls and the __u* types
// used by the original header.

pub const SYSCALL_MAP_SIZE: usize = ((NR_syscalls + 31) / 32);

#[cfg(CONFIG_PPC64)]
#[repr(C)]
pub struct vdso_arch_data {
    pub tb_ticks_per_sec: u64,          /* Timebase tics / sec */
    pub dcache_block_size: u32,         /* L1 d-cache block size     */
    pub icache_block_size: u32,         /* L1 i-cache block size     */
    pub dcache_log_block_size: u32,     /* L1 d-cache log block size */
    pub icache_log_block_size: u32,     /* L1 i-cache log block size */
    pub syscall_map: [u32; SYSCALL_MAP_SIZE], /* Map of syscalls  */
    pub compat_syscall_map: [u32; SYSCALL_MAP_SIZE], /* Map of compat syscalls */
}

#[cfg(not(CONFIG_PPC64))]
#[repr(C)]
pub struct vdso_arch_data {
    pub tb_ticks_per_sec: u64,          /* Timebase tics / sec */
    pub syscall_map: [u32; SYSCALL_MAP_SIZE], /* Map of syscalls */
    pub compat_syscall_map: [u32; 0],   /* No compat syscalls on PPC32 */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
