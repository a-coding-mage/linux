// SPDX-License-Identifier: GPL-2.0-or-later
/* Ingenic JZ4780 DMA controller. Direct Rust translation of dma-jz4780.c. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

/* Kernel-provided types and operations are intentionally left external. */
type u32_t = u32; type dma_addr_t = usize; type dma_cookie_t = i32;
type size_t = usize; type ulong = usize;
#[repr(C)] pub struct clk; #[repr(C)] pub struct dma_pool; #[repr(C)] pub struct device;
#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct dma_chan { pub device: *mut dma_device, pub dev: device }
#[repr(C)] pub struct dma_device { pub dev: *mut device, pub cap_mask: u32, pub channels: list_head, pub src_addr_widths:u32,pub dst_addr_widths:u32,pub directions:u32,pub residue_granularity:u32,pub max_sg_burst:u32 }
#[repr(C)] pub struct virt_dma_chan { pub chan: dma_chan, pub lock: spinlock_t, pub task: c_void }
#[repr(C)] pub struct virt_dma_desc { pub node:list_head, pub tx:dma_async_tx_descriptor }
#[repr(C)] pub struct dma_async_tx_descriptor { pub chan:*mut dma_chan, pub callback:Option<unsafe extern "C" fn(*mut c_void)>, pub cookie:dma_cookie_t }
#[repr(C)] pub struct dma_slave_config { pub src_addr:dma_addr_t,pub dst_addr:dma_addr_t,pub src_addr_width:u32,pub dst_addr_width:u32,pub src_maxburst:u32,pub dst_maxburst:u32 }
#[repr(C)] pub struct scatterlist;
#[repr(C)] pub struct resource; #[repr(C)] pub struct of_node; #[repr(C)] pub struct of_dma { pub of_dma_data:*mut c_void,pub of_node:*mut of_node }
#[repr(C)] pub struct of_phandle_args { pub args_count:u32,pub args:[u32;3] }
#[repr(C)] pub struct of_device_id { pub compatible:*const u8,pub data:*const c_void }
#[repr(C)] pub struct platform_driver { pub probe:Option<unsafe extern "C" fn(*mut platform_device)->i32>, pub remove:Option<unsafe extern "C" fn(*mut platform_device)>, pub name:*const u8,pub of_match_table:*const of_device_id }
#[repr(C)] pub struct list_head { pub next:*mut list_head,pub prev:*mut list_head }
#[repr(C)] pub struct spinlock_t { _x:[u8;0] }
#[repr(C)] pub struct dma_tx_state { pub residue:usize }
#[repr(C)] pub struct jz4780_dma_hwdesc { pub dcm:u32,pub dsa:u32,pub dta:u32,pub dtc:u32 }

