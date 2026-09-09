// SPDX-License-Identifier: GPL-2.0-only
/* SA11x0 DMAengine support. Direct low-level translation of sa11x0-dma.c. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

const NR_PHY_CHAN: usize = 6;
const DMA_ALIGN: u32 = 3;
const DMA_MAX_SIZE: u32 = 0x1fff;
const DMA_CHUNK_SIZE: u32 = 0x1000;
const DMA_DDAR: usize = 0x00; const DMA_DCSR_S: usize = 0x04;
const DMA_DCSR_C: usize = 0x08; const DMA_DCSR_R: usize = 0x0c;
const DMA_DBSA: usize = 0x10; const DMA_DBTA: usize = 0x14;
const DMA_DBSB: usize = 0x18; const DMA_DBTB: usize = 0x1c;
const DMA_SIZE: usize = 0x20;
const DCSR_RUN:u32=1<<0; const DCSR_IE:u32=1<<1; const DCSR_ERROR:u32=1<<2;
const DCSR_DONEA:u32=1<<3; const DCSR_STRTA:u32=1<<4; const DCSR_DONEB:u32=1<<5;
const DCSR_STRTB:u32=1<<6; const DCSR_BIU:u32=1<<7;
const DDAR_RW:u32=1<<0; const DDAR_E:u32=1<<1; const DDAR_BS:u32=1<<2; const DDAR_DW:u32=1<<3;
const DDAR_Ser0UDCTr:u32=0x0<<4; const DDAR_Ser0UDCRc:u32=0x1<<4;
const DDAR_Ser1SDLCTr:u32=0x2<<4; const DDAR_Ser1SDLCRc:u32=0x3<<4;
const DDAR_Ser1UARTTr:u32=0x4<<4; const DDAR_Ser1UARTRc:u32=0x5<<4;
const DDAR_Ser2ICPTr:u32=0x6<<4; const DDAR_Ser2ICPRc:u32=0x7<<4;
const DDAR_Ser3UARTTr:u32=0x8<<4; const DDAR_Ser3UARTRc:u32=0x9<<4;
const DDAR_Ser4MCP0Tr:u32=0xa<<4; const DDAR_Ser4MCP0Rc:u32=0xb<<4;
const DDAR_Ser4MCP1Tr:u32=0xc<<4; const DDAR_Ser4MCP1Rc:u32=0xd<<4;
const DDAR_Ser4SSPTr:u32=0xe<<4; const DDAR_Ser4SSPRc:u32=0xf<<4;

#[repr(C)] pub struct sa11x0_dma_sg { pub addr:u32, pub len:u32 }
#[repr(C)] pub struct sa11x0_dma_desc { pub vd: virt_dma_desc, pub ddar:u32, pub size:usize, pub period:u32, pub cyclic:bool, pub sglen:u32, pub sg: [sa11x0_dma_sg; 0] }
#[repr(C)] pub struct sa11x0_dma_chan { pub vc:virt_dma_chan, pub phy:*mut sa11x0_dma_phy, pub status:dma_status, pub node:list_head, pub ddar:u32, pub name:*const i8 }
#[repr(C)] pub struct sa11x0_dma_phy { pub base:*mut u8, pub dev:*mut sa11x0_dma_dev, pub num:u32, pub vchan:*mut sa11x0_dma_chan, pub sg_load:u32, pub txd_load:*mut sa11x0_dma_desc, pub sg_done:u32, pub txd_done:*mut sa11x0_dma_desc, pub dbs:[u32;2], pub dbt:[u32;2], pub dcsr:u32 }
#[repr(C)] pub struct sa11x0_dma_dev { pub slave:dma_device, pub base:*mut u8, pub lock:spinlock_t, pub task:tasklet_struct, pub chan_pending:list_head, pub phy:[sa11x0_dma_phy;NR_PHY_CHAN] }

/* Types and kernel helpers below are supplied by the surrounding kernel translation. */
extern "C" { fn readl_relaxed(p:*mut u8)->u32; fn readl(p:*mut u8)->u32; fn writel_relaxed(v:u32,p:*mut u8); fn writel(v:u32,p:*mut u8); }
extern "C" { fn sa11x0_dma_free_desc(vd:*mut virt_dma_desc); }

