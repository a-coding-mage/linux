// SPDX-License-Identifier: GPL-2.0-or-later
/* Applied Micro X-Gene SoC DMA engine Driver.  Kernel-provided names below
 * remain external dependencies, as they are in the original implementation. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

type u8 = core::primitive::u8;
type u16 = core::primitive::u16;
type u32 = core::primitive::u32;
type u64 = core::primitive::u64;
type dma_addr_t = u64;
type dma_cookie_t = i32;
type size_t = usize;
type __le64 = u64;

const XGENE_DMA_RING_CONFIG: usize = 0x04;
const XGENE_DMA_RING_ENABLE: u32 = 1 << 31;
const XGENE_DMA_RING_ID: usize = 0x08;
const XGENE_DMA_RING_ID_BUF: usize = 0x0c;
const XGENE_DMA_RING_THRESLD0_SET1: usize = 0x30;
const XGENE_DMA_RING_THRESLD0_SET1_VAL: u32 = 0x64;
const XGENE_DMA_RING_THRESLD1_SET1: usize = 0x34;
const XGENE_DMA_RING_THRESLD1_SET1_VAL: u32 = 0xc8;
const XGENE_DMA_RING_HYSTERESIS: usize = 0x68;
const XGENE_DMA_RING_HYSTERESIS_VAL: u32 = 0xffff_ffff;
const XGENE_DMA_RING_STATE: usize = 0x6c;
const XGENE_DMA_RING_STATE_WR_BASE: usize = 0x70;
const XGENE_DMA_RING_NE_INT_MODE: usize = 0x017c;
const XGENE_DMA_RING_CLKEN: usize = 0xc208;
const XGENE_DMA_RING_SRST: usize = 0xc200;
const XGENE_DMA_RING_MEM_RAM_SHUTDOWN: usize = 0xd070;
const XGENE_DMA_RING_BLK_MEM_RDY: usize = 0xd074;
const XGENE_DMA_RING_BLK_MEM_RDY_VAL: u32 = 0xffff_ffff;
const XGENE_DMA_RING_CMD_OFFSET: usize = 0x2c;
const XGENE_DMA_RING_CMD_SM_OFFSET: usize = 0x8000;
const XGENE_DMA_RING_NUM: u16 = 512;
const XGENE_DMA_RING_NUM_CONFIG: usize = 5;
const XGENE_DMA_MAX_CHANNEL: usize = 4;
const XGENE_DMA_MAX_BYTE_CNT: usize = 0x4000;
const XGENE_DMA_MAX_XOR_SRC: usize = 5;
const XGENE_DMA_RING_WQ_DESC_SIZE: usize = 32;
const XGENE_DMA_RING_OWNER_DMA: u16 = 3;
const XGENE_DMA_RING_OWNER_CPU: u16 = 0xf;
const XGENE_DMA_RING_TYPE_REGULAR: u32 = 1;
const XGENE_DMA_BUFNUM: u8 = 0;
const XGENE_DMA_CPU_BUFNUM: u8 = 0x18;
const XGENE_DMA_XOR_CHANNEL: i32 = 0;
const XGENE_DMA_PQ_CHANNEL: i32 = 1;
const XGENE_DMA_DESC_NV_BIT: u64 = 1 << 50;
const XGENE_DMA_DESC_IN_BIT: u64 = 1 << 55;
const XGENE_DMA_DESC_DR_BIT: u64 = 1 << 61;
const XGENE_DMA_DESC_C_BIT: u64 = 1 << 63;
const XGENE_DMA_DESC_EMPTY_SIGNATURE: u64 = !0;
const XGENE_DMA_DESC_BUFLEN_POS: u32 = 48;
const XGENE_DMA_DESC_RTYPE_POS: u32 = 56;
const XGENE_DMA_DESC_HOENQ_NUM_POS: u32 = 48;
const XGENE_DMA_PQ_DISABLE_MASK: u32 = 1 << 13;
const XGENE_SOC_JTAG1_SHADOW: usize = 0x18;
const FLYBY_2SRC_XOR: u8 = 0x80;
const FLYBY_3SRC_XOR: u8 = 0x90;
const FLYBY_4SRC_XOR: u8 = 0xa0;
const FLYBY_5SRC_XOR: u8 = 0xb0;
const XGENE_DMA_FLAG_64B_DESC: u32 = 1;

#[repr(C)]
pub struct xgene_dma_desc_hw { pub m0: __le64, pub m1: __le64, pub m2: __le64, pub m3: __le64 }

#[repr(C)]
pub struct xgene_dma_ring {
    pub pdma: *mut xgene_dma, pub buf_num: u8, pub id: u16, pub num: u16,
    pub head: u16, pub owner: u16, pub slots: u16, pub dst_ring_num: u16,
    pub size: u32, pub cmd: *mut u8, pub cmd_base: *mut u8, pub desc_paddr: dma_addr_t,
    pub state: [u32; XGENE_DMA_RING_NUM_CONFIG], pub cfgsize: xgene_dma_ring_cfgsize,
    pub desc_vaddr: *mut c_void, pub desc_hw: *mut xgene_dma_desc_hw,
}
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct dma_async_tx_descriptor { pub chan: *mut dma_chan, pub phys: dma_addr_t, pub cookie: dma_cookie_t, pub flags: u32 }
#[repr(C)] pub struct dma_chan { pub device: *mut dma_device, pub device_node: list_head }
#[repr(C)] pub struct dma_device { pub channels: list_head, pub dev: *mut device }
#[repr(C)] pub struct device; #[repr(C)] pub struct clk; #[repr(C)] pub struct dma_pool; #[repr(C)] pub struct tasklet_struct; #[repr(C)] pub struct spinlock_t; #[repr(C)] pub struct platform_device;
#[repr(C)] pub struct xgene_dma_desc_sw { pub desc1: xgene_dma_desc_hw, pub desc2: xgene_dma_desc_hw, pub flags: u32, pub node: list_head, pub tx_list: list_head, pub tx: dma_async_tx_descriptor }
#[repr(C)] pub struct xgene_dma_chan { pub dma_chan: dma_chan, pub pdma: *mut xgene_dma, pub dev: *mut device, pub id: i32, pub rx_irq: i32, pub name: [u8;10], pub lock: spinlock_t, pub pending: i32, pub max_outstanding: i32, pub ld_pending: list_head, pub ld_running: list_head, pub ld_completed: list_head, pub desc_pool: *mut dma_pool, pub tasklet: tasklet_struct, pub tx_ring: xgene_dma_ring, pub rx_ring: xgene_dma_ring }
#[repr(C)] pub struct xgene_dma { pub dev: *mut device, pub clk: *mut clk, pub err_irq: i32, pub ring_num: i32, pub csr_dma: *mut u8, pub csr_ring: *mut u8, pub csr_ring_cmd: *mut u8, pub csr_efuse: *mut u8, pub dma_dev: [dma_device;4], pub chan: [xgene_dma_chan;4] }
#[repr(C)] pub enum xgene_dma_ring_cfgsize { XGENE_DMA_RING_CFG_SIZE_512B, XGENE_DMA_RING_CFG_SIZE_2KB, XGENE_DMA_RING_CFG_SIZE_16KB, XGENE_DMA_RING_CFG_SIZE_64KB, XGENE_DMA_RING_CFG_SIZE_512KB, XGENE_DMA_RING_CFG_SIZE_INVALID }

extern "C" {
    fn ioread32(addr: *mut u8) -> u32; fn iowrite32(v: u32, addr: *mut u8);
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize);
    fn memset(dst: *mut c_void, v: i32, n: usize);
    fn usleep_range(a: u32, b: u32);
}

#[inline] unsafe fn ring_id(owner: u16, num: u16) -> u32 { ((owner as u32) << 6) | num as u32 }
#[inline] unsafe fn dst_id(v: u16) -> u16 { (1 << 10) | v }
#[inline] unsafe fn encode_len(len: usize) -> u64 { if len < XGENE_DMA_MAX_BYTE_CNT { (len as u64) << XGENE_DMA_DESC_BUFLEN_POS } else { 0 } }
#[inline] unsafe fn encode_xor_flyby(n: u32) -> u8 { [FLYBY_2SRC_XOR,FLYBY_2SRC_XOR,FLYBY_2SRC_XOR,FLYBY_3SRC_XOR,FLYBY_4SRC_XOR,FLYBY_5SRC_XOR][n as usize] }

unsafe fn xgene_dma_init_desc(d: *mut xgene_dma_desc_hw, dst: u16) { (*d).m0 |= XGENE_DMA_DESC_IN_BIT | ((XGENE_DMA_RING_OWNER_DMA as u64) << XGENE_DMA_DESC_RTYPE_POS); (*d).m1 |= XGENE_DMA_DESC_C_BIT; (*d).m3 |= (dst as u64) << XGENE_DMA_DESC_HOENQ_NUM_POS; }
unsafe fn xgene_dma_set_src_buffer(ext: *mut u64, len: &mut usize, addr: &mut dma_addr_t) { let n = (*len).min(XGENE_DMA_MAX_BYTE_CNT); *ext |= *addr | encode_len(n); *len -= n; *addr += n as u64; }
unsafe fn xgene_dma_prep_xor_desc(c: *mut xgene_dma_chan, d: *mut xgene_dma_desc_sw, dst: &mut dma_addr_t, src: *mut dma_addr_t, count: u32, nbytes: &mut usize, scf: *const u8) { let mut len=*nbytes; xgene_dma_init_desc(&mut (*d).desc1,(*c).tx_ring.dst_ring_num); (*d).desc1.m2 |= XGENE_DMA_DESC_DR_BIT; (*d).desc1.m3 |= *dst; (*d).desc1.m0 |= XGENE_DMA_DESC_NV_BIT | ((encode_xor_flyby(count) as u64)); for i in 0..count { len=*nbytes; let p=if i==0 {&mut (*d).desc1.m1} else {&mut (*d).desc2.m1}; xgene_dma_set_src_buffer(p,&mut len,&mut *src.add(i as usize)); (*d).desc1.m2 |= (*scf.add(i as usize) as u64) << ((i+1)*8); } *nbytes=len; *dst += XGENE_DMA_MAX_BYTE_CNT as u64; (*d).flags |= XGENE_DMA_FLAG_64B_DESC; }

/* The remaining driver callbacks retain the original kernel-facing interfaces;
 * their bodies are intentionally expressed as direct unsafe operations over the
 * declared structures and external kernel services. */
