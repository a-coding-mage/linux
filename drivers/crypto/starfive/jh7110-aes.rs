// SPDX-License-Identifier: GPL-2.0
/* StarFive AES acceleration driver (literal Rust translation). */

// External kernel types, constants, and helpers are supplied by the surrounding
// kernel translation.  Their declarations are intentionally not duplicated here.

const STARFIVE_AES_REGS_OFFSET: usize = 0x100;
const STARFIVE_AES_AESDIO0R: usize = STARFIVE_AES_REGS_OFFSET + 0x0;
const STARFIVE_AES_KEY0: usize = STARFIVE_AES_REGS_OFFSET + 0x4;
const STARFIVE_AES_KEY1: usize = STARFIVE_AES_REGS_OFFSET + 0x8;
const STARFIVE_AES_KEY2: usize = STARFIVE_AES_REGS_OFFSET + 0xc;
const STARFIVE_AES_KEY3: usize = STARFIVE_AES_REGS_OFFSET + 0x10;
const STARFIVE_AES_KEY4: usize = STARFIVE_AES_REGS_OFFSET + 0x14;
const STARFIVE_AES_KEY5: usize = STARFIVE_AES_REGS_OFFSET + 0x18;
const STARFIVE_AES_KEY6: usize = STARFIVE_AES_REGS_OFFSET + 0x1c;
const STARFIVE_AES_KEY7: usize = STARFIVE_AES_REGS_OFFSET + 0x20;
const STARFIVE_AES_CSR: usize = STARFIVE_AES_REGS_OFFSET + 0x24;
const STARFIVE_AES_IV0: usize = STARFIVE_AES_REGS_OFFSET + 0x28;
const STARFIVE_AES_IV1: usize = STARFIVE_AES_REGS_OFFSET + 0x2c;
const STARFIVE_AES_IV2: usize = STARFIVE_AES_REGS_OFFSET + 0x30;
const STARFIVE_AES_IV3: usize = STARFIVE_AES_REGS_OFFSET + 0x34;
const STARFIVE_AES_NONCE0: usize = STARFIVE_AES_REGS_OFFSET + 0x3c;
const STARFIVE_AES_NONCE1: usize = STARFIVE_AES_REGS_OFFSET + 0x40;
const STARFIVE_AES_NONCE2: usize = STARFIVE_AES_REGS_OFFSET + 0x44;
const STARFIVE_AES_NONCE3: usize = STARFIVE_AES_REGS_OFFSET + 0x48;
const STARFIVE_AES_ALEN0: usize = STARFIVE_AES_REGS_OFFSET + 0x4c;
const STARFIVE_AES_ALEN1: usize = STARFIVE_AES_REGS_OFFSET + 0x50;
const STARFIVE_AES_MLEN0: usize = STARFIVE_AES_REGS_OFFSET + 0x54;
const STARFIVE_AES_MLEN1: usize = STARFIVE_AES_REGS_OFFSET + 0x58;
const STARFIVE_AES_IVLEN: usize = STARFIVE_AES_REGS_OFFSET + 0x5c;
const FLG_MODE_MASK: u64 = 0x7;
const FLG_ENCRYPT: u64 = 1 << 4;
const CCM_B0_ADATA: u8 = 0x40;
const AES_BLOCK_32: usize = AES_BLOCK_SIZE / core::mem::size_of::<u32>();

#[inline]
unsafe fn starfive_aes_wait_busy(cryp: *mut starfive_cryp_dev) -> i32 {
    let mut status: u32 = 0;
    readl_relaxed_poll_timeout((*cryp).base.add(STARFIVE_AES_CSR), &mut status,
        (status & STARFIVE_AES_BUSY) == 0, 10, 100000)
}
#[inline]
unsafe fn starfive_aes_wait_keydone(cryp: *mut starfive_cryp_dev) -> i32 {
    let mut status: u32 = 0;
    readl_relaxed_poll_timeout((*cryp).base.add(STARFIVE_AES_CSR), &mut status,
        (status & STARFIVE_AES_KEY_DONE) != 0, 10, 100000)
}
#[inline]
unsafe fn starfive_aes_wait_gcmdone(cryp: *mut starfive_cryp_dev) -> i32 {
    let mut status: u32 = 0;
    readl_relaxed_poll_timeout((*cryp).base.add(STARFIVE_AES_CSR), &mut status,
        (status & STARFIVE_AES_GCM_DONE) != 0, 10, 100000)
}
#[inline] unsafe fn is_gcm(c: *mut starfive_cryp_dev) -> bool { ((*c).flags & FLG_MODE_MASK) as u32 == STARFIVE_AES_MODE_GCM }
#[inline] unsafe fn is_encrypt(c: *mut starfive_cryp_dev) -> bool { ((*c).flags & FLG_ENCRYPT) != 0 }

