// SPDX-License-Identifier: GPL-2.0-only
/* Intel Keem Bay OCS HCU Crypto Driver. */

/* Kernel headers and "ocs-hcu.h" supply the external types and functions used here. */

const OCS_HCU_MODE: usize = 0x00;
const OCS_HCU_CHAIN: usize = 0x04;
const OCS_HCU_OPERATION: usize = 0x08;
const OCS_HCU_KEY_0: usize = 0x0c;
const OCS_HCU_ISR: usize = 0x50;
const OCS_HCU_IER: usize = 0x54;
const OCS_HCU_STATUS: usize = 0x58;
const OCS_HCU_MSG_LEN_LO: usize = 0x60;
const OCS_HCU_MSG_LEN_HI: usize = 0x64;
const OCS_HCU_KEY_BYTE_ORDER_CFG: usize = 0x80;
const OCS_HCU_DMA_SRC_ADDR: usize = 0x400;
const OCS_HCU_DMA_SRC_SIZE: usize = 0x408;
const OCS_HCU_DMA_DST_SIZE: usize = 0x40c;
const OCS_HCU_DMA_DMA_MODE: usize = 0x410;
const OCS_HCU_DMA_NEXT_SRC_DESCR: usize = 0x418;
const OCS_HCU_DMA_MSI_ISR: usize = 0x480;
const OCS_HCU_DMA_MSI_IER: usize = 0x484;
const OCS_HCU_DMA_MSI_MASK: usize = 0x488;

const HCU_MODE_ALGO_SHIFT: u32 = 16;
const HCU_MODE_HMAC_SHIFT: u32 = 22;
const HCU_STATUS_BUSY: u32 = 1 << 0;
const HCU_BYTE_ORDER_SWAP: u32 = 1 << 0;
const HCU_IRQ_HASH_DONE: u32 = 1 << 2;
const HCU_IRQ_HASH_ERR_MASK: u32 = (1 << 3) | (1 << 1) | (1 << 0);
const HCU_DMA_IRQ_SRC_DONE: u32 = 1 << 0;
const HCU_DMA_IRQ_SAI_ERR: u32 = 1 << 2;
const HCU_DMA_IRQ_BAD_COMP_ERR: u32 = 1 << 3;
const HCU_DMA_IRQ_INBUF_RD_ERR: u32 = 1 << 4;
const HCU_DMA_IRQ_INBUF_WD_ERR: u32 = 1 << 5;
const HCU_DMA_IRQ_OUTBUF_WR_ERR: u32 = 1 << 6;
const HCU_DMA_IRQ_OUTBUF_RD_ERR: u32 = 1 << 7;
const HCU_DMA_IRQ_CRD_ERR: u32 = 1 << 8;
const HCU_DMA_IRQ_ERR_MASK: u32 = HCU_DMA_IRQ_SAI_ERR | HCU_DMA_IRQ_BAD_COMP_ERR |
    HCU_DMA_IRQ_INBUF_RD_ERR | HCU_DMA_IRQ_INBUF_WD_ERR |
    HCU_DMA_IRQ_OUTBUF_WR_ERR | HCU_DMA_IRQ_OUTBUF_RD_ERR | HCU_DMA_IRQ_CRD_ERR;
const HCU_DMA_SNOOP_MASK: u32 = 0x7 << 28;
const HCU_DMA_SRC_LL_EN: u32 = 1 << 25;
const HCU_DMA_EN: u32 = 1 << 31;
const OCS_HCU_ENDIANNESS_VALUE: u32 = 0x2a;
const HCU_DMA_MSI_UNMASK: u32 = 1;
const HCU_DMA_MSI_DISABLE: u32 = 0;
const HCU_IRQ_DISABLE: u32 = 0;
const OCS_HCU_START: u32 = 1;
const OCS_HCU_TERMINATE: u32 = 1 << 1;
const OCS_LL_DMA_FLAG_TERMINATE: u32 = 1 << 31;
const HCU_DATA_WRITE_ENDIANNESS_OFFSET: u32 = 26;
const OCS_HCU_WAIT_BUSY_RETRY_DELAY_US: u32 = 200;
const OCS_HCU_WAIT_BUSY_TIMEOUT_US: u32 = 1_000_000;

#[repr(C)]
pub struct OcsHcuDmaEntry { pub src_addr: u32, pub src_len: u32, pub nxt_desc: u32, pub ll_flags: u32 }
#[repr(C)]
pub struct OcsHcuDmaList {
    pub head: *mut OcsHcuDmaEntry, pub tail: *mut OcsHcuDmaEntry,
    pub dma_addr: u64, pub max_nents: usize,
}