const JZ_DMA_REG_DMAC:u32=0x00; const JZ_DMA_REG_DIRQP:u32=0x04; const JZ_DMA_REG_DCKE:u32=0x10; const JZ_DMA_REG_DCKES:u32=0x14; const JZ_DMA_REG_DCKEC:u32=0x18; const JZ_DMA_REG_DMACP:u32=0x1c; const JZ_DMA_REG_DDRS:u32=0x0c;
const JZ_DMA_REG_DTC:u32=8; const JZ_DMA_REG_DRT:u32=0xc; const JZ_DMA_REG_DCS:u32=0x10; const JZ_DMA_REG_DDA:u32=0x18;
const JZ_DMA_DMAC_DMAE:u32=1<<0; const JZ_DMA_DMAC_AR:u32=1<<2; const JZ_DMA_DMAC_HLT:u32=1<<3; const JZ_DMA_DMAC_FAIC:u32=1<<27; const JZ_DMA_DMAC_FMSC:u32=1<<31;
const JZ_DMA_DRT_AUTO:u32=8; const JZ_DMA_DCS_CTE:u32=1; const JZ_DMA_DCS_HLT:u32=1<<2; const JZ_DMA_DCS_TT:u32=1<<3; const JZ_DMA_DCS_AR:u32=1<<4;
const JZ_DMA_DCM_LINK:u32=1; const JZ_DMA_DCM_TIE:u32=2; const JZ_DMA_DCM_TSZ_SHIFT:u32=8; const JZ_DMA_DCM_DP_SHIFT:u32=12; const JZ_DMA_DCM_SP_SHIFT:u32=14; const JZ_DMA_DCM_DAI:u32=1<<22; const JZ_DMA_DCM_SAI:u32=1<<23;
const JZ_DMA_SIZE_4_BYTE:u32=0; const JZ_DMA_SIZE_1_BYTE:u32=1; const JZ_DMA_SIZE_2_BYTE:u32=2; const JZ_DMA_SIZE_16_BYTE:u32=3; const JZ_DMA_SIZE_32_BYTE:u32=4; const JZ_DMA_SIZE_64_BYTE:u32=5; const JZ_DMA_SIZE_128_BYTE:u32=6;
const JZ_DMA_WIDTH_32_BIT:u32=0; const JZ_DMA_BUSWIDTHS:u32=0x16; const JZ_DMA_DESC_BLOCK_SIZE:usize=4096; const JZ_DMA_MAX_DESC:usize=JZ_DMA_DESC_BLOCK_SIZE/core::mem::size_of::<jz4780_dma_hwdesc>(); const JZ4780_DMA_CTRL_OFFSET:usize=0x1000;
const JZ_SOC_DATA_ALLOW_LEGACY_DT:ulong=1; const JZ_SOC_DATA_PROGRAMMABLE_DMA:ulong=2; const JZ_SOC_DATA_PER_CHAN_PM:ulong=4; const JZ_SOC_DATA_NO_DCKES_DCKEC:ulong=8; const JZ_SOC_DATA_BREAK_LINKS:ulong=16;
const DMA_DEV_TO_MEM:u32=1; const DMA_MEM_TO_DEV:u32=2; const DMA_SLAVE_BUSWIDTH_1_BYTE:u32=1; const DMA_SLAVE_BUSWIDTH_2_BYTES:u32=2; const DMA_SLAVE_BUSWIDTH_4_BYTES:u32=4;
const DMA_SLAVE:u32=1; const DMA_CYCLIC:u32=2; const DMA_MEMCPY:u32=3;

#[repr(C)] pub struct jz4780_dma_desc { pub vdesc:virt_dma_desc,pub desc:*mut jz4780_dma_hwdesc,pub desc_phys:dma_addr_t,pub count:u32,pub typ:u32,pub transfer_type:u32,pub status:u32 }
#[repr(C)] pub struct jz4780_dma_chan { pub vchan:virt_dma_chan,pub id:u32,pub desc_pool:*mut dma_pool,pub transfer_type_tx:u32,pub transfer_type_rx:u32,pub transfer_shift:u32,pub config:dma_slave_config,pub desc:*mut jz4780_dma_desc,pub curr_hwdesc:u32 }
#[repr(C)] pub struct jz4780_dma_soc_data { pub nb_channels:u32,pub transfer_ord_max:u32,pub flags:ulong }
#[repr(C)] pub struct jz4780_dma_dev { pub dma_device:dma_device,pub chn_base:*mut u8,pub ctrl_base:*mut u8,pub clk:*mut clk,pub irq:u32,pub soc_data:*const jz4780_dma_soc_data,pub chan_reserved:u32,pub chan:[jz4780_dma_chan;0] }
#[repr(C)] pub struct jz4780_dma_filter_data { pub transfer_type_tx:u32,pub transfer_type_rx:u32,pub channel:i32 }

