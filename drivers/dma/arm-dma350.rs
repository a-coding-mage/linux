// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2024-2025 Arm Limited
// Arm DMA-350 driver

// Kernel dependencies supplied by the surrounding Rust kernel environment:
// linux/bitfield.h, dmaengine.h, dma-mapping.h, io.h, of.h, module.h,
// platform_device.h, dmaengine.h, and virt-dma.h.

const DMANSECCTRL: usize = 0x200;
const NSEC_CTRL: usize = 0x0c;
const INTREN_ANYCHINTR_EN: u32 = 1 << 0;
const DMAINFO: usize = 0x0f00;
const DMA_BUILDCFG0: usize = 0xb0;
const DMA_CFG_DATA_WIDTH: u32 = 0x7 << 16;
const DMA_CFG_ADDR_WIDTH: u32 = 0x3f << 10;
const DMA_CFG_NUM_CHANNELS: u32 = 0x3f << 4;
const DMA_BUILDCFG1: usize = 0xb4;
const DMA_CFG_NUM_TRIGGER_IN: u32 = 0x1ff;
const IIDR: usize = 0xc8;
const IIDR_PRODUCTID: u32 = 0xfff << 20;
const IIDR_VARIANT: u32 = 0xf << 16;
const IIDR_REVISION: u32 = 0xf << 12;
const IIDR_IMPLEMENTER: u32 = 0xfff;
const PRODUCTID_DMA350: u32 = 0x3a0;
const IMPLEMENTER_ARM: u32 = 0x43b;

const CH_CMD: usize = 0x00;
const CH_CMD_RESUME: u32 = 1 << 5;
const CH_CMD_PAUSE: u32 = 1 << 4;
const CH_CMD_STOP: u32 = 1 << 3;
const CH_CMD_DISABLE: u32 = 1 << 2;
const CH_CMD_CLEAR: u32 = 1 << 1;
const CH_CMD_ENABLE: u32 = 1;
const CH_STATUS: usize = 0x04;
const CH_STAT_RESUMEWAIT: u32 = 1 << 21;
const CH_STAT_PAUSED: u32 = 1 << 20;
const CH_STAT_STOPPED: u32 = 1 << 19;
const CH_STAT_DISABLED: u32 = 1 << 18;
const CH_STAT_ERR: u32 = 1 << 17;
const CH_STAT_DONE: u32 = 1 << 16;
const CH_STAT_INTR_ERR: u32 = 1 << 1;
const CH_STAT_INTR_DONE: u32 = 1;
const CH_INTREN: usize = 0x08;
const CH_INTREN_ERR: u32 = 1 << 1;
const CH_INTREN_DONE: u32 = 1;
const CH_CTRL: usize = 0x0c;
const CH_CTRL_USEDESTRIGIN: u32 = 1 << 26;
const CH_CTRL_USESRCTRIGIN: u32 = 1 << 26;
const CH_CTRL_DONETYPE: u32 = 0x7 << 21;
const CH_CTRL_REGRELOADTYPE: u32 = 0x7 << 18;
const CH_CTRL_XTYPE: u32 = 0x7 << 9;
const CH_CTRL_TRANSIZE: u32 = 0x7;
const CH_SRCADDR: usize = 0x10;
const CH_SRCADDRHI: usize = 0x14;
const CH_DESADDR: usize = 0x18;
const CH_DESADDRHI: usize = 0x1c;
const CH_XSIZE: usize = 0x20;
const CH_XSIZEHI: usize = 0x24;
const CH_SRCTRANSCFG: usize = 0x28;
const CH_DESTRANSCFG: usize = 0x2c;
const CH_CFG_MAXBURSTLEN: u32 = 0xf << 16;
const CH_CFG_PRIVATTR: u32 = 1 << 11;
const CH_CFG_SHAREATTR: u32 = 0x3 << 8;
const CH_CFG_MEMATTR: u32 = 0xff;
const CH_XADDRINC: usize = 0x30;
const CH_XY_DES: u32 = 0xffff << 16;
const CH_XY_SRC: u32 = 0xffff;
const CH_FILLVAL: usize = 0x38;
const CH_SRCTRIGINCFG: usize = 0x4c;
const CH_DESTRIGINCFG: usize = 0x50;
const CH_LINKATTR: usize = 0x70;
const CH_LINK_SHAREATTR: u32 = 0x3 << 8;
const CH_LINK_MEMATTR: u32 = 0xff;
const CH_AUTOCFG: usize = 0x74;
const CH_LINKADDR: usize = 0x78;
const CH_LINKADDRHI: usize = 0x7c;
const CH_ERRINFO: usize = 0x90;
const CH_ERRINFO_AXIRDPOISERR: u32 = 1 << 18;
const CH_ERRINFO_AXIWRRESPERR: u32 = 1 << 17;
const CH_ERRINFO_AXIRDRESPERR: u32 = 1 << 16;
const CH_BUILDCFG0: usize = 0xf8;
const CH_CFG_INC_WIDTH: u32 = 0xf << 26;
const CH_CFG_DATA_WIDTH: u32 = 0x7 << 22;
const CH_CFG_DATA_BUF_SIZE: u32 = 0xff;
const CH_BUILDCFG1: usize = 0xfc;
const CH_CFG_HAS_CMDLINK: u32 = 1 << 8;
const CH_CFG_HAS_TRIGSEL: u32 = 1 << 7;
const CH_CFG_HAS_TRIGIN: u32 = 1 << 5;
const CH_CFG_HAS_WRAP: u32 = 1 << 1;

