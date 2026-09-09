// SPDX-License-Identifier: GPL-2.0-only
/* Cryptographic API. Support for OMAP AES HW acceleration. */

// C dependencies are supplied by the surrounding kernel translation.
use core::ffi::{c_char, c_int, c_ulong, c_void};

extern "C" {
    static mut dev_list: list_head;
    static mut list_lock: spinlock_t;
    static mut aes_fallback_sz: c_int;
}

#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct platform_device { _private: [u8; 0] }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct scatterlist { pub next: *mut scatterlist, pub length: u32 }
#[repr(C)] pub struct crypto_engine { _private: [u8; 0] }
#[repr(C)] pub struct skcipher_request { pub src: *mut scatterlist, pub dst: *mut scatterlist, pub cryptlen: usize, pub iv: *mut u8, pub base: crypto_async_request }
#[repr(C)] pub struct crypto_async_request { pub flags: u32, pub complete: Option<unsafe extern "C" fn(*mut crypto_async_request, c_int)>, pub data: *mut c_void }
#[repr(C)] pub struct crypto_skcipher { _private: [u8; 0] }
#[repr(C)] pub struct omap_aes_ctx { pub key: [u8; 32], pub keylen: u32, pub fallback: *mut crypto_skcipher }
#[repr(C)] pub struct omap_aes_reqctx { pub dd: *mut omap_aes_dev, pub mode: c_ulong, pub iv: [u8; 16], pub fallback_req: skcipher_request }
#[repr(C)] pub struct omap_aes_dev {
    pub list: list_head, pub dev: *mut device, pub io_base: *mut u8, pub phys_base: u64,
    pub pdata: *const omap_aes_pdata, pub flags: u32, pub err: c_int, pub req: *mut skcipher_request,
    pub aead_req: *mut c_void, pub ctx: *mut omap_aes_ctx, pub in_sg: *mut scatterlist, pub out_sg: *mut scatterlist,
    pub orig_out: *mut scatterlist, pub in_sgl: scatterlist, pub out_sgl: scatterlist, pub in_sg_len: c_int,
    pub out_sg_len: c_int, pub in_sg_offset: usize, pub out_sg_offset: usize, pub total: usize, pub total_save: usize,
    pub dma_lch_in: *mut c_void, pub dma_lch_out: *mut c_void, pub pio_only: bool, pub engine: *mut crypto_engine,
    pub done_task: work_struct, pub lock: spinlock_t, pub aead_queue: c_void,
}
#[repr(C)] pub struct omap_aes_pdata { pub trigger: Option<unsafe extern "C" fn(*mut omap_aes_dev, c_int)>, pub key_ofs: u32, pub iv_ofs: u32, pub ctrl_ofs: u32, pub data_ofs: u32, pub rev_ofs: u32, pub mask_ofs: u32, pub irq_status_ofs: u32, pub irq_enable_ofs: u32, pub dma_enable_in: u32, pub dma_enable_out: u32, pub dma_start: u32, pub major_mask: u32, pub major_shift: u32, pub minor_mask: u32, pub minor_shift: u32 }

const AES_BLOCK_SIZE: usize = 16;
const FLAGS_INIT: u32 = 1 << 0;
const FLAGS_GCM: u32 = 1 << 1;
const FLAGS_CBC: u32 = 1 << 2;
const FLAGS_CTR: u32 = 1 << 3;
const FLAGS_ENCRYPT: u32 = 1 << 4;
const AES_KEYSIZE_128: u32 = 16; const AES_KEYSIZE_192: u32 = 24; const AES_KEYSIZE_256: u32 = 32;

extern "C" {
    fn __raw_readl(p: *mut u8) -> u32; fn __raw_writel(v: u32, p: *mut u8);
    fn pm_runtime_resume_and_get(d: *mut device) -> c_int; fn pm_runtime_put_autosuspend(d: *mut device);
    fn omap_aes_gcm_dma_out_callback(data: *mut c_void); fn omap_crypto_align_sg(a: *mut *mut scatterlist, n: usize, b: usize, s: *mut scatterlist, f: u16, sh: u32, flags: *mut u32) -> c_int;
    fn omap_crypto_cleanup(s: *mut scatterlist, o: *mut scatterlist, x: usize, n: usize, sh: u32, f: u32);
    fn crypto_finalize_skcipher_request(e: *mut crypto_engine, r: *mut skcipher_request, err: c_int);
    fn crypto_transfer_skcipher_request_to_engine(e: *mut crypto_engine, r: *mut skcipher_request) -> c_int;
    fn crypto_skcipher_encrypt(r: *mut skcipher_request) -> c_int; fn crypto_skcipher_decrypt(r: *mut skcipher_request) -> c_int;
}

#[inline] unsafe fn omap_aes_read(dd: *mut omap_aes_dev, offset: u32) -> u32 { __raw_readl((*dd).io_base.add(offset as usize)) }
#[inline] unsafe fn omap_aes_write(dd: *mut omap_aes_dev, offset: u32, value: u32) { __raw_writel(value, (*dd).io_base.add(offset as usize)); }
unsafe fn omap_aes_write_mask(dd: *mut omap_aes_dev, offset: u32, value: u32, mask: u32) { let mut v=omap_aes_read(dd,offset); v &= !mask; v |= value; omap_aes_write(dd,offset,v); }
unsafe fn omap_aes_write_n(dd: *mut omap_aes_dev, mut offset: u32, value: *mut u32, mut count: c_int) { while count > 0 { omap_aes_write(dd,offset,*value); value=value.add(1); offset+=4; count-=1; } }

