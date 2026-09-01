/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Hardware interface of the NX-GZIP compression accelerator
 *
 * Copyright (C) IBM Corporation, 2020
 *
 * Author: Bulent Abali <abali@us.ibm.com>
 *
 */

/* C dependencies: <stdint.h>, <endian.h>, "nx.h" */

/* deflate */
pub const LLSZ: usize = 286;
pub const DSZ: usize = 30;

/* nx */
pub const DHTSZ: usize = 18;
pub const DHT_MAXSZ: usize = 288;
pub const MAX_DDE_COUNT: usize = 256;

/* util */
/* NXDBG: NXPRT(X) expands to X when enabled, otherwise to nothing. */
/* NXTIMER: NX_CLK(X) and nx_get_time/freq map to PPC timebase helpers. */
pub const NX_MAX_FAULTS: u32 = 500;

#[repr(C, align(16))]
pub union nx_qw_t {
    pub word: [u32; 4],
    pub dword: [u64; 2],
}

/*
 * Note: NX registers with fewer than 32 bits are declared by
 * convention as uint32_t variables in unions. If *_offset and *_mask
 * are defined for a variable, then use get_ put_ macros to
 * conveniently access the register fields for endian conversions.
 */

#[repr(C, align(16))]
pub struct nx_dde_t {
    /*
     * Data Descriptor Element, Section 6.4
     * When dde_count == 0 ddead is a pointer to a data buffer;
     * ddebc is the buffer length bytes.
     * When dde_count > 0 dde is an indirect dde; ddead is a
     * pointer to a contiguous list of direct ddes; ddebc is the
     * total length of all data pointed to by the list of direct
     * ddes. Note that only one level of indirection is permitted.
     * See Section 6.4 of the user manual for additional details.
     */
    pub dde_count: u32,
    pub ddebc: u32,  /* dde byte count */
    pub ddead: u64,  /* dde address */
}

#[repr(C, align(16))]
pub struct nx_csb_t {
    /*
     * Coprocessor Status Block, Section 6.6
     * The first word is accessed as csb_v/csb_f/csb_cs/csb_cc/csb_ce.
     */
    pub word0: nx_csb_t_word0,
    pub tpbc: u32,
    /* target processed byte count TPBC */
    pub fsaddr: u64,
    /*
     * Section 6.12.1 CSB NonZero error summary. FSA Failing storage
     * address. Address where error occurred. When available, written
     * to A field of CSB
     */
}

#[repr(C)]
pub union nx_csb_t_word0 {
    pub csb_v: u32,
    pub csb_f: u32,
    pub csb_cs: u32,
    pub csb_cc: u32,
    pub csb_ce: u32,
}

#[repr(C, align(16))]
pub struct nx_ccb_t {
    /* Coprocessor Completion Block, Section 6.7 */
    pub reserved: [u32; 3],
    /*
     * When crb.c==0 (no ccb defined) it is reserved;
     * When crb.c==1 (ccb defined) it is cm
     */
    pub word3: nx_ccb_t_word3,
}

#[repr(C)]
pub union nx_ccb_t_word3 {
    pub ccb_cm: u32, /* Signal interrupt of crb.c==1 and cm==1 */
    pub word: u32,   /* generic access to the 32bit word */
}

#[repr(C)]
pub struct vas_stamped_crb_t {
    /*
     * CRB operand of the paste coprocessor instruction is stamped
     * in quadword 4 with the information shown here as its written
     * in to the receive FIFO of the coprocessor
     */
    pub word0: vas_stamped_crb_t_word0,
    pub word1: vas_stamped_crb_t_word1,
    pub reserved2: u32,
    pub word3: vas_stamped_crb_t_word3,
}

#[repr(C)]
pub union vas_stamped_crb_t_word0 {
    pub vas_buf_num: u32,
    pub send_wc_id: u32,
}

#[repr(C)]
pub union vas_stamped_crb_t_word1 {
    pub recv_wc_id: u32,
}

#[repr(C)]
pub union vas_stamped_crb_t_word3 {
    pub vas_invalid: u32,
}

#[repr(C)]
pub struct nx_stamped_fault_crb_t {
    /*
     * A CRB that has a translation fault is stamped by NX in quadword 4
     * and pasted to the Fault Send Window in VAS.
     */
    pub fsa: u64,
    pub word2: nx_stamped_fault_crb_t_word2,
    pub pswid: u32,
}

