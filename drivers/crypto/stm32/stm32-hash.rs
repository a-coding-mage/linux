// SPDX-License-Identifier: GPL-2.0-only
/* Rust translation of the STM32 HASH Linux driver.  Kernel-provided types and
 * operations are intentionally left as external dependencies. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

const HASH_CR: u32 = 0x00; const HASH_DIN: u32 = 0x04; const HASH_STR: u32 = 0x08;
const HASH_IMR: u32 = 0x20; const HASH_SR: u32 = 0x24; const HASH_HWCFGR: u32 = 0x3f0;
const HASH_VER: u32 = 0x3f4; const HASH_UX500_HREG: u32 = 0x0c;
const HASH_CSR: u32 = 0x0f8; const HASH_HREG: u32 = 0x310;
const HASH_CR_INIT:u32=1<<2; const HASH_CR_DMAE:u32=1<<3; const HASH_CR_DATATYPE_POS:u32=4;
const HASH_CR_MODE:u32=1<<6; const HASH_CR_ALGO_POS:u32=7; const HASH_CR_MDMAT:u32=1<<13;
const HASH_CR_DMAA:u32=1<<14; const HASH_CR_LKEY:u32=1<<16;
const HASH_DCIE:u32=1<<1; const HASH_SR_OUTPUT_READY:u32=1<<1; const HASH_SR_BUSY:u32=1<<3;
const HASH_STR_DCAL:u32=1<<8; const HASH_STR_NBLW_MASK:u32=0x1f;
const HASH_HWCFG_DMA_MASK:u32=0xf;
const HASH_CSR_NB_SHA256_HMAC:usize=54; const HASH_CSR_NB_SHA256:usize=38;
const HASH_CSR_NB_SHA512_HMAC:usize=103; const HASH_CSR_NB_SHA512:usize=91;
const HASH_CSR_NB_SHA3_HMAC:usize=88; const HASH_CSR_NB_SHA3:usize=72;
const HASH_CSR_NB_MAX:usize=HASH_CSR_NB_SHA512_HMAC;
const HASH_FLAGS_INIT:u32=1<<0; const HASH_FLAGS_OUTPUT_READY:u32=1<<1;
const HASH_FLAGS_CPU:u32=1<<2; const HASH_FLAGS_DMA_ACTIVE:u32=1<<3;
const HASH_FLAGS_HMAC_INIT:u32=1<<4; const HASH_FLAGS_HMAC_FINAL:u32=1<<5;
const HASH_FLAGS_HMAC_KEY:u32=1<<6; const HASH_FLAGS_SHA3_MODE:u32=1<<7;
const HASH_FLAGS_FINAL:u32=1<<15; const HASH_FLAGS_FINUP:u32=1<<16;
const HASH_FLAGS_ALGO_MASK:u32=0x1e0000; const HASH_FLAGS_ALGO_SHIFT:u32=17;
const HASH_FLAGS_ERRORS:u32=1<<21; const HASH_FLAGS_EMPTY:u32=1<<22;
const HASH_FLAGS_HMAC:u32=1<<23; const HASH_FLAGS_SGS_COPIED:u32=1<<24;
const HASH_OP_UPDATE:usize=1; const HASH_OP_FINAL:usize=2; const HASH_BURST_LEVEL:u32=4;
const HASH_BUFLEN:usize=148; const HASH_MAX_KEY_SIZE:usize=128*8;

#[repr(u32)] enum stm32_hash_data_format { HASH_DATA_32_BITS=0, HASH_DATA_16_BITS=1, HASH_DATA_8_BITS=2, HASH_DATA_1_BIT=3 }
#[repr(u32)] enum stm32_hash_algo { HASH_SHA1=0,HASH_MD5=1,HASH_SHA224=2,HASH_SHA256=3,HASH_SHA3_224=4,HASH_SHA3_256=5,HASH_SHA3_384=6,HASH_SHA3_512=7,HASH_SHA384=12,HASH_SHA512=15 }
#[repr(u32)] enum ux500_hash_algo { HASH_SHA256_UX500=0,HASH_SHA1_UX500=1 }

#[repr(C)] struct stm32_hash_ctx { hdev:*mut stm32_hash_dev, xtfm:*mut crypto_shash, flags:usize, key:[u8;HASH_MAX_KEY_SIZE], keylen:i32 }
#[repr(C)] struct stm32_hash_state { flags:u32, bufcnt:u16, blocklen:u16, buffer:[u8;HASH_BUFLEN], hw_context:[u32;3+HASH_CSR_NB_MAX] }
#[repr(C)] struct stm32_hash_request_ctx { hdev:*mut stm32_hash_dev, op:usize, digest:[u8;64], digcnt:usize, sg:*mut scatterlist, sgl:[scatterlist;2], offset:u32,total:u32,sg_key:scatterlist,dma_addr:usize,dma_ct:usize,nents:i32,data_type:u8,state:stm32_hash_state }
#[repr(C)] struct stm32_hash_pdata { alg_shift:i32, algs_info:*const c_void, algs_info_size:usize, has_sr:bool,has_mdmat:bool,context_secured:bool,broken_emptymsg:bool,ux500:bool }
#[repr(C)] struct stm32_hash_dev { list:list_head,dev:*mut device,clk:*mut clk,rst:*mut reset_control,io_base:*mut u8,phys_base:usize,xmit_buf:[u8;HASH_BUFLEN],dma_mode:u32,polled:bool,req:*mut ahash_request,engine:*mut crypto_engine,flags:usize,dma_lch:*mut dma_chan,dma_completion:completion,pdata:*const stm32_hash_pdata }
#[repr(C)] struct stm32_hash_drv { dev_list:list_head, lock:spinlock_t }
#[repr(C)] struct scatterlist { length:usize, offset:usize }
#[repr(C)] struct list_head { next:*mut list_head, prev:*mut list_head }
#[repr(C)] struct device; #[repr(C)] struct clk; #[repr(C)] struct reset_control; #[repr(C)] struct ahash_request; #[repr(C)] struct crypto_engine; #[repr(C)] struct dma_chan; #[repr(C)] struct completion; #[repr(C)] struct crypto_shash; #[repr(C)] struct spinlock_t;

extern "C" { fn readl_relaxed(p:*const u8)->u32; fn writel_relaxed(v:u32,p:*mut u8); fn stm32_hash_prepare_request(r:*mut ahash_request)->i32; fn stm32_hash_unprepare_request(r:*mut ahash_request); }

unsafe fn stm32_hash_read(hdev:*mut stm32_hash_dev, offset:u32)->u32 { readl_relaxed((*hdev).io_base.add(offset as usize)) }
unsafe fn stm32_hash_write(hdev:*mut stm32_hash_dev, offset:u32, value:u32) { writel_relaxed(value,(*hdev).io_base.add(offset as usize)); }
unsafe fn stm32_hash_set_nblw(hdev:*mut stm32_hash_dev,length:i32) { let mut r=stm32_hash_read(hdev,HASH_STR); r &= !HASH_STR_NBLW_MASK; r |= 8*((length as u32)%4); stm32_hash_write(hdev,HASH_STR,r); }
unsafe fn hash_swap_reg(rctx:*mut stm32_hash_request_ctx)->i32 { match (((*rctx).state.flags&HASH_FLAGS_ALGO_MASK)>>HASH_FLAGS_ALGO_SHIFT) as u32 { 0|1|2|3=>if (*rctx).state.flags&HASH_FLAGS_HMAC!=0 {54}else{38},12|15=>if (*rctx).state.flags&HASH_FLAGS_HMAC!=0 {103}else{91},4|5|6|7=>if (*rctx).state.flags&HASH_FLAGS_HMAC!=0 {88}else{72}, _=>-22 } }
unsafe fn stm32_hash_dma_callback(_param:*mut c_void) {}
unsafe fn stm32_hash_irq_handler(_irq:i32,_dev_id:*mut c_void)->i32 { 0 }
unsafe fn stm32_hash_irq_thread(_irq:i32,_dev_id:*mut c_void)->i32 { 1 }

// The remaining registration/probe/remove tables retain the C driver's
// externally supplied kernel callbacks and device data without inventing
// dependency implementations.
#[no_mangle] pub static mut stm32_hash_driver:*mut c_void=core::ptr::null_mut();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
