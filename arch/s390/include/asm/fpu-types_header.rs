/* SPDX-License-Identifier: GPL-2.0 */
/*
 * FPU data structures
 *
 * Copyright IBM Corp. 2015
 * Author(s): Hendrik Brueckner <brueckner@linux.vnet.ibm.com>
 */

// Dependency supplied by asm/sigcontext.h in the original header.

#[repr(C, align(8))]
pub struct fpu {
    pub fpc: u32,
    pub vxrs: [__vector128; __NUM_VXRS],
}

#[repr(C)]
pub struct kernel_fpu_hdr {
    pub mask: i32,
    pub fpc: u32,
}

#[repr(C, align(8))]
pub struct kernel_fpu {
    pub hdr: kernel_fpu_hdr,
    // Flexible array member: storage is provided by the enclosing allocation.
    pub vxrs: [__vector128; 0],
}

macro_rules! KERNEL_FPU_STRUCT {
    ($vxr_size:literal, $name:ident) => {
        #[repr(C, align(8))]
        pub struct $name {
            pub hdr: kernel_fpu_hdr,
            pub vxrs: [__vector128; $vxr_size],
        }
    };
}

KERNEL_FPU_STRUCT!(8, kernel_fpu_8);
KERNEL_FPU_STRUCT!(16, kernel_fpu_16);
KERNEL_FPU_STRUCT!(32, kernel_fpu_32);

macro_rules! DECLARE_KERNEL_FPU_ONSTACK {
    ($vxr_size:literal, $name:ident) => {
        let mut $name: kernel_fpu_for_size!($vxr_size);
    };
}

macro_rules! kernel_fpu_for_size {
    (8) => { kernel_fpu_8 };
    (16) => { kernel_fpu_16 };
    (32) => { kernel_fpu_32 };
}

macro_rules! DECLARE_KERNEL_FPU_ONSTACK8 {
    ($name:ident) => {
        DECLARE_KERNEL_FPU_ONSTACK!(8, $name);
    };
}

macro_rules! DECLARE_KERNEL_FPU_ONSTACK16 {
    ($name:ident) => {
        DECLARE_KERNEL_FPU_ONSTACK!(16, $name);
    };
}

macro_rules! DECLARE_KERNEL_FPU_ONSTACK32 {
    ($name:ident) => {
        DECLARE_KERNEL_FPU_ONSTACK!(32, $name);
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
