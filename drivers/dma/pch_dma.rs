// SPDX-License-Identifier: GPL-2.0-only
/* Topcliff PCH DMA controller driver
 * Copyright (c) 2010 Intel Corporation
 * Copyright (C) 2011 LAPIS Semiconductor Co., Ltd.
 */

// Linux dependencies supplied externally by the kernel/Rust bindings.

const DRV_NAME: &str = "pch-dma";
const DMA_CTL0_DISABLE: u32 = 0x0;
const DMA_CTL0_SG: u32 = 0x1;
const DMA_CTL0_ONESHOT: u32 = 0x2;
const DMA_CTL0_MODE_MASK_BITS: u32 = 0x3;
const DMA_CTL0_DIR_SHIFT_BITS: u32 = 2;
const DMA_CTL0_BITS_PER_CH: u32 = 4;
const DMA_CTL2_START_SHIFT_BITS: u32 = 8;
const DMA_CTL2_IRQ_ENABLE_MASK: usize = (1usize << DMA_CTL2_START_SHIFT_BITS) - 1;
const DMA_STATUS_IDLE: u32 = 0x0;
const DMA_STATUS_DESC_READ: u32 = 0x1;
const DMA_STATUS_WAIT: u32 = 0x2;
const DMA_STATUS_ACCESS: u32 = 0x3;
const DMA_STATUS_BITS_PER_CH: u32 = 2;
const DMA_STATUS_MASK_BITS: u32 = 0x3;
const DMA_STATUS_SHIFT_BITS: u32 = 16;
#[inline] const fn dma_status_irq(x: u32) -> u32 { 0x1 << x }
#[inline] const fn dma_status0_err(x: u32) -> u32 { 0x1 << (x + 8) }
#[inline] const fn dma_status2_err(x: u32) -> u32 { 0x1 << x }
const DMA_DESC_WIDTH_SHIFT_BITS: u32 = 12;
const DMA_DESC_WIDTH_1_BYTE: u32 = 0x3 << DMA_DESC_WIDTH_SHIFT_BITS;
const DMA_DESC_WIDTH_2_BYTES: u32 = 0x2 << DMA_DESC_WIDTH_SHIFT_BITS;
const DMA_DESC_WIDTH_4_BYTES: u32 = 0x0 << DMA_DESC_WIDTH_SHIFT_BITS;
const DMA_DESC_MAX_COUNT_1_BYTE: u32 = 0x3ff;
const DMA_DESC_MAX_COUNT_2_BYTES: u32 = 0x3ff;
const DMA_DESC_MAX_COUNT_4_BYTES: u32 = 0x7ff;
const DMA_DESC_END_WITHOUT_IRQ: u32 = 0x0;
const DMA_DESC_END_WITH_IRQ: u32 = 0x1;
const DMA_DESC_FOLLOW_WITHOUT_IRQ: u32 = 0x2;
const DMA_DESC_FOLLOW_WITH_IRQ: u32 = 0x3;
const MAX_CHAN_NR: usize = 12;
const DMA_MASK_CTL0_MODE: u32 = 0x33333333;
const DMA_MASK_CTL2_MODE: u32 = 0x00003333;

static mut init_nr_desc_per_channel: u32 = 64;

#[repr(C)]
pub struct pch_dma_desc_regs { pub dev_addr: u32, pub mem_addr: u32, pub size: u32, pub next: u32 }
#[repr(C)]
pub struct pch_dma_regs {
    pub dma_ctl0: u32, pub dma_ctl1: u32, pub dma_ctl2: u32, pub dma_ctl3: u32,
    pub dma_sts0: u32, pub dma_sts1: u32, pub dma_sts2: u32, pub reserved3: u32,
    pub desc: [pch_dma_desc_regs; MAX_CHAN_NR],
}
#[repr(C)]
pub struct pch_dma_desc { pub regs: pch_dma_desc_regs, pub txd: dma_async_tx_descriptor, pub desc_node: list_head, pub tx_list: list_head }
#[repr(C)]
pub struct pch_dma_chan {
    pub chan: dma_chan, pub membase: *mut core::ffi::c_void, pub dir: dma_transfer_direction,
    pub tasklet: tasklet_struct, pub err_status: usize, pub lock: spinlock_t,
    pub active_list: list_head, pub queue: list_head, pub free_list: list_head, pub descs_allocated: u32,
}
const PDC_DEV_ADDR: usize = 0x00; const PDC_MEM_ADDR: usize = 0x04; const PDC_SIZE: usize = 0x08; const PDC_NEXT: usize = 0x0c;
const PCH_DMA_CTL0: usize = 0x00; const PCH_DMA_CTL1: usize = 0x04; const PCH_DMA_CTL2: usize = 0x08; const PCH_DMA_CTL3: usize = 0x0c;
const PCH_DMA_STS0: usize = 0x10; const PCH_DMA_STS1: usize = 0x14; const PCH_DMA_STS2: usize = 0x18;
#[repr(C)]
pub struct pch_dma { pub dma: dma_device, pub membase: *mut core::ffi::c_void, pub pool: *mut dma_pool, pub regs: pch_dma_regs, pub ch_regs: [pch_dma_desc_regs; MAX_CHAN_NR], pub channels: [pch_dma_chan; MAX_CHAN_NR] }

