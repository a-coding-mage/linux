// SPDX-License-Identifier: GPL-2.0-only
/*
 * Support functions for the OMAP internal DMA channels.
 * Rust translation of the original implementation source.
 */

// Dependencies supplied by the surrounding kernel translation.

const MAX_LOGICAL_DMA_CH_COUNT: usize = 32;
const OMAP_DMA_ACTIVE: u32 = 0x01;
const OMAP_FUNC_MUX_ARM_BASE: u32 = 0xfffe1000 + 0xec;

static mut p: *mut omap_system_dma_plat_info = core::ptr::null_mut();
static mut d: *mut omap_dma_dev_attr = core::ptr::null_mut();
static mut enable_1510_mode: i32 = 0;
static mut errata: u32 = 0;
static mut dma_lch_count: i32 = 0;
static mut dma_chan_count: i32 = 0;
static mut omap_dma_reserve_channels: i32 = 0;
static mut dma_chan: *mut omap_dma_lch = core::ptr::null_mut();

unsafe fn omap_disable_channel_irq(lch: i32) {
    (*p).dma_write(0, CICR, lch);
    (*p).dma_read(CSR, lch);
}

unsafe fn set_gdma_dev(req: i32, dev: i32) {
    let reg = OMAP_FUNC_MUX_ARM_BASE + (((req - 1) / 5) * 4) as u32;
    let shift = ((req - 1) % 5) * 6;
    let mut l = omap_readl(reg);
    l &= !(0x3f << shift);
    l |= ((dev - 1) << shift) as u32;
    omap_writel(l, reg);
}

#[cfg(CONFIG_FB_OMAP)]
pub unsafe fn omap_set_dma_priority(_lch: i32, dst_port: i32, priority: i32) {
    if dma_omap1() {
        let reg = match dst_port {
            OMAP_DMA_PORT_OCP_T1 => OMAP_TC_OCPT1_PRIOR,
            OMAP_DMA_PORT_OCP_T2 => OMAP_TC_OCPT2_PRIOR,
            OMAP_DMA_PORT_EMIFF => OMAP_TC_EMIFF_PRIOR,
            OMAP_DMA_PORT_EMIFS => OMAP_TC_EMIFS_PRIOR,
            _ => { BUG(); return; }
        };
        let mut l = omap_readl(reg);
        l &= !(0xf << 8);
        l |= ((priority & 0xf) << 8) as u32;
        omap_writel(l, reg);
    }
}

#[cfg(CONFIG_USB_OMAP)]
unsafe fn omap_dma_in_1510_mode() -> i32 { enable_1510_mode }

#[cfg(not(CONFIG_ARCH_OMAP15XX))]
unsafe fn omap_dma_in_1510_mode() -> i32 { 0 }

#[cfg(CONFIG_USB_OMAP)]
pub unsafe fn omap_set_dma_transfer_params(lch: i32, data_type: i32, elem_count: i32, frame_count: i32, sync_mode: i32, _dma_trigger: i32, _src_or_dst_synch: i32) {
    let mut l = (*p).dma_read(CSDP, lch); l &= !0x03; l |= data_type as u32; (*p).dma_write(l, CSDP, lch);
    let mut ccr = (*p).dma_read(CCR, lch) as u16; ccr &= !(1 << 5); if sync_mode == OMAP_DMA_SYNC_FRAME { ccr |= 1 << 5; } (*p).dma_write(ccr as u32, CCR, lch);
    ccr = (*p).dma_read(CCR2, lch) as u16; ccr &= !(1 << 2); if sync_mode == OMAP_DMA_SYNC_BLOCK { ccr |= 1 << 2; } (*p).dma_write(ccr as u32, CCR2, lch);
    (*p).dma_write(elem_count as u32, CEN, lch); (*p).dma_write(frame_count as u32, CFN, lch);
}

pub unsafe fn omap_set_dma_channel_mode(lch: i32, mode: omap_dma_channel_mode) {
    if !dma_omap15xx() { let mut l = (*p).dma_read(LCH_CTRL, lch); l &= !0x7; l |= mode as u32; (*p).dma_write(l, LCH_CTRL, lch); }
}