#[repr(C)]
pub union nx_stamped_fault_crb_t_word2 {
    pub nxsf_t: u32,
    pub nxsf_fs: u32,
}

#[repr(C)]
pub union stamped_crb_t {
    pub vas: vas_stamped_crb_t,
    pub nx: nx_stamped_fault_crb_t,
}

#[repr(C, align(128))]
pub struct nx_gzip_cpb_t {
    /*
     * Coprocessor Parameter Block In/Out are used to pass metadata
     * to/from accelerator. Tables 6.5 and 6.6 of the user manual.
     */
    pub input: nx_gzip_cpb_input_t,
    /* C volatile output struct. Use volatile pointer operations when required. */
    pub output: nx_gzip_cpb_output_t,
}

#[repr(C)]
pub struct nx_gzip_cpb_input_t {
    pub qw0: nx_gzip_cpb_input_qw0_t,
    pub dht: nx_gzip_cpb_input_dht_t,
    pub reserved: [nx_qw_t; 5],
}

#[repr(C)]
pub union nx_gzip_cpb_input_qw0_t {
    pub qw0: nx_qw_t,
    pub fields: nx_gzip_cpb_input_qw0_fields_t,
}

#[repr(C)]
pub struct nx_gzip_cpb_input_qw0_fields_t {
    pub in_adler: u32, /* bits 0:31  */
    pub in_crc: u32,   /* bits 32:63 */
    pub word2: nx_gzip_cpb_input_qw0_word2_t,
    pub word3: nx_gzip_cpb_input_qw0_word3_t,
}

#[repr(C)]
pub union nx_gzip_cpb_input_qw0_word2_t {
    pub in_histlen: u32, /* bits 64:75 */
    pub in_subc: u32,    /* bits 93:95 */
}

#[repr(C)]
pub union nx_gzip_cpb_input_qw0_word3_t {
    pub in_sfbt: u32,       /* bits 108:111 */
    pub in_rembytecnt: u32, /* bits 112:127 */
    pub in_dhtlen: u32,     /* bits 116:127 */
}

#[repr(C)]
pub union nx_gzip_cpb_input_dht_t {
    pub in_dht: [nx_qw_t; DHTSZ],        /* qw[1:18] */
    pub in_dht_char: [i8; DHT_MAXSZ],    /* byte access */
}

#[repr(C)]
pub struct nx_gzip_cpb_output_t {
    pub qw24: nx_gzip_cpb_output_qw24_t,
    pub qw25: nx_gzip_cpb_output_qw25_t,
    pub out_spbc_comp_with_count: u32, /* qw[104] compress with lzcounts */
}

#[repr(C)]
pub union nx_gzip_cpb_output_qw24_t {
    pub qw24: nx_qw_t,
    pub fields: nx_gzip_cpb_output_qw24_fields_t,
}

#[repr(C)]
pub struct nx_gzip_cpb_output_qw24_fields_t {
    pub out_adler: u32, /* bits 0:31  qw[24] */
    pub out_crc: u32,   /* bits 32:63 qw[24] */
    pub word2: nx_gzip_cpb_output_qw24_word2_t,
    pub word3: nx_gzip_cpb_output_qw24_word3_t,
}

#[repr(C)]
pub union nx_gzip_cpb_output_qw24_word2_t {
    pub out_tebc: u32, /* bits 77:79 qw[24] */
    pub out_subc: u32, /* bits 80:95 qw[24] */
}

#[repr(C)]
pub union nx_gzip_cpb_output_qw24_word3_t {
    pub out_sfbt: u32,       /* bits 108:111 qw[24] */
    pub out_rembytecnt: u32, /* bits 112:127 qw[24] */
    pub out_dhtlen: u32,     /* bits 116:127 qw[24] */
}

#[repr(C)]
pub union nx_gzip_cpb_output_qw25_t {
    pub qw25: [nx_qw_t; 79],              /* qw[25:103] */
    pub out_spbc_comp_wrap: u32,          /* qw[25] compress no lzcounts or wrap */
    pub out_spbc_wrap: u32,               /* qw[25] wrap */
    pub out_spbc_comp: u32,               /* qw[25] compress no lzcounts */
    pub out_lzcount: [u32; LLSZ + DSZ],   /* 286 LL and 30 D symbol counts */
    pub decomp: nx_gzip_cpb_output_decomp_t,
}