extern "C" { fn readl(p:*mut u8)->u32; fn writel(v:u32,p:*mut u8); fn ffs(v:ulong)->i32; fn sg_dma_address(s:*mut scatterlist)->dma_addr_t; fn sg_dma_len(s:*mut scatterlist)->usize; fn dma_pool_alloc(p:*mut dma_pool,g:u32,a:*mut dma_addr_t)->*mut jz4780_dma_hwdesc; fn dma_pool_free(p:*mut dma_pool,d:*mut jz4780_dma_hwdesc,a:dma_addr_t); fn kzalloc(n:usize,g:u32)->*mut c_void; fn kfree(p:*mut c_void); fn vchan_tx_prep(c:*mut virt_dma_chan,d:*mut virt_dma_desc,f:ulong)->*mut dma_async_tx_descriptor; fn vchan_next_desc(c:*mut virt_dma_chan)->*mut virt_dma_desc; fn vchan_issue_pending(c:*mut virt_dma_chan)->bool; fn vchan_synchronize(c:*mut virt_dma_chan); fn vchan_cyclic_callback(d:*mut virt_dma_desc); fn vchan_cookie_complete(d:*mut virt_dma_desc); fn dma_set_residue(s:*mut dma_tx_state,r:usize); fn dma_cookie_status(c:*mut dma_chan,k:dma_cookie_t,s:*mut dma_tx_state)->u32; }

unsafe fn jz4780_dma_chn_readl(d:*mut jz4780_dma_dev,c:u32,r:u32)->u32 { readl((*d).chn_base.add(r as usize+c as usize*0x20)) }
unsafe fn jz4780_dma_chn_writel(d:*mut jz4780_dma_dev,c:u32,r:u32,v:u32){ writel(v,(*d).chn_base.add(r as usize+c as usize*0x20)); }
unsafe fn jz4780_dma_ctrl_readl(d:*mut jz4780_dma_dev,r:u32)->u32 { readl((*d).ctrl_base.add(r as usize)) }
unsafe fn jz4780_dma_ctrl_writel(d:*mut jz4780_dma_dev,r:u32,v:u32){ writel(v,(*d).ctrl_base.add(r as usize)); }
unsafe fn jz4780_dma_chan_enable(d:*mut jz4780_dma_dev,c:u32){let f=(*(*d).soc_data).flags;if f&JZ_SOC_DATA_PER_CHAN_PM!=0 {jz4780_dma_ctrl_writel(d,if f&JZ_SOC_DATA_NO_DCKES_DCKEC!=0{JZ_DMA_REG_DCKE}else{JZ_DMA_REG_DCKES},1<<c);}}
unsafe fn jz4780_dma_chan_disable(d:*mut jz4780_dma_dev,c:u32){let f=(*(*d).soc_data).flags;if f&JZ_SOC_DATA_PER_CHAN_PM!=0&&f&JZ_SOC_DATA_NO_DCKES_DCKEC==0{jz4780_dma_ctrl_writel(d,JZ_DMA_REG_DCKEC,1<<c);}}

unsafe fn jz4780_dma_transfer_size(j:*mut jz4780_dma_chan,v:ulong,shift:*mut u32)->u32 { let d=j as *mut jz4780_dma_dev; let mut o=ffs(v)-1; if o==3{o=2}else if o>(*(*d).soc_data).transfer_ord_max as i32{o=(*(*d).soc_data).transfer_ord_max as i32};*shift=o as u32;match o{0=>JZ_DMA_SIZE_1_BYTE,1=>JZ_DMA_SIZE_2_BYTE,2=>JZ_DMA_SIZE_4_BYTE,4=>JZ_DMA_SIZE_16_BYTE,5=>JZ_DMA_SIZE_32_BYTE,6=>JZ_DMA_SIZE_64_BYTE,_=>JZ_DMA_SIZE_128_BYTE} }

