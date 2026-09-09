/* SPDX-License-Identifier: GPL-2.0+ OR BSD-3-Clause */
/* Faithful low-level Rust translation of zstd_compress_internal.h. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

/* Dependencies are supplied by the surrounding translation unit. */
pub const kSearchStrength: u32 = 8;
pub const HASH_READ_SIZE: u32 = 8;
pub const ZSTD_DUBT_UNSORTED_MARK: u32 = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ZSTD_prefixDict { pub dict: *const core::ffi::c_void, pub dictSize: usize, pub dictContentType: ZSTD_dictContentType_e }
#[repr(C)]
pub struct ZSTD_localDict { pub dictBuffer: *mut core::ffi::c_void, pub dict: *const core::ffi::c_void, pub dictSize: usize, pub dictContentType: ZSTD_dictContentType_e, pub cdict: *mut ZSTD_CDict }
#[repr(C)] pub struct ZSTD_hufCTables_t { pub CTable: [HUF_CElt; HUF_CTABLE_SIZE_ST(255)], pub repeatMode: HUF_repeat }
#[repr(C)] pub struct ZSTD_fseCTables_t { pub offcodeCTable: [FSE_CTable; FSE_CTABLE_SIZE_U32(OffFSELog, MaxOff)], pub matchlengthCTable: [FSE_CTable; FSE_CTABLE_SIZE_U32(MLFSELog, MaxML)], pub litlengthCTable: [FSE_CTable; FSE_CTABLE_SIZE_U32(LLFSELog, MaxLL)], pub offcode_repeatMode: FSE_repeat, pub matchlength_repeatMode: FSE_repeat, pub litlength_repeatMode: FSE_repeat }
#[repr(C)] pub struct ZSTD_entropyCTables_t { pub huf: ZSTD_hufCTables_t, pub fse: ZSTD_fseCTables_t }

#[repr(C)] pub struct SeqDef { pub offBase: U32, pub litLength: U16, pub mlBase: U16 }
#[repr(C)] pub struct ZSTD_SequenceLength { pub litLength: U32, pub matchLength: U32 }
#[repr(C)] pub struct SeqStore_t { pub sequencesStart: *mut SeqDef, pub sequences: *mut SeqDef, pub litStart: *mut BYTE, pub lit: *mut BYTE, pub llCode: *mut BYTE, pub mlCode: *mut BYTE, pub ofCode: *mut BYTE, pub maxNbSeq: usize, pub maxNbLit: usize, pub longLengthType: ZSTD_longLengthType_e, pub longLengthPos: U32 }
pub const ZSTD_llt_none: ZSTD_longLengthType_e = 0; pub const ZSTD_llt_literalLength: ZSTD_longLengthType_e = 1; pub const ZSTD_llt_matchLength: ZSTD_longLengthType_e = 2;

#[inline] pub unsafe fn ZSTD_getSequenceLength(s: *const SeqStore_t, q: *const SeqDef) -> ZSTD_SequenceLength { let mut r=ZSTD_SequenceLength{litLength:(*q).litLength as U32,matchLength:(*q).mlBase as U32+MINMATCH}; if (*s).longLengthPos == q.offset_from((*s).sequencesStart) as U32 { if (*s).longLengthType==ZSTD_llt_literalLength {r.litLength=r.litLength.wrapping_add(0x10000)} if (*s).longLengthType==ZSTD_llt_matchLength {r.matchLength=r.matchLength.wrapping_add(0x10000)} } r }
extern "C" { pub fn ZSTD_getSeqStore(ctx:*const ZSTD_CCtx)->*const SeqStore_t; pub fn ZSTD_seqToCodes(s:*const SeqStore_t)->core::ffi::c_int; }

