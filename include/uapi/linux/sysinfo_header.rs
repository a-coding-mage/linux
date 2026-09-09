/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency supplied by the Linux types translation.

pub const SI_LOAD_SHIFT: i32 = 16;

#[repr(C)]
pub struct sysinfo {
    pub uptime: __kernel_long_t, /* Seconds since boot */
    pub loads: [__kernel_ulong_t; 3], /* 1, 5, and 15 minute load averages */
    pub totalram: __kernel_ulong_t, /* Total usable main memory size */
    pub freeram: __kernel_ulong_t, /* Available memory size */
    pub sharedram: __kernel_ulong_t, /* Amount of shared memory */
    pub bufferram: __kernel_ulong_t, /* Memory used by buffers */
    pub totalswap: __kernel_ulong_t, /* Total swap space size */
    pub freeswap: __kernel_ulong_t, /* swap space still available */
    pub procs: __u16, /* Number of current processes */
    pub pad: __u16, /* Explicit padding for m68k */
    pub totalhigh: __kernel_ulong_t, /* Total high memory size */
    pub freehigh: __kernel_ulong_t, /* Available high memory size */
    pub mem_unit: __u32, /* Memory unit size in bytes */
    pub _f: [i8; 20 - 2 * core::mem::size_of::<__kernel_ulong_t>() - core::mem::size_of::<__u32>()], /* Padding: libc5 uses this.. */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
