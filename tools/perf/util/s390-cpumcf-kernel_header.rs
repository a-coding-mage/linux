/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Support for s390 CPU measurement counter set diagnostic facility
 *
 * Copyright IBM Corp. 2019
   Author(s): Hendrik Brueckner <brueckner@linux.ibm.com>
 *		Thomas Richter <tmricht@linux.ibm.com>
 */

pub const S390_CPUMCF_DIAG_DEF: u32 = 0xfeef; /* Counter diagnostic entry ID */
pub const PERF_EVENT_CPUM_CF_DIAG: u32 = 0xBC000; /* Event: Counter sets */
pub const PERF_EVENT_CPUM_SF_DIAG: u32 = 0xBD000; /* Event: Combined-sampling */
pub const PERF_EVENT_PAI_CRYPTO_ALL: u32 = 0x1000; /* Event: CRYPTO_ALL */
pub const PERF_EVENT_PAI_NNPA_ALL: u32 = 0x1800; /* Event: NNPA_ALL */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cf_ctrset_entry {
    /* CPU-M CF counter set entry (8 byte) */
    pub def: u16,  /* 0-15  Data Entry Format */
    pub set: u16,  /* 16-23 Counter set identifier */
    pub ctr: u16,  /* 24-39 Number of stored counters */
    pub res1: u16, /* 40-63 Reserved */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cf_trailer_entry_flags {
    pub bits: cf_trailer_entry_flags_bits,
    pub flags: libc::c_ulong, /* 0-63    All indicators */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cf_trailer_entry_flags_bits {
    pub flags: libc::c_ulong,
}

impl cf_trailer_entry_flags_bits {
    pub const CLOCK_BASE: libc::c_ulong = 1 << 0; /* TOD clock base */
    pub const SPEED: libc::c_ulong = 1 << 1; /* CPU speed */
    /* Measurement alerts */
    pub const MTDA: libc::c_ulong = 1 << 2; /* Loss of MT ctr. data alert */
    pub const CACA: libc::c_ulong = 1 << 3; /* Counter auth. change alert */
    pub const LCDA: libc::c_ulong = 1 << 4; /* Loss of counter data alert */

    pub fn clock_base(&self) -> libc::c_ulong {
        (self.flags >> 0) & 1
    }

    pub fn speed(&self) -> libc::c_ulong {
        (self.flags >> 1) & 1
    }

    pub fn mtda(&self) -> libc::c_ulong {
        (self.flags >> 2) & 1
    }

    pub fn caca(&self) -> libc::c_ulong {
        (self.flags >> 3) & 1
    }

    pub fn lcda(&self) -> libc::c_ulong {
        (self.flags >> 4) & 1
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cf_trailer_entry_progusage {
    pub fields: cf_trailer_entry_progusage_fields,
    pub progusage: [libc::c_ulong; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cf_trailer_entry_progusage_fields {
    pub progusage1: libc::c_ulong,
    pub progusage2: libc::c_ulong,
    pub progusage3: libc::c_ulong,
    pub tod_base: libc::c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cf_trailer_entry {
    /* CPU-M CF trailer for raw traces (64 byte) */
    /* 0 - 7 */
    pub flags: cf_trailer_entry_flags,
    /* 8 - 15 */
    pub cfvn: u16,      /* 64-79   Ctr First Version */
    pub csvn: u16,      /* 80-95   Ctr Second Version */
    pub cpu_speed: u32, /* 96-127  CPU speed */
    /* 16 - 23 */
    pub timestamp: libc::c_ulong, /* 128-191 Timestamp (TOD) */
    /* 24 - 55 */
    pub progusage: cf_trailer_entry_progusage,
    /* 56 - 63 */
    pub mach_type: u16, /* Machine type */
    pub res1: u16,      /* Reserved */
    pub res2: u32,      /* Reserved */
}

pub const CPUMF_CTR_SET_BASIC: u32 = 0; /* Basic Counter Set */
pub const CPUMF_CTR_SET_USER: u32 = 1; /* Problem-State Counter Set */
pub const CPUMF_CTR_SET_CRYPTO: u32 = 2; /* Crypto-Activity Counter Set */
pub const CPUMF_CTR_SET_EXT: u32 = 3; /* Extended Counter Set */
pub const CPUMF_CTR_SET_MT_DIAG: u32 = 4; /* MT-diagnostic Counter Set */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