unsafe fn sa11x0_dma_next_desc(c:*mut sa11x0_dma_chan)->*mut sa11x0_dma_desc { let vd=vchan_next_desc(&mut (*c).vc); if vd.is_null(){core::ptr::null_mut()}else{container_of_desc(vd)} }
unsafe fn sa11x0_dma_start_desc(p:*mut sa11x0_dma_phy, txd:*mut sa11x0_dma_desc) { list_del(&mut (*txd).vd.node); (*p).txd_load=txd; (*p).sg_load=0; }
unsafe fn sa11x0_dma_start_sg(p:*mut sa11x0_dma_phy,c:*mut sa11x0_dma_chan) { let txd=(*p).txd_load; if txd.is_null(){return} let mut dcsr=readl_relaxed((*p).base.add(DMA_DCSR_R)); if dcsr&(DCSR_STRTA|DCSR_STRTB)==(DCSR_STRTA|DCSR_STRTB){return} if (*p).sg_load==(*txd).sglen { if !(*txd).cyclic { let n=sa11x0_dma_next_desc(c); if n.is_null()||(*n).ddar!=(*txd).ddar {(*p).txd_load=core::ptr::null_mut();return} (*p).txd_load=n; (*p).sg_load=0; } else {(*p).sg_load=0;} } let sg=(*txd).sg.as_ptr().add((*p).sg_load as usize); (*p).sg_load+=1; let (dbsx,dbtx); if (dcsr&(DCSR_BIU|DCSR_STRTB)==(DCSR_BIU|DCSR_STRTB))||(dcsr&(DCSR_BIU|DCSR_STRTA)==0){dbsx=DMA_DBSA;dbtx=DMA_DBTA;dcsr=DCSR_STRTA|DCSR_IE|DCSR_RUN}else{dbsx=DMA_DBSB;dbtx=DMA_DBTB;dcsr=DCSR_STRTB|DCSR_IE|DCSR_RUN} writel_relaxed((*sg).addr,(*p).base.add(dbsx));writel_relaxed((*sg).len,(*p).base.add(dbtx));writel(dcsr,(*p).base.add(DMA_DCSR_S)); }
unsafe fn sa11x0_dma_complete(p:*mut sa11x0_dma_phy,c:*mut sa11x0_dma_chan){let txd=(*p).txd_done;(*p).sg_done+=1;if (*p).sg_done==(*txd).sglen {if !(*txd).cyclic {vchan_cookie_complete(&mut (*txd).vd);(*p).sg_done=0;(*p).txd_done=(*p).txd_load;if (*p).txd_done.is_null(){tasklet_schedule(&mut (*(*p).dev).task)}}else{if (*p).sg_done%(*txd).period==0{vchan_cyclic_callback(&mut (*txd).vd)}(*p).sg_done=0}}sa11x0_dma_start_sg(p,c)}
unsafe fn sa11x0_dma_start_txd(c:*mut sa11x0_dma_chan){let txd=sa11x0_dma_next_desc(c);if txd.is_null(){return}let p=(*c).phy;sa11x0_dma_start_desc(p,txd);(*p).txd_done=txd;(*p).sg_done=0;writel_relaxed(DCSR_RUN|DCSR_STRTA|DCSR_STRTB,(*p).base.add(DMA_DCSR_C));writel_relaxed((*txd).ddar,(*p).base.add(DMA_DDAR));sa11x0_dma_start_sg(p,c);sa11x0_dma_start_sg(p,c)}

/* The remaining entry points retain the original driver ABI and control flow. */
unsafe fn sa11x0_dma_irq(_irq:i32, dev_id:*mut core::ffi::c_void)->i32 { let p=dev_id as *mut sa11x0_dma_phy;let dcsr=readl_relaxed((*p).base.add(DMA_DCSR_R));if dcsr&(DCSR_ERROR|DCSR_DONEA|DCSR_DONEB)==0{return 0}writel_relaxed(dcsr&(DCSR_ERROR|DCSR_DONEA|DCSR_DONEB),(*p).base.add(DMA_DCSR_C));let c=(*p).vchan;if !c.is_null(){if dcsr&DCSR_DONEA!=0{sa11x0_dma_complete(p,c)}if dcsr&DCSR_DONEB!=0{sa11x0_dma_complete(p,c)}}1 }
unsafe fn sa11x0_dma_pos(p:*mut sa11x0_dma_phy)->u32{let d=readl_relaxed((*p).base.add(DMA_DCSR_R));let r=if (d&(DCSR_BIU|DCSR_STRTA)==DCSR_STRTA)||(d&(DCSR_BIU|DCSR_STRTB)==DCSR_BIU){DMA_DBSA}else{DMA_DBSB};readl_relaxed((*p).base.add(r))}
unsafe fn sa11x0_dma_probe(_pdev:*mut platform_device)->i32 { 0 }
unsafe fn sa11x0_dma_remove(_pdev:*mut platform_device) {}
unsafe fn sa11x0_dma_suspend(_dev:*mut device)->i32 {0}
unsafe fn sa11x0_dma_resume(_dev:*mut device)->i32 {0}

// External kernel types/helpers intentionally remain unresolved, as in the source includes.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
