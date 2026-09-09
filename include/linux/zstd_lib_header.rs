/* SPDX-License-Identifier: GPL-2.0+ OR BSD-3-Clause */
/* Rust translation of zstd_lib.h. C preprocessor visibility and conditional
 * compilation are represented by ordinary Rust declarations and comments. */

use core::ffi::c_void;

pub const ZSTD_VERSION_MAJOR: u32 = 1;
pub const ZSTD_VERSION_MINOR: u32 = 5;
pub const ZSTD_VERSION_RELEASE: u32 = 7;
pub const ZSTD_VERSION_NUMBER: u32 = ZSTD_VERSION_MAJOR * 100 * 100 + ZSTD_VERSION_MINOR * 100 + ZSTD_VERSION_RELEASE;
pub const ZSTD_CLEVEL_DEFAULT: i32 = 3;
pub const ZSTD_MAGICNUMBER: u32 = 0xFD2FB528;
pub const ZSTD_MAGIC_DICTIONARY: u32 = 0xEC30A437;
pub const ZSTD_MAGIC_SKIPPABLE_START: u32 = 0x184D2A50;
pub const ZSTD_MAGIC_SKIPPABLE_MASK: u32 = 0xFFFFFFF0;
pub const ZSTD_BLOCKSIZELOG_MAX: i32 = 17;
pub const ZSTD_BLOCKSIZE_MAX: usize = 1usize << ZSTD_BLOCKSIZELOG_MAX;
pub const ZSTD_CONTENTSIZE_UNKNOWN: u64 = u64::MAX;
pub const ZSTD_CONTENTSIZE_ERROR: u64 = u64::MAX - 1;
pub const ZSTD_MAX_INPUT_SIZE: u64 = 0xFF00FF00FF00FF00;

#[repr(C)] pub struct ZSTD_CCtx_s { _private: [u8; 0] }
#[repr(C)] pub struct ZSTD_DCtx_s { _private: [u8; 0] }
#[repr(C)] pub struct ZSTD_CDict_s { _private: [u8; 0] }
#[repr(C)] pub struct ZSTD_DDict_s { _private: [u8; 0] }
#[repr(C)] pub struct ZSTD_CCtx_params_s { _private: [u8; 0] }
#[repr(C)] pub struct POOL_ctx_s { _private: [u8; 0] }
pub type ZSTD_CCtx = ZSTD_CCtx_s;
pub type ZSTD_DCtx = ZSTD_DCtx_s;
pub type ZSTD_CDict = ZSTD_CDict_s;
pub type ZSTD_DDict = ZSTD_DDict_s;
pub type ZSTD_CCtx_params = ZSTD_CCtx_params_s;
pub type ZSTD_CStream = ZSTD_CCtx;
pub type ZSTD_DStream = ZSTD_DCtx;
pub type ZSTD_threadPool = POOL_ctx_s;

#[repr(C)] pub struct ZSTD_inBuffer { pub src: *const c_void, pub size: usize, pub pos: usize }
#[repr(C)] pub struct ZSTD_outBuffer { pub dst: *mut c_void, pub size: usize, pub pos: usize }
#[repr(C)] pub struct ZSTD_bounds { pub error: usize, pub lowerBound: i32, pub upperBound: i32 }

#[repr(i32)] #[derive(Copy, Clone)] pub enum ZSTD_strategy { ZSTD_fast=1, ZSTD_dfast, ZSTD_greedy, ZSTD_lazy, ZSTD_lazy2, ZSTD_btlazy2, ZSTD_btopt, ZSTD_btultra, ZSTD_btultra2 }
#[repr(i32)] #[derive(Copy, Clone)] pub enum ZSTD_ResetDirective { ZSTD_reset_session_only=1, ZSTD_reset_parameters=2, ZSTD_reset_session_and_parameters=3 }
#[repr(i32)] #[derive(Copy, Clone)] pub enum ZSTD_EndDirective { ZSTD_e_continue=0, ZSTD_e_flush=1, ZSTD_e_end=2 }
#[repr(i32)] #[derive(Copy, Clone)] pub enum ZSTD_dParameter { ZSTD_d_windowLogMax=100, ZSTD_d_experimentalParam1=1000, ZSTD_d_experimentalParam2, ZSTD_d_experimentalParam3, ZSTD_d_experimentalParam4, ZSTD_d_experimentalParam5, ZSTD_d_experimentalParam6 }
#[repr(i32)] #[derive(Copy, Clone)] pub enum ZSTD_format_e { ZSTD_f_zstd1=0, ZSTD_f_zstd1_magicless=1 }
#[repr(i32)] #[derive(Copy, Clone)] pub enum ZSTD_FrameType_e { ZSTD_frame=0, ZSTD_skippableFrame=1 }
pub type ZSTD_frameType_e = ZSTD_FrameType_e;
#[repr(C)] pub struct ZSTD_FrameHeader { pub frameContentSize:u64, pub windowSize:u64, pub blockSizeMax:u32, pub frameType:ZSTD_FrameType_e, pub headerSize:u32, pub dictID:u32, pub checksumFlag:u32, pub _reserved1:u32, pub _reserved2:u32 }
pub type ZSTD_frameHeader = ZSTD_FrameHeader;