pub unsafe fn omap_set_dma_src_params(lch: i32, src_port: i32, src_amode: i32, src_start: usize, src_ei: i32, src_fi: i32) {
    let mut w = (*p).dma_read(CSDP, lch) as u16; w &= !(0x1f << 2); w |= (src_port << 2) as u16; (*p).dma_write(w as u32, CSDP, lch);
    let mut l = (*p).dma_read(CCR, lch); l &= !(0x03 << 12); l |= (src_amode << 12) as u32; (*p).dma_write(l, CCR, lch);
    (*p).dma_write(src_start as u32, CSSA, lch); (*p).dma_write(src_ei as u32, CSEI, lch); (*p).dma_write(src_fi as u32, CSFI, lch);
}

pub unsafe fn omap_set_dma_src_data_pack(lch: i32, enable: i32) { let mut l=(*p).dma_read(CSDP,lch); l &= !(1<<6); if enable != 0 { l |= 1<<6; } (*p).dma_write(l,CSDP,lch); }

pub unsafe fn omap_set_dma_src_burst_mode(lch: i32, burst_mode: omap_dma_burst_mode) {
    let mut burst=0; let mut l=(*p).dma_read(CSDP,lch); l &= !(0x03<<7);
    match burst_mode { OMAP_DMA_DATA_BURST_DIS=>{}, OMAP_DMA_DATA_BURST_4=>burst=2, OMAP_DMA_DATA_BURST_8|OMAP_DMA_DATA_BURST_16=>{BUG();}, _=>{BUG();} }
    l |= burst<<7; (*p).dma_write(l,CSDP,lch);
}

pub unsafe fn omap_set_dma_dest_params(lch:i32,dest_port:i32,dest_amode:i32,dest_start:usize,dst_ei:i32,dst_fi:i32) { let mut l=(*p).dma_read(CSDP,lch); l &= !(0x1f<<9); l |= (dest_port<<9) as u32; (*p).dma_write(l,CSDP,lch); l=(*p).dma_read(CCR,lch); l &= !(0x03<<14); l |= (dest_amode<<14) as u32; (*p).dma_write(l,CCR,lch); (*p).dma_write(dest_start as u32,CDSA,lch); (*p).dma_write(dst_ei as u32,CDEI,lch); (*p).dma_write(dst_fi as u32,CDFI,lch); }
pub unsafe fn omap_set_dma_dest_data_pack(lch:i32,enable:i32) { let mut l=(*p).dma_read(CSDP,lch); l &= !(1<<13); if enable!=0 {l|=1<<13;} (*p).dma_write(l,CSDP,lch); }
pub unsafe fn omap_set_dma_dest_burst_mode(lch:i32,burst_mode:omap_dma_burst_mode) { let mut burst=0; let mut l=(*p).dma_read(CSDP,lch); l &= !(0x03<<14); match burst_mode {OMAP_DMA_DATA_BURST_DIS=>{},OMAP_DMA_DATA_BURST_4=>burst=2,OMAP_DMA_DATA_BURST_8=>burst=3,_=>{printk(KERN_ERR,"Invalid DMA burst mode\\n");BUG();return;}} l|=burst<<14;(*p).dma_write(l,CSDP,lch); }

#[cfg(CONFIG_USB_OMAP)]
unsafe fn omap_enable_channel_irq(lch:i32) { (*p).dma_read(CSR,lch); (*p).dma_write((*dma_chan.add(lch as usize)).enabled_irqs as u32,CICR,lch); }
pub unsafe fn omap_disable_dma_irq(lch:i32,bits:u16) { (*dma_chan.add(lch as usize)).enabled_irqs &= !bits; }
#[cfg(CONFIG_USB_OMAP)]
unsafe fn enable_lnk(lch:i32) { let mut l=(*p).dma_read(CLNK_CTRL,lch); l &= !(1<<14); if (*dma_chan.add(lch as usize)).next_lch != -1 {l=((*dma_chan.add(lch as usize)).next_lch as u32)|(1<<15);} (*p).dma_write(l,CLNK_CTRL,lch); }
#[cfg(CONFIG_USB_OMAP)]
unsafe fn disable_lnk(lch:i32) { let mut l=(*p).dma_read(CLNK_CTRL,lch); omap_disable_channel_irq(lch); l|=1<<14; (*p).dma_write(l,CLNK_CTRL,lch); (*dma_chan.add(lch as usize)).flags &= !OMAP_DMA_ACTIVE as u16; }

