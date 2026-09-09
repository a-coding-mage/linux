/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Derived from IRIX <sys/SN/SN0/hubni.h>, Revision 1.27.
 *
 * Copyright (C) 1992-1997, 1999 Silicon Graphics, Inc.
 * Copyright (C) 1999 by Ralf Baechle
 */

/* Hub Network Interface registers. */

pub const NI_BASE: u64 = 0x600000;
pub const NI_BASE_TABLES: u64 = 0x630000;
pub const NI_STATUS_REV_ID: u64 = 0x600000;
pub const NI_PORT_RESET: u64 = 0x600008;
pub const NI_PROTECTION: u64 = 0x600010;
pub const NI_GLOBAL_PARMS: u64 = 0x600018;
pub const NI_SCRATCH_REG0: u64 = 0x600100;
pub const NI_SCRATCH_REG1: u64 = 0x600108;
pub const NI_DIAG_PARMS: u64 = 0x600110;
pub const NI_VECTOR_PARMS: u64 = 0x600200;
pub const NI_VECTOR: u64 = 0x600208;
pub const NI_VECTOR_DATA: u64 = 0x600210;
pub const NI_VECTOR_STATUS: u64 = 0x600300;
pub const NI_RETURN_VECTOR: u64 = 0x600308;
pub const NI_VECTOR_READ_DATA: u64 = 0x600310;
pub const NI_VECTOR_CLEAR: u64 = 0x600380;
pub const NI_IO_PROTECT: u64 = 0x600400;
pub const NI_IO_PROT_OVRRD: u64 = 0x600408;
pub const NI_AGE_CPU0_MEMORY: u64 = 0x600500;
pub const NI_AGE_CPU0_PIO: u64 = 0x600508;
pub const NI_AGE_CPU1_MEMORY: u64 = 0x600510;
pub const NI_AGE_CPU1_PIO: u64 = 0x600518;
pub const NI_AGE_GBR_MEMORY: u64 = 0x600520;
pub const NI_AGE_GBR_PIO: u64 = 0x600528;
pub const NI_AGE_IO_MEMORY: u64 = 0x600530;
pub const NI_AGE_IO_PIO: u64 = 0x600538;
pub const NI_AGE_REG_MIN: u64 = NI_AGE_CPU0_MEMORY;
pub const NI_AGE_REG_MAX: u64 = NI_AGE_IO_PIO;
pub const NI_PORT_PARMS: u64 = 0x608000;
pub const NI_PORT_ERROR: u64 = 0x608008;
pub const NI_PORT_ERROR_CLEAR: u64 = 0x608088;
pub const NI_META_TABLE0: u64 = 0x638000;
pub const NI_META_ENTRIES: usize = 32;
pub const NI_LOCAL_TABLE0: u64 = 0x638100;
pub const NI_LOCAL_ENTRIES: usize = 16;

#[inline]
pub const fn NI_META_TABLE(x: u64) -> u64 { NI_META_TABLE0 + 8 * x }
#[inline]
pub const fn NI_LOCAL_TABLE(x: u64) -> u64 { NI_LOCAL_TABLE0 + 8 * x }

