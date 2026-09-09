// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * SHA-384, SHA-512, HMAC-SHA384, and HMAC-SHA512 library functions
 *
 * Copyright (c) Jean-Luc Cooke <jlcooke@certainkey.com>
 * Copyright (c) Andrew McDonald <andrew@mcdonald.org.uk>
 * Copyright (c) 2003 Kyle McMartin <kyle@debian.org>
 * Copyright 2025 Google LLC
 */

// Kernel headers and architecture-specific declarations are supplied by the
// surrounding translation unit.

static const sha512_block_state sha384_iv: sha512_block_state = sha512_block_state { h: [
    SHA384_H0, SHA384_H1, SHA384_H2, SHA384_H3,
    SHA384_H4, SHA384_H5, SHA384_H6, SHA384_H7,
] };

static const sha512_block_state sha512_iv: sha512_block_state = sha512_block_state { h: [
    SHA512_H0, SHA512_H1, SHA512_H2, SHA512_H3,
    SHA512_H4, SHA512_H5, SHA512_H6, SHA512_H7,
] };

static const sha512_K: [u64; 80] = [
    0x428a2f98d728ae22,0x7137449123ef65cd,0xb5c0fbcfec4d3b2f,0xe9b5dba58189dbbc,
    0x3956c25bf348b538,0x59f111f1b605d019,0x923f82a4af194f9b,0xab1c5ed5da6d8118,
    0xd807aa98a3030242,0x12835b0145706fbe,0x243185be4ee4b28c,0x550c7dc3d5ffb4e2,
    0x72be5d74f27b896f,0x80deb1fe3b1696b1,0x9bdc06a725c71235,0xc19bf174cf692694,
    0xe49b69c19ef14ad2,0xefbe4786384f25e3,0x0fc19dc68b8cd5b5,0x240ca1cc77ac9c65,
    0x2de92c6f592b0275,0x4a7484aa6ea6e483,0x5cb0a9dcbd41fbd4,0x76f988da831153b5,
    0x983e5152ee66dfab,0xa831c66d2db43210,0xb00327c898fb213f,0xbf597fc7beef0ee4,
    0xc6e00bf33da88fc2,0xd5a79147930aa725,0x06ca6351e003826f,0x142929670a0e6e70,
    0x27b70a8546d22ffc,0x2e1b21385c26c926,0x4d2c6dfc5ac42aed,0x53380d139d95b3df,
    0x650a73548baf63de,0x766a0abb3c77b2a8,0x81c2c92e47edaee6,0x92722c851482353b,
    0xa2bfe8a14cf10364,0xa81a664bbc423001,0xc24b8b70d0f89791,0xc76c51a30654be30,
    0xd192e819d6ef5218,0xd69906245565a910,0xf40e35855771202a,0x106aa07032bbd1b8,
    0x19a4c116b8d2d0c8,0x1e376c085141ab53,0x2748774cdf8eeb99,0x34b0bcb5e19b48a8,
    0x391c0cb3c5c95a63,0x4ed8aa4ae3418acb,0x5b9cca4f7763e373,0x682e6ff3d6b2b8a3,
    0x748f82ee5defb2fc,0x78a5636f43172f60,0x84c87814a1f0ab72,0x8cc702081a6439ec,
    0x90befffa23631e28,0xa4506cebde82bde9,0xbef9a3f7b2c67915,0xc67178f2e372532b,
    0xca273eceea26619c,0xd186b8c721c0c207,0xeada7dd6cde0eb1e,0xf57d4f7fee6ed178,
    0x06f067aa72176fba,0x0a637dc5a2c898a6,0x113f9804bef90dae,0x1b710b35131c471b,
    0x28db77f523047d84,0x32caab7b40c72493,0x3c9ebe0a15c9bebc,0x431d67c49c100d4c,
    0x4cc5d4becb3e42b6,0x597f299cfc657e2a,0x5fcb6fab3ad6faec,0x6c44198c4a475817,
];