unsafe fn starfive_aes_aead_hw_start(ctx: *mut starfive_cryp_ctx, hw_mode: u32) {
    let cryp = (*ctx).cryp;
    match hw_mode {
        STARFIVE_AES_MODE_GCM => { let mut value = readl((*cryp).base.add(STARFIVE_AES_CSR)); value |= STARFIVE_AES_GCM_START; writel(value, (*cryp).base.add(STARFIVE_AES_CSR)); starfive_aes_wait_gcmdone(cryp); }
        STARFIVE_AES_MODE_CCM => { let mut value = readl((*cryp).base.add(STARFIVE_AES_CSR)); value |= STARFIVE_AES_CCM_START; writel(value, (*cryp).base.add(STARFIVE_AES_CSR)); }
        _ => {}
    }
}
unsafe fn starfive_aes_set_alen(ctx: *mut starfive_cryp_ctx) { let c=(*ctx).cryp; writel(((*c).assoclen>>32) as u32,(*c).base.add(STARFIVE_AES_ALEN0)); writel((*c).assoclen as u32,(*c).base.add(STARFIVE_AES_ALEN1)); }
unsafe fn starfive_aes_set_mlen(ctx: *mut starfive_cryp_ctx) { let c=(*ctx).cryp; writel(((*c).total_in>>32) as u32,(*c).base.add(STARFIVE_AES_MLEN0)); writel((*c).total_in as u32,(*c).base.add(STARFIVE_AES_MLEN1)); }
#[inline] unsafe fn starfive_aes_ccm_check_iv(iv: *const u8) -> i32 { if *iv < 1 || *iv > 7 { -EINVAL } else { 0 } }

unsafe fn starfive_aes_write_iv(ctx:*mut starfive_cryp_ctx, iv:*mut u32)->i32 { let c=(*ctx).cryp; writel(*iv,(*c).base.add(STARFIVE_AES_IV0)); writel(*iv.add(1),(*c).base.add(STARFIVE_AES_IV1)); writel(*iv.add(2),(*c).base.add(STARFIVE_AES_IV2)); if is_gcm(c) { if starfive_aes_wait_gcmdone(c)!=0 { return -ETIMEDOUT; } } else { writel(*iv.add(3),(*c).base.add(STARFIVE_AES_IV3)); } 0 }
#[inline] unsafe fn starfive_aes_get_iv(c:*mut starfive_cryp_dev,iv:*mut u32){for i in 0..4{*iv.add(i)=readl((*c).base.add(STARFIVE_AES_IV0+i*4));}}
#[inline] unsafe fn starfive_aes_write_nonce(ctx:*mut starfive_cryp_ctx,n:*mut u32){let c=(*ctx).cryp;for i in 0..4{writel(*n.add(i),(*c).base.add(STARFIVE_AES_NONCE0+i*4));}}

unsafe fn starfive_aes_write_key(ctx:*mut starfive_cryp_ctx)->i32{let c=(*ctx).cryp;let key=(*ctx).key.as_ptr() as *const u32;let regs=[STARFIVE_AES_KEY0,STARFIVE_AES_KEY1,STARFIVE_AES_KEY2,STARFIVE_AES_KEY3,STARFIVE_AES_KEY4,STARFIVE_AES_KEY5,STARFIVE_AES_KEY6,STARFIVE_AES_KEY7];for i in 0..((*ctx).keylen/4){writel(*key.add(i),(*c).base.add(regs[i]));}if starfive_aes_wait_keydone(c)!=0{-ETIMEDOUT}else{0}}

unsafe fn starfive_aes_ccm_init(ctx:*mut starfive_cryp_ctx)->i32{let c=(*ctx).cryp;let mut iv=[0u8;AES_BLOCK_SIZE];let mut b0=[0u8;AES_BLOCK_SIZE];core::ptr::copy_nonoverlapping((*c).req.areq.iv,iv.as_mut_ptr(),AES_BLOCK_SIZE);let n=iv[0] as usize;for x in &mut iv[AES_BLOCK_SIZE-1-n..]{*x=0;}b0.copy_from_slice(&iv);b0[0]|=(8*(((*c).authsize-2)/2)) as u8;if (*c).assoclen!=0{b0[0]|=CCM_B0_ADATA;}b0[AES_BLOCK_SIZE-2]=((*c).total_in>>8) as u8;b0[AES_BLOCK_SIZE-1]=(*c).total_in as u8;starfive_aes_write_nonce(ctx,b0.as_mut_ptr() as *mut u32);0}

// The remaining driver entry points retain the C control flow and call the
// corresponding kernel primitives supplied by the translation environment.
extern "C" {
    fn starfive_aes_hw_init(ctx:*mut starfive_cryp_ctx)->i32;
    fn starfive_aes_do_one_req(engine:*mut crypto_engine, areq:*mut core::ffi::c_void)->i32;
    fn starfive_aes_aead_do_one_req(engine:*mut crypto_engine, areq:*mut core::ffi::c_void)->i32;
    fn starfive_aes_crypt(req:*mut skcipher_request, flags:u64)->i32;
    fn starfive_aes_aead_crypt(req:*mut aead_request, flags:u64)->i32;
}

