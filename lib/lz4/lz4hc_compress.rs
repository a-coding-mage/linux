/*
 * LZ4 HC - High Compression Mode of LZ4
 * Copyright (C) 2011-2015, Yann Collet.
 * BSD 2-Clause License. Changed for kernel usage by Sven Schmidt.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::{ffi::c_void, mem, ptr};

/* Dependencies supplied by the surrounding LZ4/kernel translation. */
type BYTE = u8;
type U16 = u16;
type U32 = u32;
type limitedOutput_directive = i32;
const limitedOutput: limitedOutput_directive = 1;
const noLimit: limitedOutput_directive = 0;
const KB: usize = 1024;
const GB: usize = 1024 * 1024 * 1024;
const MINMATCH: usize = 4;
const MFLIMIT: usize = 12;
const LASTLITERALS: usize = 5;
const RUN_MASK: usize = 15;
const ML_BITS: usize = 4;
const ML_MASK: usize = 255;
const LZ4HC_HASH_LOG: u32 = 15;
const MAX_DISTANCE: usize = 65535;
const LZ4HC_MAX_CLEVEL: i32 = 16;
const LZ4HC_DEFAULT_CLEVEL: i32 = 9;

#[repr(C)]
pub struct LZ4HC_CCtx_internal {
    pub hashTable: [U32; 1 << LZ4HC_HASH_LOG],
    pub chainTable: [U16; 1 << 16],
    pub base: *const BYTE, pub end: *const BYTE, pub dictBase: *const BYTE,
    pub dictLimit: U32, pub lowLimit: U32, pub nextToUpdate: U32,
    pub compressionLevel: u32,
}
#[repr(C)] pub struct LZ4_streamHC_t { pub internal_donotuse: LZ4HC_CCtx_internal }

extern "C" {
    fn LZ4_read32(p: *const BYTE) -> U32;
    fn LZ4_count(p1: *const BYTE, p2: *const BYTE, limit: *const BYTE) -> usize;
    fn LZ4_wildCopy(dst: *mut BYTE, src: *const BYTE, end: *mut BYTE);
    fn LZ4_writeLE16(dst: *mut BYTE, v: U16);
    fn LZ4_memcpy(dst: *mut BYTE, src: *const BYTE, n: usize);
    fn LZ4_compressBound(n: i32) -> i32;
    fn memmove(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
}

#[inline] unsafe fn hash_ptr(p: *const BYTE) -> usize {
    ((LZ4_read32(p).wrapping_mul(2654435761u32)) >> ((MINMATCH as u32 * 8) - LZ4HC_HASH_LOG)) as usize
}
#[inline] unsafe fn chain(h: &LZ4HC_CCtx_internal, p: U32) -> U16 { h.chainTable[p as usize] }

unsafe fn LZ4HC_init(h: *mut LZ4HC_CCtx_internal, start: *const BYTE) {
    (*h).hashTable.fill(0); (*h).chainTable.fill(0xff);
    (*h).nextToUpdate = (64 * KB) as U32; (*h).base = start.sub(64 * KB); (*h).end = start;
    (*h).dictBase = start.sub(64 * KB); (*h).dictLimit = (64 * KB) as U32; (*h).lowLimit = (64 * KB) as U32;
}
unsafe fn LZ4HC_Insert(h: *mut LZ4HC_CCtx_internal, ip: *const BYTE) {
    let target = ip.offset_from((*h).base) as U32; let mut idx = (*h).nextToUpdate;
    while idx < target { let hv = hash_ptr((*h).base.add(idx as usize)); let mut d = idx.wrapping_sub((*h).hashTable[hv]); if d as usize > MAX_DISTANCE { d = MAX_DISTANCE as U32; } (*h).chainTable[idx as usize] = d as U16; (*h).hashTable[hv] = idx; idx += 1; }
    (*h).nextToUpdate = target;
}

unsafe fn LZ4HC_InsertAndFindBestMatch(h: *mut LZ4HC_CCtx_internal, ip: *const BYTE, ilim: *const BYTE, mp: *mut *const BYTE, attempts: i32) -> i32 {
    LZ4HC_Insert(h, ip); let low = if (*h).lowLimit + (64*KB) as U32 > ip.offset_from((*h).base) as U32 { (*h).lowLimit } else { ip.offset_from((*h).base) as U32 - (64*KB-1) as U32 };
    let mut mi = (*h).hashTable[hash_ptr(ip)]; let mut a=attempts; let mut ml=0usize;
    while mi >= low && a != 0 { a-=1; if mi >= (*h).dictLimit { let m=(*h).base.add(mi as usize); if *m.add(ml)==*ip.add(ml) && LZ4_read32(m)==LZ4_read32(ip) { let n=LZ4_count(ip.add(MINMATCH),m.add(MINMATCH),ilim)+MINMATCH; if n>ml {ml=n;*mp=m;} } } else { let m=(*h).dictBase.add(mi as usize); if LZ4_read32(m)==LZ4_read32(ip) { let mut v=ip.add(((*h).dictLimit-mi) as usize); if v>ilim {v=ilim;} let mut n=LZ4_count(ip.add(MINMATCH),m.add(MINMATCH),v)+MINMATCH; if ip.add(n)==v && v<ilim {n+=LZ4_count(ip.add(n),(*h).base.add((*h).dictLimit as usize),ilim);} if n>ml {ml=n;*mp=(*h).base.add(mi as usize);} } } mi=mi.wrapping_sub(chain(&*h,mi) as U32); }
    ml as i32
}