#[inline] fn ch(x:u64,y:u64,z:u64)->u64 { z ^ (x & (y ^ z)) }
#[inline] fn maj(x:u64,y:u64,z:u64)->u64 { (x & y) | (z & (x | y)) }
#[inline] fn e0(x:u64)->u64 { x.rotate_right(28)^x.rotate_right(34)^x.rotate_right(39) }
#[inline] fn e1(x:u64)->u64 { x.rotate_right(14)^x.rotate_right(18)^x.rotate_right(41) }
#[inline] fn s0(x:u64)->u64 { x.rotate_right(1)^x.rotate_right(8)^(x>>7) }
#[inline] fn s1(x:u64)->u64 { x.rotate_right(19)^x.rotate_right(61)^(x>>6) }

unsafe fn sha512_block_generic(state: *mut sha512_block_state, data: *const u8) {
    let mut a=(*state).h[0]; let mut b=(*state).h[1]; let mut c=(*state).h[2]; let mut d=(*state).h[3];
    let mut e=(*state).h[4]; let mut f=(*state).h[5]; let mut g=(*state).h[6]; let mut h=(*state).h[7];
    let mut w=[0u64;16];
    for j in 0..16 { w[j]=get_unaligned_be64(data.add(j*8)); }
    for i in (0..80).step_by(8) {
        if i != 0 { for j in 0..16 { w[j&15]=w[j&15].wrapping_add(s1(w[(j+14)&15])).wrapping_add(w[(j+9)&15]).wrapping_add(s0(w[(j+1)&15])); } }
        macro_rules! r { ($x:ident,$y:ident,$z:ident,$q:expr,$k:expr,$p:expr,$o:expr) => {{ let t1=$x.wrapping_add(e1($y)).wrapping_add(ch($y,$z,$p)).wrapping_add(sha512_K[i+$k]).wrapping_add(w[(i&15)+$k]); let t2=e0($q).wrapping_add(maj($q,$x,$z)); $o=$o.wrapping_add(t1); ($x,t1.wrapping_add(t2)) }} }
        let t1=h.wrapping_add(e1(e)).wrapping_add(ch(e,f,g)).wrapping_add(sha512_K[i]).wrapping_add(w[i&15]); let t2=e0(a).wrapping_add(maj(a,b,c)); d=d.wrapping_add(t1); h=t1.wrapping_add(t2);
        let t1=g.wrapping_add(e1(d)).wrapping_add(ch(d,e,f)).wrapping_add(sha512_K[i+1]).wrapping_add(w[(i&15)+1]); let t2=e0(h).wrapping_add(maj(h,a,b)); c=c.wrapping_add(t1); g=t1.wrapping_add(t2);
        let t1=f.wrapping_add(e1(c)).wrapping_add(ch(c,d,e)).wrapping_add(sha512_K[i+2]).wrapping_add(w[(i&15)+2]); let t2=e0(g).wrapping_add(maj(g,h,a)); b=b.wrapping_add(t1); f=t1.wrapping_add(t2);
        let t1=e.wrapping_add(e1(b)).wrapping_add(ch(b,c,d)).wrapping_add(sha512_K[i+3]).wrapping_add(w[(i&15)+3]); let t2=e0(f).wrapping_add(maj(f,g,h)); a=a.wrapping_add(t1); e=t1.wrapping_add(t2);
        let t1=d.wrapping_add(e1(a)).wrapping_add(ch(a,b,c)).wrapping_add(sha512_K[i+4]).wrapping_add(w[(i&15)+4]); let t2=e0(e).wrapping_add(maj(e,f,g)); h=h.wrapping_add(t1); d=t1.wrapping_add(t2);
        let t1=c.wrapping_add(e1(h)).wrapping_add(ch(h,a,b)).wrapping_add(sha512_K[i+5]).wrapping_add(w[(i&15)+5]); let t2=e0(d).wrapping_add(maj(d,e,f)); g=g.wrapping_add(t1); c=t1.wrapping_add(t2);
        let t1=b.wrapping_add(e1(g)).wrapping_add(ch(g,h,a)).wrapping_add(sha512_K[i+6]).wrapping_add(w[(i&15)+6]); let t2=e0(c).wrapping_add(maj(c,d,e)); f=f.wrapping_add(t1); b=t1.wrapping_add(t2);
        let t1=a.wrapping_add(e1(f)).wrapping_add(ch(f,g,h)).wrapping_add(sha512_K[i+7]).wrapping_add(w[(i&15)+7]); let t2=e0(b).wrapping_add(maj(b,c,d)); e=e.wrapping_add(t1); a=t1.wrapping_add(t2);
    }
    (*state).h[0]=(*state).h[0].wrapping_add(a); (*state).h[1]=(*state).h[1].wrapping_add(b); (*state).h[2]=(*state).h[2].wrapping_add(c); (*state).h[3]=(*state).h[3].wrapping_add(d);
    (*state).h[4]=(*state).h[4].wrapping_add(e); (*state).h[5]=(*state).h[5].wrapping_add(f); (*state).h[6]=(*state).h[6].wrapping_add(g); (*state).h[7]=(*state).h[7].wrapping_add(h);
}

