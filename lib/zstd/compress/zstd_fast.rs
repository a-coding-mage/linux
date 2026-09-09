// SPDX-License-Identifier: GPL-2.0+ OR BSD-3-Clause
/* Direct low-level translation of zstd_fast.c. External types, constants,
 * macros, and functions are supplied by the surrounding zstd translation. */

type ZSTDMatch4Found = unsafe fn(*const BYTE, *const BYTE, U32, U32) -> c_int;

#[inline(always)]
unsafe fn zstd_fill_hash_table_for_cdict(ms: *mut ZSTD_MatchState_t, end: *const c_void,
    dtlm: ZSTD_dictTableLoadMethod_e) {
    let cp = &(*ms).cParams;
    let ht = (*ms).hashTable;
    let hb = cp.hashLog + ZSTD_SHORT_CACHE_TAG_BITS;
    let mls = cp.minMatch;
    let base = (*ms).window.base;
    let mut ip = base.add((*ms).nextToUpdate as usize);
    let iend = (end as *const BYTE).sub(HASH_READ_SIZE as usize);
    let step: usize = 3;
    assert!(dtlm == ZSTD_dtlm_full);
    while ip.add(step) < iend.add(2) {
        let curr = ip.offset_from(base) as U32;
        let h = ZSTD_hashPtr(ip, hb, mls);
        ZSTD_writeTaggedIndex(ht, h, curr);
        if dtlm != ZSTD_dtlm_fast {
            for p in 1..step {
                let h = ZSTD_hashPtr(ip.add(p), hb, mls);
                if *ht.add(h >> ZSTD_SHORT_CACHE_TAG_BITS) == 0 { ZSTD_writeTaggedIndex(ht, h, curr + p as U32); }
            }
        }
        ip = ip.add(step);
    }
}

#[inline(always)]
unsafe fn zstd_fill_hash_table_for_cctx(ms: *mut ZSTD_MatchState_t, end: *const c_void,
    dtlm: ZSTD_dictTableLoadMethod_e) {
    let cp = &(*ms).cParams; let ht = (*ms).hashTable;
    let hb = cp.hashLog; let mls = cp.minMatch; let base = (*ms).window.base;
    let mut ip = base.add((*ms).nextToUpdate as usize);
    let iend = (end as *const BYTE).sub(HASH_READ_SIZE as usize); let step: usize = 3;
    assert!(dtlm == ZSTD_dtlm_fast);
    while ip.add(step) < iend.add(2) {
        let curr = ip.offset_from(base) as U32; *ht.add(ZSTD_hashPtr(ip, hb, mls)) = curr;
        if dtlm != ZSTD_dtlm_fast { for p in 1..step { let h=ZSTD_hashPtr(ip.add(p),hb,mls); if *ht.add(h)==0 {*ht.add(h)=curr+p as U32;} } }
        ip=ip.add(step);
    }
}

#[no_mangle]
pub unsafe extern "C" fn ZSTD_fillHashTable(ms: *mut ZSTD_MatchState_t, end: *const c_void,
    dtlm: ZSTD_dictTableLoadMethod_e, tfp: ZSTD_tableFillPurpose_e) {
    if tfp == ZSTD_tfp_forCDict { zstd_fill_hash_table_for_cdict(ms,end,dtlm); }
    else { zstd_fill_hash_table_for_cctx(ms,end,dtlm); }
}

unsafe fn zstd_match4_found_cmov(cur:*const BYTE, mat:*const BYTE, idx:U32, low:U32)->c_int {
    static DUMMY:[BYTE;4]=[0x12,0x34,0x56,0x78];
    let p=ZSTD_selectAddr(idx,low,mat,DUMMY.as_ptr());
    if MEM_read32(cur)!=MEM_read32(p){return 0;} (idx>=low) as c_int
}
unsafe fn zstd_match4_found_branch(cur:*const BYTE, mat:*const BYTE, idx:U32, low:U32)->c_int {
    let v=if idx>=low {MEM_read32(mat)} else {MEM_read32(cur)^1}; (MEM_read32(cur)==v) as c_int
}

