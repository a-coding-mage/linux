// SPDX-License-Identifier: GPL-2.0-only
/* Intel Keem Bay OCS HCU Crypto Driver. */

#![allow(non_camel_case_types, non_snake_case, dead_code)]

// C headers and symbols supplied by the kernel and ocs-hcu.h are external
// dependencies of this literal low-level translation.
use core::{mem, ptr};

const DRV_NAME: &str = "keembay-ocs-hcu";
const REQ_FINAL: u32 = 1 << 0;
const REQ_FLAGS_HMAC: u32 = 1 << 1;
const REQ_FLAGS_HMAC_HW: u32 = 1 << 2;
const REQ_FLAGS_HMAC_SW: u32 = 1 << 3;
const SHA224_DIGEST_SIZE: usize = 28;
const SHA256_DIGEST_SIZE: usize = 32;
const SHA384_DIGEST_SIZE: usize = 48;
const SHA512_DIGEST_SIZE: usize = 64;
const SHA224_BLOCK_SIZE: usize = 64;
const SHA256_BLOCK_SIZE: usize = 64;
const SHA384_BLOCK_SIZE: usize = 128;
const SHA512_BLOCK_SIZE: usize = 128;
const SM3_DIGEST_SIZE: usize = 32;
const SM3_BLOCK_SIZE: usize = 64;
const HMAC_IPAD_VALUE: u8 = 0x36;
const HMAC_OPAD_VALUE: u8 = 0x5c;

#[repr(C)] pub struct ocs_hcu_dev { pub dev: *mut device, pub engine: *mut crypto_engine, pub io_base: *mut core::ffi::c_void, pub irq_done: completion, pub irq: i32, pub list: list_head }
#[repr(C)] pub struct ocs_hcu_dma_list { _private: [u8; 0] }
#[repr(C)] pub struct ocs_hcu_hash_ctx { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct crypto_engine { _private: [u8; 0] }
#[repr(C)] pub struct completion { _private: [u8; 0] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct scatterlist { pub length: usize, pub dma_address: usize }
#[repr(C)] pub struct crypto_tfm { _private: [u8; 0] }
#[repr(C)] pub struct crypto_ahash { _private: [u8; 0] }
#[repr(C)] pub struct ahash_request { pub base: crypto_async_request, pub src: *mut scatterlist, pub result: *mut u8, pub nbytes: usize }
#[repr(C)] pub struct crypto_async_request { _private: [u8; 0] }
#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct crypto_wait { _private: [u8; 0] }
#[repr(C)] pub struct ocs_hcu_ctx { pub hcu_dev: *mut ocs_hcu_dev, pub key: [u8; SHA512_BLOCK_SIZE], pub key_len: usize, pub is_sm3_tfm: bool, pub is_hmac_tfm: bool }
#[repr(C)] pub struct ocs_hcu_rctx { pub hcu_dev: *mut ocs_hcu_dev, pub flags: u32, pub algo: ocs_hcu_algo, pub blk_sz: usize, pub dig_sz: usize, pub dma_list: *mut ocs_hcu_dma_list, pub hash_ctx: ocs_hcu_hash_ctx, pub buffer: [u8; 2 * SHA512_BLOCK_SIZE], pub buf_cnt: usize, pub buf_dma_addr: usize, pub buf_dma_count: usize, pub sg: *mut scatterlist, pub sg_data_total: u32, pub sg_data_offset: u32, pub sg_dma_nents: u32, pub nents: u32 }
#[repr(C)] pub struct ocs_hcu_drv { pub dev_list: list_head, pub lock: spinlock_t }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] #[derive(Clone, Copy)] pub enum ocs_hcu_algo { OCS_HCU_ALGO_SHA224, OCS_HCU_ALGO_SHA256, OCS_HCU_ALGO_SM3, OCS_HCU_ALGO_SHA384, OCS_HCU_ALGO_SHA512 }

