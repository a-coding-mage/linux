/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * asm/bootinfo.h -- Definition of the Linux/m68k boot information structure
 *
 * Copyright 1992 by Greg Harp
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file COPYING in the main directory of this archive
 * for more details.
 */

/* __be16 and __be32 are represented by their fixed-width integer storage types. */

/*
 *  Bootinfo definitions
 *
 *  This is an easily parsable and extendable structure containing all
 *  information to be passed from the bootstrap to the kernel.
 *
 *  This way I hope to keep all future changes back/forewards compatible.
 *  Thus, keep your fingers crossed...
 *
 *  This structure is copied right after the kernel by the bootstrap
 *  routine.
 */

#[repr(C)]
pub struct bi_record {
    pub tag: u16,       /* tag ID */
    pub size: u16,      /* size of record (in bytes) */
    pub data: [u32; 0], /* data */
}

#[repr(C)]
pub struct mem_info {
    pub addr: u32, /* physical address of memory chunk */
    pub size: u32, /* length of memory chunk (in bytes) */
}

/*
 *  Tag Definitions
 *
 *  Machine independent tags start counting from 0x0000
 *  Machine dependent tags start counting from 0x8000
 */

pub const BI_LAST: u32 = 0x0000; /* last record (sentinel) */
pub const BI_MACHTYPE: u32 = 0x0001; /* machine type (__be32) */
pub const BI_CPUTYPE: u32 = 0x0002; /* cpu type (__be32) */
pub const BI_FPUTYPE: u32 = 0x0003; /* fpu type (__be32) */
pub const BI_MMUTYPE: u32 = 0x0004; /* mmu type (__be32) */
pub const BI_MEMCHUNK: u32 = 0x0005; /* memory chunk address and size */
/* (struct mem_info) */
pub const BI_RAMDISK: u32 = 0x0006; /* ramdisk address and size */
/* (struct mem_info) */
pub const BI_COMMAND_LINE: u32 = 0x0007; /* kernel command line parameters */
/* (string) */
/*
 * A random seed used to initialize the RNG. Record format:
 *
 *   - length       [ 2 bytes, 16-bit big endian ]
 *   - seed data    [ `length` bytes, padded to preserve 4-byte struct alignment ]
 */
pub const BI_RNG_SEED: u32 = 0x0008;

/* Linux/m68k Architectures (BI_MACHTYPE) */
pub const MACH_AMIGA: u32 = 1;
pub const MACH_ATARI: u32 = 2;
pub const MACH_MAC: u32 = 3;
pub const MACH_APOLLO: u32 = 4;
pub const MACH_SUN3: u32 = 5;
pub const MACH_MVME147: u32 = 6;
pub const MACH_MVME16x: u32 = 7;
pub const MACH_BVME6000: u32 = 8;
pub const MACH_HP300: u32 = 9;
pub const MACH_Q40: u32 = 10;
pub const MACH_SUN3X: u32 = 11;
pub const MACH_M54XX: u32 = 12;
pub const MACH_M5441X: u32 = 13;
pub const MACH_VIRT: u32 = 14;

/* CPU, FPU and MMU types (BI_CPUTYPE, BI_FPUTYPE, BI_MMUTYPE) */
pub const CPUB_68020: u32 = 0;
pub const CPUB_68030: u32 = 1;
pub const CPUB_68040: u32 = 2;
pub const CPUB_68060: u32 = 3;
pub const CPUB_COLDFIRE: u32 = 4;
pub const CPU_68020: u32 = 1 << CPUB_68020;
pub const CPU_68030: u32 = 1 << CPUB_68030;
pub const CPU_68040: u32 = 1 << CPUB_68040;
pub const CPU_68060: u32 = 1 << CPUB_68060;
pub const CPU_COLDFIRE: u32 = 1 << CPUB_COLDFIRE;

pub const FPUB_68881: u32 = 0;
pub const FPUB_68882: u32 = 1;
pub const FPUB_68040: u32 = 2; /* Internal FPU */
pub const FPUB_68060: u32 = 3; /* Internal FPU */
pub const FPUB_SUNFPA: u32 = 4; /* Sun-3 FPA */
pub const FPUB_COLDFIRE: u32 = 5; /* ColdFire FPU */
pub const FPU_68881: u32 = 1 << FPUB_68881;
pub const FPU_68882: u32 = 1 << FPUB_68882;
pub const FPU_68040: u32 = 1 << FPUB_68040;
pub const FPU_68060: u32 = 1 << FPUB_68060;
pub const FPU_SUNFPA: u32 = 1 << FPUB_SUNFPA;
pub const FPU_COLDFIRE: u32 = 1 << FPUB_COLDFIRE;

pub const MMUB_68851: u32 = 0;
pub const MMUB_68030: u32 = 1; /* Internal MMU */
pub const MMUB_68040: u32 = 2; /* Internal MMU */
pub const MMUB_68060: u32 = 3; /* Internal MMU */
pub const MMUB_APOLLO: u32 = 4; /* Custom Apollo */
pub const MMUB_SUN3: u32 = 5; /* Custom Sun-3 */
pub const MMUB_COLDFIRE: u32 = 6; /* Internal MMU */
pub const MMU_68851: u32 = 1 << MMUB_68851;
pub const MMU_68030: u32 = 1 << MMUB_68030;
pub const MMU_68040: u32 = 1 << MMUB_68040;
pub const MMU_68060: u32 = 1 << MMUB_68060;
pub const MMU_SUN3: u32 = 1 << MMUB_SUN3;
pub const MMU_APOLLO: u32 = 1 << MMUB_APOLLO;
pub const MMU_COLDFIRE: u32 = 1 << MMUB_COLDFIRE;

/* Stuff for bootinfo interface versioning. */
pub const BOOTINFOV_MAGIC: u32 = 0x4249561A; /* 'BIV^Z' */

#[inline]
pub const fn MK_BI_VERSION(major: u32, minor: u32) -> u32 {
    (major << 16) + minor
}

#[inline]
pub const fn BI_VERSION_MAJOR(v: u32) -> u32 {
    (v >> 16) & 0xffff
}

#[inline]
pub const fn BI_VERSION_MINOR(v: u32) -> u32 {
    v & 0xffff
}

#[repr(C, packed)]
pub struct bootversion {
    pub branch: u16,
    pub magic: u32,
    pub machversions: [bootversion_machversion; 0],
}

#[repr(C)]
pub struct bootversion_machversion {
    pub machtype: u32,
    pub version: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