unsafe fn sha512_blocks_generic(s:*mut sha512_block_state,d:*const u8,n:usize){ for i in 0..n { sha512_block_generic(s,d.add(i*SHA512_BLOCK_SIZE)); } }
unsafe fn __sha512_init(c:*mut __sha512_ctx,iv:*const sha512_block_state,n:u64){ (*c).state=*iv;(*c).bytecount_lo=n;(*c).bytecount_hi=0; }
pub unsafe fn sha384_init(c:*mut sha384_ctx){__sha512_init(&mut (*c).ctx,&sha384_iv,0)}
pub unsafe fn sha512_init(c:*mut sha512_ctx){__sha512_init(&mut (*c).ctx,&sha512_iv,0)}
pub unsafe fn __sha512_update(c:*mut __sha512_ctx,data:*const u8,mut len:usize){let mut p=(*c).bytecount_lo as usize%SHA512_BLOCK_SIZE;(*c).bytecount_lo=(*c).bytecount_lo.wrapping_add(len as u64);if (*c).bytecount_lo < len as u64 {(*c).bytecount_hi=(*c).bytecount_hi.wrapping_add(1)} if p+len>=SHA512_BLOCK_SIZE {if p!=0 {let l=SHA512_BLOCK_SIZE-p;core::ptr::copy_nonoverlapping(data,(*c).buf.as_mut_ptr().add(p),l);data=data.add(l);len-=l;sha512_blocks_generic(&mut (*c).state,(*c).buf.as_ptr(),1)}let n=len/SHA512_BLOCK_SIZE;len%=SHA512_BLOCK_SIZE;if n!=0 {sha512_blocks_generic(&mut (*c).state,data,n);data=data.add(n*SHA512_BLOCK_SIZE)}p=0}if len!=0 {core::ptr::copy_nonoverlapping(data,(*c).buf.as_mut_ptr().add(p),len)}}
unsafe fn __sha512_final(c:*mut __sha512_ctx,out:*mut u8,ds:usize){let hi=((*c).bytecount_hi<<3)|((*c).bytecount_lo>>61);let lo=(*c).bytecount_lo<<3;let mut p=(*c).bytecount_lo as usize%SHA512_BLOCK_SIZE;(*c).buf[p]=0x80;p+=1;if p>SHA512_BLOCK_SIZE-16 {(*c).buf[p..].fill(0);sha512_blocks_generic(&mut (*c).state,(*c).buf.as_ptr(),1);p=0}(*c).buf[p..SHA512_BLOCK_SIZE-16].fill(0);put_unaligned_be64(hi,(*c).buf.as_mut_ptr().add(SHA512_BLOCK_SIZE-16));put_unaligned_be64(lo,(*c).buf.as_mut_ptr().add(SHA512_BLOCK_SIZE-8));sha512_blocks_generic(&mut (*c).state,(*c).buf.as_ptr(),1);for i in (0..ds).step_by(8){put_unaligned_be64((*c).state.h[i/8],out.add(i))}}
pub unsafe fn sha384_final(c:*mut sha384_ctx,o:*mut u8){__sha512_final(&mut (*c).ctx,o,SHA384_DIGEST_SIZE);memzero_explicit(c,core::mem::size_of::<sha384_ctx>())}
pub unsafe fn sha512_final(c:*mut sha512_ctx,o:*mut u8){__sha512_final(&mut (*c).ctx,o,SHA512_DIGEST_SIZE);memzero_explicit(c,core::mem::size_of::<sha512_ctx>())}
pub unsafe fn sha384(d:*const u8,l:usize,o:*mut u8){let mut c=core::mem::zeroed();sha384_init(&mut c);sha384_update(&mut c,d,l);sha384_final(&mut c,o)}
pub unsafe fn sha512(d:*const u8,l:usize,o:*mut u8){let mut c=core::mem::zeroed();sha512_init(&mut c);sha512_update(&mut c,d,l);sha512_final(&mut c,o)}