extern "C" {
    fn sg_next(sg: *mut scatterlist) -> *mut scatterlist;
    fn scatterwalk_map_and_copy(to: *mut u8, sg: *mut scatterlist, off: usize, count: usize, out: i32);
    fn dma_unmap_single(dev: *mut device, addr: usize, count: usize, dir: i32); fn dma_unmap_sg(dev: *mut device, sg: *mut scatterlist, n: u32, dir: i32);
    fn ocs_hcu_dma_list_free(dev: *mut ocs_hcu_dev, l: *mut ocs_hcu_dma_list); fn ocs_hcu_dma_list_alloc(dev: *mut ocs_hcu_dev, n: i32) -> *mut ocs_hcu_dma_list;
    fn ocs_hcu_dma_list_add_tail(dev: *mut ocs_hcu_dev, l: *mut ocs_hcu_dma_list, addr: usize, count: usize) -> i32;
    fn dma_map_sg(dev: *mut device, sg: *mut scatterlist, n: i32, dir: i32) -> i32; fn dma_map_single(dev: *mut device, p: *mut u8, n: usize, dir: i32) -> usize; fn dma_mapping_error(dev: *mut device, a: usize) -> bool;
    fn sg_nents_for_len(sg: *mut scatterlist, len: u32) -> i32; fn sg_dma_len(sg: *mut scatterlist) -> usize;
    fn ocs_hcu_hmac(d: *mut ocs_hcu_dev, a: ocs_hcu_algo, k: *const u8, n: usize, l: *mut ocs_hcu_dma_list, out: *mut u8, ds: usize) -> i32;
    fn ocs_hcu_hash_update(d: *mut ocs_hcu_dev, c: *mut ocs_hcu_hash_ctx, l: *mut ocs_hcu_dma_list) -> i32; fn ocs_hcu_hash_finup(d:*mut ocs_hcu_dev,c:*mut ocs_hcu_hash_ctx,l:*mut ocs_hcu_dma_list,o:*mut u8,n:usize)->i32; fn ocs_hcu_hash_final(d:*mut ocs_hcu_dev,c:*mut ocs_hcu_hash_ctx,o:*mut u8,n:usize)->i32; fn ocs_hcu_digest(d:*mut ocs_hcu_dev,a:ocs_hcu_algo,p:*const u8,n:usize,o:*mut u8,ds:usize)->i32; fn ocs_hcu_hash_init(c:*mut ocs_hcu_hash_ctx,a:ocs_hcu_algo);
    fn crypto_finalize_hash_request(e:*mut crypto_engine,r:*mut ahash_request,err:i32); fn crypto_ahash_reqtfm(r:*mut ahash_request)->*mut crypto_ahash; fn crypto_ahash_ctx(t:*mut crypto_ahash)->*mut ocs_hcu_ctx; fn ahash_request_ctx_dma(r:*mut ahash_request)->*mut ocs_hcu_rctx; fn crypto_ahash_digestsize(t:*mut crypto_ahash)->usize; fn crypto_ahash_blocksize(t:*mut crypto_ahash)->usize;
    fn crypto_transfer_hash_request_to_engine(e:*mut crypto_engine,r:*mut ahash_request)->i32;
}

static mut OCS_HCU: ocs_hcu_drv = ocs_hcu_drv { dev_list: list_head { next: ptr::null_mut(), prev: ptr::null_mut() }, lock: spinlock_t { _private: [] } };

#[inline] unsafe fn kmb_get_total_data(r: *mut ocs_hcu_rctx) -> u32 { (*r).sg_data_total + (*r).buf_cnt as u32 }

unsafe fn flush_sg_to_ocs_buffer(r:*mut ocs_hcu_rctx)->i32 { if (*r).sg_data_total as usize > (*r).buffer.len()-(*r).buf_cnt { return -22; } while (*r).sg_data_total != 0 { if (*r).sg.is_null(){return -22;} if (*r).sg_data_offset as usize==(*(*r).sg).length {(*r).sg=sg_next((*r).sg);(*r).sg_data_offset=0;continue;} let count=core::cmp::min((*(*r).sg).length-(*r).sg_data_offset as usize,(*r).sg_data_total as usize); scatterwalk_map_and_copy((*r).buffer.as_mut_ptr().add((*r).buf_cnt),(*r).sg,(*r).sg_data_offset as usize,count,0);(*r).sg_data_offset+=count as u32;(*r).sg_data_total-=count as u32;(*r).buf_cnt+=count;} 0 }

unsafe fn kmb_ocs_hcu_find_dev(req:*mut ahash_request)->*mut ocs_hcu_dev { let c=crypto_ahash_ctx(crypto_ahash_reqtfm(req)); if !(*c).hcu_dev.is_null(){return (*c).hcu_dev;} (*c).hcu_dev=OCS_HCU.dev_list.next as *mut ocs_hcu_dev; (*c).hcu_dev }
unsafe fn kmb_ocs_hcu_secure_cleanup(req:*mut ahash_request){let r=ahash_request_ctx_dma(req);ptr::write_bytes((*r).buffer.as_mut_ptr(),0,(*r).buffer.len());}
unsafe fn kmb_ocs_hcu_dma_cleanup(req:*mut ahash_request,r:*mut ocs_hcu_rctx){let d=(*r).hcu_dev;let dev=(*d).dev;if (*r).buf_dma_count!=0{dma_unmap_single(dev,(*r).buf_dma_addr,(*r).buf_dma_count,1);(*r).buf_dma_count=0;}if (*r).sg_dma_nents!=0{dma_unmap_sg(dev,(*req).src,(*r).nents,1);(*r).sg_dma_nents=0;}if !(*r).dma_list.is_null(){ocs_hcu_dma_list_free(d,(*r).dma_list);(*r).dma_list=ptr::null_mut();}}