pub const NSRI_8BITMODE_SHFT: u32 = 30;
pub const NSRI_8BITMODE_MASK: u64 = 1u64 << 30;
pub const NSRI_LINKUP_SHFT: u32 = 29;
pub const NSRI_LINKUP_MASK: u64 = 1u64 << 29;
pub const NSRI_DOWNREASON_SHFT: u32 = 28;
pub const NSRI_DOWNREASON_MASK: u64 = 1u64 << 28;
pub const NSRI_MORENODES_SHFT: u32 = 18;
pub const NSRI_MORENODES_MASK: u64 = 1u64 << 18;
pub const MORE_MEMORY: u64 = 0;
pub const MORE_NODES: u64 = 1;
pub const NSRI_REGIONSIZE_SHFT: u32 = 17;
pub const NSRI_REGIONSIZE_MASK: u64 = 1u64 << 17;
pub const REGIONSIZE_FINE: u64 = 1;
pub const REGIONSIZE_COARSE: u64 = 0;
pub const NSRI_NODEID_SHFT: u32 = 8;
pub const NSRI_NODEID_MASK: u64 = 0x1ff << 8;
pub const NSRI_REV_SHFT: u32 = 4;
pub const NSRI_REV_MASK: u64 = 0xf << 4;
pub const NSRI_CHIPID_SHFT: u32 = 0;
pub const NSRI_CHIPID_MASK: u64 = 0xf;
pub const NASID_TO_FINEREG_SHFT: i32 = 0;
pub const NASID_TO_COARSEREG_SHFT: i32 = 3;
pub const NPR_PORTRESET: u64 = 1 << 7;
pub const NPR_LINKRESET: u64 = 1 << 1;
pub const NPR_LOCALRESET: u64 = 1;
pub const NPROT_RESETOK: u64 = 1;
pub const NGP_MAXRETRY_SHFT: u32 = 48;
pub const NGP_MAXRETRY_MASK: u64 = 0x3ff << 48;
pub const NGP_TAILTOWRAP_SHFT: u32 = 32;
pub const NGP_TAILTOWRAP_MASK: u64 = 0xffff << 32;
pub const NGP_CREDITTOVAL_SHFT: u32 = 16;
pub const NGP_CREDITTOVAL_MASK: u64 = 0xf << 16;
pub const NGP_TAILTOVAL_SHFT: u32 = 4;
pub const NGP_TAILTOVAL_MASK: u64 = 0xf << 4;
pub const NDP_PORTTORESET: u64 = 1 << 18;
pub const NDP_LLP8BITMODE: u64 = 1 << 12;
pub const NDP_PORTDISABLE: u64 = 1 << 6;
pub const NDP_SENDERROR: u64 = 1;
pub const NVP_PIOID_SHFT: u32 = 40;
pub const NVP_PIOID_MASK: u64 = 0x3ff << 40;
pub const NVP_WRITEID_SHFT: u32 = 32;
pub const NVP_WRITEID_MASK: u64 = 0xff << 32;
pub const NVP_ADDRESS_MASK: u64 = 0xffff8;
pub const NVP_TYPE_SHFT: u32 = 0;
pub const NVP_TYPE_MASK: u64 = 3;
pub const NVS_VALID: u64 = 1 << 63;
pub const NVS_OVERRUN: u64 = 1 << 62;
pub const NVS_TARGET_SHFT: u32 = 51;
pub const NVS_TARGET_MASK: u64 = 0x3ff << 51;
pub const NVS_PIOID_SHFT: u32 = 40;
pub const NVS_PIOID_MASK: u64 = 0x3ff << 40;
pub const NVS_WRITEID_SHFT: u32 = 32;
pub const NVS_WRITEID_MASK: u64 = 0xff << 32;
pub const NVS_ADDRESS_MASK: u64 = 0xfffffff8;
pub const NVS_TYPE_SHFT: u32 = 0;
pub const NVS_TYPE_MASK: u64 = 7;
pub const NVS_ERROR_MASK: u64 = 4;
pub const PIOTYPE_READ: u64 = 0;
pub const PIOTYPE_WRITE: u64 = 1;
pub const PIOTYPE_UNDEFINED: u64 = 2;
pub const PIOTYPE_EXCHANGE: u64 = 3;
pub const PIOTYPE_ADDR_ERR: u64 = 4;
pub const PIOTYPE_CMD_ERR: u64 = 5;
pub const PIOTYPE_PROT_ERR: u64 = 6;
pub const PIOTYPE_UNKNOWN: u64 = 7;
pub const NAGE_VCH_SHFT: u32 = 10;
pub const NAGE_VCH_MASK: u64 = 3 << 10;
pub const NAGE_CC_SHFT: u32 = 8;
pub const NAGE_CC_MASK: u64 = 3 << 8;
pub const NAGE_AGE_SHFT: u32 = 0;
pub const NAGE_AGE_MASK: u64 = 0xff;
pub const NAGE_MASK: u64 = NAGE_VCH_MASK | NAGE_CC_MASK | NAGE_AGE_MASK;
pub const VCHANNEL_A: u64 = 0;
pub const VCHANNEL_B: u64 = 1;
pub const VCHANNEL_ANY: u64 = 2;
pub const NPP_NULLTO_SHFT: u32 = 10;
pub const NPP_NULLTO_MASK: u64 = 0x3f << 16;
pub const NPP_MAXBURST_SHFT: u32 = 0;
pub const NPP_MAXBURST_MASK: u64 = 0x3ff;
pub const NPP_RESET_DFLT_HUB20: u64 = (1 << NPP_NULLTO_SHFT) | (0x3f0 << NPP_MAXBURST_SHFT);
pub const NPP_RESET_DEFAULTS: u64 = (6 << NPP_NULLTO_SHFT) | (0x3f0 << NPP_MAXBURST_SHFT);
pub const NPE_LINKRESET: u64 = 1 << 37;
pub const NPE_INTERNALERROR: u64 = 1 << 36;
pub const NPE_BADMESSAGE: u64 = 1 << 35;
pub const NPE_BADDEST: u64 = 1 << 34;
pub const NPE_FIFOOVERFLOW: u64 = 1 << 33;
pub const NPE_CREDITTO_SHFT: u32 = 28;
pub const NPE_CREDITTO_MASK: u64 = 0xf << 28;
pub const NPE_TAILTO_SHFT: u32 = 24;
pub const NPE_TAILTO_MASK: u64 = 0xf << 24;
pub const NPE_RETRYCOUNT_SHFT: u32 = 16;
pub const NPE_RETRYCOUNT_MASK: u64 = 0xff << 16;
pub const NPE_CBERRCOUNT_SHFT: u32 = 8;
pub const NPE_CBERRCOUNT_MASK: u64 = 0xff << 8;
pub const NPE_SNERRCOUNT_SHFT: u32 = 0;
pub const NPE_SNERRCOUNT_MASK: u64 = 0xff;
pub const NPE_MASK: u64 = 0x3effffffff;
pub const NPE_COUNT_MAX: u64 = 0xff;
pub const NPE_FATAL_ERRORS: u64 = NPE_LINKRESET | NPE_INTERNALERROR | NPE_BADMESSAGE | NPE_BADDEST | NPE_FIFOOVERFLOW | NPE_CREDITTO_MASK | NPE_TAILTO_MASK;
pub const NMT_EXIT_PORT_MASK: u64 = 0xf;
pub const NLT_EXIT_PORT_MASK: u64 = 0xf;

#[repr(C)]
pub struct hubni_port_error_fields_t {
    pub value: u64,
}

#[repr(C)]
pub union hubni_port_error_t {
    pub nipe_reg_value: u64,
    pub nipe_fields_s: hubni_port_error_fields_t,
}

pub const NI_LLP_RETRY_MAX: u64 = 0xff;
pub const NI_LLP_CB_MAX: u64 = 0xff;
pub const NI_LLP_SN_MAX: u64 = 0xff;

/* LOCAL_HUB_L is supplied by the surrounding platform headers. */
extern "C" {
    fn LOCAL_HUB_L(register: u64) -> u64;
}

#[inline]
pub unsafe fn get_region_shift() -> i32 {
    if LOCAL_HUB_L(NI_STATUS_REV_ID) & NSRI_REGIONSIZE_MASK != 0 {
        NASID_TO_FINEREG_SHFT
    } else {
        NASID_TO_COARSEREG_SHFT
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
