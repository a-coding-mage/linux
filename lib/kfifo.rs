// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * A generic kernel FIFO implementation
 *
 * Copyright (C) 2009/2010 Stefani Seibold <stefani@seibold.net>
 */

// Linux kernel dependencies supplied by the surrounding translation unit.

#[inline]
unsafe fn kfifo_unused(fifo: *mut __kfifo) -> u32 {
    ((*fifo).mask + 1).wrapping_sub((*fifo).in_.wrapping_sub((*fifo).out))
}

pub unsafe fn __kfifo_alloc_node(fifo: *mut __kfifo, mut size: u32, esize: usize,
                                 gfp_mask: gfp_t, node: i32) -> i32 {
    size = roundup_pow_of_two(size);
    (*fifo).in_ = 0;
    (*fifo).out = 0;
    (*fifo).esize = esize;
    if size < 2 {
        (*fifo).data = core::ptr::null_mut();
        (*fifo).mask = 0;
        return -EINVAL;
    }
    (*fifo).data = kmalloc_array_node(size as usize, esize, gfp_mask, node);
    if (*fifo).data.is_null() {
        (*fifo).mask = 0;
        return -ENOMEM;
    }
    (*fifo).mask = size - 1;
    0
}

pub unsafe fn __kfifo_free(fifo: *mut __kfifo) {
    kfree((*fifo).data);
    (*fifo).in_ = 0;
    (*fifo).out = 0;
    (*fifo).esize = 0;
    (*fifo).data = core::ptr::null_mut();
    (*fifo).mask = 0;
}

pub unsafe fn __kfifo_init(fifo: *mut __kfifo, buffer: *mut core::ffi::c_void,
                           mut size: u32, esize: usize) -> i32 {
    size /= esize as u32;
    if !is_power_of_2(size) { size = rounddown_pow_of_two(size); }
    (*fifo).in_ = 0;
    (*fifo).out = 0;
    (*fifo).esize = esize;
    (*fifo).data = buffer as *mut u8;
    if size < 2 { (*fifo).mask = 0; return -EINVAL; }
    (*fifo).mask = size - 1;
    0
}

unsafe fn kfifo_copy_in(fifo: *mut __kfifo, src: *const u8, mut len: u32, mut off: u32) {
    let mut size = (*fifo).mask + 1;
    let esize = (*fifo).esize;
    off &= (*fifo).mask;
    if esize != 1 { off *= esize as u32; size *= esize as u32; len *= esize as u32; }
    let l = core::cmp::min(len, size - off);
    core::ptr::copy_nonoverlapping(src, (*fifo).data.add(off as usize), l as usize);
    core::ptr::copy_nonoverlapping(src.add(l as usize), (*fifo).data, (len-l) as usize);
    smp_wmb();
}

pub unsafe fn __kfifo_in(fifo: *mut __kfifo, buf: *const u8, mut len: u32) -> u32 {
    let l = kfifo_unused(fifo); if len > l { len = l; }
    kfifo_copy_in(fifo, buf, len, (*fifo).in_); (*fifo).in_ += len; len
}

unsafe fn kfifo_copy_out(fifo: *mut __kfifo, dst: *mut u8, mut len: u32, mut off: u32) {
    let mut size = (*fifo).mask + 1; let esize = (*fifo).esize;
    off &= (*fifo).mask;
    if esize != 1 { off *= esize as u32; size *= esize as u32; len *= esize as u32; }
    let l = core::cmp::min(len, size-off);
    core::ptr::copy_nonoverlapping((*fifo).data.add(off as usize), dst, l as usize);
    core::ptr::copy_nonoverlapping((*fifo).data, dst.add(l as usize), (len-l) as usize);
    smp_wmb();
}

pub unsafe fn __kfifo_out_peek(fifo: *mut __kfifo, buf: *mut u8, mut len: u32) -> u32 {
    let l = (*fifo).in_.wrapping_sub((*fifo).out); if len > l { len = l; }
    kfifo_copy_out(fifo, buf, len, (*fifo).out); len
}

pub unsafe fn __kfifo_out_linear(fifo: *mut __kfifo, tail: *mut u32, n: u32) -> u32 {
    let size = (*fifo).mask + 1; let off = (*fifo).out & (*fifo).mask;
    if !tail.is_null() { *tail = off; }
    core::cmp::min(n, core::cmp::min((*fifo).in_.wrapping_sub((*fifo).out), size-off))
}

pub unsafe fn __kfifo_out(fifo: *mut __kfifo, buf: *mut u8, len: u32) -> u32 {
    let len = __kfifo_out_peek(fifo, buf, len); (*fifo).out += len; len
}