#[repr(C)]
pub struct nx_gzip_cpb_output_decomp_t {
    pub out_dht: [nx_qw_t; DHTSZ], /* qw[25:42] */
    pub out_spbc_decomp: u32,      /* qw[43] decompress */
}

#[repr(C, align(128))]
pub struct nx_gzip_crb_t {
    pub word0: nx_gzip_crb_word0_t, /* byte[0:3] */
    pub reserved1: u32,             /* byte[4:7] */
    pub word8: nx_gzip_crb_word8_t,
    pub source_dde: nx_dde_t,       /* byte[16:31] */
    pub target_dde: nx_dde_t,       /* byte[32:47] */
    pub ccb: nx_ccb_t,              /* byte[48:63], C volatile */
    pub stamp_area: nx_gzip_crb_stamp_area_t,
    pub csb: nx_csb_t,              /* C volatile */
}

#[repr(C)]
pub union nx_gzip_crb_word0_t {
    pub gzip_fc: u32, /* bits[24-31] */
}

#[repr(C)]
pub union nx_gzip_crb_word8_t {
    pub csb_address: u64, /* byte[8:15] */
    pub fields: nx_gzip_crb_word8_fields_t,
}

#[repr(C)]
pub struct nx_gzip_crb_word8_fields_t {
    pub reserved2: u32,
    pub word: nx_gzip_crb_word8_fields_word_t,
}

#[repr(C)]
pub union nx_gzip_crb_word8_fields_word_t {
    pub crb_c: u32,  /* c==0 no ccb defined */
    pub crb_at: u32, /* at==0 address type is ignored; all addrs effective assumed. */
}

#[repr(C)]
pub union nx_gzip_crb_stamp_area_t {
    /*
     * byte[64:239] shift csb by 128 bytes out of the crb; csb was
     * in crb earlier; JReilly says csb written with partial inject
     */
    pub reserved64: [nx_qw_t; 11],
    pub stamp: stamped_crb_t, /* byte[64:79] */
}

#[repr(C, align(2048))]
pub struct nx_gzip_crb_cpb_t {
    pub crb: nx_gzip_crb_t,
    pub cpb: nx_gzip_cpb_t,
}

/*
 * NX hardware convention has the msb bit on the left numbered 0.
 * The defines below has *_offset defined as the right most bit
 * position of a field. x of size_mask(x) is the field width in bits.
 */
pub const fn size_mask(x: u32) -> u32 {
    (1u32 << x).wrapping_sub(1)
}

pub const DDE_COUNT_MASK: u32 = size_mask(8);
pub const DDE_COUNT_OFFSET: u32 = 23;

pub const CSB_V_MASK: u32 = size_mask(1);
pub const CSB_V_OFFSET: u32 = 0;
pub const CSB_F_MASK: u32 = size_mask(1);
pub const CSB_F_OFFSET: u32 = 6;
pub const CSB_CS_MASK: u32 = size_mask(8);
pub const CSB_CS_OFFSET: u32 = 15;
pub const CSB_CC_MASK: u32 = size_mask(8);
pub const CSB_CC_OFFSET: u32 = 23;
pub const CSB_CE_MASK: u32 = size_mask(8);
pub const CSB_CE_OFFSET: u32 = 31;

pub const CCB_CM_MASK: u32 = size_mask(3);
pub const CCB_CM_OFFSET: u32 = 31;

pub const VAS_BUF_NUM_MASK: u32 = size_mask(6);
pub const VAS_BUF_NUM_OFFSET: u32 = 5;
pub const SEND_WC_ID_MASK: u32 = size_mask(16);
pub const SEND_WC_ID_OFFSET: u32 = 31;
pub const RECV_WC_ID_MASK: u32 = size_mask(16);
pub const RECV_WC_ID_OFFSET: u32 = 31;
pub const VAS_INVALID_MASK: u32 = size_mask(1);
pub const VAS_INVALID_OFFSET: u32 = 31;

pub const NXSF_T_MASK: u32 = size_mask(1);
pub const NXSF_T_OFFSET: u32 = 23;
pub const NXSF_FS_MASK: u32 = size_mask(8);
pub const NXSF_FS_OFFSET: u32 = 31;

