/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Derived from IRIX <sys/SN/gda.h>.
 *
 * Copyright (C) 1992 - 1997, 2000 Silicon Graphics, Inc.
 *
 * gda.h -- Contains the data structure for the global data area,
 *	The GDA contains information communicated between the
 *	PROM, SYMMON, and the kernel.
 */

/* Dependency supplied by the surrounding translation unit: asm/sn/addrs.h. */

/*
 * GDA Version History
 *
 * Version #	| Change
 * -------------+-------------------------------------------------------
 *	1	| Initial SN0 version
 *	2	| Prom sets g_partid field to the partition number. 0 IS
 *		| a valid partition #.
 */

pub const GDA_VERSION: u32 = 2; /* Current GDA version # */

pub const G_MAGICOFF: u32 = 0;
pub const G_VERSIONOFF: u32 = 4;
pub const G_PROMOPOFF: u32 = 6;
pub const G_MASTEROFF: u32 = 8;
pub const G_VDSOFF: u32 = 12;
pub const G_HKDNORMOFF: u32 = 16;
pub const G_HKDUTLBOFF: u32 = 24;
pub const G_HKDXUTLBOFF: u32 = 32;
pub const G_PARTIDOFF: u32 = 40;
pub const G_TABLEOFF: u32 = 128;

#[repr(C)]
pub struct gda {
    pub g_magic: u32,             /* GDA magic number */
    pub g_version: u16,           /* Version of this structure */
    pub g_masterid: u16,          /* The NASID:CPUNUM of the master cpu */
    pub g_promop: u32,            /* Passes requests from the kernel to prom */
    pub g_vds: u32,               /* Store the virtual dipswitches here */
    pub g_hooked_norm: *mut *mut core::ffi::c_void, /* ptr to pda loc for norm hndlr */
    pub g_hooked_utlb: *mut *mut core::ffi::c_void, /* ptr to pda loc for utlb hndlr */
    pub g_hooked_xtlb: *mut *mut core::ffi::c_void, /* ptr to pda loc for xtlb hndlr */
    pub g_partid: i32,            /* partition id */
    pub g_symmax: i32,            /* Max symbols in name table. */
    pub g_dbstab: *mut core::ffi::c_void, /* Address of idbg symbol table */
    pub g_nametab: *mut i8,       /* Address of idbg name table */
    pub g_ktext_repmask: *mut core::ffi::c_void,
    /* Pointer to a mask of nodes with copies
     * of the kernel. */
    pub g_padding: [i8; 56],      /* pad out to 128 bytes */
    pub g_nasidtable: [nasid_t; MAX_NUMNODES], /* NASID of each node */
}

pub type gda_t = gda;

/* C macro: ((gda_t*) GDA_ADDR(get_nasid())) */
pub unsafe fn GDA() -> *mut gda_t {
    GDA_ADDR(get_nasid()) as *mut gda_t
}

/*
 * Define:	PART_GDA_VERSION
 * Purpose:	Define the minimum version of the GDA required, lower
 *		revisions assume GDA is NOT set up, and read partition
 *		information from the board info.
 */
pub const PART_GDA_VERSION: u32 = 2;

/* The following requests can be sent to the PROM during startup. */
pub const PROMOP_MAGIC: u32 = 0x0ead0000;
pub const PROMOP_MAGIC_MASK: u32 = 0x0fff0000;

pub const PROMOP_BIST_SHIFT: u32 = 11;
pub const PROMOP_BIST_MASK: u32 = 0x3 << 11;

pub const PROMOP_REG: u64 = PI_ERR_STACK_ADDR_A;

pub const PROMOP_INVALID: u32 = PROMOP_MAGIC | 0x00;
pub const PROMOP_HALT: u32 = PROMOP_MAGIC | 0x10;
pub const PROMOP_POWERDOWN: u32 = PROMOP_MAGIC | 0x20;
pub const PROMOP_RESTART: u32 = PROMOP_MAGIC | 0x30;
pub const PROMOP_REBOOT: u32 = PROMOP_MAGIC | 0x40;
pub const PROMOP_IMODE: u32 = PROMOP_MAGIC | 0x50;

pub const PROMOP_CMD_MASK: u32 = 0x00f0;
pub const PROMOP_OPTIONS_MASK: u32 = 0xfff0;

pub const PROMOP_SKIP_DIAGS: u32 = 0x0100;  /* don't bother running diags */
pub const PROMOP_SKIP_MEMINIT: u32 = 0x0200; /* don't bother initing memory */
pub const PROMOP_SKIP_DEVINIT: u32 = 0x0400; /* don't bother initing devices */
pub const PROMOP_BIST1: u32 = 0x0800;       /* keep track of which BIST ran */
pub const PROMOP_BIST2: u32 = 0x1000;       /* keep track of which BIST ran */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