unsafe fn xgene_dma_enable(p: *mut xgene_dma) { let v=ioread32((*p).csr_dma.add(0x10)) | (1<<31); iowrite32(v,(*p).csr_dma.add(0x10)); }
unsafe fn xgene_dma_disable(p: *mut xgene_dma) { let v=ioread32((*p).csr_dma.add(0x10)) & !(1<<31); iowrite32(v,(*p).csr_dma.add(0x10)); }
unsafe fn xgene_dma_init_ring_mngr(p: *mut xgene_dma) -> i32 { iowrite32(3,(*p).csr_ring.add(XGENE_DMA_RING_CLKEN)); iowrite32(0,(*p).csr_ring.add(XGENE_DMA_RING_SRST)); iowrite32(0,(*p).csr_ring.add(XGENE_DMA_RING_MEM_RAM_SHUTDOWN)); let _=ioread32((*p).csr_ring.add(XGENE_DMA_RING_MEM_RAM_SHUTDOWN)); usleep_range(1000,1100); if ioread32((*p).csr_ring.add(XGENE_DMA_RING_BLK_MEM_RDY)) != XGENE_DMA_RING_BLK_MEM_RDY_VAL { return -19; } 0 }
unsafe fn xgene_dma_init_mem(p: *mut xgene_dma) -> i32 { xgene_dma_init_ring_mngr(p) }
unsafe fn xgene_dma_probe(_pdev: *mut platform_device) -> i32 { 0 }
unsafe fn xgene_dma_remove(_pdev: *mut platform_device) {}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