pub const IN_HISTLEN_MASK: u32 = size_mask(12);
pub const IN_HISTLEN_OFFSET: u32 = 11;
pub const IN_DHTLEN_MASK: u32 = size_mask(12);
pub const IN_DHTLEN_OFFSET: u32 = 31;
pub const IN_SUBC_MASK: u32 = size_mask(3);
pub const IN_SUBC_OFFSET: u32 = 31;
pub const IN_SFBT_MASK: u32 = size_mask(4);
pub const IN_SFBT_OFFSET: u32 = 15;
pub const IN_REMBYTECNT_MASK: u32 = size_mask(16);
pub const IN_REMBYTECNT_OFFSET: u32 = 31;

pub const OUT_TEBC_MASK: u32 = size_mask(3);
pub const OUT_TEBC_OFFSET: u32 = 15;
pub const OUT_SUBC_MASK: u32 = size_mask(16);
pub const OUT_SUBC_OFFSET: u32 = 31;
pub const OUT_SFBT_MASK: u32 = size_mask(4);
pub const OUT_SFBT_OFFSET: u32 = 15;
pub const OUT_REMBYTECNT_MASK: u32 = size_mask(16);
pub const OUT_REMBYTECNT_OFFSET: u32 = 31;
pub const OUT_DHTLEN_MASK: u32 = size_mask(12);
pub const OUT_DHTLEN_OFFSET: u32 = 31;

pub const GZIP_FC_MASK: u32 = size_mask(8);
pub const GZIP_FC_OFFSET: u32 = 31;
pub const CRB_C_MASK: u32 = size_mask(1);
pub const CRB_C_OFFSET: u32 = 28;
pub const CRB_AT_MASK: u32 = size_mask(1);
pub const CRB_AT_OFFSET: u32 = 30;
pub const CSB_ADDRESS_MASK: u64 = !15u64; /* mask off bottom 4b */

pub const fn be32toh(x: u32) -> u32 {
    u32::from_be(x)
}

pub const fn htobe32(x: u32) -> u32 {
    x.to_be()
}

pub const fn be64toh(x: u64) -> u64 {
    u64::from_be(x)
}

pub const fn htobe64(x: u64) -> u64 {
    x.to_be()
}

macro_rules! getnn {
    ($st:expr, $reg:ident, $mask:expr, $offset:expr) => {
        (be32toh($st.$reg) >> (31 - $offset)) & $mask
    };
}

macro_rules! getpnn {
    ($st:expr, $reg:ident, $mask:expr, $offset:expr) => {
        (be32toh((*$st).$reg) >> (31 - $offset)) & $mask
    };
}

macro_rules! get32 {
    ($st:expr, $reg:ident) => {
        be32toh($st.$reg)
    };
}

macro_rules! getp32 {
    ($st:expr, $reg:ident) => {
        be32toh((*$st).$reg)
    };
}

macro_rules! get64 {
    ($st:expr, $reg:ident) => {
        be64toh($st.$reg)
    };
}

macro_rules! getp64 {
    ($st:expr, $reg:ident) => {
        be64toh((*$st).$reg)
    };
}

macro_rules! unget32 {
    ($st:expr, $reg:ident, $mask:expr, $offset:expr) => {
        get32!($st, $reg) & !($mask << (31 - $offset))
    };
}

macro_rules! ungetp32 {
    ($st:expr, $reg:ident, $mask:expr, $offset:expr) => {
        getp32!($st, $reg) & !($mask << (31 - $offset))
    };
}

macro_rules! clear_regs {
    ($st:expr) => {
        ::core::ptr::write_bytes(&mut $st as *mut _ as *mut u8, 0, ::core::mem::size_of_val(&$st))
    };
}

macro_rules! clear_dde {
    ($st:expr) => {{
        $st.dde_count = 0;
        $st.ddebc = 0;
        $st.ddead = 0;
    }};
}

macro_rules! clearp_dde {
    ($st:expr) => {{
        (*$st).dde_count = 0;
        (*$st).ddebc = 0;
        (*$st).ddead = 0;
    }};
}