pub unsafe fn omap_request_dma(dev_id:i32,dev_name:*const i8,callback:Option<unsafe extern "C" fn(i32,u16,*mut core::ffi::c_void)>,data:*mut core::ffi::c_void,dma_ch_out:*mut i32)->i32 { let mut free=-1; for ch in 0..dma_chan_count {if (*dma_chan.add(ch as usize)).dev_id==-1 {free=ch;break;}} if free==-1{return -EBUSY;} let chan=dma_chan.add(free as usize);(*chan).dev_id=dev_id; if let Some(f)=(*p).clear_lch_regs {f(free);} (*chan).dev_name=dev_name;(*chan).callback=callback;(*chan).data=data;(*chan).flags=0;(*chan).enabled_irqs=OMAP_DMA_DROP_IRQ|OMAP_DMA_BLOCK_IRQ|OMAP1_DMA_TOUT_IRQ; if dma_omap16xx(){if dev_id!=0{set_gdma_dev(free+1,dev_id);}(*p).dma_write(dev_id as u32|(1<<10),CCR,free);}else{(*p).dma_write(dev_id as u32,CCR,free);}*dma_ch_out=free;0 }

pub unsafe fn omap_free_dma(lch:i32){let c=dma_chan.add(lch as usize);if (*c).dev_id==-1{pr_err("omap_dma: trying to free unallocated DMA channel %d\\n",lch);return;}omap_disable_channel_irq(lch);(*p).dma_write(0,CCR,lch);(*c).dev_id=-1;(*c).next_lch=-1;(*c).callback=None;}
unsafe fn omap_clear_dma(lch:i32){let flags=0;local_irq_save(flags);(*p).clear_dma(lch);local_irq_restore(flags);}

#[cfg(CONFIG_USB_OMAP)]
pub unsafe fn omap_start_dma(lch:i32){if dma_omap15xx(){(*p).dma_write(0,CPC,lch)}else{(*p).dma_write(0,CDAC,lch)};omap_enable_channel_irq(lch);let mut l=(*p).dma_read(CCR,lch);if IS_DMA_ERRATA(DMA_ERRATA_IFRAME_BUFFERING){l|=OMAP_DMA_CCR_BUFFERING_DISABLE;}l|=OMAP_DMA_CCR_EN;mb();(*p).dma_write(l,CCR,lch);(*dma_chan.add(lch as usize)).flags|=OMAP_DMA_ACTIVE as u16;}
#[cfg(CONFIG_USB_OMAP)]
pub unsafe fn omap_stop_dma(lch:i32){omap_disable_channel_irq(lch);let mut l=(*p).dma_read(CCR,lch);l&=!OMAP_DMA_CCR_EN;(*p).dma_write(l,CCR,lch);mb();(*dma_chan.add(lch as usize)).flags&=!OMAP_DMA_ACTIVE as u16;}

#[cfg(CONFIG_USB_OMAP)]
pub unsafe fn omap_get_dma_src_pos(lch:i32)->dma_addr_t{let mut o=if dma_omap15xx(){(*p).dma_read(CPC,lch)}else{(*p).dma_read(CSAC,lch)};if !dma_omap15xx(){o=if (*p).dma_read(CDAC,lch)!=0{(*p).dma_read(CSAC,lch)}else{(*p).dma_read(CSSA,lch)};}o|((*p).dma_read(CSSA,lch)&0xffff0000)}
#[cfg(CONFIG_USB_OMAP)]
pub unsafe fn omap_get_dma_dst_pos(lch:i32)->dma_addr_t{let mut o=if dma_omap15xx(){(*p).dma_read(CPC,lch)}else{(*p).dma_read(CDAC,lch)};if !dma_omap15xx()&&o==0{o=(*p).dma_read(CDAC,lch);if o==0{o=(*p).dma_read(CDSA,lch);}}o|((*p).dma_read(CDSA,lch)&0xffff0000)}
#[cfg(CONFIG_USB_OMAP)]
pub unsafe fn omap_get_dma_active_status(lch:i32)->i32{(((*p).dma_read(CCR,lch)&OMAP_DMA_CCR_EN)!=0) as i32}

