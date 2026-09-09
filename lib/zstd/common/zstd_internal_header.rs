/* SPDX-License-Identifier: GPL-2.0+ OR BSD-3-Clause */
/* Direct Rust translation of zstd_internal.h. */

/* Dependencies are supplied by the surrounding translation unit. */

pub const ZSTD_TRACE: i32 = 0;

pub const ZSTD_OPT_NUM: u32 = 1 << 12;
pub const ZSTD_REP_NUM: usize = 3;
pub static REP_START_VALUE: [U32; ZSTD_REP_NUM] = [1, 4, 8];

pub const BIT7: u32 = 128;
pub const BIT6: u32 = 64;
pub const BIT5: u32 = 32;
pub const BIT4: u32 = 16;
pub const BIT1: u32 = 2;
pub const BIT0: u32 = 1;

pub const ZSTD_WINDOWLOG_ABSOLUTEMIN: u32 = 10;
pub static ZSTD_FCS_FIELD_SIZE: [usize; 4] = [0, 2, 4, 8];
pub static ZSTD_DID_FIELD_SIZE: [usize; 4] = [0, 1, 2, 4];
pub const ZSTD_FRAMEIDSIZE: usize = 4;
pub const ZSTD_BLOCKHEADERSIZE: usize = 3;
pub static ZSTD_BLOCK_HEADER_SIZE: usize = ZSTD_BLOCKHEADERSIZE;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum blockType_e { bt_raw, bt_rle, bt_compressed, bt_reserved }

pub const ZSTD_FRAMECHECKSUMSIZE: usize = 4;
pub const MIN_SEQUENCES_SIZE: usize = 1;
pub const MIN_CBLOCK_SIZE: usize = 2;
pub const MIN_LITERALS_FOR_4_STREAMS: usize = 6;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SymbolEncodingType_e { set_basic, set_rle, set_compressed, set_repeat }

pub const LONGNBSEQ: u32 = 0x7F00;
pub const MINMATCH: usize = 3;
pub const Litbits: usize = 8;
pub const LitHufLog: usize = 11;
pub const MaxLit: usize = (1 << Litbits) - 1;
pub const MaxML: usize = 52;
pub const MaxLL: usize = 35;
pub const DefaultMaxOff: usize = 28;
pub const MaxOff: usize = 31;
pub const MaxSeq: usize = if MaxLL > MaxML { MaxLL } else { MaxML };
pub const MLFSELog: usize = 9;
pub const LLFSELog: usize = 9;
pub const OffFSELog: usize = 8;
pub const MaxFSELog: usize = if MLFSELog > LLFSELog { if MLFSELog > OffFSELog { MLFSELog } else { OffFSELog } } else if LLFSELog > OffFSELog { LLFSELog } else { OffFSELog };
pub const MaxMLBits: usize = 16;
pub const MaxLLBits: usize = 16;
pub const ZSTD_MAX_HUF_HEADER_SIZE: usize = 128;
pub const ZSTD_MAX_FSE_HEADERS_SIZE: usize = ((MaxML + 1) * MLFSELog + (MaxLL + 1) * LLFSELog + (MaxOff + 1) * OffFSELog + 7) / 8;

pub static LL_bits: [U8; MaxLL + 1] = [0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 1,1,1,1,2,2,3,3, 4,6,7,8,9,10,11,12, 13,14,15,16];
pub static LL_defaultNorm: [S16; MaxLL + 1] = [4,3,2,2,2,2,2,2, 2,2,2,2,2,1,1,1, 2,2,2,2,2,2,2,2, 2,3,2,1,1,1,1,1, -1,-1,-1,-1];
pub const LL_DEFAULTNORMLOG: U32 = 6;
pub static LL_defaultNormLog: U32 = LL_DEFAULTNORMLOG;
pub static ML_bits: [U8; MaxML + 1] = [0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 1,1,1,1,2,2,3,3, 4,4,5,7,8,9,10,11, 12,13,14,15,16];
pub static ML_defaultNorm: [S16; MaxML + 1] = [1,4,3,2,2,2,2,2, 2,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,-1,-1, -1,-1,-1,-1,-1];
pub const ML_DEFAULTNORMLOG: U32 = 6;
pub static ML_defaultNormLog: U32 = ML_DEFAULTNORMLOG;
pub static OF_defaultNorm: [S16; DefaultMaxOff + 1] = [1,1,1,1,1,1,2,2, 2,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, -1,-1,-1,-1,-1];
pub const OF_DEFAULTNORMLOG: U32 = 5;
pub static OF_defaultNormLog: U32 = OF_DEFAULTNORMLOG;

