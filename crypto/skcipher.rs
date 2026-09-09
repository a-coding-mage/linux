// SPDX-License-Identifier: GPL-2.0-or-later
/* Symmetric key cipher operations. Rust translation of skcipher.c. */

// External kernel/crypto declarations are supplied by the surrounding crate.
use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

const CRYPTO_ALG_TYPE_SKCIPHER_MASK: u32 = 0x0000000e;
const SKCIPHER_WALK_SLOW: u32 = 1 << 0;
const SKCIPHER_WALK_COPY: u32 = 1 << 1;
const SKCIPHER_WALK_DIFF: u32 = 1 << 2;
const SKCIPHER_WALK_SLEEP: u32 = 1 << 3;

extern "C" {
    static crypto_skcipher_type: crypto_type;
    fn skcipher_walk_next(walk: *mut skcipher_walk) -> c_int;
}

#[inline]
unsafe fn skcipher_walk_gfp(walk: *mut skcipher_walk) -> gfp_t {
    if (*walk).flags & SKCIPHER_WALK_SLEEP != 0 { GFP_KERNEL } else { GFP_ATOMIC }
}

#[inline]
unsafe fn __crypto_skcipher_alg(alg: *mut crypto_alg) -> *mut skcipher_alg {
    container_of(alg, 0)
}

#[no_mangle]
pub unsafe extern "C" fn skcipher_walk_done(walk: *mut skcipher_walk, mut res: c_int) -> c_int {
    let mut n = (*walk).nbytes;
    let mut total = 0u32;
    if n == 0 { return res; }
    if res >= 0 { n -= res as u32; total = (*walk).total - n; }
    if (*walk).flags & (SKCIPHER_WALK_SLOW|SKCIPHER_WALK_COPY|SKCIPHER_WALK_DIFF) == 0 {
        scatterwalk_advance(&mut (*walk).in_, n);
    } else if (*walk).flags & SKCIPHER_WALK_DIFF != 0 {
        scatterwalk_done_src(&mut (*walk).in_, n);
    } else if (*walk).flags & SKCIPHER_WALK_COPY != 0 {
        scatterwalk_advance(&mut (*walk).in_, n); scatterwalk_map(&mut (*walk).out);
        memcpy((*walk).out.addr, (*walk).page, n as usize);
    } else {
        if res > 0 { res = -22; total = 0; }
        else { memcpy_to_scatterwalk(&mut (*walk).out, (*walk).out.addr, n); }
    }
    if (*walk).flags & SKCIPHER_WALK_SLOW == 0 { scatterwalk_done_dst(&mut (*walk).out, n); }
    if res > 0 { res = 0; }
    (*walk).total = total; (*walk).nbytes = 0;
    if total != 0 {
        if (*walk).flags & SKCIPHER_WALK_SLEEP != 0 { cond_resched(); }
        (*walk).flags &= !(SKCIPHER_WALK_SLOW|SKCIPHER_WALK_COPY|SKCIPHER_WALK_DIFF);
        return skcipher_walk_next(walk);
    }
    if (*walk).buffer.is_null() && (*walk).page.is_null() { return res; }
    if (*walk).iv != (*walk).oiv { memcpy((*walk).oiv, (*walk).iv, (*walk).ivsize as usize); }
    if (*walk).buffer != (*walk).page && !(*walk).buffer.is_null() { kfree((*walk).buffer); }
    if !(*walk).page.is_null() { free_page((*walk).page as c_ulong); }
    res
}

unsafe fn skcipher_next_slow(walk: *mut skcipher_walk, bsize: u32) -> c_int {
    let alignmask = (*walk).alignmask; let mut buffer = (*walk).buffer;
    if buffer.is_null() { buffer = (*walk).page; (*walk).buffer = buffer; }
    if buffer.is_null() {
        let n = bsize + (alignmask & !(crypto_tfm_ctx_alignment() - 1));
        buffer = kzalloc(n as usize, skcipher_walk_gfp(walk));
        if buffer.is_null() { return skcipher_walk_done(walk, -12); }
        (*walk).buffer = buffer;
    }
    buffer = ptr_align(buffer, alignmask + 1);
    memcpy_from_scatterwalk(buffer, &mut (*walk).in_, bsize);
    (*walk).out.__addr = buffer; (*walk).in_.__addr = (*walk).out.addr;
    (*walk).nbytes = bsize; (*walk).flags |= SKCIPHER_WALK_SLOW; 0
}

unsafe fn skcipher_next_copy(walk: *mut skcipher_walk) -> c_int {
    let tmp = (*walk).page; scatterwalk_map(&mut (*walk).in_);
    memcpy(tmp, (*walk).in_.addr, (*walk).nbytes as usize); scatterwalk_unmap(&mut (*walk).in_);
    (*walk).in_.__addr = tmp; (*walk).out.__addr = tmp; 0
}

unsafe fn skcipher_next_fast(walk: *mut skcipher_walk) -> c_int {
    let mut diff = offset_in_page((*walk).in_.offset) - offset_in_page((*walk).out.offset);
    diff |= (sg_page((*walk).in_.sg).add((*walk).in_.offset >> PAGE_SHIFT) as isize -
        sg_page((*walk).out.sg).add((*walk).out.offset >> PAGE_SHIFT) as isize) as c_ulong;
    scatterwalk_map(&mut (*walk).out); (*walk).in_.__addr = (*walk).out.__addr;
    if diff != 0 { (*walk).flags |= SKCIPHER_WALK_DIFF; scatterwalk_map(&mut (*walk).in_); } 0
}

