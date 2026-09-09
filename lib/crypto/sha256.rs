// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * SHA-224, SHA-256, HMAC-SHA224, and HMAC-SHA256 library functions
 *
 * Copyright (c) Jean-Luc Cooke <jlcooke@certainkey.com>
 * Copyright (c) Andrew McDonald <andrew@mcdonald.org.uk>
 * Copyright (c) 2002 James Morris <jmorris@intercode.com.au>
 * Copyright (c) 2014 Red Hat Inc.
 * Copyright 2025 Google LLC
 */

// Types, constants, and helper functions below are supplied by the corresponding kernel headers.

static const sha256_block_state sha224_iv: sha256_block_state = sha256_block_state {
    h: [SHA224_H0, SHA224_H1, SHA224_H2, SHA224_H3,
        SHA224_H4, SHA224_H5, SHA224_H6, SHA224_H7],
};

static const sha256_ctx initial_sha256_ctx: sha256_ctx = sha256_ctx {
    ctx: __sha256_ctx {
        state: sha256_block_state {
            h: [SHA256_H0, SHA256_H1, SHA256_H2, SHA256_H3,
                SHA256_H4, SHA256_H5, SHA256_H6, SHA256_H7],
        },
        bytecount: 0,
        ..unsafe { core::mem::zeroed() },
    },
    ..unsafe { core::mem::zeroed() },
};

static SHA256_K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1,
    0x923f82a4, 0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3,
    0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786,
    0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147,
    0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13,
    0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
    0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a,
    0x5b9cca4f, 0x682e6ff3, 0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208,
    0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

#[inline] unsafe fn load_op(i: usize, w: *mut u32, input: *const u8) {
    *w.add(i) = get_unaligned_be32(input.add(i * 4) as *const u32);
}
#[inline] unsafe fn blend_op(i: usize, w: *mut u32) {
    *w.add(i) = s1(*w.add(i - 2)).wrapping_add(*w.add(i - 7))
        .wrapping_add(s0(*w.add(i - 15))).wrapping_add(*w.add(i - 16));
}

#[inline] unsafe fn sha256_round(i: usize, a: &mut u32, b: u32, c: u32, d: &mut u32,
                                 e: u32, f: u32, g: u32, h: &mut u32, w: *const u32) {
    let t1 = h.wrapping_add(e1(e)).wrapping_add(ch(e, f, g))
        .wrapping_add(SHA256_K[i]).wrapping_add(*w.add(i));
    let t2 = e0(*a).wrapping_add(maj(*a, b, c));
    *d = d.wrapping_add(t1);
    *h = t1.wrapping_add(t2);
}

unsafe fn sha256_block_generic(state: *mut sha256_block_state, input: *const u8, w: *mut u32) {
    for i in (0..16).step_by(8) { for j in 0..8 { load_op(i + j, w, input); } }
    for i in (16..64).step_by(8) { for j in 0..8 { blend_op(i + j, w); } }
    let mut a = (*state).h[0]; let mut b = (*state).h[1]; let mut c = (*state).h[2]; let mut d = (*state).h[3];
    let mut e = (*state).h[4]; let mut f = (*state).h[5]; let mut g = (*state).h[6]; let mut h = (*state).h[7];
    for i in (0..64).step_by(8) {
        sha256_round(i, &mut a,b,c,&mut d,e,f,g,&mut h,w); sha256_round(i+1,&mut h,a,b,&mut c,d,e,f,&mut g,w);
        sha256_round(i+2,&mut g,h,a,&mut b,c,d,e,&mut f,w); sha256_round(i+3,&mut f,g,h,&mut a,b,c,d,&mut e,w);
        sha256_round(i+4,&mut e,f,g,&mut h,a,b,c,&mut d,w); sha256_round(i+5,&mut d,e,f,&mut g,h,a,b,&mut c,w);
        sha256_round(i+6,&mut c,d,e,&mut f,g,h,a,&mut b,w); sha256_round(i+7,&mut b,c,d,&mut e,f,g,h,&mut a,w);
    }
    for i in 0..8 { (*state).h[i] = (*state).h[i].wrapping_add([a,b,c,d,e,f,g,h][i]); }
}

unsafe fn sha256_blocks_generic(state: *mut sha256_block_state, mut data: *const u8, mut nblocks: usize) {
    let mut w = [0u32; 64];
    loop { sha256_block_generic(state, data, w.as_mut_ptr()); data = data.add(SHA256_BLOCK_SIZE); nblocks -= 1; if nblocks == 0 { break; } }
    memzero_explicit(w.as_mut_ptr() as *mut u8, core::mem::size_of_val(&w));
}

#[inline] unsafe fn __sha256_init(ctx: *mut __sha256_ctx, iv: *const sha256_block_state, initial_bytecount: u64) {
    (*ctx).state = *iv; (*ctx).bytecount = initial_bytecount;
}
pub unsafe fn sha224_init(ctx: *mut sha224_ctx) { __sha256_init(&mut (*ctx).ctx, &sha224_iv, 0); }
pub unsafe fn sha256_init(ctx: *mut sha256_ctx) { __sha256_init(&mut (*ctx).ctx, &initial_sha256_ctx.ctx.state, 0); }

