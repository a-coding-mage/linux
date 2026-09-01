/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Auxtrace support for s390 CPU measurement sampling facility
 *
 *  Copyright IBM Corp. 2018
 *  Author(s): Hendrik Brueckner <brueckner@linux.ibm.com>
 *	       Thomas Richter <tmricht@linux.ibm.com>
 */

pub const S390_CPUMSF_PAGESZ: u32 = 4096; /* Size of sample block units */
pub const S390_CPUMSF_DIAG_DEF_FIRST: u32 = 0x8001; /* Diagnostic entry lowest id */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hws_basic_entry {
    /*
     * unsigned int def:16;      0-15  Data Entry Format
     * unsigned int R:4;         16-19 reserved
     * unsigned int U:4;         20-23 Number of unique instruct.
     * unsigned int z:2;         zeros
     * unsigned int T:1;         26 PSW DAT mode
     * unsigned int W:1;         27 PSW wait state
     * unsigned int P:1;         28 PSW Problem state
     * unsigned int AS:2;        29-30 PSW address-space control
     * unsigned int I:1;         31 entry valid or invalid
     * unsigned int CL:2;        32-33 Configuration Level
     * unsigned int:14;
     * unsigned int prim_asn:16; primary ASN
     */
    pub bitfields: [u32; 2],
    pub ia: u64,  /* Instruction Address */
    pub gpp: u64, /* Guest Program Parameter */
    pub hpp: u64, /* Host Program Parameter */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hws_diag_entry {
    /*
     * unsigned int def:16; 0-15  Data Entry Format
     * unsigned int R:15;   16-19 and 20-30 reserved
     * unsigned int I:1;    31 entry valid or invalid
     */
    pub bitfields: u32,
    pub data: [u8; 0], /* Machine-dependent sample data */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hws_combined_entry {
    pub basic: hws_basic_entry, /* Basic-sampling data entry */
    pub diag: hws_diag_entry,   /* Diagnostic-sampling data entry */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hws_trailer_entry_flags_bits {
    /*
     * unsigned int f:1;       0 - Block Full Indicator
     * unsigned int a:1;       1 - Alert request control
     * unsigned int t:1;       2 - Timestamp format
     * unsigned int:29;        3 - 31: Reserved
     * unsigned int bsdes:16;  32-47: size of basic SDE
     * unsigned int dsdes:16;  48-63: size of diagnostic SDE
     */
    pub bitfields: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hws_trailer_entry_flags {
    pub bits: hws_trailer_entry_flags_bits,
    pub flags: u64, /* 0 - 64: All indicators */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hws_trailer_entry_progusage_bits {
    /*
     * unsigned long long clock_base:1; in progusage2
     * unsigned long long progusage1:63;
     * unsigned long long progusage2;
     */
    pub bitfields: u64,
    pub progusage2: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hws_trailer_entry_progusage {
    pub bits: hws_trailer_entry_progusage_bits,
    pub progusage: [u64; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hws_trailer_entry {
    pub flags: hws_trailer_entry_flags,
    pub overflow: u64,          /* 64 - sample Overflow count */
    pub timestamp: [u8; 16],    /* 16 - 31 timestamp */
    pub reserved1: u64,         /* 32 -Reserved */
    pub reserved2: u64,         /* */
    pub progusage: hws_trailer_entry_progusage, /* 48 - reserved for programming use */
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