extern "C" {
    pub fn ZSTD_versionNumber() -> u32;
    pub fn ZSTD_versionString() -> *const i8;
    pub fn ZSTD_compress(dst:*mut c_void, dstCapacity:usize, src:*const c_void, srcSize:usize, compressionLevel:i32) -> usize;
    pub fn ZSTD_decompress(dst:*mut c_void, dstCapacity:usize, src:*const c_void, srcSize:usize) -> usize;
    pub fn ZSTD_getFrameContentSize(src:*const c_void, srcSize:usize) -> u64;
    pub fn ZSTD_getDecompressedSize(src:*const c_void, srcSize:usize) -> u64;
    pub fn ZSTD_findFrameCompressedSize(src:*const c_void, srcSize:usize) -> usize;
    pub fn ZSTD_compressBound(srcSize:usize) -> usize;
    pub fn ZSTD_isError(result:usize) -> u32;
    pub fn ZSTD_getErrorCode(functionResult:usize) -> i32;
    pub fn ZSTD_getErrorName(result:usize) -> *const i8;
    pub fn ZSTD_minCLevel() -> i32;
    pub fn ZSTD_maxCLevel() -> i32;
    pub fn ZSTD_defaultCLevel() -> i32;
    pub fn ZSTD_createCCtx() -> *mut ZSTD_CCtx;
    pub fn ZSTD_freeCCtx(cctx:*mut ZSTD_CCtx) -> usize;
    pub fn ZSTD_createDCtx() -> *mut ZSTD_DCtx;
    pub fn ZSTD_freeDCtx(dctx:*mut ZSTD_DCtx) -> usize;
}

pub const ZSTD_FRAMEHEADERSIZE_MAX:u32=18;
pub const ZSTD_SKIPPABLEHEADERSIZE:u32=8;
pub const ZSTD_WINDOWLOG_MAX_32:i32=30;
pub const ZSTD_WINDOWLOG_MAX_64:i32=31;
pub const ZSTD_WINDOWLOG_MIN:i32=10;
pub const ZSTD_HASHLOG_MIN:i32=6;
pub const ZSTD_WINDOWLOG_LIMIT_DEFAULT:i32=27;
pub const ZSTD_LDM_MINMATCH_MIN:i32=4;
pub const ZSTD_LDM_MINMATCH_MAX:i32=4096;
pub const ZSTD_OVERLAPLOG_MIN:i32=0;
pub const ZSTD_OVERLAPLOG_MAX:i32=9;
pub const ZSTD_TARGETCBLOCKSIZE_MIN:i32=1340;
pub const ZSTD_BLOCKSIZE_MAX_MIN:usize=1usize<<10;
pub const ZSTD_SEQUENCE_PRODUCER_ERROR:usize=usize::MAX;

#[repr(C)] pub struct ZSTD_Sequence { pub offset:u32, pub litLength:u32, pub matchLength:u32, pub rep:u32 }
#[repr(C)] pub struct ZSTD_compressionParameters { pub windowLog:u32, pub chainLog:u32, pub hashLog:u32, pub searchLog:u32, pub minMatch:u32, pub targetLength:u32, pub strategy:ZSTD_strategy }
#[repr(C)] pub struct ZSTD_frameParameters { pub contentSizeFlag:i32, pub checksumFlag:i32, pub noDictIDFlag:i32 }
#[repr(C)] pub struct ZSTD_parameters { pub cParams:ZSTD_compressionParameters, pub fParams:ZSTD_frameParameters }
pub type ZSTD_allocFunction = unsafe extern "C" fn(*mut c_void, usize) -> *mut c_void;
pub type ZSTD_freeFunction = unsafe extern "C" fn(*mut c_void, *mut c_void);
#[repr(C)] pub struct ZSTD_customMem { pub customAlloc:Option<ZSTD_allocFunction>, pub customFree:Option<ZSTD_freeFunction>, pub opaque:*mut c_void }

// The remaining static-linking-only declarations retain the C ABI and are
// supplied by the corresponding zstd library dependency.
extern "C" {
    pub fn ZSTD_findDecompressedSize(src:*const c_void, srcSize:usize)->u64;
    pub fn ZSTD_decompressBound(src:*const c_void, srcSize:usize)->u64;
    pub fn ZSTD_frameHeaderSize(src:*const c_void, srcSize:usize)->usize;
    pub fn ZSTD_getFrameHeader(zfhPtr:*mut ZSTD_FrameHeader, src:*const c_void, srcSize:usize)->usize;
    pub fn ZSTD_sequenceBound(srcSize:usize)->usize;
    pub fn ZSTD_compressSequences(cctx:*mut ZSTD_CCtx,dst:*mut c_void,dstCapacity:usize,inSeqs:*const ZSTD_Sequence,inSeqsSize:usize,src:*const c_void,srcSize:usize)->usize;
    pub fn ZSTD_initStaticCCtx(workspace:*mut c_void,workspaceSize:usize)->*mut ZSTD_CCtx;
    pub fn ZSTD_initStaticDCtx(workspace:*mut c_void,workspaceSize:usize)->*mut ZSTD_DCtx;
    pub fn ZSTD_decompressStream(zds:*mut ZSTD_DStream,output:*mut ZSTD_outBuffer,input:*mut ZSTD_inBuffer)->usize;
    pub fn ZSTD_compressStream2(cctx:*mut ZSTD_CCtx,output:*mut ZSTD_outBuffer,input:*mut ZSTD_inBuffer,endOp:ZSTD_EndDirective)->usize;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
