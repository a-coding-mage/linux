// SPDX-License-Identifier: GPL-2.0+
// Actions Semi Owl SoCs DMA driver
// Copyright (c) 2014 Actions Semi Inc.
// Copyright (c) 2018 Linaro Ltd.

// Linux kernel dependencies are supplied by the surrounding translation.

const OWL_DMA_FRAME_MAX_LENGTH: u32 = 0xfffff;
const OWL_DMA_IRQ_PD0: u32 = 0x00;
const OWL_DMA_IRQ_PD1: u32 = 0x04;
const OWL_DMA_IRQ_PD2: u32 = 0x08;
const OWL_DMA_IRQ_PD3: u32 = 0x0c;
const OWL_DMA_IRQ_EN0: u32 = 0x10;
const OWL_DMA_IRQ_EN1: u32 = 0x14;
const OWL_DMA_IRQ_EN2: u32 = 0x18;
const OWL_DMA_IRQ_EN3: u32 = 0x1c;
const OWL_DMA_SECURE_ACCESS_CTL: u32 = 0x20;
const OWL_DMA_NIC_QOS: u32 = 0x24;
const OWL_DMA_DBGSEL: u32 = 0x28;
const OWL_DMA_IDLE_STAT: u32 = 0x2c;
const OWL_DMAX_MODE: u32 = 0; const OWL_DMAX_SOURCE: u32 = 4;
const OWL_DMAX_DESTINATION: u32 = 8; const OWL_DMAX_FRAME_LEN: u32 = 0xc;
const OWL_DMAX_FRAME_CNT: u32 = 0x10; const OWL_DMAX_REMAIN_FRAME_CNT: u32 = 0x14;
const OWL_DMAX_REMAIN_CNT: u32 = 0x18; const OWL_DMAX_SOURCE_STRIDE: u32 = 0x1c;
const OWL_DMAX_DESTINATION_STRIDE: u32 = 0x20; const OWL_DMAX_START: u32 = 0x24;
const OWL_DMAX_PAUSE: u32 = 0x28; const OWL_DMAX_CHAINED_CTL: u32 = 0x2c;
const OWL_DMAX_CONSTANT: u32 = 0x30; const OWL_DMAX_LINKLIST_CTL: u32 = 0x34;
const OWL_DMAX_NEXT_DESCRIPTOR: u32 = 0x38; const OWL_DMAX_CURRENT_DESCRIPTOR_NUM: u32 = 0x3c;
const OWL_DMAX_INT_CTL: u32 = 0x40; const OWL_DMAX_INT_STATUS: u32 = 0x44;
const OWL_DMAX_CURRENT_SOURCE_POINTER: u32 = 0x48; const OWL_DMAX_CURRENT_DESTINATION_POINTER: u32 = 0x4c;

const fn chan_base(i: u32) -> u32 { 0x100 + i * 0x100 }
const fn bits(v: u32, n: u32) -> u32 { if n == 32 { v } else { v & ((1 << n) - 1) } }
const fn mode_ts(x:u32)->u32{bits(x,6)} const fn mode_st(x:u32)->u32{bits(x,2)<<8}
const fn mode_dt(x:u32)->u32{bits(x,2)<<10} const fn mode_sam(x:u32)->u32{bits(x,2)<<16}
const fn mode_dam(x:u32)->u32{bits(x,2)<<18} const fn mode_pw(x:u32)->u32{bits(x,3)<<20}
const OWL_DMA_MODE_ST_DEV:u32=mode_st(0); const OWL_DMA_MODE_ST_DCU:u32=mode_st(2); const OWL_DMA_MODE_ST_SRAM:u32=mode_st(3);
const OWL_DMA_MODE_DT_DEV:u32=mode_dt(0); const OWL_DMA_MODE_DT_DCU:u32=mode_dt(2); const OWL_DMA_MODE_DT_SRAM:u32=mode_dt(3);
const OWL_DMA_MODE_SAM_CONST:u32=mode_sam(0); const OWL_DMA_MODE_SAM_INC:u32=mode_sam(1); const OWL_DMA_MODE_SAM_STRIDE:u32=mode_sam(2);
const OWL_DMA_MODE_DAM_CONST:u32=mode_dam(0); const OWL_DMA_MODE_DAM_INC:u32=mode_dam(1); const OWL_DMA_MODE_DAM_STRIDE:u32=mode_dam(2);
const OWL_DMA_MODE_CB:u32=1<<23; const OWL_DMA_MODE_NDDBW_32BIT:u32=0; const OWL_DMA_MODE_NDDBW_8BIT:u32=1<<28;
const OWL_DMA_MODE_CFE:u32=1<<29; const OWL_DMA_MODE_LME:u32=1<<30; const OWL_DMA_MODE_CME:u32=1<<31;
const OWL_DMA_LLC_SAV_INC:u32=0; const OWL_DMA_LLC_SAV_LOAD_NEXT:u32=1<<8; const OWL_DMA_LLC_SAV_LOAD_PREV:u32=2<<8;
const OWL_DMA_LLC_DAV_INC:u32=0; const OWL_DMA_LLC_DAV_LOAD_NEXT:u32=1<<10; const OWL_DMA_LLC_DAV_LOAD_PREV:u32=2<<10; const OWL_DMA_LLC_SUSPEND:u32=1<<16;
const OWL_DMA_INTCTL_BLOCK:u32=1; const OWL_DMA_INTCTL_SUPER_BLOCK:u32=2; const OWL_DMA_INTCTL_FRAME:u32=4; const OWL_DMA_INTCTL_HALF_FRAME:u32=8; const OWL_DMA_INTCTL_LAST_FRAME:u32=16;
const FCNT_VAL:u32=1;

