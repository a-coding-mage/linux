// SPDX-License-Identifier: GPL-2.0+ OR BSD-3-Clause
/* Faithful low-level translation of zstd_ldm.c. External types and functions
 * are supplied by the surrounding zstd translation. */

const LDM_BUCKET_SIZE_LOG: u32 = 4;
const LDM_MIN_MATCH_LENGTH: u32 = 64;
const LDM_HASH_RLOG: u32 = 7;

#[repr(C)]
#[derive(Copy, Clone)]
struct ldmRollingHashState_t { rolling: U64, stopMask: U64 }

unsafe fn ZSTD_ldm_gear_init(state: *mut ldmRollingHashState_t, params: *const ldmParams_t) {
    let max_bits = core::cmp::min((*params).minMatchLength, 64);
    let rate = (*params).hashRateLog;
    (*state).rolling = !(0u32) as U64;
    (*state).stopMask = if rate > 0 && rate <= max_bits {
        (((1u64 << rate) - 1) << (max_bits - rate))
    } else { (1u64 << rate) - 1 };
}

unsafe fn ZSTD_ldm_gear_reset(state: *mut ldmRollingHashState_t, data: *const BYTE, min_match: usize) {
    let mut hash = (*state).rolling; let mut n = 0usize;
    while n + 3 < min_match { for _ in 0..4 { hash = (hash << 1).wrapping_add(ZSTD_ldm_gearTab[*data.add(n) as usize]); n += 1; } }
    while n < min_match { hash = (hash << 1).wrapping_add(ZSTD_ldm_gearTab[*data.add(n) as usize]); n += 1; }
    (*state).rolling = hash;
}

unsafe fn ZSTD_ldm_gear_feed(state: *mut ldmRollingHashState_t, data: *const BYTE, size: usize, splits: *mut usize, num: *mut u32) -> usize {
    let mut hash = (*state).rolling; let mask = (*state).stopMask; let mut n = 0usize;
    while n < size {
        hash = (hash << 1).wrapping_add(ZSTD_ldm_gearTab[*data.add(n) as usize]); n += 1;
        if (hash & mask) == 0 { *splits.add(*num as usize) = n; *num += 1; if *num == LDM_BATCH_SIZE { break; } }
    }
    (*state).rolling = hash; n
}

pub unsafe fn ZSTD_ldm_adjustParameters(params: *mut ldmParams_t, cp: *const ZSTD_compressionParameters) {
    (*params).windowLog = (*cp).windowLog;
    if (*params).hashRateLog == 0 {
        if (*params).hashLog > 0 { assert!((*params).hashLog <= ZSTD_HASHLOG_MAX); if (*params).windowLog > (*params).hashLog { (*params).hashRateLog = (*params).windowLog - (*params).hashLog; } }
        else { (*params).hashRateLog = 7 - ((*cp).strategy / 3); }
    }
    if (*params).hashLog == 0 { (*params).hashLog = BOUNDED(ZSTD_HASHLOG_MIN, (*params).windowLog - (*params).hashRateLog, ZSTD_HASHLOG_MAX); }
    if (*params).minMatchLength == 0 { (*params).minMatchLength = LDM_MIN_MATCH_LENGTH; if (*cp).strategy >= ZSTD_btultra { (*params).minMatchLength /= 2; } }
    if (*params).bucketSizeLog == 0 { (*params).bucketSizeLog = BOUNDED(LDM_BUCKET_SIZE_LOG, (*cp).strategy as U32, ZSTD_LDM_BUCKETSIZELOG_MAX); }
    (*params).bucketSizeLog = core::cmp::min((*params).bucketSizeLog, (*params).hashLog);
}

pub unsafe fn ZSTD_ldm_getTableSize(params: ldmParams_t) -> usize { let hs = 1usize << params.hashLog; let bl = core::cmp::min(params.bucketSizeLog, params.hashLog); let bs = 1usize << (params.hashLog-bl); let total = ZSTD_cwksp_alloc_size(bs) + ZSTD_cwksp_alloc_size(hs * core::mem::size_of::<ldmEntry_t>()); if params.enableLdm == ZSTD_ps_enable { total } else { 0 } }
pub unsafe fn ZSTD_ldm_getMaxNbSeq(params: ldmParams_t, max: usize) -> usize { if params.enableLdm == ZSTD_ps_enable { max / params.minMatchLength as usize } else { 0 } }