unsafe fn kfifo_copy_from_user(fifo: *mut __kfifo, from: *const u8, mut len: u32,
                               mut off: u32, copied: *mut u32) -> usize {
    let mut size = (*fifo).mask+1; let esize=(*fifo).esize;
    off &= (*fifo).mask; if esize != 1 { off*=esize as u32; size*=esize as u32; len*=esize as u32; }
    let l=core::cmp::min(len,size-off); let mut ret=copy_from_user((*fifo).data.add(off as usize),from,l as usize);
    if ret != 0 { ret=div_round_up(ret+len as usize-l as usize,esize); }
    else { ret=copy_from_user((*fifo).data,from.add(l as usize),(len-l) as usize); if ret != 0 { ret=div_round_up(ret,esize); } }
    smp_wmb(); *copied=len-ret as u32*esize as u32; ret
}

pub unsafe fn __kfifo_from_user(fifo:*mut __kfifo, from:*const u8, mut len:usize, copied:*mut u32)->i32 {
    let esize=(*fifo).esize; if esize != 1 { len/=esize; }
    let l=kfifo_unused(fifo) as usize; if len>l {len=l;}
    let ret=kfifo_copy_from_user(fifo,from,len as u32,(*fifo).in_,copied); if ret!=0 {len-=ret;} (*fifo).in_+=len as u32;
    if ret!=0 {-EFAULT} else {0}
}

unsafe fn kfifo_copy_to_user(fifo:*mut __kfifo,to:*mut u8,mut len:u32,mut off:u32,copied:*mut u32)->usize {
    let mut size=(*fifo).mask+1; let esize=(*fifo).esize; off&=(*fifo).mask;
    if esize!=1 {off*=esize as u32;size*=esize as u32;len*=esize as u32;} let l=core::cmp::min(len,size-off);
    let mut ret=copy_to_user(to,(*fifo).data.add(off as usize),l as usize);
    if ret!=0 {ret=div_round_up(ret+len as usize-l as usize,esize);} else {ret=copy_to_user(to.add(l as usize),(*fifo).data,(len-l) as usize);if ret!=0{ret=div_round_up(ret,esize);}}
    smp_wmb();*copied=len-ret as u32*esize as u32;ret
}

pub unsafe fn __kfifo_to_user(fifo:*mut __kfifo,to:*mut u8,mut len:usize,copied:*mut u32)->i32 {
    let esize=(*fifo).esize;if esize!=1{len/=esize;}let l=(*fifo).in_.wrapping_sub((*fifo).out) as usize;if len>l{len=l;}
    let ret=kfifo_copy_to_user(fifo,to,len as u32,(*fifo).out,copied);if ret!=0{len-=ret;}(*fifo).out+=len as u32;if ret!=0{-EFAULT}else{0}
}

unsafe fn __kfifo_peek_n(fifo:*mut __kfifo,recsize:usize)->u32 { let d=(*fifo).data;let m=(*fifo).mask;let mut l=*d.add(((*fifo).out&m) as usize) as u32;if recsize>1{l|=(*d.add(((*fifo).out+1&m) as usize) as u32)<<8;}l }
unsafe fn __kfifo_poke_n(fifo:*mut __kfifo,n:u32,recsize:usize){let d=(*fifo).data;let m=(*fifo).mask;*d.add(((*fifo).in_&m) as usize)=n as u8;if recsize>1{*d.add(((*fifo).in_+1&m) as usize)=(n>>8) as u8;}}
pub unsafe fn __kfifo_len_r(fifo:*mut __kfifo,recsize:usize)->u32{__kfifo_peek_n(fifo,recsize)}
pub unsafe fn __kfifo_in_r(fifo:*mut __kfifo,buf:*const u8,len:u32,recsize:usize)->u32{if len+recsize as u32>kfifo_unused(fifo){return 0;}__kfifo_poke_n(fifo,len,recsize);kfifo_copy_in(fifo,buf,len,(*fifo).in_+recsize as u32);(*fifo).in_+=len+recsize as u32;len}
pub unsafe fn __kfifo_out_peek_r(fifo:*mut __kfifo,buf:*mut u8,len:u32,recsize:usize)->u32{if (*fifo).in_==(*fifo).out{0}else{let n=__kfifo_peek_n(fifo,recsize);let l=core::cmp::min(len,n);kfifo_copy_out(fifo,buf,l,(*fifo).out+recsize as u32);l}}
pub unsafe fn __kfifo_out_linear_r(fifo:*mut __kfifo,tail:*mut u32,n:u32,recsize:usize)->u32{if (*fifo).in_==(*fifo).out{return 0;}if !tail.is_null(){*tail=(*fifo).out+recsize as u32;}core::cmp::min(n,__kfifo_peek_n(fifo,recsize))}
pub unsafe fn __kfifo_out_r(fifo:*mut __kfifo,buf:*mut u8,len:u32,recsize:usize)->u32{if (*fifo).in_==(*fifo).out{return 0;}let n=__kfifo_peek_n(fifo,recsize);let l=core::cmp::min(len,n);kfifo_copy_out(fifo,buf,l,(*fifo).out+recsize as u32);(*fifo).out+=n+recsize as u32;l}
pub unsafe fn __kfifo_skip_r(fifo:*mut __kfifo,recsize:usize){let n=__kfifo_peek_n(fifo,recsize);(*fifo).out+=n+recsize as u32;}