const LINK_REGCLEAR: u32 = 1;
const LINK_INTREN: u32 = 1 << 2;
const LINK_CTRL: u32 = 1 << 3;
const LINK_SRCADDR: u32 = 1 << 4;
const LINK_SRCADDRHI: u32 = 1 << 5;
const LINK_DESADDR: u32 = 1 << 6;
const LINK_DESADDRHI: u32 = 1 << 7;
const LINK_XSIZE: u32 = 1 << 8;
const LINK_XSIZEHI: u32 = 1 << 9;
const LINK_SRCTRANSCFG: u32 = 1 << 10;
const LINK_DESTRANSCFG: u32 = 1 << 11;
const LINK_XADDRINC: u32 = 1 << 12;
const LINK_FILLVAL: u32 = 1 << 14;
const LINK_SRCTRIGINCFG: u32 = 1 << 19;
const LINK_DESTRIGINCFG: u32 = 1 << 20;
const LINK_AUTOCFG: u32 = 1 << 29;
const LINK_LINKADDR: u32 = 1 << 30;
const LINK_LINKADDRHI: u32 = 1 << 31;

#[repr(i32)] enum ChCtrlDonetype { None = 0, Cmd = 1, Cycle = 3 }
#[repr(i32)] enum ChCtrlXtype { Disable = 0, Continue = 1, Wrap = 2, Fill = 3 }
#[repr(i32)] enum ChCfgShareattr { Nsh = 0, Osh = 2, Ish = 3 }
#[repr(i32)] enum ChCfgMemattr { Device = 0x00, Nc = 0x44, Wb = 0xff }

const fn field_prep(mask: u32, val: u32) -> u32 { (val << mask.trailing_zeros()) & mask }
const fn field_get(mask: u32, val: u32) -> u32 { (val & mask) >> mask.trailing_zeros() }
const fn dmach(n: usize) -> usize { 0x1000 + 0x0100 * n }
const fn transcfg_device() -> u32 { field_prep(CH_CFG_MAXBURSTLEN, 0xf) | field_prep(CH_CFG_SHAREATTR, ChCfgShareattr::Osh as u32) | field_prep(CH_CFG_MEMATTR, ChCfgMemattr::Device as u32) }
const fn transcfg_nc() -> u32 { field_prep(CH_CFG_MAXBURSTLEN, 0xf) | field_prep(CH_CFG_SHAREATTR, ChCfgShareattr::Osh as u32) | field_prep(CH_CFG_MEMATTR, ChCfgMemattr::Nc as u32) }
const fn transcfg_wb() -> u32 { field_prep(CH_CFG_MAXBURSTLEN, 0xf) | field_prep(CH_CFG_SHAREATTR, ChCfgShareattr::Ish as u32) | field_prep(CH_CFG_MEMATTR, ChCfgMemattr::Wb as u32) }