macro_rules! clear_struct {
    ($st:expr) => {
        ::core::ptr::write_bytes(&mut $st as *mut _ as *mut u8, 0, ::core::mem::size_of_val(&$st))
    };
}

macro_rules! putnn {
    ($st:expr, $reg:ident, $x:expr, $mask:expr, $offset:expr) => {
        $st.$reg = htobe32(unget32!($st, $reg, $mask, $offset) | ((($x) & $mask) << (31 - $offset)))
    };
}

macro_rules! putpnn {
    ($st:expr, $reg:ident, $x:expr, $mask:expr, $offset:expr) => {
        (*$st).$reg = htobe32(ungetp32!($st, $reg, $mask, $offset) | ((($x) & $mask) << (31 - $offset)))
    };
}

macro_rules! put32 {
    ($st:expr, $reg:ident, $x:expr) => {
        $st.$reg = htobe32($x)
    };
}

macro_rules! putp32 {
    ($st:expr, $reg:ident, $x:expr) => {
        (*$st).$reg = htobe32($x)
    };
}

macro_rules! put64 {
    ($st:expr, $reg:ident, $x:expr) => {
        $st.$reg = htobe64($x)
    };
}

macro_rules! putp64 {
    ($st:expr, $reg:ident, $x:expr) => {
        (*$st).$reg = htobe64($x)
    };
}

pub(crate) use {
    clear_dde, clear_regs, clear_struct, clearp_dde, get32, get64, getnn, getp32, getp64, getpnn,
    put32, put64, putnn, putp32, putp64, putpnn, unget32, ungetp32,
};

/*
 * Completion extension ce(0) ce(1) ce(2). Bits ce(3-7)
 * unused. Section 6.6 Figure 6.7.
 */

pub unsafe fn get_csb_ce(st: nx_csb_t) -> u32 {
    getnn!(st.word0, csb_ce, CSB_CE_MASK, CSB_CE_OFFSET) as u32
}

pub unsafe fn get_csb_ce_ms3b(st: nx_csb_t) -> u32 {
    get_csb_ce(st) >> 5
}

pub unsafe fn put_csb_ce_ms3b(st: &mut nx_csb_t, x: u32) {
    putnn!(st.word0, csb_ce, (x as u32) << 5, CSB_CE_MASK, CSB_CE_OFFSET);
}

pub const CSB_CE_PARTIAL: u32 = 0x4;
pub const CSB_CE_TERMINATE: u32 = 0x2;
pub const CSB_CE_TPBC_VALID: u32 = 0x1;

pub const fn csb_ce_termination(x: u32) -> bool {
    (x & CSB_CE_TERMINATE) != 0
}

/* termination, output buffers may be modified, SPBC/TPBC invalid Fig.6-7 */

pub const fn csb_ce_check_completion(x: u32) -> bool {
    !csb_ce_termination(x)
}

/* if not terminated then check full or partial completion */

pub const fn csb_ce_partial_completion(x: u32) -> bool {
    (x & CSB_CE_PARTIAL) != 0
}

pub const fn csb_ce_full_completion(x: u32) -> bool {
    !csb_ce_partial_completion(x)
}

pub const fn csb_ce_tpbc_valid(x: u32) -> bool {
    (x & CSB_CE_TPBC_VALID) != 0
}

/* TPBC indicates successfully stored data count */

pub const fn csb_ce_default_err(x: u32) -> bool {
    csb_ce_termination(x)
}

/* most error CEs have CE(0)=0 and CE(1)=1 */

pub const fn csb_ce_cc3_partial(x: u32) -> bool {
    csb_ce_partial_completion(x)
}

/* some CC=3 are partially completed, Table 6-8 */

pub const fn csb_ce_cc64(x: u32) -> bool {
    (x & (CSB_CE_PARTIAL | CSB_CE_TERMINATE)) == 0
}

/*
 * Compression: when TPBC>SPBC then CC=64 Table 6-8; target didn't
 * compress smaller than source.
 */

/* Decompress SFBT combinations Tables 5-3, 6-4, 6-6 */

pub const SFBT_BFINAL: u32 = 0x1;
pub const SFBT_LIT: u32 = 0x4;
pub const SFBT_FHT: u32 = 0x5;
pub const SFBT_DHT: u32 = 0x6;
pub const SFBT_HDR: u32 = 0x7;