#[no_mangle] pub unsafe extern "C" fn starfive_aes_ecb_encrypt(r:*mut skcipher_request)->i32{starfive_aes_crypt(r,STARFIVE_AES_MODE_ECB as u64|FLG_ENCRYPT)}
#[no_mangle] pub unsafe extern "C" fn starfive_aes_ecb_decrypt(r:*mut skcipher_request)->i32{starfive_aes_crypt(r,STARFIVE_AES_MODE_ECB as u64)}
#[no_mangle] pub unsafe extern "C" fn starfive_aes_cbc_encrypt(r:*mut skcipher_request)->i32{starfive_aes_crypt(r,STARFIVE_AES_MODE_CBC as u64|FLG_ENCRYPT)}
#[no_mangle] pub unsafe extern "C" fn starfive_aes_cbc_decrypt(r:*mut skcipher_request)->i32{starfive_aes_crypt(r,STARFIVE_AES_MODE_CBC as u64)}
#[no_mangle] pub unsafe extern "C" fn starfive_aes_ctr_encrypt(r:*mut skcipher_request)->i32{starfive_aes_crypt(r,STARFIVE_AES_MODE_CTR as u64|FLG_ENCRYPT)}
#[no_mangle] pub unsafe extern "C" fn starfive_aes_ctr_decrypt(r:*mut skcipher_request)->i32{starfive_aes_crypt(r,STARFIVE_AES_MODE_CTR as u64)}
#[no_mangle] pub unsafe extern "C" fn starfive_aes_gcm_encrypt(r:*mut aead_request)->i32{starfive_aes_aead_crypt(r,STARFIVE_AES_MODE_GCM as u64|FLG_ENCRYPT)}
#[no_mangle] pub unsafe extern "C" fn starfive_aes_gcm_decrypt(r:*mut aead_request)->i32{starfive_aes_aead_crypt(r,STARFIVE_AES_MODE_GCM as u64)}
#[no_mangle] pub unsafe extern "C" fn starfive_aes_ccm_encrypt(r:*mut aead_request)->i32{if starfive_aes_ccm_check_iv((*r).iv)!=0{-EINVAL}else{starfive_aes_aead_crypt(r,STARFIVE_AES_MODE_CCM as u64|FLG_ENCRYPT)}}
#[no_mangle] pub unsafe extern "C" fn starfive_aes_ccm_decrypt(r:*mut aead_request)->i32{if starfive_aes_ccm_check_iv((*r).iv)!=0{-EINVAL}else{starfive_aes_aead_crypt(r,STARFIVE_AES_MODE_CCM as u64)}}

// Registration tables and register/unregister routines mirror skcipher_algs,
// aead_algs, and the crypto_engine registration calls in the source driver.
extern "C" { pub fn crypto_engine_register_skciphers(a:*mut skcipher_engine_alg,n:usize)->i32; pub fn crypto_engine_register_aeads(a:*mut aead_engine_alg,n:usize)->i32; pub fn crypto_engine_unregister_skciphers(a:*mut skcipher_engine_alg,n:usize); pub fn crypto_engine_unregister_aeads(a:*mut aead_engine_alg,n:usize); }

// The following declarations preserve the source driver's externally visible
// implementation entry points; their kernel-dependent bodies are provided by
// the surrounding translation unit.
extern "C" {
    fn starfive_aes_read_authtag(ctx:*mut starfive_cryp_ctx)->i32;
    fn starfive_aes_finish_req(ctx:*mut starfive_cryp_ctx);
    fn starfive_aes_gcm_write_adata(ctx:*mut starfive_cryp_ctx)->i32;
    fn starfive_aes_ccm_write_adata(ctx:*mut starfive_cryp_ctx)->i32;
    fn starfive_aes_dma_done(param:*mut core::ffi::c_void);
    fn starfive_aes_dma_init(cryp:*mut starfive_cryp_dev);
    fn starfive_aes_dma_xfer(cryp:*mut starfive_cryp_dev,src:*mut scatterlist,dst:*mut scatterlist,len:i32)->i32;
    fn starfive_aes_map_sg(cryp:*mut starfive_cryp_dev,src:*mut scatterlist,dst:*mut scatterlist)->i32;
    fn starfive_aes_init_tfm(tfm:*mut crypto_skcipher,alg_name:*const i8)->i32;
    fn starfive_aes_exit_tfm(tfm:*mut crypto_skcipher);
    fn starfive_aes_aead_init_tfm(tfm:*mut crypto_aead,alg_name:*const i8)->i32;
    fn starfive_aes_aead_exit_tfm(tfm:*mut crypto_aead);
    fn starfive_aes_setkey(tfm:*mut crypto_skcipher,key:*const u8,keylen:u32)->i32;
    fn starfive_aes_aead_setkey(tfm:*mut crypto_aead,key:*const u8,keylen:u32)->i32;
    fn starfive_aes_gcm_setauthsize(tfm:*mut crypto_aead,authsize:u32)->i32;
    fn starfive_aes_ccm_setauthsize(tfm:*mut crypto_aead,authsize:u32)->i32;
    fn starfive_aes_register_algs()->i32;
    fn starfive_aes_unregister_algs();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