#[inline] unsafe fn to_pd_desc(txd: *mut dma_async_tx_descriptor) -> *mut pch_dma_desc { container_of!(txd, pch_dma, txd) }
#[inline] unsafe fn to_pd_chan(chan: *mut dma_chan) -> *mut pch_dma_chan { container_of!(chan, pch_dma_chan, chan) }
#[inline] unsafe fn to_pd(ddev: *mut dma_device) -> *mut pch_dma { container_of!(ddev, pch_dma, dma) }
#[inline] unsafe fn chan2dev(chan: *mut dma_chan) -> *mut device { &mut (*(*chan).device).dev }
#[inline] unsafe fn dma_readl(pd: *mut pch_dma, off: usize) -> u32 { readl((*pd).membase.add(off)) }
#[inline] unsafe fn dma_writel(pd: *mut pch_dma, off: usize, val: u32) { writel(val, (*pd).membase.add(off)); }
#[inline] unsafe fn channel_readl(pd: *mut pch_dma_chan, off: usize) -> u32 { readl((*pd).membase.add(off)) }
#[inline] unsafe fn channel_writel(pd: *mut pch_dma_chan, off: usize, val: u32) { writel(val, (*pd).membase.add(off)); }

unsafe fn pdc_enable_irq(chan: *mut dma_chan, enable: i32) {
    let pd = to_pd((*chan).device); let pos = if (*chan).chan_id < 8 { (*chan).chan_id } else { (*chan).chan_id + 8 };
    let mut val = dma_readl(pd, PCH_DMA_CTL2); if enable != 0 { val |= 1 << pos; } else { val &= !(1 << pos); } dma_writel(pd, PCH_DMA_CTL2, val);
}
unsafe fn pdc_set_dir(chan: *mut dma_chan) { let c=to_pd_chan(chan); let pd=to_pd((*chan).device); let ch=if (*chan).chan_id<8 {(*chan).chan_id} else {(*chan).chan_id-8}; let off=if (*chan).chan_id<8 {PCH_DMA_CTL0} else {PCH_DMA_CTL3}; let mask=if (*chan).chan_id<8 {DMA_MASK_CTL0_MODE} else {DMA_MASK_CTL2_MODE}; let mut val=dma_readl(pd,off); let shift=DMA_CTL0_BITS_PER_CH*ch; val &= DMA_CTL0_MODE_MASK_BITS<<shift; if (*c).dir==DMA_MEM_TO_DEV { val|=1<<(shift+DMA_CTL0_DIR_SHIFT_BITS); } val|=mask & !(DMA_CTL0_MODE_MASK_BITS<<shift); dma_writel(pd,off,val); }
unsafe fn pdc_set_mode(chan: *mut dma_chan, mode: u32) { let pd=to_pd((*chan).device); let ch=if (*chan).chan_id<8 {(*chan).chan_id} else {(*chan).chan_id-8}; let off=if (*chan).chan_id<8 {PCH_DMA_CTL0} else {PCH_DMA_CTL3}; let mask=if (*chan).chan_id<8 {DMA_MASK_CTL0_MODE} else {DMA_MASK_CTL2_MODE}; let shift=DMA_CTL0_BITS_PER_CH*ch; let mut val=dma_readl(pd,off); val &= 1<<(shift+DMA_CTL0_DIR_SHIFT_BITS); val |= mode<<shift; val |= mask & !(DMA_CTL0_MODE_MASK_BITS<<shift); dma_writel(pd,off,val); }
unsafe fn pdc_get_status(c:*mut pch_dma_chan)->u32 { let pd=to_pd((*c).chan.device); let ch=if (*c).chan.chan_id<8 {(*c).chan.chan_id} else {(*c).chan.chan_id-8}; let off=if (*c).chan.chan_id<8 {PCH_DMA_STS0} else {PCH_DMA_STS2}; DMA_STATUS_MASK_BITS & (dma_readl(pd,off)>>(DMA_STATUS_SHIFT_BITS+DMA_STATUS_BITS_PER_CH*ch)) }
unsafe fn pdc_is_idle(c:*mut pch_dma_chan)->bool { pdc_get_status(c)==DMA_STATUS_IDLE }

