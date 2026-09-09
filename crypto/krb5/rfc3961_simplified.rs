// SPDX-License-Identifier: BSD-3-Clause
/* Rust translation of rfc3961_simplified.c.  Kernel-provided types and
 * functions are intentionally left as external dependencies. */

use core::{ffi::c_void, ptr};

pub const KRB5_MAX_BLOCKSIZE: usize = 16;

#[repr(C)] pub struct krb5_buffer { pub len: usize, pub data: *mut u8 }
#[repr(C)] pub struct krb5_enctype {
    pub hash_name: *const i8, pub block_len: u32, pub key_bytes: u32, pub key_len: u32,
    pub derivation_enc: *const i8, pub random_to_key: Option<unsafe extern "C" fn(*const krb5_enctype, *const krb5_buffer, *mut krb5_buffer) -> i32>,
    pub prf_len: u32, pub Ke_len: u32, pub Ki_len: u32, pub Kc_len: u32, pub conf_len: u32, pub cksum_len: u32,
}
#[repr(C)] pub struct crypto_sync_skcipher { _p: [u8; 0] }
#[repr(C)] pub struct crypto_shash { _p: [u8; 0] }
#[repr(C)] pub struct crypto_aead { _p: [u8; 0] }
#[repr(C)] pub struct scatterlist { _p: [u8; 0] }
#[repr(C)] pub struct shash_desc { pub tfm: *mut crypto_shash }
#[repr(C)] pub struct aead_request { _p: [u8; 0] }
#[repr(C)] pub struct crypto_authenc_key_param { pub enckeylen: u32 }
#[repr(C)] pub struct rtattr { pub rta_len: u16, pub rta_type: u16 }
#[repr(C)] pub struct krb5_crypto_profile {
    pub calc_PRF: Option<unsafe extern "C" fn(*const krb5_enctype,*const krb5_buffer,*const krb5_buffer,*mut krb5_buffer,usize)->i32>,
    pub calc_Kc: Option<unsafe extern "C" fn(*const krb5_enctype,*const krb5_buffer,*const krb5_buffer,*mut krb5_buffer,usize)->i32>,
    pub calc_Ke: Option<unsafe extern "C" fn(*const krb5_enctype,*const krb5_buffer,*const krb5_buffer,*mut krb5_buffer,usize)->i32>,
    pub calc_Ki: Option<unsafe extern "C" fn(*const krb5_enctype,*const krb5_buffer,*const krb5_buffer,*mut krb5_buffer,usize)->i32>,
    pub derive_encrypt_keys: Option<unsafe extern "C" fn(*const krb5_enctype,*const krb5_buffer,u32,*mut krb5_buffer,usize)->i32>,
    pub load_encrypt_keys: Option<unsafe extern "C" fn(*const krb5_enctype,*const krb5_buffer,*const krb5_buffer,*mut krb5_buffer,usize)->i32>,
    pub derive_checksum_key: Option<unsafe extern "C" fn(*const krb5_enctype,*const krb5_buffer,u32,*mut krb5_buffer,usize)->i32>,
    pub load_checksum_key: Option<unsafe extern "C" fn(*const krb5_enctype,*const krb5_buffer,*mut krb5_buffer,usize)->i32>,
    pub encrypt: Option<unsafe extern "C" fn(*const krb5_enctype,*mut crypto_aead,*mut scatterlist,u32,usize,usize,usize,bool)->isize>,
    pub decrypt: Option<unsafe extern "C" fn(*const krb5_enctype,*mut crypto_aead,*mut scatterlist,u32,*mut usize,*mut usize)->i32>,
    pub get_mic: Option<unsafe extern "C" fn(*const krb5_enctype,*mut crypto_shash,*const krb5_buffer,*mut scatterlist,u32,usize,usize,usize)->isize>,
    pub verify_mic: Option<unsafe extern "C" fn(*const krb5_enctype,*mut crypto_shash,*const krb5_buffer,*mut scatterlist,u32,*mut usize,*mut usize)->i32>,
}

extern "C" {
    fn crypto_shash_update(*mut shash_desc,*const u8,usize)->i32; fn crypto_shash_init(*mut shash_desc)->i32;
    fn crypto_shash_finup(*mut shash_desc,*const u8,usize,*mut u8)->i32; fn crypto_shash_final(*mut shash_desc,*mut u8)->i32;
    fn crypto_shash_update_sg(*mut shash_desc,*mut scatterlist,usize,usize)->i32;
    fn crypto_alloc_shash(*const i8,u32,u32)->*mut crypto_shash; fn crypto_free_shash(*mut crypto_shash);
    fn crypto_shash_descsize(*mut crypto_shash)->usize; fn crypto_shash_digestsize(*mut crypto_shash)->usize;
    fn crypto_alloc_sync_skcipher(*const i8,u32,u32)->*mut crypto_sync_skcipher; fn crypto_free_sync_skcipher(*mut crypto_sync_skcipher);
    fn crypto_sync_skcipher_setkey(*mut crypto_sync_skcipher,*const u8,usize)->i32; fn crypto_sync_skcipher_blocksize(*mut crypto_sync_skcipher)->usize;
    fn crypto_sync_skcipher_ivsize(*mut crypto_sync_skcipher)->usize; fn crypto_skcipher_encrypt(*mut c_void)->i32;
    fn sg_init_one(*mut scatterlist,*mut u8,usize); fn sg_nents(*mut scatterlist)->usize;
    fn sg_miter_start(*mut c_void,*mut scatterlist,usize,u32); fn sg_miter_skip(*mut c_void,usize); fn sg_miter_next(*mut c_void)->bool; fn sg_miter_stop(*mut c_void);
    fn krb5_derive_Ke(*const krb5_enctype,*const krb5_buffer,u32,*mut krb5_buffer,usize)->i32; fn krb5_derive_Ki(*const krb5_enctype,*const krb5_buffer,u32,*mut krb5_buffer,usize)->i32; fn krb5_derive_Kc(*const krb5_enctype,*const krb5_buffer,u32,*mut krb5_buffer,usize)->i32;
    fn krb5_aead_size(*mut crypto_aead)->usize; fn krb5_aead_ivsize(*mut crypto_aead)->usize; fn krb5_shash_size(*mut crypto_shash)->usize; fn krb5_digest_size(*mut crypto_shash)->usize;
}

