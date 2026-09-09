/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Derived from IRIX <sys/SN/nmi.h>, Revision 1.5.
 *
 * Copyright (C) 1992 - 1997 Silicon Graphics, Inc.
 */

// Dependency supplied by asm/sn/addrs.h is intentionally not implemented here.

/*
 * The launch data structure resides at a fixed place in each node's memory
 * and is used to communicate between the master processor and the slave
 * processors.
 *
 * The master stores launch parameters in the launch structure
 * corresponding to a target processor that is in a slave loop, then sends
 * an interrupt to the slave processor.  The slave calls the desired
 * function, followed by an optional rendezvous function, then returns to
 * the slave loop.  The master does not wait for the slaves before
 * returning.
 *
 * There is an array of launch structures, one per CPU on the node.  One
 * interrupt level is used per CPU.
 */

pub const NMI_MAGIC: u64 = 0x48414d4d455201;
pub const NMI_SIZEOF: usize = 0x40;

pub const NMI_OFF_MAGIC: usize = 0x00; // Struct offsets for assembly
pub const NMI_OFF_FLAGS: usize = 0x08;
pub const NMI_OFF_CALL: usize = 0x10;
pub const NMI_OFF_CALLC: usize = 0x18;
pub const NMI_OFF_CALLPARM: usize = 0x20;
pub const NMI_OFF_GMASTER: usize = 0x28;

/*
 * The NMI routine is called only if the complement address is
 * correct.
 *
 * Before control is transferred to a routine, the complement address
 * is zeroed (invalidated) to prevent an accidental call from a spurious
 * interrupt.
 */

#[repr(C)]
pub struct nmi_s {
    pub magic: u64,                // Magic number
    pub flags: u64,                // Combination of flags above
    pub call_addr: *mut core::ffi::c_void, // Routine for slave to call
    pub call_addr_c: *mut core::ffi::c_void, // 1's complement of address
    pub call_parm: *mut core::ffi::c_void, // Single parm passed to call
    pub gmaster: u64,              // Flag true only on global master
}

pub type nmi_t = nmi_s;

/* Following definitions are needed both in the prom & the kernel
 * to identify the format of the nmi cpu register save area in the
 * low memory on each node.
 */

#[repr(C)]
pub struct reg_struct {
    pub gpr: [u64; 32],
    pub sr: u64,
    pub cause: u64,
    pub epc: u64,
    pub badva: u64,
    pub error_epc: u64,
    pub cache_err: u64,
    pub nmi_sr: u64,
}

/* These are the assembly language offsets into the reg_struct structure */

pub const R0_OFF: usize = 0x0;
pub const R1_OFF: usize = 0x8;
pub const R2_OFF: usize = 0x10;
pub const R3_OFF: usize = 0x18;
pub const R4_OFF: usize = 0x20;
pub const R5_OFF: usize = 0x28;
pub const R6_OFF: usize = 0x30;
pub const R7_OFF: usize = 0x38;
pub const R8_OFF: usize = 0x40;
pub const R9_OFF: usize = 0x48;
pub const R10_OFF: usize = 0x50;
pub const R11_OFF: usize = 0x58;
pub const R12_OFF: usize = 0x60;
pub const R13_OFF: usize = 0x68;
pub const R14_OFF: usize = 0x70;
pub const R15_OFF: usize = 0x78;
pub const R16_OFF: usize = 0x80;
pub const R17_OFF: usize = 0x88;
pub const R18_OFF: usize = 0x90;
pub const R19_OFF: usize = 0x98;
pub const R20_OFF: usize = 0xa0;
pub const R21_OFF: usize = 0xa8;
pub const R22_OFF: usize = 0xb0;
pub const R23_OFF: usize = 0xb8;
pub const R24_OFF: usize = 0xc0;
pub const R25_OFF: usize = 0xc8;
pub const R26_OFF: usize = 0xd0;
pub const R27_OFF: usize = 0xd8;
pub const R28_OFF: usize = 0xe0;
pub const R29_OFF: usize = 0xe8;
pub const R30_OFF: usize = 0xf0;
pub const R31_OFF: usize = 0xf8;
pub const SR_OFF: usize = 0x100;
pub const CAUSE_OFF: usize = 0x108;
pub const EPC_OFF: usize = 0x110;
pub const BADVA_OFF: usize = 0x118;
pub const ERROR_EPC_OFF: usize = 0x120;
pub const CACHE_ERR_OFF: usize = 0x128;
pub const NMISR_OFF: usize = 0x130;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