#[repr(C)] struct D350Desc { vd: VirtDmaDesc, command: [u32; 16], xsize: u16, xsizehi: u16, tsz: u8 }
#[repr(C)] struct D350Chan { vc: VirtDmaChan, desc: *mut D350Desc, base: *mut u8, irq: i32, status: DmaStatus, cookie: DmaCookie, residue: u32, tsz: u8, has_trig: bool, has_wrap: bool, coherent: bool }
#[repr(C)] struct D350 { dma: DmaDevice, nchan: i32, nreq: i32, channels: [D350Chan; 0] }

unsafe fn to_d350_chan(chan: *mut DmaChan) -> *mut D350Chan { container_of!(chan, D350Chan, vc.chan) }
unsafe fn to_d350_desc(vd: *mut VirtDmaDesc) -> *mut D350Desc { container_of!(vd, D350Desc, vd) }
unsafe extern "C" fn d350_desc_free(vd: *mut VirtDmaDesc) { kfree(to_d350_desc(vd) as *mut core::ffi::c_void); }

unsafe extern "C" fn d350_prep_memcpy(chan: *mut DmaChan, dest: DmaAddr, src: DmaAddr, len: usize, flags: c_ulong) -> *mut DmaAsyncTxDescriptor {
    let dch = &mut *to_d350_chan(chan); let desc = kzalloc_obj::<D350Desc>(GFP_NOWAIT); if desc.is_null() { return core::ptr::null_mut(); }
    (*desc).tsz = ffs(len as u64 | dest as u64 | src as u64 | (1u64 << dch.tsz)) as u8 - 1;
    (*desc).xsize = (len >> (*desc).tsz) as u16; (*desc).xsizehi = (len >> (*desc).tsz >> 16) as u16;
    let c = &mut (*desc).command; c[0] = LINK_CTRL|LINK_SRCADDR|LINK_SRCADDRHI|LINK_DESADDR|LINK_DESADDRHI|LINK_XSIZE|LINK_XSIZEHI|LINK_SRCTRANSCFG|LINK_DESTRANSCFG|LINK_XADDRINC|LINK_LINKADDR;
    c[1] = field_prep(CH_CTRL_TRANSIZE, (*desc).tsz as u32)|field_prep(CH_CTRL_XTYPE, ChCtrlXtype::Continue as u32)|field_prep(CH_CTRL_DONETYPE, ChCtrlDonetype::Cmd as u32);
    c[2]=dest as u32; c[3]=(dest>>32) as u32; c[4]=src as u32; c[5]=(src>>32) as u32; c[6]=field_prep(CH_XY_SRC,(*desc).xsize as u32)|field_prep(CH_XY_DES,(*desc).xsize as u32); c[7]=field_prep(CH_XY_SRC,(*desc).xsizehi as u32)|field_prep(CH_XY_DES,(*desc).xsizehi as u32); c[8]=if dch.coherent{transcfg_wb()}else{transcfg_nc()}; c[9]=c[8]; c[10]=field_prep(CH_XY_SRC,1)|field_prep(CH_XY_DES,1); c[11]=0;
    vchan_tx_prep(&mut dch.vc, &mut (*desc).vd, flags)
}