pub unsafe fn __sha256_update(ctx: *mut __sha256_ctx, mut data: *const u8, mut len: usize) {
    let mut partial = (*ctx).bytecount as usize % SHA256_BLOCK_SIZE; (*ctx).bytecount = (*ctx).bytecount.wrapping_add(len as u64);
    if partial + len >= SHA256_BLOCK_SIZE {
        if partial != 0 { let l = SHA256_BLOCK_SIZE - partial; core::ptr::copy_nonoverlapping(data, (*ctx).buf.as_mut_ptr().add(partial), l); data=data.add(l); len-=l; sha256_blocks_generic(&mut (*ctx).state,(*ctx).buf.as_ptr(),1); }
        let nblocks=len/SHA256_BLOCK_SIZE; len%=SHA256_BLOCK_SIZE; if nblocks != 0 { sha256_blocks_generic(&mut (*ctx).state,data,nblocks); data=data.add(nblocks*SHA256_BLOCK_SIZE); } partial=0;
    } if len != 0 { core::ptr::copy_nonoverlapping(data,(*ctx).buf.as_mut_ptr().add(partial),len); }
}

unsafe fn __sha256_final(ctx: *mut __sha256_ctx, out: *mut u8, digest_size: usize) {
    let bitcount=(*ctx).bytecount<<3; let mut partial=(*ctx).bytecount as usize%SHA256_BLOCK_SIZE; (*ctx).buf[partial]=0x80; partial+=1;
    if partial>SHA256_BLOCK_SIZE-8 { core::ptr::write_bytes((*ctx).buf.as_mut_ptr().add(partial),0,SHA256_BLOCK_SIZE-partial); sha256_blocks_generic(&mut (*ctx).state,(*ctx).buf.as_ptr(),1); partial=0; }
    core::ptr::write_bytes((*ctx).buf.as_mut_ptr().add(partial),0,SHA256_BLOCK_SIZE-8-partial); *(*ctx).buf.as_mut_ptr().add(SHA256_BLOCK_SIZE-8) = (bitcount>>56) as u8;
    for i in 0..8 { *(*ctx).buf.as_mut_ptr().add(SHA256_BLOCK_SIZE-8+i)=(bitcount>>(56-8*i)) as u8; } sha256_blocks_generic(&mut (*ctx).state,(*ctx).buf.as_ptr(),1);
    for i in (0..digest_size).step_by(4) { put_unaligned_be32((*ctx).state.h[i/4],out.add(i)); }
}

pub unsafe fn sha224_final(ctx:*mut sha224_ctx,out:*mut u8){__sha256_final(&mut (*ctx).ctx,out,SHA224_DIGEST_SIZE);memzero_explicit(ctx as *mut u8,core::mem::size_of::<sha224_ctx>());}
pub unsafe fn sha256_final(ctx:*mut sha256_ctx,out:*mut u8){__sha256_final(&mut (*ctx).ctx,out,SHA256_DIGEST_SIZE);memzero_explicit(ctx as *mut u8,core::mem::size_of::<sha256_ctx>());}
pub unsafe fn sha224(data:*const u8,len:usize,out:*mut u8){let mut c:sha224_ctx=core::mem::zeroed();sha224_init(&mut c);sha224_update(&mut c,data,len);sha224_final(&mut c,out);}
pub unsafe fn sha256(data:*const u8,len:usize,out:*mut u8){let mut c:sha256_ctx=core::mem::zeroed();sha256_init(&mut c);sha256_update(&mut c,data,len);sha256_final(&mut c,out);}

unsafe fn __hmac_sha256_final(ctx:*mut __hmac_sha256_ctx,out:*mut u8,digest_size:usize){
    __sha256_final(&mut (*ctx).sha_ctx,(*ctx).sha_ctx.buf.as_mut_ptr(),digest_size);
    core::ptr::write_bytes((*ctx).sha_ctx.buf.as_mut_ptr().add(digest_size),0,SHA256_BLOCK_SIZE-digest_size);
    (*ctx).sha_ctx.buf[digest_size]=0x80;
    let n=8*(SHA256_BLOCK_SIZE+digest_size); for i in 0..4 { (*ctx).sha_ctx.buf[SHA256_BLOCK_SIZE-4+i]=(n>>(24-8*i)) as u8; }
    sha256_blocks_generic(&mut (*ctx).ostate,(*ctx).sha_ctx.buf.as_ptr(),1);
    for i in (0..digest_size).step_by(4){put_unaligned_be32((*ctx).ostate.h[i/4],out.add(i));}
    memzero_explicit(ctx as *mut u8,core::mem::size_of::<__hmac_sha256_ctx>());
}
pub unsafe fn hmac_sha224_final(ctx:*mut hmac_sha224_ctx,out:*mut u8){__hmac_sha256_final(&mut (*ctx).ctx,out,SHA224_DIGEST_SIZE);}
pub unsafe fn hmac_sha256_final(ctx:*mut hmac_sha256_ctx,out:*mut u8){__hmac_sha256_final(&mut (*ctx).ctx,out,SHA256_DIGEST_SIZE);}

// Header-provided HMAC initialization and update routines are external dependencies.
extern "C" { fn hmac_sha224_init(ctx:*mut hmac_sha224_ctx,key:*const hmac_sha224_key); fn hmac_sha224_update(ctx:*mut hmac_sha224_ctx,data:*const u8,len:usize); fn hmac_sha256_init(ctx:*mut hmac_sha256_ctx,key:*const hmac_sha256_key); fn hmac_sha256_update(ctx:*mut hmac_sha256_ctx,data:*const u8,len:usize); }
pub unsafe fn hmac_sha224(key:*const hmac_sha224_key,data:*const u8,len:usize,out:*mut u8){let mut c:hmac_sha224_ctx=core::mem::zeroed();hmac_sha224_init(&mut c,key);hmac_sha224_update(&mut c,data,len);hmac_sha224_final(&mut c,out);}
pub unsafe fn hmac_sha256(key:*const hmac_sha256_key,data:*const u8,len:usize,out:*mut u8){let mut c:hmac_sha256_ctx=core::mem::zeroed();hmac_sha256_init(&mut c,key);hmac_sha256_update(&mut c,data,len);hmac_sha256_final(&mut c,out);}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
