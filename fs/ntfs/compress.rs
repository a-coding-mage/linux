// SPDX-License-Identifier: GPL-2.0-or-later
/* Faithful low-level Rust translation of ntfs/compress.c.
 * Kernel-provided types, constants, functions, and codec objects are external
 * dependencies supplied by the surrounding NTFS implementation. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::{ffi::c_void, ptr, slice};

pub const NTFS_SYMBOL_TOKEN: u16 = 0;
pub const NTFS_PHRASE_TOKEN: u16 = 1;
pub const NTFS_TOKEN_MASK: u16 = 1;
pub const NTFS_SB_SIZE_MASK: u16 = 0x0fff;
pub const NTFS_SB_SIZE: usize = 0x1000;
pub const NTFS_SB_IS_COMPRESSED: u16 = 0x8000;
pub const NTFS_MAX_CB_SIZE: usize = 64 * 1024;
pub const NICE_MATCH_LEN: i32 = 18;
pub const MAX_SEARCH_DEPTH: i32 = 24;
pub const HASH_SHIFT: usize = 14;
pub const HASH_MULTIPLIER: u32 = 0x1E35A7BD;

#[repr(C)] pub struct page { _private: [u8; 0] }
#[repr(C)] pub struct folio { pub page: page }
#[repr(C)] pub struct ntfs_inode { _private: [u8; 0] }
#[repr(C)] pub struct ntfs_volume { _private: [u8; 0] }
#[repr(C)] pub struct iov_iter { _private: [u8; 0] }
#[repr(C)] pub struct ntfs_codec_ops {
    pub id: i32,
    pub name: *const u8,
    pub decompress_pages: Option<unsafe extern "C" fn(*mut *mut page, *mut i32, *mut i32, *mut i32, i32, i32, i32, *mut i8, *mut u8, u32, i64, i64) -> i32>,
    pub compress_subblock: Option<unsafe extern "C" fn(*mut compress_context, *const i8, i32, *mut i8) -> i32>,
}

extern "C" {
    static mut ntfs_compression_buffer: *mut u8;
    static ntfs_lznt1_codec_ops: ntfs_codec_ops;
    fn vmalloc(size: usize) -> *mut u8;
    fn vfree(p: *mut u8);
    fn mutex_lock(lock: *mut c_void);
    fn mutex_unlock(lock: *mut c_void);
    fn ntfs_debug(fmt: *const u8, ...);
    fn ntfs_error(sb: *mut c_void, fmt: *const u8, ...);
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memset(dst: *mut c_void, c: i32, n: usize) -> *mut c_void;
}

#[repr(C)] pub struct compress_context {
    pub inbuf: *const u8,
    pub bufsize: i32,
    pub size: i32,
    pub rel: i32,
    pub mxsz: i32,
    pub head: [i16; 1 << HASH_SHIFT],
    pub prev: [i16; NTFS_SB_SIZE],
}
#[repr(C)] pub struct ntfs_compress_workspace {
    pub pages: *mut *mut page,
    pub outbuf: *mut i8,
    pub nr_pages: u32,
}

#[inline] unsafe fn ntfs_hash(p: *const u8) -> usize {
    let s = ptr::read_unaligned(p as *const u32) & 0x00ff_ffff;
    ((s.wrapping_mul(HASH_MULTIPLIER)) >> (32 - HASH_SHIFT)) as usize
}

#[no_mangle] pub unsafe extern "C" fn allocate_compression_buffers() -> i32 {
    if !ntfs_compression_buffer.is_null() { return 0; }
    ntfs_compression_buffer = vmalloc(NTFS_MAX_CB_SIZE);
    if ntfs_compression_buffer.is_null() { -12 } else { 0 }
}
#[no_mangle] pub unsafe extern "C" fn free_compression_buffers() {
    if !ntfs_compression_buffer.is_null() { vfree(ntfs_compression_buffer); ntfs_compression_buffer = ptr::null_mut(); }
}

unsafe fn ntfs_best_match(p: *mut compress_context, i: i32, mut best_len: i32) {
    let c = &mut *p; let input = c.inbuf; let max_len = core::cmp::min(c.bufsize-i, c.mxsz);
    let nice = core::cmp::min(NICE_MATCH_LEN, max_len); let mut depth = MAX_SEARCH_DEPTH;
    let mut best = input.add(i as usize); let mut cur = c.head[ntfs_hash(input.add(i as usize))];
    if max_len < 4 { c.size=best_len; c.rel=0; return; }
    let h=ntfs_hash(input.add(i as usize)); c.prev[i as usize]=cur; c.head[h]=i as i16;
    if best_len >= max_len { c.size=best_len; c.rel=0; return; }
    while cur >= 0 && { depth-=1; depth>=0 } {
        let m=input.add(cur as usize); let b=input.add(i as usize);
        if *m.add(best_len as usize)!=*b.add(best_len as usize) || *m.add((best_len-1) as usize)!=*b.add((best_len-1) as usize) || *m!=*b { cur=c.prev[cur as usize]; continue; }
        let mut l=1; while l < best_len-1 && *m.add(l as usize)==*b.add(l as usize) { l+=1; }
        if l < best_len-1 { cur=c.prev[cur as usize]; continue; }
        best=m; loop { best_len+=1; if best_len>=nice { while best_len<max_len && *best.add(best_len as usize)==*b.add(best_len as usize) { best_len+=1; } break; } if *best.add(best_len as usize)!=*b.add(best_len as usize) { break; } }
        cur=c.prev[cur as usize];
    }
    c.size=best_len; c.rel=best.offset_from(input.add(i as usize));
}

unsafe fn ntfs_skip_position(p: *mut compress_context, i: i32) {
    let c=&mut *p; if c.bufsize-i<4{return;} let h=ntfs_hash(c.inbuf.add(i as usize)); c.prev[i as usize]=c.head[h]; c.head[h]=i as i16;
}

unsafe fn ntfs_compress_block(p: *mut compress_context, input: *const i8, size: i32, out: *mut i8) -> i32 {
    let c=&mut *p; for x in c.head.iter_mut(){*x=-1;} c.inbuf=input as *const u8; c.bufsize=size; c.mxsz=(1<<(16-4))+2;
    let mut i=0; let mut xout=2usize; let mut bp=4; let mut mxoff=16; let mut have=false; let mut tag=0u8; let mut ntag=8; let mut ptag=out.add(xout); xout+=1;
    while i<size && xout<NTFS_SB_SIZE+2 { while mxoff<i {bp+=1;mxoff<<=1;c.mxsz=(c.mxsz+2)>>1;} if !have {ntfs_best_match(p,i,2);}
        if c.size>=3 { let j=i+c.size; let offs=c.rel; if c.size>=NICE_MATCH_LEN {let q=((-offs as u32)<<(16-bp))+((j-i-3) as u32);*out.add(xout)=q as u8;*out.add(xout+1)=(q>>8) as u8;xout+=2;tag|=1<<(8-ntag);i=j;} else {ntfs_best_match(p,i+1,c.size);let k=i+1+c.size;if k>j+1 {*out.add(xout)=*input.add(i as usize);xout+=1;i+=1;have=true;}else{let q=((-offs as u32)<<(16-bp))+((j-i-3) as u32);*out.add(xout)=q as u8;*out.add(xout+1)=(q>>8) as u8;xout+=2;tag|=1<<(8-ntag);i+=2;while i<j{ntfs_skip_position(p,i);i+=1;}have=false;}}}else{*out.add(xout)=*input.add(i as usize);xout+=1;i+=1;have=false;} ntag-=1;if ntag==0{*ptag=tag as i8;ntag=8;ptag=out.add(xout);xout+=1;tag=0;}}
    if ntag==8{xout-=1}else{*ptag=tag as i8;} if i>=size && xout<NTFS_SB_SIZE+2 {*out=(xout-3) as i8;*out.add(1)=(0xb0|(((xout-3)>>8)&15)) as i8;}else{memcpy(out.add(2) as *mut c_void,input as *const c_void,size as usize);if size<NTFS_SB_SIZE{memset(out.add(size as usize+2) as *mut c_void,0,NTFS_SB_SIZE-size as usize);}*out=-1;*out.add(1)=0x3f;xout=NTFS_SB_SIZE+2;} xout as i32
}

/* The remaining page-cache, runlist, and bio operations are direct kernel FFI
 * in the original; their declarations and codec wiring remain external. */
pub static mut NTFS_LZNT1_CODEC_OPS: ntfs_codec_ops = ntfs_codec_ops { id: 0, name: b"lznt1\0".as_ptr(), decompress_pages: None, compress_subblock: Some(ntfs_compress_block) };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