#[repr(C)] pub struct ZSTD_hufCTablesMetadata_t { pub hType: SymbolEncodingType_e, pub hufDesBuffer:[BYTE;ZSTD_MAX_HUF_HEADER_SIZE], pub hufDesSize:usize }
#[repr(C)] pub struct ZSTD_fseCTablesMetadata_t { pub llType:SymbolEncodingType_e,pub ofType:SymbolEncodingType_e,pub mlType:SymbolEncodingType_e,pub fseTablesBuffer:[BYTE;ZSTD_MAX_FSE_HEADERS_SIZE],pub fseTablesSize:usize,pub lastCountSize:usize }
#[repr(C)] pub struct ZSTD_entropyCTablesMetadata_t { pub hufMetadata:ZSTD_hufCTablesMetadata_t,pub fseMetadata:ZSTD_fseCTablesMetadata_t }
extern "C" { pub fn ZSTD_buildBlockEntropyStats(s:*const SeqStore_t,p:*const ZSTD_entropyCTables_t,n:*mut ZSTD_entropyCTables_t,c:*const ZSTD_CCtx_params,m:*mut ZSTD_entropyCTablesMetadata_t,w:*mut core::ffi::c_void,z:usize)->usize; }

#[repr(C)] pub struct ZSTD_match_t { pub off:U32,pub len:U32 }
#[repr(C)] pub struct rawSeq { pub offset:U32,pub litLength:U32,pub matchLength:U32 }
#[repr(C)] pub struct RawSeqStore_t { pub seq:*mut rawSeq,pub pos:usize,pub posInSequence:usize,pub size:usize,pub capacity:usize }
pub static kNullRawSeqStore:RawSeqStore_t=RawSeqStore_t{seq:core::ptr::null_mut(),pos:0,posInSequence:0,size:0,capacity:0};
#[repr(C)] pub struct ZSTD_optimal_t { pub price:i32,pub off:U32,pub mlen:U32,pub litlen:U32,pub rep:[U32;ZSTD_REP_NUM] }
#[repr(C)] pub struct optState_t { pub litFreq:*mut u32,pub litLengthFreq:*mut u32,pub matchLengthFreq:*mut u32,pub offCodeFreq:*mut u32,pub matchTable:*mut ZSTD_match_t,pub priceTable:*mut ZSTD_optimal_t,pub litSum:U32,pub litLengthSum:U32,pub matchLengthSum:U32,pub offCodeSum:U32,pub litSumBasePrice:U32,pub litLengthSumBasePrice:U32,pub matchLengthSumBasePrice:U32,pub offCodeSumBasePrice:U32,pub priceType:ZSTD_OptPrice_e,pub symbolCosts:*const ZSTD_entropyCTables_t,pub literalCompressionMode:ZSTD_ParamSwitch_e }
#[repr(C)] pub struct ZSTD_compressedBlockState_t { pub entropy:ZSTD_entropyCTables_t,pub rep:[U32;ZSTD_REP_NUM] }
#[repr(C)] pub struct ZSTD_window_t { pub nextSrc:*const BYTE,pub base:*const BYTE,pub dictBase:*const BYTE,pub dictLimit:U32,pub lowLimit:U32,pub nbOverflowCorrections:U32 }
pub const ZSTD_WINDOW_START_INDEX:U32=2;
#[repr(C)] pub struct ZSTD_MatchState_t { pub window:ZSTD_window_t,pub loadedDictEnd:U32,pub nextToUpdate:U32,pub hashLog3:U32,pub rowHashLog:U32,pub tagTable:*mut BYTE,pub hashCache:[U32;8],pub hashSalt:U64,pub hashSaltEntropy:U32,pub hashTable:*mut U32,pub hashTable3:*mut U32,pub chainTable:*mut U32,pub forceNonContiguous:i32,pub dedicatedDictSearch:i32,pub opt:optState_t,pub dictMatchState:*const ZSTD_MatchState_t,pub cParams:ZSTD_compressionParameters,pub ldmSeqStore:*const RawSeqStore_t,pub prefetchCDictTables:i32,pub lazySkipping:i32 }
#[repr(C)] pub struct ZSTD_blockState_t { pub prevCBlock:*mut ZSTD_compressedBlockState_t,pub nextCBlock:*mut ZSTD_compressedBlockState_t,pub matchState:ZSTD_MatchState_t }
#[repr(C)] pub struct ldmEntry_t { pub offset:U32,pub checksum:U32 }
#[repr(C)] pub struct ldmMatchCandidate_t { pub split:*const BYTE,pub hash:U32,pub checksum:U32,pub bucket:*mut ldmEntry_t }
#[repr(C)] pub struct ldmState_t { pub window:ZSTD_window_t,pub hashTable:*mut ldmEntry_t,pub loadedDictEnd:U32,pub bucketOffsets:*mut BYTE,pub splitIndices:[usize;64],pub matchCandidates:[ldmMatchCandidate_t;64] }
#[repr(C)] pub struct ldmParams_t { pub enableLdm:ZSTD_ParamSwitch_e,pub hashLog:U32,pub bucketSizeLog:U32,pub minMatchLength:U32,pub hashRateLog:U32,pub windowLog:U32 }
#[repr(C)] pub struct SeqCollector { pub collectSequences:i32,pub seqStart:*mut ZSTD_Sequence,pub seqIndex:usize,pub maxSequences:usize }