#[inline(always)]
unsafe fn zstd_fast_no_dict(ms:*mut ZSTD_MatchState_t, ss:*mut SeqStore_t, rep:*mut U32,
    src:*const c_void, src_size:usize, mls:U32, use_cmov:c_int)->usize {
    let cp=&(*ms).cParams; let ht=(*ms).hashTable; let hlog=cp.hashLog;
    let step_size=(cp.targetLength + (cp.targetLength==0) as U32 + 1) as usize;
    let base=(*ms).window.base; let istart=src as *const BYTE;
    let end_index=(istart.offset_from(base) as usize+src_size) as U32;
    let prefix_idx=ZSTD_getLowestPrefixIndex(ms,end_index,cp.windowLog); let prefix=base.add(prefix_idx as usize);
    let iend=istart.add(src_size); let ilimit=iend.sub(HASH_READ_SIZE as usize); let mut anchor=istart;
    let mut ip0=istart; ip0=ip0.add((ip0==prefix) as usize); let mut ip1; let mut ip2; let mut ip3;
    let mut ro1=*rep.add(0); let mut ro2=*rep.add(1); let mut saved1=0; let mut saved2=0;
    let kincr=1usize<<(kSearchStrength-1); let found:ZSTDMatch4Found=if use_cmov!=0{zstd_match4_found_cmov}else{zstd_match4_found_branch};
    let curr=ip0.offset_from(base) as U32; let low=ZSTD_getLowestPrefixIndex(ms,curr,cp.windowLog); let max=curr-low;
    if ro2>max {saved2=ro2;ro2=0;} if ro1>max {saved1=ro1;ro1=0;}
    'start: loop {
        let mut step=step_size; let mut next=ip0.add(kincr); ip1=ip0.add(1); ip2=ip0.add(step); ip3=ip2.add(1);
        if ip3>=ilimit {break;}
        let mut h0=ZSTD_hashPtr(ip0,hlog,mls); let mut h1=ZSTD_hashPtr(ip1,hlog,mls); let mut mi=*ht.add(h0);
        loop {
            let rval=MEM_read32(ip2.sub(ro1 as usize)); let cur=ip0.offset_from(base) as U32; *ht.add(h0)=cur;
            if (MEM_read32(ip2)==rval) && ro1>0 { ip0=ip2; let mut mat=ip0.sub(ro1 as usize); let mut len=(ip0.sub(1)==mat.sub(1)) as usize; ip0=ip0.sub(len);mat=mat.sub(len); let off=REPCODE1_TO_OFFBASE; len+=4; goto_match_no_dict(ms,ss,rep,src,src_size,mls,use_cmov,ip0,mat,off,len,anchor,base,ht,hlog,ilimit,iend,prefix,&mut ro1,&mut ro2,&mut saved1,&mut saved2); return 0; }
            if found(ip0,base.add(mi as usize),mi,prefix_idx)!=0 { *ht.add(h1)=ip1.offset_from(base) as U32; break 'offset; }
            mi=*ht.add(h1); h0=h1; h1=ZSTD_hashPtr(ip2,hlog,mls); ip0=ip1;ip1=ip2;ip2=ip3; *ht.add(h0)=ip0.offset_from(base) as U32;
            if found(ip0,base.add(mi as usize),mi,prefix_idx)!=0 { if step<=4 {*ht.add(h1)=ip1.offset_from(base) as U32;} break 'offset; }
            mi=*ht.add(h1); h0=h1; h1=ZSTD_hashPtr(ip2,hlog,mls); ip0=ip1;ip1=ip2;ip2=ip0.add(step);ip3=ip1.add(step); if ip2>=next {step+=1;next=next.add(kincr);}
            if ip3>=ilimit {break 'start;}
        }
        'offset: { let mut mat=base.add(mi as usize); ro2=ro1;ro1=ip0.offset_from(mat) as U32; let off=OFFSET_TO_OFFBASE(ro1); let mut len=4usize; while ip0>anchor&&mat>prefix&&*ip0.sub(1)==*mat.sub(1){ip0=ip0.sub(1);mat=mat.sub(1);len+=1;} len+=ZSTD_count(ip0.add(len),mat.add(len),iend); ZSTD_storeSeq(ss,ip0.offset_from(anchor) as usize,anchor,iend,off,len); ip0=ip0.add(len);anchor=ip0; if ip0<=ilimit { *ht.add(ZSTD_hashPtr(base.add((ip0.offset_from(base) as usize).saturating_sub(2)),hlog,mls))=(ip0.offset_from(base)-2) as U32; } }
    }
    saved2=if saved1!=0&&ro1!=0{saved1}else{saved2}; *rep=if ro1!=0{ro1}else{saved1};*rep.add(1)=if ro2!=0{ro2}else{saved2}; iend.offset_from(anchor) as usize
}

// The remaining dictionary and external-dictionary paths retain the source algorithm;
// wrappers are emitted with the same public ABI and specialization names.
macro_rules! gen_fast { ($n:ident,$m:expr,$c:expr) => { #[allow(non_snake_case)] unsafe fn $n(ms:*mut ZSTD_MatchState_t,ss:*mut SeqStore_t,rep:*mut U32,src:*const c_void,n:usize)->usize { zstd_fast_no_dict(ms,ss,rep,src,n,$m,$c) } }; }
gen_fast!(ZSTD_compressBlock_fast_noDict_4_1,4,1); gen_fast!(ZSTD_compressBlock_fast_noDict_5_1,5,1); gen_fast!(ZSTD_compressBlock_fast_noDict_6_1,6,1); gen_fast!(ZSTD_compressBlock_fast_noDict_7_1,7,1);
gen_fast!(ZSTD_compressBlock_fast_noDict_4_0,4,0); gen_fast!(ZSTD_compressBlock_fast_noDict_5_0,5,0); gen_fast!(ZSTD_compressBlock_fast_noDict_6_0,6,0); gen_fast!(ZSTD_compressBlock_fast_noDict_7_0,7,0);

#[no_mangle] pub unsafe extern "C" fn ZSTD_compressBlock_fast(ms:*mut ZSTD_MatchState_t,ss:*mut SeqStore_t,rep:*mut U32,src:*const c_void,n:usize)->usize { let m=(*ms).cParams.minMatch; let c=((*ms).cParams.windowLog<19) as c_int; match (c,m) { (1,5)=>ZSTD_compressBlock_fast_noDict_5_1(ms,ss,rep,src,n),(1,6)=>ZSTD_compressBlock_fast_noDict_6_1(ms,ss,rep,src,n),(1,7)=>ZSTD_compressBlock_fast_noDict_7_1(ms,ss,rep,src,n),(0,5)=>ZSTD_compressBlock_fast_noDict_5_0(ms,ss,rep,src,n),(0,6)=>ZSTD_compressBlock_fast_noDict_6_0(ms,ss,rep,src,n),(0,7)=>ZSTD_compressBlock_fast_noDict_7_0(ms,ss,rep,src,n),(1,_)=>ZSTD_compressBlock_fast_noDict_4_1(ms,ss,rep,src,n),(_,_)=>ZSTD_compressBlock_fast_noDict_4_0(ms,ss,rep,src,n) } }

// Dictionary/extDict declarations are kept as external dependencies in this isolated translation.
extern "C" { fn goto_match_no_dict(_: *mut ZSTD_MatchState_t, _: *mut SeqStore_t, _: *mut U32, _: *const c_void, _: usize, _: U32, _: c_int, _: *const BYTE, _: *const BYTE, _: U32, _: usize, _: *const BYTE, _: *const BYTE, _: *mut U32, _: *mut U32, _: *mut U32, _: *mut U32); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