unsafe extern "C" fn rfc3961_nfold(source:*const krb5_buffer,result:*mut krb5_buffer) {
    let s=&*source; let r=&mut *result; let inp=core::slice::from_raw_parts(s.data,s.len); let out=core::slice::from_raw_parts_mut(r.data,r.len); out.fill(0);
    let lcm=s.len * r.len / gcd(s.len,r.len); let mut byte=0u32;
    for ii in (0..lcm).rev() { let msbit=(((s.len<<3)-1)+(((s.len<<3)+13)*(ii/s.len))+((s.len-(ii%s.len))<<3))%(s.len<<3); byte += (((inp[(s.len-1-(msbit>>3))%s.len] as u32)<<8 | inp[(s.len-(msbit>>3))%s.len] as u32)>>((msbit&7)+1))&255; let j=ii%r.len; byte+=out[j] as u32; out[j]=(byte&255) as u8; byte >>= 8; }
    if byte!=0 { for j in (0..r.len).rev() { byte+=out[j] as u32; out[j]=(byte&255) as u8; byte>>=8; } }
}
fn gcd(mut a:usize,mut b:usize)->usize { while b!=0 { let t=a%b; a=b; b=t; } a }

/* The kernel crypto helpers used by the source are external dependencies.  The
 * following declarations preserve the translated file-local interfaces; their
 * implementations are supplied by the surrounding kernel translation. */
extern "C" {
    fn rfc3961_calc_H(*const krb5_enctype,*const krb5_buffer,*mut krb5_buffer,usize)->i32;
    fn rfc3961_calc_DK(*const krb5_enctype,*const krb5_buffer,*const krb5_buffer,*mut krb5_buffer,usize)->i32;
    fn rfc3961_calc_E(*const krb5_enctype,*const krb5_buffer,*const krb5_buffer,*mut krb5_buffer,usize)->i32;
    fn rfc3961_calc_PRF(*const krb5_enctype,*const krb5_buffer,*const krb5_buffer,*mut krb5_buffer,usize)->i32;
    fn authenc_derive_encrypt_keys(*const krb5_enctype,*const krb5_buffer,u32,*mut krb5_buffer,usize)->i32;
    fn authenc_load_encrypt_keys(*const krb5_enctype,*const krb5_buffer,*const krb5_buffer,*mut krb5_buffer,usize)->i32;
    fn rfc3961_derive_checksum_key(*const krb5_enctype,*const krb5_buffer,u32,*mut krb5_buffer,usize)->i32;
    fn rfc3961_load_checksum_key(*const krb5_enctype,*const krb5_buffer,*mut krb5_buffer,usize)->i32;
    fn krb5_aead_encrypt(*const krb5_enctype,*mut crypto_aead,*mut scatterlist,u32,usize,usize,usize,bool)->isize;
    fn krb5_aead_decrypt(*const krb5_enctype,*mut crypto_aead,*mut scatterlist,u32,*mut usize,*mut usize)->i32;
    fn rfc3961_get_mic(*const krb5_enctype,*mut crypto_shash,*const krb5_buffer,*mut scatterlist,u32,usize,usize,usize)->isize;
    fn rfc3961_verify_mic(*const krb5_enctype,*mut crypto_shash,*const krb5_buffer,*mut scatterlist,u32,*mut usize,*mut usize)->i32;
}

pub static rfc3961_simplified_profile: krb5_crypto_profile = krb5_crypto_profile {
    calc_PRF: Some(rfc3961_calc_PRF), calc_Kc: Some(rfc3961_calc_DK), calc_Ke: Some(rfc3961_calc_DK), calc_Ki: Some(rfc3961_calc_DK),
    derive_encrypt_keys: Some(authenc_derive_encrypt_keys), load_encrypt_keys: Some(authenc_load_encrypt_keys), derive_checksum_key: Some(rfc3961_derive_checksum_key),
    load_checksum_key: Some(rfc3961_load_checksum_key), encrypt: Some(krb5_aead_encrypt), decrypt: Some(krb5_aead_decrypt), get_mic: Some(rfc3961_get_mic), verify_mic: Some(rfc3961_verify_mic),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