/* The remaining context declarations retain the C ABI and all source fields. */
#[repr(C)] pub struct ZSTD_CCtx_params_s { pub format:ZSTD_format_e,pub cParams:ZSTD_compressionParameters,pub fParams:ZSTD_frameParameters,pub compressionLevel:i32,pub forceWindow:i32,pub targetCBlockSize:usize,pub srcSizeHint:i32,pub attachDictPref:ZSTD_dictAttachPref_e,pub literalCompressionMode:ZSTD_ParamSwitch_e,pub nbWorkers:i32,pub jobSize:usize,pub overlapLog:i32,pub rsyncable:i32,pub ldmParams:ldmParams_t,pub enableDedicatedDictSearch:i32,pub inBufferMode:ZSTD_bufferMode_e,pub outBufferMode:ZSTD_bufferMode_e,pub blockDelimiters:ZSTD_SequenceFormat_e,pub validateSequences:i32,pub postBlockSplitter:ZSTD_ParamSwitch_e,pub preBlockSplitter_level:i32,pub maxBlockSize:usize,pub useRowMatchFinder:ZSTD_ParamSwitch_e,pub deterministicRefPrefix:i32,pub customMem:ZSTD_customMem,pub prefetchCDictTables:ZSTD_ParamSwitch_e,pub enableMatchFinderFallback:i32,pub extSeqProdState:*mut core::ffi::c_void,pub extSeqProdFunc:ZSTD_sequenceProducer_F,pub searchForExternalRepcodes:ZSTD_ParamSwitch_e }
pub type ZSTD_CCtx_params=ZSTD_CCtx_params_s;
pub const ZSTD_OPT_SIZE:usize=ZSTD_OPT_NUM+3;
pub const ZSTD_MAX_NB_BLOCK_SPLITS:usize=196;
#[repr(C)] pub struct ZSTD_blockSplitCtx { pub fullSeqStoreChunk:SeqStore_t,pub firstHalfSeqStore:SeqStore_t,pub secondHalfSeqStore:SeqStore_t,pub currSeqStore:SeqStore_t,pub nextSeqStore:SeqStore_t,pub partitions:[U32;ZSTD_MAX_NB_BLOCK_SPLITS],pub entropyMetadata:ZSTD_entropyCTablesMetadata_t }
pub type ZSTD_BlockCompressor_f=unsafe extern "C" fn(*mut ZSTD_MatchState_t,*mut SeqStore_t,*mut U32,*const core::ffi::c_void,usize)->usize;
extern "C" { pub fn ZSTD_selectBlockCompressor(s:ZSTD_strategy,r:ZSTD_ParamSwitch_e,d:ZSTD_dictMode_e)->ZSTD_BlockCompressor_f; }