pub unsafe fn ZSTD_copy8(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void) { ZSTD_memcpy(dst, src, 8); }
pub unsafe fn ZSTD_copy16(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void) {
    let mut buf = [0u8; 16];
    ZSTD_memcpy(buf.as_mut_ptr() as *mut _, src, 16);
    ZSTD_memcpy(dst, buf.as_ptr() as *const _, 16);
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ZSTD_overlap_e { ZSTD_no_overlap, ZSTD_overlap_src_before_dst }

pub unsafe fn ZSTD_wildcopy(dst: *mut BYTE, src: *const BYTE, length: ptrdiff_t, ovtype: ZSTD_overlap_e) {
    let diff = dst.offset_from(src);
    let mut ip = src;
    let mut op = dst;
    let oend = op.offset(length);
    if ovtype == ZSTD_overlap_e::ZSTD_overlap_src_before_dst && diff < 16 {
        loop { ZSTD_copy8(op as *mut _, ip as *const _); op = op.add(8); ip = ip.add(8); if op >= oend { break; } }
    } else {
        ZSTD_copy16(op as *mut _, ip as *const _);
        if 16 >= length { return; }
        op = op.add(16); ip = ip.add(16);
        loop { ZSTD_copy16(op as *mut _, ip as *const _); op = op.add(16); ip = ip.add(16); ZSTD_copy16(op as *mut _, ip as *const _); op = op.add(16); ip = ip.add(16); if op >= oend { break; } }
    }
}

pub unsafe fn ZSTD_limitCopy(dst: *mut core::ffi::c_void, dstCapacity: usize, src: *const core::ffi::c_void, srcSize: usize) -> usize {
    let length = if dstCapacity < srcSize { dstCapacity } else { srcSize };
    if length > 0 { ZSTD_memcpy(dst, src, length); }
    length
}

pub const ZSTD_WORKSPACETOOLARGE_FACTOR: usize = 3;
pub const ZSTD_WORKSPACETOOLARGE_MAXDURATION: usize = 128;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ZSTD_bufferMode_e { ZSTD_bm_buffered = 0, ZSTD_bm_stable = 1 }

#[repr(C)]
pub struct ZSTD_frameSizeInfo { pub nbBlocks: usize, pub compressedSize: usize, pub decompressedBound: u64 }

extern "C" {
    pub fn ZSTD_invalidateRepCodes(cctx: *mut ZSTD_CCtx);
    pub fn ZSTD_getcBlockSize(src: *const core::ffi::c_void, srcSize: usize, bpPtr: *mut blockProperties_t) -> usize;
    pub fn ZSTD_decodeSeqHeaders(dctx: *mut ZSTD_DCtx, nbSeqPtr: *mut i32, src: *const core::ffi::c_void, srcSize: usize) -> usize;
}

#[repr(C)]
pub struct blockProperties_t { pub blockType: blockType_e, pub lastBlock: U32, pub origSize: U32 }

pub unsafe fn ZSTD_cpuSupportsBmi2() -> i32 {
    let cpuid = ZSTD_cpuid();
    if ZSTD_cpuid_bmi1(cpuid) != 0 && ZSTD_cpuid_bmi2(cpuid) != 0 { 1 } else { 0 }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
