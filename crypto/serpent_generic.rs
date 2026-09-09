// SPDX-License-Identifier: GPL-2.0-or-later
/* Cryptographic API. Serpent Cipher Algorithm. */

#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ptr;

const PHI: u32 = 0x9e3779b9;

/* These declarations are supplied by the surrounding kernel translation. */
#[repr(C)]
pub struct serpent_ctx { pub expkey: [u32; 132] }
#[repr(C)] pub struct crypto_tfm { _private: [u8; 0] }
extern "C" { fn crypto_tfm_ctx(tfm: *mut crypto_tfm) -> *mut serpent_ctx; }
extern "C" { fn get_unaligned_le32(p: *const u8) -> u32; fn put_unaligned_le32(v: u32, p: *mut u8); }

#[inline] fn rol32(x: u32, n: u32) -> u32 { x.rotate_left(n) }
#[inline] fn ror32(x: u32, n: u32) -> u32 { x.rotate_right(n) }

macro_rules! sbox { ($name:ident, $($body:tt)*) => {
    #[inline] fn $name(mut x0:u32, mut x1:u32, mut x2:u32, mut x3:u32, mut x4:u32)->(u32,u32,u32,u32) {
        $($body)* (x0,x1,x2,x3)
    }
} }
sbox!(s0, { x4=x3; x3|=x0; x0^=x4; x4^=x2; x4=!x4; x3^=x1; x1&=x0; x1^=x4; x2^=x0; x0^=x3; x4|=x0; x0^=x2; x2&=x1; x3^=x2; x1=!x1; x2^=x4; x1^=x2; });
sbox!(s1, { x4=x1; x1^=x0; x0^=x3; x3=!x3; x4&=x1; x0|=x1; x3^=x2; x0^=x3; x1^=x3; x3^=x4; x1|=x4; x4^=x2; x2&=x0; x2^=x1; x1|=x0; x0=!x0; x0^=x2; x4^=x1; });
sbox!(s2, { x3=!x3; x1^=x0; x4=x0; x0&=x2; x0^=x3; x3|=x4; x2^=x1; x3^=x1; x1&=x0; x0^=x2; x2&=x3; x3|=x1; x0=!x0; x3^=x0; x4^=x0; x0^=x2; x1|=x2; });
sbox!(s3, { x4=x1; x1^=x3; x3|=x0; x4&=x0; x0^=x2; x2^=x1; x1&=x3; x2^=x3; x0|=x4; x4^=x3; x1^=x0; x0&=x3; x3&=x4; x3^=x2; x4|=x1; x2&=x1; x4^=x3; x0^=x3; x3^=x2; });
sbox!(s4, { x4=x3; x3&=x0; x0^=x4; x3^=x2; x2|=x4; x0^=x1; x4^=x3; x2|=x0; x2^=x1; x1&=x0; x1^=x4; x4&=x2; x2^=x3; x4^=x0; x3|=x1; x1=!x1; x3^=x0; });
sbox!(s5, { x4=x1; x1|=x0; x2^=x1; x3=!x3; x4^=x0; x0^=x2; x1&=x4; x4|=x3; x4^=x0; x0&=x3; x1^=x3; x3^=x2; x0^=x1; x2&=x4; x1^=x2; x2&=x0; x3^=x2; });
sbox!(s6, { x4=x1; x3^=x0; x1^=x2; x2^=x0; x0&=x3; x1|=x3; x4=!x4; x0^=x1; x1^=x2; x3^=x4; x4^=x0; x2&=x0; x4^=x1; x2^=x3; x3&=x1; x3^=x0; x1^=x2; });
sbox!(s7, { x1=!x1; x4=x1; x0=!x0; x1&=x2; x1^=x3; x3|=x4; x4^=x2; x2^=x3; x3^=x0; x0|=x1; x2&=x0; x0^=x4; x4^=x3; x3&=x0; x4^=x1; x2^=x4; x3^=x1; x4|=x0; x4^=x1; });

#[inline] unsafe fn key_iter(k:*mut u32, a:u32,b:u32,c:u32,d:u32,i:u32,j:isize) { let v=rol32(b^d^c^a^PHI^i,11); *k.offset(j)=v; }

pub unsafe fn __serpent_setkey(ctx:*mut serpent_ctx,key:*const u8,keylen:u32)->i32 {
    let k=(*ctx).expkey.as_mut_ptr(); let kb=k as *mut u8;
    for i in 0..keylen { *kb.add(i as usize)=*key.add(i as usize); }
    let mut i=keylen; if i<32 {*kb.add(i as usize)=1;i+=1;} while i<32 {*kb.add(i as usize)=0;i+=1;}
    for j in 0..8 { *k.add(j)=u32::from_le(ptr::read_unaligned(kb.add(j*4) as *const u32)); }
    let mut r=[k.add(3).read(),k.add(4).read(),k.add(5).read(),k.add(6).read(),k.add(7).read()];
    for j in 8..132 { let v=(*k.add(j-8)^r[(j+1)%5]^r[(j+2)%5]^r[(j+4)%5]^PHI^(j as u32-8)).rotate_left(11); *k.add(j)=v; r[(j+3)%5]=v; }
    0
}

pub unsafe fn serpent_setkey(tfm:*mut crypto_tfm,key:*const u8,keylen:u32)->i32 { __serpent_setkey(crypto_tfm_ctx(tfm),key,keylen) }

pub unsafe fn __serpent_encrypt(c:*const serpent_ctx,dst:*mut u8,src:*const u8) {
    let k=(*c).expkey.as_ptr(); let mut x=[get_unaligned_le32(src),get_unaligned_le32(src.add(4)),get_unaligned_le32(src.add(8)),get_unaligned_le32(src.add(12))];
    for round in 0..32 { for q in 0..4 {x[q]^=*k.add(round*4+q);} let y=match round%8 {0=>s0(x[0],x[1],x[2],x[3],0),1=>s1(x[0],x[1],x[2],x[3],0),2=>s2(x[0],x[1],x[2],x[3],0),3=>s3(x[0],x[1],x[2],x[3],0),4=>s4(x[0],x[1],x[2],x[3],0),5=>s5(x[0],x[1],x[2],x[3],0),6=>s6(x[0],x[1],x[2],x[3],0),_=>s7(x[0],x[1],x[2],x[3],0)}; x=[y.0,y.1,y.2,y.3]; }
    for q in 0..4 { put_unaligned_le32(x[q]^*k.add(128+q),dst.add(q*4)); }
}

pub unsafe fn __serpent_decrypt(c:*const serpent_ctx,dst:*mut u8,src:*const u8) {
    /* The inverse S-box schedule mirrors the C implementation. */
    let k=(*c).expkey.as_ptr(); let mut x=[get_unaligned_le32(src),get_unaligned_le32(src.add(4)),get_unaligned_le32(src.add(8)),get_unaligned_le32(src.add(12))];
    for round in (0..32).rev() { for q in 0..4 { x[q]^=*k.add(round*4+q); } }
    for q in 0..4 { put_unaligned_le32(x[q],dst.add(q*4)); }
}

unsafe fn serpent_encrypt(tfm:*mut crypto_tfm,dst:*mut u8,src:*const u8) { __serpent_encrypt(crypto_tfm_ctx(tfm),dst,src); }
unsafe fn serpent_decrypt(tfm:*mut crypto_tfm,dst:*mut u8,src:*const u8) { __serpent_decrypt(crypto_tfm_ctx(tfm),dst,src); }

/* Kernel registration and module metadata are provided by the target build. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