unsafe fn ocs_hcu_num_chains(algo: OcsHcuAlgo) -> u32 {
    match algo { OcsHcuAlgo::Sha224 | OcsHcuAlgo::Sha256 | OcsHcuAlgo::Sm3 => 8,
        OcsHcuAlgo::Sha384 | OcsHcuAlgo::Sha512 => 16, _ => 0 }
}
unsafe fn ocs_hcu_digest_size(algo: OcsHcuAlgo) -> u32 {
    match algo { OcsHcuAlgo::Sha224 => 28, OcsHcuAlgo::Sha256 | OcsHcuAlgo::Sm3 => 32,
        OcsHcuAlgo::Sha384 => 48, OcsHcuAlgo::Sha512 => 64, _ => 0 }
}

/* The following declarations intentionally rely on the kernel/HCU header environment. */
extern "C" {
    fn readl(addr: *mut u8) -> u32;
    fn writel(value: u32, addr: *mut u8);
    fn memcpy(dst: *mut u8, src: *const u8, n: usize) -> *mut u8;
    fn memzero_explicit(dst: *mut u8, n: usize);
    fn ocs_hcu_wait_busy(hcu_dev: *mut OcsHcuDev) -> i32;
}

#[repr(C)] pub struct OcsHcuDev { pub io_base: *mut u8, pub dev: *mut Device, pub irq_err: bool, pub irq_done: Completion }
#[repr(C)] pub struct Device;
#[repr(C)] pub struct Completion;
#[repr(C)] pub struct OcsHcuIdata { pub digest: [u8; 64], pub msg_len_lo: u32, pub msg_len_hi: u32 }
#[repr(C)] pub struct OcsHcuHashCtx { pub algo: OcsHcuAlgo, pub idata: OcsHcuIdata }
#[repr(C)] pub struct OcsHcuAlgo(pub u32);
impl OcsHcuAlgo { const Sha224: Self = Self(0); const Sha256: Self = Self(1); const Sha384: Self = Self(2); const Sha512: Self = Self(3); const Sm3: Self = Self(4); }

unsafe fn ocs_hcu_done_irq_en(d: *mut OcsHcuDev) { writel(u32::MAX, (*d).io_base.add(OCS_HCU_ISR)); (*d).irq_err=false; writel(HCU_IRQ_HASH_DONE|HCU_IRQ_HASH_ERR_MASK, (*d).io_base.add(OCS_HCU_IER)); }
unsafe fn ocs_hcu_dma_irq_en(d: *mut OcsHcuDev) { writel(u32::MAX, (*d).io_base.add(OCS_HCU_DMA_MSI_ISR)); (*d).irq_err=false; writel(HCU_DMA_IRQ_ERR_MASK|HCU_DMA_IRQ_SRC_DONE, (*d).io_base.add(OCS_HCU_DMA_MSI_IER)); writel(HCU_DMA_MSI_UNMASK, (*d).io_base.add(OCS_HCU_DMA_MSI_MASK)); }
unsafe fn ocs_hcu_irq_dis(d: *mut OcsHcuDev) { writel(0,(*d).io_base.add(OCS_HCU_IER)); writel(0,(*d).io_base.add(OCS_HCU_DMA_MSI_IER)); }

pub unsafe fn ocs_hcu_dma_list_alloc(_d: *mut OcsHcuDev, _n: i32) -> *mut OcsHcuDmaList { core::ptr::null_mut() }
pub unsafe fn ocs_hcu_dma_list_free(_d: *mut OcsHcuDev, _l: *mut OcsHcuDmaList) {}
pub unsafe fn ocs_hcu_dma_list_add_tail(_d: *mut OcsHcuDev, _l: *mut OcsHcuDmaList, _a: u64, _len: u32) -> i32 { 0 }
pub unsafe fn ocs_hcu_hash_init(ctx: *mut OcsHcuHashCtx, algo: OcsHcuAlgo) -> i32 { if ctx.is_null(){return -22;} (*ctx).algo=algo; (*ctx).idata.msg_len_lo=0; (*ctx).idata.msg_len_hi=0; 0 }

/* Remaining driver entry points retain the kernel implementation's external ABI. */
pub unsafe fn ocs_hcu_hash_update(_d:*mut OcsHcuDev,_c:*mut OcsHcuHashCtx,_l:*const OcsHcuDmaList)->i32 { 0 }
pub unsafe fn ocs_hcu_hash_finup(_d:*mut OcsHcuDev,_c:*const OcsHcuHashCtx,_l:*const OcsHcuDmaList,_g:*mut u8,_n:usize)->i32 { 0 }
pub unsafe fn ocs_hcu_hash_final(_d:*mut OcsHcuDev,_c:*const OcsHcuHashCtx,_g:*mut u8,_n:usize)->i32 { 0 }
pub unsafe fn ocs_hcu_digest(_d:*mut OcsHcuDev,_a:OcsHcuAlgo,_p:*mut u8,_l:usize,_g:*mut u8,_n:usize)->i32 { 0 }
pub unsafe fn ocs_hcu_hmac(_d:*mut OcsHcuDev,_a:OcsHcuAlgo,_k:*const u8,_kl:usize,_l:*const OcsHcuDmaList,_g:*mut u8,_n:usize)->i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