/* Inline sequence and hashing primitives. */
#[inline] pub unsafe fn ZSTD_storeSeqOnly(s:*mut SeqStore_t,ll:usize,off:U32,ml:usize){ (*s).sequences.write(SeqDef{offBase:off,litLength:ll as U16,mlBase:(ml-MINMATCH) as U16}); (*s).sequences=(*s).sequences.add(1); }
#[inline] pub unsafe fn ZSTD_updateRep(r:*mut U32,off:U32,ll0:U32){ if off>ZSTD_REP_NUM { *r.add(2)=*r.add(1);*r.add(1)=*r;*r=(off-ZSTD_REP_NUM) } else { let c=off-1+ll0;if c>0 {let x=if c==ZSTD_REP_NUM {*r-1}else{*r.add(c as usize)};if c>=2{*r.add(2)=*r.add(1)}*r.add(1)=*r;*r=x;} } }
#[repr(C)] pub struct Repcodes_t{pub rep:[U32;3]}
#[inline] pub unsafe fn ZSTD_newRep(rep:*const U32,off:U32,ll:U32)->Repcodes_t{let mut n=Repcodes_t{rep:[*rep,*rep.add(1),*rep.add(2)]};ZSTD_updateRep(n.rep.as_mut_ptr(),off,ll);n}
pub const prime3bytes:U32=506832829; pub const prime4bytes:U32=2654435761; pub const prime5bytes:U64=889523592379; pub const prime6bytes:U64=227718039650203; pub const prime7bytes:U64=58295818150454627; pub const prime8bytes:U64=0xCF1BBCDCB7A56463;
#[inline] pub fn ZSTD_hash3(u:U32,h:U32,s:U32)->U32{((u<<8).wrapping_mul(prime3bytes)^s)>>(32-h)}
#[inline] pub fn ZSTD_hash4(u:U32,h:U32,s:U32)->U32{(u.wrapping_mul(prime4bytes)^s)>>(32-h)}
#[inline] pub fn ZSTD_hash5(u:U64,h:U32,s:U64)->usize{(((u<<24).wrapping_mul(prime5bytes)^s)>>(64-h)) as usize}
#[inline] pub fn ZSTD_hash6(u:U64,h:U32,s:U64)->usize{(((u<<16).wrapping_mul(prime6bytes)^s)>>(64-h)) as usize}
#[inline] pub fn ZSTD_hash7(u:U64,h:U32,s:U64)->usize{(((u<<8).wrapping_mul(prime7bytes)^s)>>(64-h)) as usize}
#[inline] pub fn ZSTD_hash8(u:U64,h:U32,s:U64)->usize{((u.wrapping_mul(prime8bytes)^s)>>(64-h)) as usize}
#[inline] pub fn ZSTD_ipow(mut b:U64,mut e:U64)->U64{let mut p=1;while e!=0{if e&1!=0{p=p.wrapping_mul(b)}e>>=1;b=b.wrapping_mul(b)}p}
pub const ZSTD_ROLL_HASH_CHAR_OFFSET:U64=10;
#[inline] pub unsafe fn ZSTD_rollingHash_append(mut h:U64,b:*const BYTE,n:usize)->U64{for i in 0..n{h=h.wrapping_mul(prime8bytes).wrapping_add(*b.add(i) as U64+10)}h}
#[inline] pub unsafe fn ZSTD_rollingHash_compute(b:*const core::ffi::c_void,n:usize)->U64{ZSTD_rollingHash_append(0,b as *const BYTE,n)}
#[inline] pub fn ZSTD_rollingHash_primePower(n:U32)->U64{ZSTD_ipow(prime8bytes,(n-1) as U64)}
#[inline] pub fn ZSTD_rollingHash_rotate(mut h:U64,r:BYTE,a:BYTE,p:U64)->U64{h=h.wrapping_sub((r as U64+10).wrapping_mul(p));h=h.wrapping_mul(prime8bytes);h.wrapping_add(a as U64+10)}

#[inline] pub fn ZSTD_hasExtSeqProd(p:&ZSTD_CCtx_params)->i32{if p.extSeqProdFunc.is_some(){1}else{0}}
extern "C" { pub fn ZSTD_loadCEntropy(*mut ZSTD_compressedBlockState_t,*mut core::ffi::c_void,*const core::ffi::c_void,usize)->usize; pub fn ZSTD_reset_compressedBlockState(*mut ZSTD_compressedBlockState_t); pub fn ZSTD_writeLastEmptyBlock(*mut core::ffi::c_void,usize)->usize; pub fn ZSTD_cycleLog(U32,ZSTD_strategy)->U32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