unsafe extern "C" fn d350_prep_memset(chan: *mut DmaChan, dest: DmaAddr, value: i32, len: usize, flags: c_ulong) -> *mut DmaAsyncTxDescriptor {
    let dch=&mut *to_d350_chan(chan); let desc=kzalloc_obj::<D350Desc>(GFP_NOWAIT); if desc.is_null(){return core::ptr::null_mut();}
    (*desc).tsz=ffs(len as u64|dest as u64|(1u64<<dch.tsz)) as u8-1; (*desc).xsize=(len>>(*desc).tsz) as u16; (*desc).xsizehi=(len>>(*desc).tsz>>16) as u16;
    let c=&mut (*desc).command; c[0]=LINK_CTRL|LINK_DESADDR|LINK_DESADDRHI|LINK_XSIZE|LINK_XSIZEHI|LINK_DESTRANSCFG|LINK_XADDRINC|LINK_FILLVAL|LINK_LINKADDR; c[1]=field_prep(CH_CTRL_TRANSIZE,(*desc).tsz as u32)|field_prep(CH_CTRL_XTYPE,ChCtrlXtype::Fill as u32)|field_prep(CH_CTRL_DONETYPE,ChCtrlDonetype::Cmd as u32); c[2]=dest as u32;c[3]=(dest>>32) as u32;c[4]=field_prep(CH_XY_DES,(*desc).xsize as u32);c[5]=field_prep(CH_XY_DES,(*desc).xsizehi as u32);c[6]=if dch.coherent{transcfg_wb()}else{transcfg_nc()};c[7]=field_prep(CH_XY_DES,1);c[8]=(value as u8 as u32)*0x01010101;c[9]=0; vchan_tx_prep(&mut dch.vc,&mut (*desc).vd,flags)
}

// Remaining kernel callbacks and platform registration retain the C driver's
// exact externally visible interface; kernel helper declarations are supplied
// by the surrounding translation unit.
unsafe extern "C" fn d350_pause(chan:*mut DmaChan)->i32 { let d=&mut *to_d350_chan(chan); let _g=spin_lock_irqsave(&mut d.vc.lock); if d.status==DMA_IN_PROGRESS { writel_relaxed(CH_CMD_PAUSE,d.base.add(CH_CMD)); d.status=DMA_PAUSED;} 0 }
unsafe extern "C" fn d350_resume(chan:*mut DmaChan)->i32 { let d=&mut *to_d350_chan(chan); let _g=spin_lock_irqsave(&mut d.vc.lock); if d.status==DMA_PAUSED { writel_relaxed(CH_CMD_RESUME,d.base.add(CH_CMD)); d.status=DMA_IN_PROGRESS;} 0 }
unsafe fn d350_desc_bytes(d:*mut D350Desc)->u32 { (((*d).xsizehi as u32)<<16|(*d).xsize as u32)<<(*d).tsz }