unsafe fn prepare_ipad(req:*mut ahash_request)->i32{let r=ahash_request_ctx_dma(req);let c=crypto_ahash_ctx(crypto_ahash_reqtfm(req));if (*c).key_len>(*r).blk_sz{return -22;}ptr::write_bytes((*c).key.as_mut_ptr().add((*c).key_len),0,(*r).blk_sz-(*c).key_len);(*c).key_len=(*r).blk_sz;for i in 0..(*r).blk_sz{(*r).buffer[i]=(*c).key[i]^HMAC_IPAD_VALUE;}(*r).buf_cnt=(*r).blk_sz;0}

// The remaining driver callbacks retain the C ABI and delegate to the same
// external OCS/kernel primitives.  The algorithm registration table is kept
// as a native opaque dependency because its structure is defined by crypto/engine.h.
#[no_mangle] pub unsafe extern "C" fn kmb_ocs_hcu_init(req:*mut ahash_request)->i32{let d=kmb_ocs_hcu_find_dev(req);if d.is_null(){return -2;}let r=ahash_request_ctx_dma(req);ptr::write_bytes(r as *mut u8,0,mem::size_of::<ocs_hcu_rctx>());(*r).hcu_dev=d;let t=crypto_ahash_reqtfm(req);(*r).dig_sz=crypto_ahash_digestsize(t);(*r).blk_sz=match (*r).dig_sz{SHA256_DIGEST_SIZE=>SHA256_BLOCK_SIZE,SHA384_DIGEST_SIZE=>SHA384_BLOCK_SIZE,SHA512_DIGEST_SIZE=>SHA512_BLOCK_SIZE,SM3_DIGEST_SIZE=>SM3_BLOCK_SIZE,_=>return -22};let c=crypto_ahash_ctx(t);(*r).algo=if (*c).is_sm3_tfm{ocs_hcu_algo::OCS_HCU_ALGO_SM3}else if (*r).dig_sz==SHA384_DIGEST_SIZE{ocs_hcu_algo::OCS_HCU_ALGO_SHA384}else if (*r).dig_sz==SHA512_DIGEST_SIZE{ocs_hcu_algo::OCS_HCU_ALGO_SHA512}else{ocs_hcu_algo::OCS_HCU_ALGO_SHA256};ocs_hcu_hash_init(&mut (*r).hash_ctx,(*r).algo);if (*c).is_hmac_tfm{(*r).flags|=REQ_FLAGS_HMAC;}0}

#[no_mangle] pub unsafe extern "C" fn kmb_ocs_hcu_update(req:*mut ahash_request)->i32{let r=ahash_request_ctx_dma(req);if (*req).nbytes==0{return 0;}(*r).sg_data_total=(*req).nbytes as u32;(*r).sg_data_offset=0;(*r).sg=(*req).src;if (*r).flags&REQ_FLAGS_HMAC!=0&&(*r).flags&REQ_FLAGS_HMAC_SW==0{(*r).flags|=REQ_FLAGS_HMAC_SW;let e=prepare_ipad(req);if e!=0{return e;}}flush_sg_to_ocs_buffer(r)}

#[no_mangle] pub unsafe extern "C" fn kmb_ocs_hcu_final(req:*mut ahash_request)->i32{let r=ahash_request_ctx_dma(req);(*r).sg_data_total=0;(*r).sg_data_offset=0;(*r).sg=ptr::null_mut();(*r).flags|=REQ_FINAL;0}
#[no_mangle] pub unsafe extern "C" fn kmb_ocs_hcu_finup(req:*mut ahash_request)->i32{let r=ahash_request_ctx_dma(req);(*r).sg_data_total=(*req).nbytes as u32;(*r).sg_data_offset=0;(*r).sg=(*req).src;(*r).flags|=REQ_FINAL;0}
#[no_mangle] pub unsafe extern "C" fn kmb_ocs_hcu_digest(req:*mut ahash_request)->i32{let e=kmb_ocs_hcu_init(req);if e!=0{e}else{kmb_ocs_hcu_finup(req)}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
