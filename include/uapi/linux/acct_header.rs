/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * BSD Process Accounting for Linux - Definitions
 *
 * Direct Rust translation of the corresponding C UAPI header.
 */

/* linux/types.h, asm/param.h, and asm/byteorder.h provide the source types
 * and build-time constants referenced by this header. */

/*
 * comp_t is a 16-bit "floating" point number with a 3-bit base 8
 * exponent and a 13-bit fraction.
 * comp2_t is 24-bit with 5-bit base 2 exponent and 20 bit fraction
 * (leading 1 not stored).
 */
pub type comp_t = u16;
pub type comp2_t = u32;

pub const ACCT_COMM: usize = 16;

#[repr(C)]
pub struct acct {
    pub ac_flag: i8,       /* Flags */
    pub ac_version: i8,    /* Always set to ACCT_VERSION */
    /* for binary compatibility back until 2.0 */
    pub ac_uid16: u16,     /* LSB of Real User ID */
    pub ac_gid16: u16,     /* LSB of Real Group ID */
    pub ac_tty: u16,       /* Control Terminal */
    /* __u32 range means times from 1970 to 2106 */
    pub ac_btime: u32,     /* Process Creation Time */
    pub ac_utime: comp_t,  /* User Time */
    pub ac_stime: comp_t,  /* System Time */
    pub ac_etime: comp_t,  /* Elapsed Time */
    pub ac_mem: comp_t,    /* Average Memory Usage */
    pub ac_io: comp_t,     /* Chars Transferred */
    pub ac_rw: comp_t,     /* Blocks Read or Written */
    pub ac_minflt: comp_t, /* Minor Pagefaults */
    pub ac_majflt: comp_t, /* Major Pagefaults */
    pub ac_swaps: comp_t,  /* Number of Swaps */
    /* m68k had no padding here. */
    #[cfg(not(all(target_arch = "m68k", feature = "kernel")))]
    pub ac_ahz: u16,       /* AHZ */
    pub ac_exitcode: u32,  /* Exitcode */
    pub ac_comm: [i8; ACCT_COMM + 1], /* Command Name */
    pub ac_etime_hi: u8,   /* Elapsed Time MSB */
    pub ac_etime_lo: u16,  /* Elapsed Time LSB */
    pub ac_uid: u32,       /* Real User ID */
    pub ac_gid: u32,       /* Real Group ID */
}

#[repr(C)]
pub struct acct_v3 {
    pub ac_flag: i8,       /* Flags */
    pub ac_version: i8,    /* Always set to ACCT_VERSION */
    pub ac_tty: u16,       /* Control Terminal */
    pub ac_exitcode: u32,  /* Exitcode */
    pub ac_uid: u32,       /* Real User ID */
    pub ac_gid: u32,       /* Real Group ID */
    pub ac_pid: u32,       /* Process ID */
    pub ac_ppid: u32,      /* Parent Process ID */
    /* __u32 range means times from 1970 to 2106 */
    pub ac_btime: u32,     /* Process Creation Time */
    #[cfg(feature = "kernel")]
    pub ac_etime: u32,     /* Elapsed Time */
    #[cfg(not(feature = "kernel"))]
    pub ac_etime: f32,     /* Elapsed Time */
    pub ac_utime: comp_t,  /* User Time */
    pub ac_stime: comp_t,  /* System Time */
    pub ac_mem: comp_t,    /* Average Memory Usage */
    pub ac_io: comp_t,     /* Chars Transferred */
    pub ac_rw: comp_t,     /* Blocks Read or Written */
    pub ac_minflt: comp_t, /* Minor Pagefaults */
    pub ac_majflt: comp_t, /* Major Pagefaults */
    pub ac_swaps: comp_t,  /* Number of Swaps */
    pub ac_comm: [i8; ACCT_COMM], /* Command Name */
}

/* accounting flags */
pub const AFORK: u8 = 0x01;   /* executed fork, but did not exec */
pub const ASU: u8 = 0x02;     /* used super-user privileges */
pub const ACOMPAT: u8 = 0x04; /* used compatibility mode */
pub const ACORE: u8 = 0x08;   /* dumped core */
pub const AXSIG: u8 = 0x10;   /* was killed by a signal */
pub const AGROUP: u8 = 0x20;  /* was the last task of the process */

/* Endianness is selected by the source build's byte-order configuration. */
#[cfg(any(target_endian = "big"))]
pub const ACCT_BYTEORDER: u8 = 0x80;
#[cfg(any(target_endian = "little"))]
pub const ACCT_BYTEORDER: u8 = 0x00;

/* These constants are defined only outside the kernel build. */
#[cfg(not(feature = "kernel"))]
pub const ACCT_VERSION: i32 = 2;
#[cfg(not(feature = "kernel"))]
pub const AHZ: u32 = HZ;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