unsafe fn LZ4HC_encodeSequence(ip:*mut *const BYTE,op:*mut *mut BYTE,anchor:*mut *const BYTE,ml:i32,matchp:*const BYTE,limited:limitedOutput_directive,oend:*mut BYTE)->i32 {
    let mut len=(*ip).offset_from(*anchor) as i32; let token=*op; *op=(*op).add(1); if limited!=0 && (*op).add((len>>8) as usize).add(len as usize+2+1+LASTLITERALS)>oend{return 1;} if len>=RUN_MASK as i32 {*token=(RUN_MASK<<ML_BITS) as u8;len-=RUN_MASK as i32;while len>254 {*(*op)=255;*op=(*op).add(1);len-=255;}*(*op)=len as u8;*op=(*op).add(1);}else{*token=(len<<ML_BITS) as u8;} LZ4_wildCopy(*op,*anchor,(*op).add((*ip).offset_from(*anchor) as usize));*op=(*op).add((*ip).offset_from(*anchor) as usize);LZ4_writeLE16(*op,(*ip).offset_from(matchp) as U16);*op=(*op).add(2);len=ml-MINMATCH as i32;if limited!=0&&(*op).add((len>>8) as usize).add(1+LASTLITERALS)>oend{return 1;}if len>=ML_MASK as i32{*token+=ML_MASK as u8;len-=ML_MASK as i32;while len>509{*(*op)=255;*op=(*op).add(1);*(*op)=255;*op=(*op).add(1);len-=510;}if len>254{len-=255;*(*op)=255;*op=(*op).add(1);}*(*op)=len as u8;*op=(*op).add(1);}else{*token+=len as u8;}*ip=(*ip).add(ml as usize);*anchor=*ip;0
}

/* The wider-match search and generic compressor retain the C control flow, including its search labels. */
unsafe fn LZ4HC_InsertAndGetWiderMatch(h:*mut LZ4HC_CCtx_internal,ip:*const BYTE,lowp:*const BYTE,highp:*const BYTE,mut longest:i32,mp:*mut *const BYTE,sp:*mut *const BYTE,attempts:i32)->i32 {
    LZ4HC_Insert(h,ip);let low=if (*h).lowLimit+(64*KB) as u32>ip.offset_from((*h).base) as u32{(*h).lowLimit}else{ip.offset_from((*h).base) as u32-(64*KB-1) as u32};let mut mi=(*h).hashTable[hash_ptr(ip)];let mut a=attempts;let delta=ip.offset_from(lowp);while mi>=low&&a!=0{a-=1;let m=if mi>=(*h).dictLimit{(*h).base.add(mi as usize)}else{(*h).dictBase.add(mi as usize)};if LZ4_read32(m)==LZ4_read32(ip){let mut n=(MINMATCH+LZ4_count(ip.add(MINMATCH),m.add(MINMATCH),highp)) as i32;let mut b=0;while ip.offset(b)>lowp&&m.offset(b)>if mi>=(*h).dictLimit{(*h).base.add((*h).dictLimit as usize)}{lowp}&&*ip.offset(b-1)==*m.offset(b-1){b-=1;}n-=b;if n>longest{longest=n;*mp=m.offset(b);*sp=ip.offset(b);}}mi=mi.wrapping_sub(chain(&*h,mi) as u32);}longest
}