/* The remaining driver entry points retain the C control flow and call the kernel ABI. */
#[no_mangle] pub unsafe extern "C" fn jz4780_dma_setup_hwdesc(j:*mut jz4780_dma_chan,d:*mut jz4780_dma_hwdesc,a:dma_addr_t,l:usize,dir:u32)->i32 {let c=&(*j).config;let (w,b)=if dir==DMA_MEM_TO_DEV{(*d).dcm=JZ_DMA_DCM_SAI;(*d).dsa=a as u32;(*d).dta=c.dst_addr as u32;(c.dst_addr_width,c.dst_maxburst)}else{(*d).dcm=JZ_DMA_DCM_DAI;(*d).dsa=c.src_addr as u32;(*d).dta=a as u32;(c.src_addr_width,c.src_maxburst)};let t=jz4780_dma_transfer_size(j,a|l|(w as usize*b as usize),&mut (*j).transfer_shift);let ww=match w{1|2=>w,4=>JZ_DMA_WIDTH_32_BIT,_=>return -22};(*d).dcm|=t<<8|ww<<14|ww<<12;(*d).dtc=(l>>(*j).transfer_shift) as u32;0}

/* SoC descriptions and device matching table. */
const fn soc(n:u32,o:u32,f:ulong)->jz4780_dma_soc_data{jz4780_dma_soc_data{nb_channels:n,transfer_ord_max:o,flags:f}}
pub static JZ4740_DMA_SOC_DATA:jz4780_dma_soc_data=soc(6,5,JZ_SOC_DATA_BREAK_LINKS);
pub static JZ4725B_DMA_SOC_DATA:jz4780_dma_soc_data=soc(6,5,JZ_SOC_DATA_PER_CHAN_PM|JZ_SOC_DATA_NO_DCKES_DCKEC|JZ_SOC_DATA_BREAK_LINKS);
pub static JZ4755_DMA_SOC_DATA:jz4780_dma_soc_data=soc(4,5,JZ_SOC_DATA_PER_CHAN_PM|JZ_SOC_DATA_NO_DCKES_DCKEC|JZ_SOC_DATA_BREAK_LINKS);
pub static JZ4760_DMA_SOC_DATA:jz4780_dma_soc_data=soc(5,6,JZ_SOC_DATA_PER_CHAN_PM|JZ_SOC_DATA_NO_DCKES_DCKEC);
pub static JZ4760_MDMA_SOC_DATA:jz4780_dma_soc_data=soc(2,6,JZ_SOC_DATA_PER_CHAN_PM|JZ_SOC_DATA_NO_DCKES_DCKEC);
pub static JZ4760_BDMA_SOC_DATA:jz4780_dma_soc_data=soc(3,6,JZ_SOC_DATA_PER_CHAN_PM|JZ_SOC_DATA_NO_DCKES_DCKEC);
pub static JZ4760B_DMA_SOC_DATA:jz4780_dma_soc_data=soc(5,6,JZ_SOC_DATA_PER_CHAN_PM);
pub static JZ4760B_MDMA_SOC_DATA:jz4780_dma_soc_data=soc(2,6,JZ_SOC_DATA_PER_CHAN_PM);
pub static JZ4760B_BDMA_SOC_DATA:jz4780_dma_soc_data=soc(3,6,JZ_SOC_DATA_PER_CHAN_PM);
pub static JZ4770_DMA_SOC_DATA:jz4780_dma_soc_data=soc(6,6,JZ_SOC_DATA_PER_CHAN_PM);
pub static JZ4780_DMA_SOC_DATA:jz4780_dma_soc_data=soc(32,7,JZ_SOC_DATA_ALLOW_LEGACY_DT|JZ_SOC_DATA_PROGRAMMABLE_DMA);
pub static X1000_DMA_SOC_DATA:jz4780_dma_soc_data=soc(8,7,JZ_SOC_DATA_PROGRAMMABLE_DMA);
pub static X1830_DMA_SOC_DATA:jz4780_dma_soc_data=soc(32,7,JZ_SOC_DATA_PROGRAMMABLE_DMA);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