#[repr(C)] pub enum owl_dma_id { S900_DMA, S700_DMA }
#[repr(C)] pub struct owl_dma_lli { pub hw:[u32;9], pub phys: dma_addr_t, pub node:list_head }
#[repr(C)] pub struct owl_dma_txd { pub vd:virt_dma_desc, pub lli_list:list_head, pub cyclic:bool }
#[repr(C)] pub struct owl_dma_pchan { pub id:u32, pub base:*mut u8, pub vchan:*mut owl_dma_vchan }
#[repr(C)] pub struct owl_dma_vchan { pub vc:virt_dma_chan, pub pchan:*mut owl_dma_pchan, pub txd:*mut owl_dma_txd, pub cfg:dma_slave_config, pub drq:u8 }
#[repr(C)] pub struct owl_dma { pub dma:dma_device, pub base:*mut u8, pub clk:*mut clk, pub lock:spinlock_t, pub lli_pool:*mut dma_pool, pub irq:i32, pub nr_pchans:u32, pub pchans:*mut owl_dma_pchan, pub nr_vchans:u32, pub vchans:*mut owl_dma_vchan, pub devid:owl_dma_id }

#[inline] fn field(v:u32,w:u32,s:u32,n:u32)->u32 { ((v>>s)&((1<<w)-1))<<n }
#[inline] fn llc_hw_ctrla(mode:u32,llc:u32)->u32 { field(mode,4,28,28)|field(mode,8,16,20)|field(mode,4,8,16)|field(mode,6,0,10)|field(llc,2,10,8)|field(llc,2,8,6) }
#[inline] fn llc_hw_ctrlb(v:u32)->u32 { field(v,7,0,18) }
fn llc_hw_flen(lli:*const owl_dma_lli)->u32 { unsafe{(*lli).hw[3]&OWL_DMA_FRAME_MAX_LENGTH} }

unsafe fn pchan_update(p:*mut owl_dma_pchan,r:u32,v:u32,state:bool){let mut x=readl((*p).base.add(r as usize));if state{x|=v}else{x&=!v};writel(x,(*p).base.add(r as usize));}
unsafe fn pchan_writel(p:*mut owl_dma_pchan,r:u32,v:u32){writel(v,(*p).base.add(r as usize));}
unsafe fn pchan_readl(p:*mut owl_dma_pchan,r:u32)->u32{readl((*p).base.add(r as usize))}
unsafe fn dma_update(d:*mut owl_dma,r:u32,v:u32,state:bool){let mut x=readl((*d).base.add(r as usize));if state{x|=v}else{x&=!v};writel(x,(*d).base.add(r as usize));}
unsafe fn dma_writel(d:*mut owl_dma,r:u32,v:u32){writel(v,(*d).base.add(r as usize));}
unsafe fn dma_readl(d:*mut owl_dma,r:u32)->u32{readl((*d).base.add(r as usize))}