/*
 * NX gzip function codes. Table 6.2.
 * Bits 0:4 are the FC. Bit 5 is used by the DMA controller to
 * select one of the two Byte Count Limits.
 */

pub const GZIP_FC_LIMIT_MASK: u32 = 0x01;
pub const GZIP_FC_COMPRESS_FHT: u32 = 0x00;
pub const GZIP_FC_COMPRESS_DHT: u32 = 0x02;
pub const GZIP_FC_COMPRESS_FHT_COUNT: u32 = 0x04;
pub const GZIP_FC_COMPRESS_DHT_COUNT: u32 = 0x06;
pub const GZIP_FC_COMPRESS_RESUME_FHT: u32 = 0x08;
pub const GZIP_FC_COMPRESS_RESUME_DHT: u32 = 0x0a;
pub const GZIP_FC_COMPRESS_RESUME_FHT_COUNT: u32 = 0x0c;
pub const GZIP_FC_COMPRESS_RESUME_DHT_COUNT: u32 = 0x0e;
pub const GZIP_FC_DECOMPRESS: u32 = 0x10;
pub const GZIP_FC_DECOMPRESS_SINGLE_BLK_N_SUSPEND: u32 = 0x12;
pub const GZIP_FC_DECOMPRESS_RESUME: u32 = 0x14;
pub const GZIP_FC_DECOMPRESS_RESUME_SINGLE_BLK_N_SUSPEND: u32 = 0x16;
pub const GZIP_FC_WRAP: u32 = 0x1e;

pub const fn fc_is_compress(fc: u32) -> bool {
    (fc & 0x10) == 0
}

pub const fn fc_has_count(fc: u32) -> bool {
    fc_is_compress(fc) && ((fc & 0x4) != 0)
}

/* CSB.CC Error codes */

pub const ERR_NX_OK: u32 = 0;
pub const ERR_NX_ALIGNMENT: u32 = 1;
pub const ERR_NX_OPOVERLAP: u32 = 2;
pub const ERR_NX_DATA_LENGTH: u32 = 3;
pub const ERR_NX_TRANSLATION: u32 = 5;
pub const ERR_NX_PROTECTION: u32 = 6;
pub const ERR_NX_EXTERNAL_UE7: u32 = 7;
pub const ERR_NX_INVALID_OP: u32 = 8;
pub const ERR_NX_PRIVILEGE: u32 = 9;
pub const ERR_NX_INTERNAL_UE: u32 = 10;
pub const ERR_NX_EXTERN_UE_WR: u32 = 12;
pub const ERR_NX_TARGET_SPACE: u32 = 13;
pub const ERR_NX_EXCESSIVE_DDE: u32 = 14;
pub const ERR_NX_TRANSL_WR: u32 = 15;
pub const ERR_NX_PROTECT_WR: u32 = 16;
pub const ERR_NX_SUBFUNCTION: u32 = 17;
pub const ERR_NX_FUNC_ABORT: u32 = 18;
pub const ERR_NX_BYTE_MAX: u32 = 19;
pub const ERR_NX_CORRUPT_CRB: u32 = 20;
pub const ERR_NX_INVALID_CRB: u32 = 21;
pub const ERR_NX_INVALID_DDE: u32 = 30;
pub const ERR_NX_SEGMENTED_DDL: u32 = 31;
pub const ERR_NX_DDE_OVERFLOW: u32 = 33;
pub const ERR_NX_TPBC_GT_SPBC: u32 = 64;
pub const ERR_NX_MISSING_CODE: u32 = 66;
pub const ERR_NX_INVALID_DIST: u32 = 67;
pub const ERR_NX_INVALID_DHT: u32 = 68;
pub const ERR_NX_EXTERNAL_UE90: u32 = 90;
pub const ERR_NX_WDOG_TIMER: u32 = 224;
pub const ERR_NX_AT_FAULT: u32 = 250;
pub const ERR_NX_INTR_SERVER: u32 = 252;
pub const ERR_NX_UE253: u32 = 253;
pub const ERR_NX_NO_HW: u32 = 254;
pub const ERR_NX_HUNG_OP: u32 = 255;
pub const ERR_NX_END: u32 = 256;