unsafe fn ZSTD_ldm_getBucket(s: *const ldmState_t, hash: usize, log: U32) -> *mut ldmEntry_t { (*s).hashTable.add(hash << log) }
unsafe fn ZSTD_ldm_insertEntry(s: *mut ldmState_t, hash: usize, e: ldmEntry_t, log: U32) { let p = (*s).bucketOffsets.add(hash); let o = *p as usize; *ZSTD_ldm_getBucket(s, hash, log).add(o) = e; *p = ((o+1) & ((1u32<<log)-1) as usize) as BYTE; }
unsafe fn ZSTD_ldm_countBackwardsMatch(mut a:*const BYTE, anchor:*const BYTE, mut m:*const BYTE, base:*const BYTE)->usize { let mut n=0; while a>anchor && m>base && *a.sub(1)==*m.sub(1) {a=a.sub(1);m=m.sub(1);n+=1;} n }
unsafe fn ZSTD_ldm_countBackwardsMatch_2segments(i:*const BYTE,a:*const BYTE,m:*const BYTE,mb:*const BYTE,ds:*const BYTE,de:*const BYTE)->usize { let mut n=ZSTD_ldm_countBackwardsMatch(i,a,m,mb); if m.sub(n)!=mb || mb==ds {return n;} n += ZSTD_ldm_countBackwardsMatch(i.sub(n),a,de,ds); n }

pub unsafe fn ZSTD_ldm_fillHashTable(s:*mut ldmState_t, mut ip:*const BYTE, end:*const BYTE, p:*const ldmParams_t) { let min=(*p).minMatchLength as usize; let hb=(*p).hashLog-(*p).bucketSizeLog; let base=(*s).window.base; let start=ip; let mut st=ldmRollingHashState_t{rolling:0,stopMask:0}; ZSTD_ldm_gear_init(&mut st,p); while ip<end { let mut ns=0; let h=ZSTD_ldm_gear_feed(&mut st,ip,end.offset_from(ip) as usize,(*s).splitIndices,&mut ns); for n in 0..ns as usize { let x=ip.add(*(*s).splitIndices.add(n)); if x>=start.add(min) { let sp=x.sub(min); let xx=xxh64(sp,min,0); ZSTD_ldm_insertEntry(s,(xx & (((1u64<<hb)-1))) as usize,ldmEntry_t{offset:sp.offset_from(base) as U32,checksum:(xx>>32) as U32},(*p).bucketSizeLog); } } ip=ip.add(h); } }

unsafe fn ZSTD_ldm_reduceTable(t:*mut ldmEntry_t,size:U32,r:U32){for i in 0..size as usize{if (*t.add(i)).offset<r{(*t.add(i)).offset=0}else{(*t.add(i)).offset-=r;}}}

pub unsafe fn ZSTD_ldm_skipSequences(r:*mut RawSeqStore_t,mut src:usize,min:U32){while src>0&&(*r).pos<(*r).size{let q=(*r).seq.add((*r).pos);if src<=(*q).litLength{(*q).litLength-=src as U32;return}src-=(*q).litLength as usize;(*q).litLength=0;if src<(*q).matchLength{(*q).matchLength-=src as U32;if (*q).matchLength<min{if (*r).pos+1<(*r).size{(*q.add(1)).litLength+=(*q).matchLength;}(*r).pos+=1;}return}src-=(*q).matchLength as usize;(*q).matchLength=0;(*r).pos+=1;}}
unsafe fn maybeSplitSequence(r:*mut RawSeqStore_t,rem:U32,min:U32)->rawSeq{let mut q=*(*r).seq.add((*r).pos);assert!(q.offset>0);if rem>=q.litLength+q.matchLength{(*r).pos+=1;return q}if rem<=q.litLength{q.offset=0}else{q.matchLength=rem-q.litLength;if q.matchLength<min{q.offset=0}}ZSTD_ldm_skipSequences(r,rem as usize,min);q}
pub unsafe fn ZSTD_ldm_skipRawSeqStoreBytes(r:*mut RawSeqStore_t,n:usize){let mut cur=((*r).posInSequence+n) as U32;while cur>0&&(*r).pos<(*r).size{let q=*(*r).seq.add((*r).pos);if cur>=q.litLength+q.matchLength{cur-=q.litLength+q.matchLength;(*r).pos+=1}else{(*r).posInSequence=cur;break}}if cur==0||(*r).pos==(*r).size{(*r).posInSequence=0}}

unsafe fn ZSTD_ldm_limitTableUpdate(ms:*mut ZSTD_MatchState_t,anchor:*const BYTE){let cur=anchor.offset_from((*ms).window.base) as U32;if cur>(*ms).nextToUpdate+1024{(*ms).nextToUpdate=cur-core::cmp::min(512,cur-(*ms).nextToUpdate-1024);}}

pub unsafe fn ZSTD_ldm_generateSequences(_s:*mut ldmState_t,_q:*mut RawSeqStore_t,_p:*const ldmParams_t,_src:*const core::ffi::c_void,_size:usize)->usize{unimplemented!("internal zstd window dependency")}
pub unsafe fn ZSTD_ldm_blockCompress(_raw:*mut RawSeqStore_t,_ms:*mut ZSTD_MatchState_t,_seq:*mut SeqStore_t,_rep:*mut U32,_row:ZSTD_ParamSwitch_e,_src:*const core::ffi::c_void,_size:usize)->usize{unimplemented!("dependent block compressor")}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