unsafe fn __hmac_sha512_init(c:*mut __hmac_sha512_ctx,k:*const __hmac_sha512_key){__sha512_init(&mut (*c).sha_ctx,&(*k).istate,SHA512_BLOCK_SIZE as u64);(*c).ostate=(*k).ostate}
unsafe fn __hmac_sha512_final(c:*mut __hmac_sha512_ctx,o:*mut u8,ds:usize){__sha512_final(&mut (*c).sha_ctx,(*c).sha_ctx.buf.as_mut_ptr(),ds);(*c).sha_ctx.buf[ds..].fill(0);(*c).sha_ctx.buf[ds]=0x80;put_unaligned_be32((8*(SHA512_BLOCK_SIZE+ds)) as u32,(*c).sha_ctx.buf.as_mut_ptr().add(SHA512_BLOCK_SIZE-4));sha512_blocks_generic(&mut (*c).ostate,(*c).sha_ctx.buf.as_ptr(),1);for i in (0..ds).step_by(8){put_unaligned_be64((*c).ostate.h[i/8],o.add(i))}memzero_explicit(c,core::mem::size_of::<__hmac_sha512_ctx>())}
pub unsafe fn hmac_sha384_init(c:*mut hmac_sha384_ctx,k:*const hmac_sha384_key){__hmac_sha512_init(&mut (*c).ctx,&(*k).key)}
pub unsafe fn hmac_sha512_init(c:*mut hmac_sha512_ctx,k:*const hmac_sha512_key){__hmac_sha512_init(&mut (*c).ctx,&(*k).key)}
pub unsafe fn hmac_sha384_final(c:*mut hmac_sha384_ctx,o:*mut u8){__hmac_sha512_final(&mut (*c).ctx,o,SHA384_DIGEST_SIZE)}
pub unsafe fn hmac_sha512_final(c:*mut hmac_sha512_ctx,o:*mut u8){__hmac_sha512_final(&mut (*c).ctx,o,SHA512_DIGEST_SIZE)}
pub unsafe fn hmac_sha384(k:*const hmac_sha384_key,d:*const u8,l:usize,o:*mut u8){let mut c=core::mem::zeroed();hmac_sha384_init(&mut c,k);hmac_sha384_update(&mut c,d,l);hmac_sha384_final(&mut c,o)}
pub unsafe fn hmac_sha512(k:*const hmac_sha512_key,d:*const u8,l:usize,o:*mut u8){let mut c=core::mem::zeroed();hmac_sha512_init(&mut c,k);hmac_sha512_update(&mut c,d,l);hmac_sha512_final(&mut c,o)}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