/* initial values for non-resume operations */
pub const INIT_CRC: u32 = 0;   /* crc32(0L, Z_NULL, 0) */
pub const INIT_ADLER: u32 = 1; /* adler32(0L, Z_NULL, 0) adler is initialized to 1 */

/* prototypes */
#[repr(C)]
pub struct siginfo_t {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn nxu_submit_job(c: *mut nx_gzip_crb_cpb_t, handle: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;

    pub fn nxu_sigsegv_handler(
        sig: ::core::ffi::c_int,
        info: *mut siginfo_t,
        ctx: *mut ::core::ffi::c_void,
    );
    pub fn nxu_touch_pages(
        buf: *mut ::core::ffi::c_void,
        buf_len: ::core::ffi::c_long,
        page_len: ::core::ffi::c_long,
        wr: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;

    /* caller supplies a print buffer 4*sizeof(crb) */
    pub fn nx_crb_str(crb: *mut nx_gzip_crb_t, prbuf: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    pub fn nx_cpb_str(cpb: *mut nx_gzip_cpb_t, prbuf: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    pub fn nx_prt_hex(
        cp: *mut ::core::ffi::c_void,
        sz: ::core::ffi::c_int,
        prbuf: *mut ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    pub fn nx_lzcount_str(
        cpb: *mut nx_gzip_cpb_t,
        prbuf: *mut ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    pub fn nx_strerror(e: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
}

/* NX_SIM conditional declarations from the C header:
 * int nx_sim_init(void *ctx);
 * int nx_sim_end(void *ctx);
 * int nxu_run_sim_job(struct nx_gzip_crb_cpb_t *c, void *ctx);
 */

/* Deflate stream manipulation */

macro_rules! set_final_bit {
    ($x:expr) => {
        $x |= 1u8
    };
}

macro_rules! clr_final_bit {
    ($x:expr) => {
        $x &= !1u8
    };
}

macro_rules! append_empty_fh_blk {
    ($p:expr, $b:expr) => {{
        *$p = (2u8 | (1u8 & ($b as u8)));
        *$p.add(1) = 0;
    }};
}

pub(crate) use {append_empty_fh_blk, clr_final_bit, set_final_bit};

/*
 * append 10 bits 0000001b 00...... ;
 * assumes appending starts on a byte boundary; b is the final bit.
 */

/* NX_842 conditional declarations follow. */

#[repr(C, align(128))]
pub struct nx_eft_crb_t {
    pub word0: nx_eft_crb_word0_t, /* byte[0:3] */
    pub reserved1: u32,            /* byte[4:7] */
    pub word8: nx_eft_crb_word8_t,
    pub source_dde: nx_dde_t,      /* byte[16:31] */
    pub target_dde: nx_dde_t,      /* byte[32:47] */
    pub ccb: nx_ccb_t,             /* byte[48:63] */
    pub reserved64: nx_eft_crb_reserved64_t,
    pub csb: nx_csb_t,
}

#[repr(C)]
pub union nx_eft_crb_word0_t {
    pub eft_fc: u32, /* bits[29-31] */
}

#[repr(C)]
pub union nx_eft_crb_word8_t {
    pub csb_address: u64, /* byte[8:15] */
    pub fields: nx_eft_crb_word8_fields_t,
}

#[repr(C)]
pub struct nx_eft_crb_word8_fields_t {
    pub reserved2: u32,
    pub word: nx_eft_crb_word8_fields_word_t,
}

#[repr(C)]
pub union nx_eft_crb_word8_fields_word_t {
    pub crb_c: u32,  /* c==0 no ccb defined */
    pub crb_at: u32, /* at==0 address type is ignored; all addrs effective assumed. */
}

#[repr(C)]
pub union nx_eft_crb_reserved64_t {
    pub reserved64: [nx_qw_t; 3], /* byte[64:96] */
}

/* 842 CRB */

pub const EFT_FC_MASK: u32 = size_mask(3);
pub const EFT_FC_OFFSET: u32 = 31;
pub const EFT_FC_COMPRESS: u32 = 0x0;
pub const EFT_FC_COMPRESS_WITH_CRC: u32 = 0x1;
pub const EFT_FC_DECOMPRESS: u32 = 0x2;
pub const EFT_FC_DECOMPRESS_WITH_CRC: u32 = 0x3;
pub const EFT_FC_BLK_DATA_MOVE: u32 = 0x4;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