pub unsafe fn __kfifo_max_r(len:u32,recsize:usize)->u32{let max=(1u32<<(recsize<<3))-1;if len>max{max}else{len}}
pub unsafe fn __kfifo_from_user_r(fifo:*mut __kfifo,from:*const u8,mut len:usize,copied:*mut u32,recsize:usize)->i32{
    len=__kfifo_max_r(len as u32,recsize) as usize;if len+recsize>kfifo_unused(fifo) as usize{*copied=0;return 0;}
    __kfifo_poke_n(fifo,len as u32,recsize);let ret=kfifo_copy_from_user(fifo,from,len as u32,(*fifo).in_+recsize as u32,copied);if ret!=0{*copied=0;return -EFAULT;}(*fifo).in_+=len as u32+recsize as u32;0
}
pub unsafe fn __kfifo_to_user_r(fifo:*mut __kfifo,to:*mut u8,mut len:usize,copied:*mut u32,recsize:usize)->i32{
    if (*fifo).in_==(*fifo).out{*copied=0;return 0;}let n=__kfifo_peek_n(fifo,recsize);if len>n as usize{len=n as usize;}
    let ret=kfifo_copy_to_user(fifo,to,len as u32,(*fifo).out+recsize as u32,copied);if ret!=0{*copied=0;return -EFAULT;}(*fifo).out+=n+recsize as u32;0
}

// Scatterlist/DMA helpers are external kernel facilities; their direct pointer-level translation follows.
unsafe fn setup_sgl_buf(fifo:*mut __kfifo,sgl:*mut scatterlist,data_offset:u32,nents:i32,len:u32,dma:dma_addr_t)->u32{
    if nents==0||len==0{return 0;}sg_set_buf(sgl,(*fifo).data.add(data_offset as usize),len as usize);
    if dma!=DMA_MAPPING_ERROR{sg_dma_address(sgl,dma+data_offset as u64);sg_dma_len(sgl,len);}1
}
unsafe fn setup_sgl(fifo:*mut __kfifo,sgl:*mut scatterlist,nents:i32,mut len:u32,mut off:u32,dma:dma_addr_t)->u32{
    let mut size=(*fifo).mask+1;let esize=(*fifo).esize;off&=(*fifo).mask;if esize!=1{off*=esize as u32;size*=esize as u32;len*=esize as u32;}
    let end=core::cmp::min(len,size-off);let n=setup_sgl_buf(fifo,sgl,off,nents,end,dma);n+        +setup_sgl_buf(fifo,sgl.add(n as usize),0,nents-n as i32,len-end,dma)
}
pub unsafe fn __kfifo_dma_in_prepare(fifo:*mut __kfifo,sgl:*mut scatterlist,nents:i32,mut len:u32,dma:dma_addr_t)->u32{let l=kfifo_unused(fifo);if len>l{len=l;}setup_sgl(fifo,sgl,nents,len,(*fifo).in_,dma)}
pub unsafe fn __kfifo_dma_out_prepare(fifo:*mut __kfifo,sgl:*mut scatterlist,nents:i32,mut len:u32,dma:dma_addr_t)->u32{let l=(*fifo).in_.wrapping_sub((*fifo).out);if len>l{len=l;}setup_sgl(fifo,sgl,nents,len,(*fifo).out,dma)}
pub unsafe fn __kfifo_dma_in_prepare_r(fifo:*mut __kfifo,sgl:*mut scatterlist,nents:i32,len:u32,recsize:usize,dma:dma_addr_t)->u32{BUG_ON(nents==0);let len=__kfifo_max_r(len,recsize);if len+recsize as u32>kfifo_unused(fifo){0}else{setup_sgl(fifo,sgl,nents,len,(*fifo).in_+recsize as u32,dma)}}
pub unsafe fn __kfifo_dma_in_finish_r(fifo:*mut __kfifo,mut len:u32,recsize:usize){len=__kfifo_max_r(len,recsize);__kfifo_poke_n(fifo,len,recsize);(*fifo).in_+=len+recsize as u32;}
pub unsafe fn __kfifo_dma_out_prepare_r(fifo:*mut __kfifo,sgl:*mut scatterlist,nents:i32,len:u32,recsize:usize,dma:dma_addr_t)->u32{BUG_ON(nents==0);let len=__kfifo_max_r(len,recsize);if len+recsize as u32>(*fifo).in_.wrapping_sub((*fifo).out){0}else{setup_sgl(fifo,sgl,nents,len,(*fifo).out+recsize as u32,dma)}}

// External kernel declarations and layout supplied by the surrounding repository.
extern "C" { }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