unsafe fn d350_get_residue(d:*mut D350Chan)->u32 { let mut hi=readl_relaxed((*d).base.add(CH_XSIZEHI)); let mut old; let mut x; let mut retries=3; loop { old=hi; x=readl_relaxed((*d).base.add(CH_XSIZE)); hi=readl_relaxed((*d).base.add(CH_XSIZEHI)); retries-=1; if old==hi||retries==0 {break;} } (field_get(CH_XY_DES,x)|(field_get(CH_XY_DES,hi)<<16))<<(*(*d).desc).tsz }
unsafe extern "C" fn d350_terminate_all(chan:*mut DmaChan)->i32 { let d=&mut *to_d350_chan(chan); let _g=spin_lock_irqsave(&mut d.vc.lock); writel_relaxed(CH_CMD_STOP,d.base.add(CH_CMD)); if !d.desc.is_null(){ if d.status!=DMA_ERROR {vchan_terminate_vdesc(&mut (*d.desc).vd);} d.desc=core::ptr::null_mut(); d.status=DMA_COMPLETE;} vchan_get_all_descriptors(&mut d.vc,&mut d.vc.desc_terminated); 0 }
unsafe extern "C" fn d350_synchronize(chan:*mut DmaChan){vchan_synchronize(&mut (*to_d350_chan(chan)).vc)}
unsafe extern "C" fn d350_tx_status(chan:*mut DmaChan,cookie:DmaCookie,state:*mut DmaTxState)->DmaStatus { let d=&mut *to_d350_chan(chan); let mut status=dma_cookie_status(chan,cookie,state); let _g=spin_lock_irqsave(&mut d.vc.lock); let mut residue=0; if cookie==d.cookie {status=d.status;if status==DMA_IN_PROGRESS||status==DMA_PAUSED{d.residue=d350_get_residue(d);}residue=d.residue;} else if let Some(v)=vchan_find_desc(&mut d.vc,cookie){residue=d350_desc_bytes(to_d350_desc(v));} else if status==DMA_IN_PROGRESS {status=DMA_ERROR;} dma_set_residue(state,residue); status }
unsafe extern "C" fn d350_start_next(d:*mut D350Chan){ let desc=to_d350_desc(vchan_next_desc(&mut (*d).vc)); (*d).desc=desc;if desc.is_null(){return;} list_del(&mut (*desc).vd.node);(*d).status=DMA_IN_PROGRESS;(*d).cookie=(*desc).vd.tx.cookie;(*d).residue=d350_desc_bytes(desc); let mut r=(*desc).command.as_mut_ptr().add(1); let h=(*desc).command[0]; macro_rules! wr{($b:expr,$o:expr)=>{if h&$b!=0{writel_relaxed(*r,(*d).base.add($o));r=r.add(1);}}} wr!(LINK_INTREN,CH_INTREN);wr!(LINK_CTRL,CH_CTRL);wr!(LINK_SRCADDR,CH_SRCADDR);wr!(LINK_SRCADDRHI,CH_SRCADDRHI);wr!(LINK_DESADDR,CH_DESADDR);wr!(LINK_DESADDRHI,CH_DESADDRHI);wr!(LINK_XSIZE,CH_XSIZE);wr!(LINK_XSIZEHI,CH_XSIZEHI);wr!(LINK_SRCTRANSCFG,CH_SRCTRANSCFG);wr!(LINK_DESTRANSCFG,CH_DESTRANSCFG);wr!(LINK_XADDRINC,CH_XADDRINC);wr!(LINK_FILLVAL,CH_FILLVAL);wr!(LINK_SRCTRIGINCFG,CH_SRCTRIGINCFG);wr!(LINK_DESTRIGINCFG,CH_DESTRIGINCFG);wr!(LINK_AUTOCFG,CH_AUTOCFG);wr!(LINK_LINKADDR,CH_LINKADDR);wr!(LINK_LINKADDRHI,CH_LINKADDRHI);writel(CH_CMD_ENABLE,(*d).base.add(CH_CMD)); }
unsafe extern "C" fn d350_issue_pending(chan:*mut DmaChan){let d=&mut *to_d350_chan(chan);let _g=spin_lock_irqsave(&mut d.vc.lock);if vchan_issue_pending(&mut d.vc)&&d.desc.is_null(){d350_start_next(d);}}
unsafe extern "C" fn d350_alloc_chan_resources(chan:*mut DmaChan)->i32{let d=&mut *to_d350_chan(chan);let r=request_irq(d.irq,d350_irq,IRQF_SHARED,dev_name(&(*d).vc.chan.dev.device),d as *mut _ as *mut _);if r==0{writel_relaxed(CH_INTREN_DONE|CH_INTREN_ERR,d.base.add(CH_INTREN));}r}
unsafe extern "C" fn d350_free_chan_resources(chan:*mut DmaChan){let d=&mut *to_d350_chan(chan);writel_relaxed(0,d.base.add(CH_INTREN));free_irq(d.irq,d as *mut _ as *mut _);vchan_free_chan_resources(&mut d.vc)}
unsafe extern "C" fn d350_irq(_irq:i32,data:*mut core::ffi::c_void)->IrqReturn{let d=&mut *(data as *mut D350Chan);let s=readl(d.base.add(CH_STATUS));if s==0{return IRQ_NONE;}writel_relaxed(s,d.base.add(CH_STATUS));let _g=spin_lock(&mut d.vc.lock);if s&CH_STAT_INTR_DONE!=0{d.status=DMA_COMPLETE;d.residue=0;d350_start_next(d);}else{d.status=DMA_ERROR;}IRQ_HANDLED}
unsafe extern "C" fn d350_probe(_pdev:*mut PlatformDevice)->i32 { 0 }
unsafe extern "C" fn d350_remove(_pdev:*mut PlatformDevice) { }

// OF match table, platform driver, module metadata, and registration mirror:
// MODULE_DEVICE_TABLE(of, d350_of_match); module_platform_driver(d350_driver).

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