unsafe fn owl_dma_cfg_lli(v:*mut owl_dma_vchan,l:*mut owl_dma_lli,src:dma_addr_t,dst:dma_addr_t,len:u32,dir:dma_transfer_direction,c:&dma_slave_config,cyclic:bool)->i32{
 let mut mode=mode_pw(0); match dir { DMA_MEM_TO_MEM=>mode|=mode_ts(0)|OWL_DMA_MODE_ST_DCU|OWL_DMA_MODE_DT_DCU|OWL_DMA_MODE_SAM_INC|OWL_DMA_MODE_DAM_INC,
 DMA_MEM_TO_DEV=>{mode|=mode_ts((*v).drq)|OWL_DMA_MODE_ST_DCU|OWL_DMA_MODE_DT_DEV|OWL_DMA_MODE_SAM_INC|OWL_DMA_MODE_DAM_CONST;if c.dst_addr_width==DMA_SLAVE_BUSWIDTH_1_BYTE{mode|=OWL_DMA_MODE_NDDBW_8BIT}},
 DMA_DEV_TO_MEM=>{mode|=mode_ts((*v).drq)|OWL_DMA_MODE_ST_DEV|OWL_DMA_MODE_DT_DCU|OWL_DMA_MODE_SAM_CONST|OWL_DMA_MODE_DAM_INC;if c.src_addr_width==DMA_SLAVE_BUSWIDTH_1_BYTE{mode|=OWL_DMA_MODE_NDDBW_8BIT}}, _=>return -22 }
 (*l).hw[6]=llc_hw_ctrla(mode,OWL_DMA_LLC_SAV_LOAD_NEXT|OWL_DMA_LLC_DAV_LOAD_NEXT);(*l).hw[7]=llc_hw_ctrlb(if cyclic{OWL_DMA_INTCTL_BLOCK}else{OWL_DMA_INTCTL_SUPER_BLOCK});(*l).hw[0]=0;(*l).hw[1]=src as u32;(*l).hw[2]=dst as u32;(*l).hw[4]=0;(*l).hw[5]=0; if (*to_owl_dma(&mut (*v).vc.chan.device)).devid as u32==S700_DMA as u32 {(*l).hw[3]=len;(*l).hw[7]|=FCNT_VAL}else{(*l).hw[3]=len|(FCNT_VAL<<20)} 0
}

unsafe fn owl_dma_free_lli(od:*mut owl_dma, lli:*mut owl_dma_lli){list_del(&mut (*lli).node);dma_pool_free((*od).lli_pool,lli,(*lli).phys);}
unsafe fn owl_dma_add_lli(txd:*mut owl_dma_txd,prev:*mut owl_dma_lli,next:*mut owl_dma_lli,cyclic:bool)->*mut owl_dma_lli{if !cyclic{list_add_tail(&mut (*next).node,&mut (*txd).lli_list)}if !prev.is_null(){(*prev).hw[0]=(*next).phys as u32;(*prev).hw[6]|=llc_hw_ctrla(OWL_DMA_MODE_LME,0)}next}
unsafe fn owl_dma_free_txd(od:*mut owl_dma,txd:*mut owl_dma_txd){if txd.is_null(){return}/* list_for_each_entry_safe */kfree(txd as *mut c_void);}
unsafe fn owl_dma_terminate_all(chan:*mut dma_chan)->i32{let _=chan;0}
unsafe fn owl_dma_config(chan:*mut dma_chan,config:*mut dma_slave_config)->i32{let _=(chan,config);0}
unsafe fn owl_dma_pause(chan:*mut dma_chan)->i32{let _=chan;0}
unsafe fn owl_dma_resume(chan:*mut dma_chan)->i32{let _=chan;0}
unsafe fn owl_dma_tx_status(chan:*mut dma_chan,cookie:dma_cookie_t,state:*mut dma_tx_state)->dma_status{dma_cookie_status(chan,cookie,state)}
unsafe fn owl_dma_issue_pending(chan:*mut dma_chan){let _=chan;}
unsafe fn owl_dma_prep_memcpy(chan:*mut dma_chan,dst:dma_addr_t,src:dma_addr_t,len:usize,flags:ulong)->*mut dma_async_tx_descriptor{let _=(chan,dst,src,len,flags);core::ptr::null_mut()}
unsafe fn owl_dma_prep_slave_sg(chan:*mut dma_chan,sgl:*mut scatterlist,sg_len:u32,dir:dma_transfer_direction,flags:ulong,context:*mut c_void)->*mut dma_async_tx_descriptor{let _=(chan,sgl,sg_len,dir,flags,context);core::ptr::null_mut()}
unsafe fn owl_prep_dma_cyclic(chan:*mut dma_chan,buf_addr:dma_addr_t,buf_len:usize,period_len:usize,dir:dma_transfer_direction,flags:ulong)->*mut dma_async_tx_descriptor{let _=(chan,buf_addr,buf_len,period_len,dir,flags);core::ptr::null_mut()}
unsafe fn owl_dma_free_chan_resources(chan:*mut dma_chan){let _=chan;}
unsafe fn owl_dma_init()->i32{platform_driver_register(&mut owl_dma_driver)}
unsafe fn owl_dma_exit(){platform_driver_unregister(&mut owl_dma_driver)}

#[no_mangle] static mut owl_dma_driver: platform_driver = platform_driver { probe:Some(owl_dma_probe), remove:Some(owl_dma_remove) };
extern "C" { fn owl_dma_probe(pdev:*mut platform_device)->i32; fn owl_dma_remove(pdev:*mut platform_device); fn dma_cookie_status(chan:*mut dma_chan,cookie:dma_cookie_t,state:*mut dma_tx_state)->dma_status; fn platform_driver_register(d:*mut platform_driver)->i32; fn platform_driver_unregister(d:*mut platform_driver); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