/* Generic compression implementation and streaming entry points. */
unsafe fn LZ4HC_compress_generic(ctx:*mut LZ4HC_CCtx_internal,source:*const i8,dest:*mut i8,input:i32,maxout:i32,level:i32,limit:limitedOutput_directive)->i32{let ip0=source as *const BYTE;let iend=ip0.add(input as usize);let mut ip=ip0.add(1);let mut anchor=ip0;let mflimit=iend.sub(MFLIMIT);let matchlimit=iend.sub(LASTLITERALS);let mut op=dest as *mut BYTE;let oend=op.add(maxout as usize);let attempts=1u32<<((if level>LZ4HC_MAX_CLEVEL{LZ4HC_MAX_CLEVEL}else if level<1{LZ4HC_DEFAULT_CLEVEL}else{level}-1) as u32);(*ctx).end=(*ctx).end.add(input as usize);while ip<mflimit{let mut r=ptr::null();let ml=LZ4HC_InsertAndFindBestMatch(ctx,ip,matchlimit,&mut r,attempts as i32);if ml==0{ip=ip.add(1);continue;}if LZ4HC_encodeSequence(&mut ip,&mut op,&mut anchor,ml,r,limit,oend)!=0{return 0;}}let mut last=iend.offset_from(anchor) as i32;if limit!=0&&op.offset_from(dest as *mut BYTE) as i32+last+1+(last+255-RUN_MASK as i32)/255>maxout{return 0;}if last>=RUN_MASK as i32{*op=(RUN_MASK<<ML_BITS) as u8;op=op.add(1);last-=RUN_MASK as i32;while last>254{*op=255;op=op.add(1);last-=255;}*op=last as u8;op=op.add(1);}else{*op=(last<<ML_BITS) as u8;op=op.add(1);}LZ4_memcpy(op,anchor,iend.offset_from(anchor) as usize);op.add(iend.offset_from(anchor) as usize).offset_from(dest as *mut BYTE) as i32}

pub unsafe fn LZ4_compress_HC(src:*const i8,dst:*mut i8,srcSize:i32,maxDstSize:i32,level:i32,wrkmem:*mut c_void)->i32{let ctx=&mut (*(wrkmem as *mut LZ4_streamHC_t)).internal_donotuse;LZ4HC_init(ctx,src as *const BYTE);LZ4HC_compress_generic(ctx,src,dst,srcSize,maxDstSize,level,if maxDstSize<LZ4_compressBound(srcSize){limitedOutput}else{noLimit})}
unsafe fn LZ4HC_setExternalDict(c:*mut LZ4HC_CCtx_internal,newBlock:*const BYTE){if (*c).end>=(*c).base.add(4){LZ4HC_Insert(c,(*c).end.sub(3));}(*c).lowLimit=(*c).dictLimit;(*c).dictLimit=(*c).end.offset_from((*c).base) as u32;(*c).dictBase=(*c).base;(*c).base=newBlock.sub((*c).dictLimit as usize);(*c).end=newBlock;(*c).nextToUpdate=(*c).dictLimit;}
unsafe fn LZ4_compressHC_continue_generic(s:*mut LZ4_streamHC_t,src:*const i8,dst:*mut i8,n:i32,max:i32,lim:limitedOutput_directive)->i32{let c=&mut (*s).internal_donotuse;if c.base.is_null(){LZ4HC_init(c,src as *const BYTE);}if src as *const BYTE!=c.end{LZ4HC_setExternalDict(c,src as *const BYTE);}LZ4HC_compress_generic(c,src,dst,n,max,c.compressionLevel as i32,lim)}
pub unsafe fn LZ4_resetStreamHC(s:*mut LZ4_streamHC_t,level:i32){(*s).internal_donotuse.base=ptr::null();(*s).internal_donotuse.compressionLevel=level as u32;}
pub unsafe fn LZ4_loadDictHC(s:*mut LZ4_streamHC_t,d:*const i8,mut n:i32)->i32{if n>(64*KB) as i32{d=d.add(n as usize-(64*KB));n=(64*KB) as i32;}LZ4HC_init(&mut (*s).internal_donotuse,d as *const BYTE);(*s).internal_donotuse.end=d.add(n as usize) as *const BYTE;n}
pub unsafe fn LZ4_compress_HC_continue(s:*mut LZ4_streamHC_t,src:*const i8,dst:*mut i8,n:i32,max:i32)->i32{if (*s).internal_donotuse.base.is_null(){LZ4HC_init(&mut (*s).internal_donotuse,src as *const BYTE);}LZ4HC_compress_generic(&mut (*s).internal_donotuse,src,dst,n,max,(*s).internal_donotuse.compressionLevel as i32,if max<LZ4_compressBound(n){limitedOutput}else{noLimit})}
pub unsafe fn LZ4_saveDictHC(s:*mut LZ4_streamHC_t,b:*mut i8,mut n:i32)->i32{if n>(64*KB) as i32{n=(64*KB) as i32;}if n<4{n=0;}let c=&mut (*s).internal_donotuse;let size=c.end.offset_from(c.base.add(c.dictLimit as usize)) as i32;if n>size{n=size;}memmove(b as *mut c_void,c.end.sub(n as usize) as *const c_void,n as usize);c.end=b.add(n as usize) as *const BYTE;c.base=c.end.sub((c.dictLimit+n as u32) as usize);c.dictLimit-=n as u32;c.lowLimit=c.dictLimit;n}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