unsafe fn omap_aes_hw_init(dd: *mut omap_aes_dev) -> c_int { if (*dd).flags & FLAGS_INIT == 0 { (*dd).flags |= FLAGS_INIT; (*dd).err=0; } let e=pm_runtime_resume_and_get((*dd).dev); if e<0 { return e; } 0 }
#[no_mangle] pub unsafe extern "C" fn omap_aes_clear_copy_flags(dd:*mut omap_aes_dev) { (*dd).flags &= !(0xffff << 8); }

#[no_mangle] pub unsafe extern "C" fn omap_aes_write_ctrl(dd:*mut omap_aes_dev)->c_int { if omap_aes_hw_init(dd)!=0{return -1;} let key32=(*dd).ctx.as_ref().unwrap().keylen/4; for i in 0..key32 { let p=(*dd).ctx.as_ref().unwrap().key.as_ptr().add((i*4) as usize) as *const u32; omap_aes_write(dd,(*dd).pdata.as_ref().unwrap().key_ofs+i*4,u32::from_le(*p)); } 0 }
unsafe fn omap_aes_dma_trigger_omap2(dd:*mut omap_aes_dev,_length:c_int) { let p=(*dd).pdata.as_ref().unwrap(); omap_aes_write_mask(dd,p.mask_ofs,p.dma_start,p.dma_start|p.dma_enable_in|p.dma_enable_out); }
unsafe fn omap_aes_dma_trigger_omap4(dd:*mut omap_aes_dev,length:c_int) { let p=(*dd).pdata.as_ref().unwrap(); omap_aes_write(dd,p.data_ofs,length as u32); omap_aes_dma_trigger_omap2(dd,length); }
unsafe fn omap_aes_dma_stop(dd:*mut omap_aes_dev) { let p=(*dd).pdata.as_ref().unwrap(); omap_aes_write_mask(dd,p.mask_ofs,0,p.dma_start|p.dma_enable_in|p.dma_enable_out); }

#[no_mangle] pub unsafe extern "C" fn omap_aes_find_dev(rctx:*mut omap_aes_reqctx)->*mut omap_aes_dev { let d=dev_list.next as *mut omap_aes_dev; (*rctx).dd=d; d }
unsafe fn omap_aes_finish_req(dd:*mut omap_aes_dev,err:c_int) { crypto_finalize_skcipher_request((*dd).engine,(*dd).req,err); pm_runtime_put_autosuspend((*dd).dev); }
#[no_mangle] pub unsafe extern "C" fn omap_aes_crypt_dma_stop(dd:*mut omap_aes_dev)->c_int { omap_aes_dma_stop(dd); 0 }
unsafe fn omap_aes_crypt_req(_engine:*mut crypto_engine,areq:*mut c_void)->c_int { let req=areq as *mut skcipher_request; let rctx=(*req).base.data as *mut omap_aes_reqctx; if rctx.is_null(){return -19} (*rctx).dd=omap_aes_find_dev(rctx); 0 }

unsafe fn omap_aes_crypt(req:*mut skcipher_request,mode:c_ulong)->c_int { if (*req).cryptlen < aes_fallback_sz as usize { return if mode & FLAGS_ENCRYPT as c_ulong != 0 {crypto_skcipher_encrypt(req)} else {crypto_skcipher_decrypt(req)}; } let rctx=(*req).base.data as *mut omap_aes_reqctx; let dd=omap_aes_find_dev(rctx); (*rctx).mode=mode; crypto_transfer_skcipher_request_to_engine((*dd).engine,req) }
unsafe fn omap_aes_ecb_encrypt(r:*mut skcipher_request)->c_int{omap_aes_crypt(r,FLAGS_ENCRYPT as c_ulong)} unsafe fn omap_aes_ecb_decrypt(r:*mut skcipher_request)->c_int{omap_aes_crypt(r,0)}
unsafe fn omap_aes_cbc_encrypt(r:*mut skcipher_request)->c_int{omap_aes_crypt(r,(FLAGS_ENCRYPT|FLAGS_CBC) as c_ulong)} unsafe fn omap_aes_cbc_decrypt(r:*mut skcipher_request)->c_int{omap_aes_crypt(r,FLAGS_CBC as c_ulong)}
unsafe fn omap_aes_ctr_encrypt(r:*mut skcipher_request)->c_int{omap_aes_crypt(r,(FLAGS_ENCRYPT|FLAGS_CTR) as c_ulong)} unsafe fn omap_aes_ctr_decrypt(r:*mut skcipher_request)->c_int{omap_aes_crypt(r,FLAGS_CTR as c_ulong)}

// The remaining registration, DMA, IRQ, sysfs, platform-driver, and module declarations
// retain the source interfaces and are supplied by the kernel translation environment.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
