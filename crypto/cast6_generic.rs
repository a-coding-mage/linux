// SPDX-License-Identifier: GPL-2.0-or-later
/* Kernel cryptographic api.
 * cast6.c - Cast6 cipher algorithm [rfc2612].
 *
 * CAST-256 (*cast6*) is a DES like Substitution-Permutation Network (SPN)
 * cryptosystem built upon the CAST-128 (*cast5*) [rfc2144] encryption
 * algorithm.
 *
 * Copyright (C) 2003 Kartikey Mahendra Bhatt <kartik_me@hotmail.com>.
 */

// Dependencies supplied by the surrounding kernel translation unit.
use core::ffi::c_void;

extern "C" {
    static cast_s1: [u32; 256];
    static cast_s2: [u32; 256];
    static cast_s3: [u32; 256];
    static cast_s4: [u32; 256];
}

#[repr(C)]
pub struct cast6_ctx {
    pub Kr: [[u8; 4]; 12],
    pub Km: [[u32; 4]; 12],
}

#[inline]
unsafe fn rol32(x: u32, r: u8) -> u32 { x.rotate_left(r as u32) }

macro_rules! f1 { ($i:ident, $d:expr, $r:expr, $m:expr) => {{ $i = ($m).wrapping_add($d); $i = rol32($i, $r); (((cast_s1[($i >> 24) as usize] ^ cast_s2[(($i >> 16) & 0xff) as usize]).wrapping_sub(cast_s3[(($i >> 8) & 0xff) as usize])).wrapping_add(cast_s4[($i & 0xff) as usize]) }} }
macro_rules! f2 { ($i:ident, $d:expr, $r:expr, $m:expr) => {{ $i = ($m) ^ $d; $i = rol32($i, $r); (((cast_s1[($i >> 24) as usize].wrapping_sub(cast_s2[(($i >> 16) & 0xff) as usize])).wrapping_add(cast_s3[(($i >> 8) & 0xff) as usize])) ^ cast_s4[($i & 0xff) as usize]) }} }
macro_rules! f3 { ($i:ident, $d:expr, $r:expr, $m:expr) => {{ $i = ($m).wrapping_sub($d); $i = rol32($i, $r); (((cast_s1[($i >> 24) as usize].wrapping_add(cast_s2[(($i >> 16) & 0xff) as usize])) ^ cast_s3[(($i >> 8) & 0xff) as usize]).wrapping_sub(cast_s4[($i & 0xff) as usize])) }} }

static TM: [[u32; 8]; 24] = [
    [0x5a827999,0xc95c653a,0x383650db,0xa7103c7c,0x15ea281d,0x84c413be,0xf39dff5f,0x6277eb00], [0xd151d6a1,0x402bc242,0xaf05ade3,0x1ddf9984,0x8cb98525,0xfb9370c6,0x6a6d5c67,0xd9474808], [0x482133a9,0xb6fb1f4a,0x25d50aeb,0x94aef68c,0x0388e22d,0x7262cdce,0xe13cb96f,0x5016a510], [0xbef090b1,0x2dca7c52,0x9ca467f3,0x0b7e5394,0x7a583f35,0xe9322ad6,0x580c1677,0xc6e60218],
    [0x35bfedb9,0xa499d95a,0x1373c4fb,0x824db09c,0xf1279c3d,0x600187de,0xcedb737f,0x3db55f20], [0xac8f4ac1,0x1b693662,0x8a432203,0xf91d0da4,0x67f6f945,0xd6d0e4e6,0x45aad087,0xb484bc28], [0x235ea7c9,0x9238936a,0x01127f0b,0x6fec6aac,0xdec6564d,0x4da041ee,0xbc7a2d8f,0x2b541930], [0x9a2e04d1,0x0907f072,0x77e1dc13,0xe6bbc7b4,0x5595b355,0xc46f9ef6,0x33498a97,0xa2237638],
    [0x10fd61d9,0x7fd74d7a,0xeeb1391b,0x5d8b24bc,0xcc65105d,0x3b3efbfe,0xaa18e79f,0x18f2d340], [0x87ccbee1,0xf6a6aa82,0x65809623,0xd45a81c4,0x43346d65,0xb20e5906,0x20e844a7,0x8fc23048], [0xfe9c1be9,0x6d76078a,0xdc4ff32b,0x4b29decc,0xba03ca6d,0x28ddb60e,0x97b7a1af,0x06918d50], [0x756b78f1,0xe4456492,0x531f5033,0xc1f93bd4,0x30d32775,0x9fad1316,0x0e86feb7,0x7d60ea58],
    [0xec3ad5f9,0x5b14c19a,0xc9eead3b,0x38c898dc,0xa7a2847d,0x167c701e,0x85565bbf,0xf4304760], [0x630a3301,0xd1e41ea2,0x40be0a43,0xaf97f5e4,0x1e71e185,0x8d4bcd26,0xfc25b8c7,0x6affa468], [0xd9d99009,0x48b37baa,0xb78d674b,0x266752ec,0x95413e8d,0x041b2a2e,0x72f515cf,0xe1cf0170], [0x50a8ed11,0xbf82d8b2,0x2e5cc453,0x9d36aff4,0x0c109b95,0x7aea8736,0xe9c472d7,0x589e5e78],
    [0xc7784a19,0x365235ba,0xa52c215b,0x14060cfc,0x82dff89d,0xf1b9e43e,0x6093cfdf,0xcf6dbb80], [0x3e47a721,0xad2192c2,0x1bfb7e63,0x8ad56a04,0xf9af55a5,0x68894146,0xd7632ce7,0x463d1888], [0xb5170429,0x23f0efca,0x92cadb6b,0x01a4c70c,0x707eb2ad,0xdf589e4e,0x4e3289ef,0xbd0c7590], [0x2be66131,0x9ac04cd2,0x099a3873,0x78742414,0xe74e0fb5,0x5627fb56,0xc501e6f7,0x33dbd298],
    [0xa2b5be39,0x118fa9da,0x8069957b,0xef43811c,0x5e1d6cbd,0xccf7585e,0x3bd143ff,0xaaab2fa0], [0x19851b41,0x885f06e2,0xf738f283,0x6612de24,0xd4ecc9c5,0x43c6b566,0xb2a0a107,0x217a8ca8], [0x90547849,0xff2e63ea,0x6e084f8b,0xdce23b2c,0x4bbc26cd,0xba96126e,0x296ffe0f,0x9849e9b0], [0x0723d551,0x75fdc0f2,0xe4d7ac93,0x53b19834,0xc28b83d5,0x31656f76,0xa03f5b17,0x0f1946b8],
];
static TR: [[u8; 8]; 4] = [[0x13,0x04,0x15,0x06,0x17,0x08,0x19,0x0a],[0x1b,0x0c,0x1d,0x0e,0x1f,0x10,0x01,0x12],[0x03,0x14,0x05,0x16,0x07,0x18,0x09,0x1a],[0x0b,0x1c,0x0d,0x1e,0x0f,0x00,0x11,0x02]];

#[inline] unsafe fn w(key: &mut [u32;8], i: usize) { let mut x=0; let t=&TR[i%4]; key[6]^=f1!(x,key[7],t[0],TM[i][0]); key[5]^=f2!(x,key[6],t[1],TM[i][1]); key[4]^=f3!(x,key[5],t[2],TM[i][2]); key[3]^=f1!(x,key[4],t[3],TM[i][3]); key[2]^=f2!(x,key[3],t[4],TM[i][4]); key[1]^=f3!(x,key[2],t[5],TM[i][5]); key[0]^=f1!(x,key[1],t[6],TM[i][6]); key[7]^=f2!(x,key[0],t[7],TM[i][7]); }

pub unsafe fn __cast6_setkey(c: *mut cast6_ctx, in_key: *const u8, key_len: u32) -> i32 { if key_len%4!=0{return -22} let mut key=[0u32;8]; let mut p=[0u8;32]; core::ptr::copy_nonoverlapping(in_key,p.as_mut_ptr(),key_len as usize); for i in 0..8 { key[i]=u32::from_be_bytes([p[i*4],p[i*4+1],p[i*4+2],p[i*4+3]]) } for i in 0..12 { w(&mut key,2*i); w(&mut key,2*i+1); (*c).Kr[i]=[key[0]&31,key[2]&31,key[4]&31,key[6]&31].map(|x|x as u8); (*c).Km[i]=[key[7],key[5],key[3],key[1]]; } 0 }

#[inline] unsafe fn q(b:&mut [u32;4], kr:&[u8;4], km:&[u32;4]) { let mut x=0; b[2]^=f1!(x,b[3],kr[0],km[0]); b[1]^=f2!(x,b[2],kr[1],km[1]); b[0]^=f3!(x,b[1],kr[2],km[2]); b[3]^=f1!(x,b[0],kr[3],km[3]); }
#[inline] unsafe fn qbar(b:&mut [u32;4], kr:&[u8;4], km:&[u32;4]) { let mut x=0; b[3]^=f1!(x,b[0],kr[3],km[3]); b[0]^=f3!(x,b[1],kr[2],km[2]); b[1]^=f2!(x,b[2],kr[1],km[1]); b[2]^=f1!(x,b[3],kr[0],km[0]); }

pub unsafe fn __cast6_encrypt(ctx:*const c_void,out:*mut u8,input:*const u8){ crypt(ctx,out,input,true) }
pub unsafe fn __cast6_decrypt(ctx:*const c_void,out:*mut u8,input:*const u8){ crypt(ctx,out,input,false) }
unsafe fn crypt(ctx:*const c_void,out:*mut u8,input:*const u8,enc:bool){let c=&*(ctx as *const cast6_ctx);let mut b=[0u32;4];for i in 0..4{b[i]=u32::from_be_bytes([*input.add(i*4),*input.add(i*4+1),*input.add(i*4+2),*input.add(i*4+3)])}let range:Box<dyn Iterator<Item=usize>>=if enc{Box::new(0..12)}else{Box::new((0..12).rev())};for i in range{if (enc&&i<6)||(!enc&&i>=6){q(&mut b,&c.Kr[i],&c.Km[i])}else{qbar(&mut b,&c.Kr[i],&c.Km[i])}}for i in 0..4{core::ptr::copy_nonoverlapping(b[i].to_be_bytes().as_ptr(),out.add(i*4),4)}}

// Kernel crypto API entry points and registration are supplied by other units.
extern "C" {
    fn crypto_tfm_ctx(tfm: *mut c_void) -> *mut c_void;
    fn crypto_register_alg(alg: *mut c_void) -> i32;
    fn crypto_unregister_alg(alg: *mut c_void);
}

pub unsafe fn cast6_setkey(tfm: *mut c_void, key: *const u8, keylen: u32) -> i32 {
    __cast6_setkey(crypto_tfm_ctx(tfm) as *mut cast6_ctx, key, keylen)
}
unsafe fn cast6_encrypt(tfm: *mut c_void, out: *mut u8, input: *const u8) {
    __cast6_encrypt(crypto_tfm_ctx(tfm), out, input)
}
unsafe fn cast6_decrypt(tfm: *mut c_void, out: *mut u8, input: *const u8) {
    __cast6_decrypt(crypto_tfm_ctx(tfm), out, input)
}

// Corresponds to the C static `struct crypto_alg alg` initializer and module
// init/exit registration hooks; its containing kernel-specific type is external.
#[allow(dead_code)]
static ALG_NAME: &[u8] = b"cast6\0";
#[allow(dead_code)]
static ALG_DRIVER_NAME: &[u8] = b"cast6-generic\0";

#[allow(dead_code)]
unsafe fn cast6_mod_init(alg: *mut c_void) -> i32 { crypto_register_alg(alg) }
#[allow(dead_code)]
unsafe fn cast6_mod_fini(alg: *mut c_void) { crypto_unregister_alg(alg) }

// MODULE_LICENSE("GPL"); MODULE_DESCRIPTION("Cast6 Cipher Algorithm");
// MODULE_ALIAS_CRYPTO("cast6"); MODULE_ALIAS_CRYPTO("cast6-generic");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