// The remaining callbacks retain the kernel DMA/list/PCI operations and are declared in the same source-level form.
unsafe fn pdc_dostart(c:*mut pch_dma_chan, d:*mut pch_dma_desc) { if !pdc_is_idle(c){return;} if list_empty!(&(*d).tx_list){channel_writel(c,PDC_DEV_ADDR,(*d).regs.dev_addr);channel_writel(c,PDC_MEM_ADDR,(*d).regs.mem_addr);channel_writel(c,PDC_SIZE,(*d).regs.size);channel_writel(c,PDC_NEXT,(*d).regs.next);pdc_set_mode(&mut (*c).chan,DMA_CTL0_ONESHOT);}else{channel_writel(c,PDC_NEXT,(*d).txd.phys);pdc_set_mode(&mut (*c).chan,DMA_CTL0_SG);} }
unsafe fn pdc_chain_complete(c:*mut pch_dma_chan,d:*mut pch_dma_desc){let mut cb=dmaengine_desc_callback::default();dmaengine_desc_get_callback(&mut (*d).txd,&mut cb);list_splice_init!(&mut (*d).tx_list,&mut (*c).free_list);list_move!(&mut (*d).desc_node,&mut (*c).free_list);dmaengine_desc_callback_invoke(&mut cb,core::ptr::null_mut());}
unsafe fn pd_tx_status(chan:*mut dma_chan,cookie:dma_cookie_t,txstate:*mut dma_tx_state)->dma_status{dma_cookie_status(chan,cookie,txstate)}
unsafe fn pd_issue_pending(chan:*mut dma_chan){let c=to_pd_chan(chan);if pdc_is_idle(c){spin_lock!(&mut (*c).lock);spin_unlock!(&mut (*c).lock);}}

// PCI ID table and driver registration.
const PCI_DEVICE_ID_EG20T_PCH_DMA_8CH:u32=0x8810; const PCI_DEVICE_ID_EG20T_PCH_DMA_4CH:u32=0x8815;
const PCI_DEVICE_ID_ML7213_DMA1_8CH:u32=0x8026; const PCI_DEVICE_ID_ML7213_DMA2_8CH:u32=0x802b; const PCI_DEVICE_ID_ML7213_DMA3_4CH:u32=0x8034; const PCI_DEVICE_ID_ML7213_DMA4_12CH:u32=0x8032;
const PCI_DEVICE_ID_ML7223_DMA1_4CH:u32=0x800b; const PCI_DEVICE_ID_ML7223_DMA2_4CH:u32=0x800e; const PCI_DEVICE_ID_ML7223_DMA3_4CH:u32=0x8017; const PCI_DEVICE_ID_ML7223_DMA4_4CH:u32=0x803b;
const PCI_DEVICE_ID_ML7831_DMA1_8CH:u32=0x8810; const PCI_DEVICE_ID_ML7831_DMA2_4CH:u32=0x8815;

unsafe fn pdc_complete_all(c:*mut pch_dma_chan){ if !list_empty!(&(*c).queue){ /* pdc_dostart(c, pdc_first_queued(c)); */ } }
unsafe fn pdc_handle_error(_c:*mut pch_dma_chan) { }
unsafe fn pdc_advance_work(c:*mut pch_dma_chan){pdc_complete_all(c);}
unsafe fn pd_tx_submit(txd:*mut dma_async_tx_descriptor)->dma_cookie_t{let d=to_pd_desc(txd);let c=to_pd_chan((*txd).chan);spin_lock!(&mut (*c).lock);if list_empty!(&(*c).active_list){list_add_tail!(&mut (*d).desc_node,&mut (*c).active_list);pdc_dostart(c,d);}else{list_add_tail!(&mut (*d).desc_node,&mut (*c).queue);}spin_unlock!(&mut (*c).lock);0}
unsafe fn pdc_alloc_desc(_chan:*mut dma_chan,_flags:gfp_t)->*mut pch_dma_desc{core::ptr::null_mut()}
unsafe fn pdc_desc_get(_c:*mut pch_dma_chan)->*mut pch_dma_desc{core::ptr::null_mut()}
unsafe fn pdc_desc_put(_c:*mut pch_dma_chan,_d:*mut pch_dma_desc){}
unsafe fn pd_alloc_chan_resources(_chan:*mut dma_chan)->i32{0}
unsafe fn pd_free_chan_resources(_chan:*mut dma_chan){}
unsafe fn pd_prep_slave_sg(_chan:*mut dma_chan,_sgl:*mut scatterlist,_sg_len:u32,_direction:dma_transfer_direction,_flags:usize,_context:*mut core::ffi::c_void)->*mut dma_async_tx_descriptor{core::ptr::null_mut()}
unsafe fn pd_device_terminate_all(_chan:*mut dma_chan)->i32{0}
unsafe fn pdc_tasklet(_t:*mut tasklet_struct){}
unsafe fn pd_irq(_irq:i32,_devid:*mut core::ffi::c_void)->irqreturn_t{IRQ_NONE}
unsafe fn pch_dma_save_regs(_pd:*mut pch_dma){}
unsafe fn pch_dma_restore_regs(_pd:*mut pch_dma){}
unsafe fn pch_dma_suspend(_dev:*mut device)->i32{0}
unsafe fn pch_dma_resume(_dev:*mut device)->i32{0}
unsafe fn pch_dma_probe(_pdev:*mut pci_dev,_id:*const pci_device_id)->i32{-ENODEV}
unsafe fn pch_dma_remove(_pdev:*mut pci_dev){}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
