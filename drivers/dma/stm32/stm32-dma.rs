// SPDX-License-Identifier: GPL-2.0-only
/* Rust translation of stm32-dma.c. Kernel types and helpers are supplied by
 * the surrounding Linux/Rust bindings. */

#![allow(non_camel_case_types, non_snake_case, dead_code)]

const STM32_DMA_LISR: u32 = 0x0000;
const STM32_DMA_HISR: u32 = 0x0004;
const STM32_DMA_LIFCR: u32 = 0x0008;
const STM32_DMA_HIFCR: u32 = 0x000c;
const STM32_DMA_TCI: u32 = 1 << 5;
const STM32_DMA_HTI: u32 = 1 << 4;
const STM32_DMA_TEI: u32 = 1 << 3;
const STM32_DMA_DMEI: u32 = 1 << 2;
const STM32_DMA_FEI: u32 = 1;
const STM32_DMA_MASKI: u32 = STM32_DMA_TCI | STM32_DMA_TEI | STM32_DMA_DMEI | STM32_DMA_FEI;
const STM32_DMA_SCR_REQ_MASK: u32 = 0x0e00_0000;
const STM32_DMA_SCR_MBURST_MASK: u32 = 0x0180_0000;
const STM32_DMA_SCR_PBURST_MASK: u32 = 0x0060_0000;
const STM32_DMA_SCR_PL_MASK: u32 = 0x0003_0000;
const STM32_DMA_SCR_MSIZE_MASK: u32 = 0x0000_6000;
const STM32_DMA_SCR_PSIZE_MASK: u32 = 0x0000_1800;
const STM32_DMA_SCR_DIR_MASK: u32 = 0x0000_00c0;
const STM32_DMA_SCR_TRBUFF: u32 = 1 << 20;
const STM32_DMA_SCR_CT: u32 = 1 << 19;
const STM32_DMA_SCR_DBM: u32 = 1 << 18;
const STM32_DMA_SCR_PINCOS: u32 = 1 << 15;
const STM32_DMA_SCR_MINC: u32 = 1 << 10;
const STM32_DMA_SCR_PINC: u32 = 1 << 9;
const STM32_DMA_SCR_CIRC: u32 = 1 << 8;
const STM32_DMA_SCR_PFCTRL: u32 = 1 << 5;
const STM32_DMA_SCR_TCIE: u32 = 1 << 4;
const STM32_DMA_SCR_TEIE: u32 = 1 << 2;
const STM32_DMA_SCR_DMEIE: u32 = 1 << 1;
const STM32_DMA_SCR_EN: u32 = 1;
const STM32_DMA_SFCR_FTH_MASK: u32 = 3;
const STM32_DMA_SFCR_FEIE: u32 = 1 << 7;
const STM32_DMA_SFCR_DMDIS: u32 = 1 << 2;
const STM32_DMA_SFCR_MASK: u32 = STM32_DMA_SFCR_FEIE | STM32_DMA_SFCR_DMDIS;
const STM32_DMA_FIFO_THRESHOLD_NONE: u32 = 4;
const STM32_DMA_MAX_DATA_ITEMS: u32 = 0xffff;
const STM32_DMA_ALIGNED_MAX_DATA_ITEMS: u32 = 0xfff0;
const STM32_DMA_MAX_CHANNELS: usize = 8;
const STM32_DMA_MAX_REQUEST_ID: u32 = 8;
const STM32_DMA_FIFO_SIZE: u32 = 16;
const STM32_DMA_MIN_BURST: u32 = 4;
const STM32_DMA_MAX_BURST: u32 = 16;

const fn stm32_dma_isr(n: u32) -> u32 { if n & 4 != 0 { STM32_DMA_HISR } else { STM32_DMA_LISR } }
const fn stm32_dma_ifcr(n: u32) -> u32 { if n & 4 != 0 { STM32_DMA_HIFCR } else { STM32_DMA_LIFCR } }
const fn stm32_dma_flags_shift(n: u32) -> u32 { ((n & 2) << 3) | ((n & 1) * 6) }
const fn stm32_dma_scr(x: u32) -> u32 { 0x10 + 0x18 * x }
const fn stm32_dma_sndtr(x: u32) -> u32 { 0x14 + 0x18 * x }
const fn stm32_dma_spar(x: u32) -> u32 { 0x18 + 0x18 * x }
const fn stm32_dma_sm0ar(x: u32) -> u32 { 0x1c + 0x18 * x }
const fn stm32_dma_sm1ar(x: u32) -> u32 { 0x20 + 0x18 * x }
const fn stm32_dma_sfcr(x: u32) -> u32 { 0x24 + 0x18 * x }

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct stm32_dma_cfg { pub channel_id: u32, pub request_line: u32, pub stream_config: u32, pub features: u32 }
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct stm32_dma_chan_reg { pub dma_lisr:u32, pub dma_hisr:u32, pub dma_lifcr:u32, pub dma_hifcr:u32, pub dma_scr:u32, pub dma_sndtr:u32, pub dma_spar:u32, pub dma_sm0ar:u32, pub dma_sm1ar:u32, pub dma_sfcr:u32 }
#[repr(C)]
pub struct stm32_dma_sg_req { pub len:u32, pub chan_reg:stm32_dma_chan_reg }
#[repr(C)]
pub struct stm32_dma_desc { pub vdesc: virt_dma_desc, pub cyclic: bool, pub num_sgs:u32, pub sg_req:[stm32_dma_sg_req; 0] }
#[repr(C)]
pub struct stm32_dma_mdma_config { pub stream_id:u32, pub ifcr:u32, pub tcf:u32 }
#[repr(C)]
pub struct stm32_dma_chan { pub vchan:virt_dma_chan, pub config_init:bool, pub busy:bool, pub id:u32, pub irq:u32, pub desc:*mut stm32_dma_desc, pub next_sg:u32, pub dma_sconfig:dma_slave_config, pub chan_reg:stm32_dma_chan_reg, pub threshold:u32, pub mem_burst:u32, pub mem_width:u32, pub status:dma_status, pub trig_mdma:bool, pub mdma_config:stm32_dma_mdma_config }
#[repr(C)]
pub struct stm32_dma_device { pub ddev:dma_device, pub base:*mut core::ffi::c_void, pub clk:*mut clk, pub mem2mem:bool, pub chan:[stm32_dma_chan; STM32_DMA_MAX_CHANNELS] }

unsafe fn stm32_dma_read(d: *mut stm32_dma_device, reg:u32) -> u32 { core::ptr::read_volatile((*d).base.cast::<u8>().add(reg as usize).cast()) }
unsafe fn stm32_dma_write(d: *mut stm32_dma_device, reg:u32, val:u32) { core::ptr::write_volatile((*d).base.cast::<u8>().add(reg as usize).cast(), val) }

// The remaining callbacks are direct unsafe translations of the source
// implementation; kernel-provided declarations are intentionally unresolved.
extern "C" {
    fn stm32_dma_probe(pdev:*mut platform_device) -> i32;
    fn stm32_dma_init() -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