pub unsafe fn omap_dma_running()->i32{if omap_lcd_dma_running(){return 1;}for lch in 0..dma_chan_count{if (*p).dma_read(CCR,lch)&OMAP_DMA_CCR_EN!=0{return 1;}}0}
pub unsafe fn omap_get_plat_info()->*mut omap_system_dma_plat_info{p}

unsafe fn omap1_dma_handle_ch(ch:i32)->i32{let c=dma_chan.add(ch as usize);let mut csr=if enable_1510_mode!=0&&ch>=6{let x=(*c).saved_csr;(*c).saved_csr=0;x}else{(*p).dma_read(CSR,ch)};if enable_1510_mode!=0&&ch<=2&&(csr>>7)!=0{(*dma_chan.add((ch+6) as usize)).saved_csr=csr>>7;csr&=0x7f;}if csr&0x3f==0{return 0;}if (*c).dev_id==-1{return 0;}if csr&OMAP1_DMA_TOUT_IRQ!=0{pr_warn("DMA timeout with device %d\\n",(*c).dev_id);}if csr&OMAP_DMA_DROP_IRQ!=0{pr_warn("DMA synchronization event drop occurred with device %d\\n",(*c).dev_id);}if csr&OMAP_DMA_BLOCK_IRQ!=0{(*c).flags&=!OMAP_DMA_ACTIVE as u16;}if let Some(cb)=(*c).callback{cb(ch,csr as u16,(*c).data);}1}

unsafe extern "C" fn omap1_dma_irq_handler(_irq:i32,dev_id:*mut core::ffi::c_void)->irqreturn_t{let ch=dev_id as usize as i32-1;let mut handled=0;loop{let mut now=omap1_dma_handle_ch(ch);if enable_1510_mode!=0&&(*dma_chan.add((ch+6) as usize)).saved_csr!=0{now+=omap1_dma_handle_ch(ch+6);}if now==0{break;}handled+=now;}if handled!=0{IRQ_HANDLED}else{IRQ_NONE}}

unsafe fn omap_system_dma_probe(pdev:*mut platform_device)->i32{p=(*pdev).dev.platform_data;if p.is_null(){return -EINVAL;}d=(*p).dma_attr;errata=(*p).errata;dma_lch_count=(*d).lch_count;dma_chan_count=dma_lch_count;enable_1510_mode=(((*d).dev_caps&ENABLE_1510_MODE)!=0) as i32;dma_chan=devm_kcalloc(&mut (*pdev).dev,dma_lch_count as usize,core::mem::size_of::<omap_dma_lch>(),GFP_KERNEL);if dma_chan.is_null(){return -ENOMEM;}for ch in 0..dma_chan_count{omap_clear_dma(ch);(*dma_chan.add(ch as usize)).dev_id=-1;(*dma_chan.add(ch as usize)).next_lch=-1;}(*p).show_dma_caps();0}
unsafe fn omap_system_dma_remove(_pdev:*mut platform_device){}
static mut omap_system_dma_driver: platform_driver = platform_driver{probe:Some(omap_system_dma_probe),remove:Some(omap_system_dma_remove),driver:driver{name:"omap_dma_system"}};
unsafe fn omap_system_dma_init()->i32{platform_driver_register(&mut omap_system_dma_driver)}
unsafe fn omap_system_dma_exit(){platform_driver_unregister(&mut omap_system_dma_driver)}
unsafe fn omap_dma_cmdline_reserve_ch(_str:*mut *mut i8)->i32{1}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