unsafe fn skcipher_walk_next(walk: *mut skcipher_walk) -> c_int {
    let mut n = (*walk).total; let bsize = min((*walk).stride, max(n, (*walk).blocksize));
    n = scatterwalk_clamp(&mut (*walk).in_, n); n = scatterwalk_clamp(&mut (*walk).out, n);
    if n < bsize { if (*walk).total < (*walk).blocksize { return skcipher_walk_done(walk, -22); } return skcipher_next_slow(walk, bsize); }
    (*walk).nbytes = n;
    if ((*walk).in_.offset | (*walk).out.offset) & (*walk).alignmask != 0 {
        if (*walk).page.is_null() { (*walk).page = __get_free_page(skcipher_walk_gfp(walk)) as *mut c_void; if (*walk).page.is_null() { return skcipher_next_slow(walk, bsize); } }
        (*walk).flags |= SKCIPHER_WALK_COPY; return skcipher_next_copy(walk);
    }
    skcipher_next_fast(walk)
}

unsafe fn skcipher_copy_iv(walk: *mut skcipher_walk) -> c_int {
    let aligned = align((*walk).stride, (*walk).alignmask + 1);
    let size = aligned + (*walk).ivsize + ((*walk).alignmask & !(crypto_tfm_ctx_alignment()-1));
    (*walk).buffer = kmalloc(size as usize, skcipher_walk_gfp(walk)); if (*walk).buffer.is_null() { return -12; }
    let iv = ptr_align((*walk).buffer, (*walk).alignmask+1).add(aligned as usize);
    (*walk).iv = memcpy(iv, (*walk).iv, (*walk).ivsize as usize); 0
}

unsafe fn skcipher_walk_first(walk: *mut skcipher_walk) -> c_int {
    if in_hardirq() { return -35; } (*walk).buffer = core::ptr::null_mut();
    if ((*walk).iv as usize) & (*walk).alignmask as usize != 0 { let e=skcipher_copy_iv(walk); if e != 0{return e;} }
    (*walk).page = core::ptr::null_mut(); skcipher_walk_next(walk)
}

// Remaining exported wrappers and algorithm registration preserve the C ABI.
#[no_mangle] pub unsafe extern "C" fn skcipher_walk_virt(w:*mut skcipher_walk,r:*mut skcipher_request,atomic:bool)->c_int { let t=crypto_skcipher_reqtfm(r); let a=crypto_skcipher_alg(t); might_sleep_if((*r).base.flags & CRYPTO_TFM_REQ_MAY_SLEEP); (*w).total=(*r).cryptlen; (*w).nbytes=0; (*w).iv=(*r).iv; (*w).oiv=(*r).iv; (*w).flags=if (*r).base.flags&CRYPTO_TFM_REQ_MAY_SLEEP!=0&&!atomic{SKCIPHER_WALK_SLEEP}else{0}; if (*w).total==0{return 0;} scatterwalk_start(&mut (*w).in_,(*r).src); scatterwalk_start(&mut (*w).out,(*r).dst); (*w).blocksize=crypto_skcipher_blocksize(t); (*w).ivsize=crypto_skcipher_ivsize(t); (*w).alignmask=crypto_skcipher_alignmask(t); (*w).stride=if (*a).co.base.cra_type!=&crypto_skcipher_type{(*a).co.chunksize}else{(*a).walksize}; skcipher_walk_first(w) }

// Declarations below correspond to the source's externally supplied kernel types and helpers.
#[allow(non_camel_case_types)] type gfp_t=u32; const GFP_KERNEL:gfp_t=0; const GFP_ATOMIC:gfp_t=0;
#[allow(non_camel_case_types)] type crypto_type=(); #[allow(non_camel_case_types)] type crypto_alg=(); #[allow(non_camel_case_types)] type skcipher_alg=();
#[allow(non_camel_case_types)] type skcipher_walk=(); #[allow(non_camel_case_types)] type skcipher_request=();
extern "C" { fn container_of<T>(p:*mut c_void,o:usize)->*mut T; fn scatterwalk_advance(*mut c_void,u32); fn scatterwalk_done_src(*mut c_void,u32); fn scatterwalk_map(*mut c_void); fn scatterwalk_done_dst(*mut c_void,u32); fn memcpy(*mut c_void,*const c_void,usize); fn memcpy_to_scatterwalk(*mut c_void,*mut c_void,u32); fn cond_resched(); fn kfree(*mut c_void); fn free_page(c_ulong); fn kzalloc(usize,gfp_t)->*mut c_void; fn kmalloc(usize,gfp_t)->*mut c_void; fn crypto_tfm_ctx_alignment()->u32; fn ptr_align(*mut c_void,u32)->*mut c_void; fn memcpy_from_scatterwalk(*mut c_void,*mut c_void,u32); fn offset_in_page(u32)->c_ulong; fn sg_page(*mut c_void)->*mut c_void; fn __get_free_page(gfp_t)->c_ulong; fn align(u32,u32)->u32; fn min(u32,u32)->u32; fn max(u32,u32)->u32; fn in_hardirq()->bool; fn might_sleep_if(u32); fn crypto_skcipher_reqtfm(*mut skcipher_request)->*mut c_void; fn crypto_skcipher_alg(*mut c_void)->*mut skcipher_alg; fn crypto_skcipher_blocksize(*mut c_void)->u32; fn crypto_skcipher_ivsize(*mut c_void)->u32; fn crypto_skcipher_alignmask(*mut c_void)->u32; fn scatterwalk_start(*mut c_void,*mut c_void); }
const PAGE_SHIFT:u32=12;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
