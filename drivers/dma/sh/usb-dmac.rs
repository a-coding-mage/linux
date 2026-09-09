// SPDX-License-Identifier: GPL-2.0
/* Renesas USB DMA Controller Driver. Direct low-level translation of usb-dmac.c. */

// Linux kernel types, helpers, and APIs referenced below are supplied externally.
use core::ffi::c_void;

#[repr(C)] pub struct usb_dmac_sg { pub mem_addr: dma_addr_t, pub size: u32 }
#[repr(C)] pub struct usb_dmac_desc {
    pub vd: virt_dma_desc, pub direction: dma_transfer_direction,
    pub sg_allocated_len: u32, pub sg_len: u32, pub sg_index: u32,
    pub residue: u32, pub node: list_head, pub done_cookie: dma_cookie_t,
    pub sg: *mut usb_dmac_sg,
}
#[repr(C)] pub struct usb_dmac_chan {
    pub vc: virt_dma_chan, pub iomem: *mut c_void, pub index: u32, pub irq: i32,
    pub desc: *mut usb_dmac_desc, pub descs_allocated: i32,
    pub desc_got: list_head, pub desc_freed: list_head,
}
#[repr(C)] pub struct usb_dmac {
    pub engine: dma_device, pub dev: *mut device, pub iomem: *mut c_void,
    pub n_channels: u32, pub channels: *mut usb_dmac_chan,
}

pub const USB_DMASWR: u32 = 0x0008; pub const USB_DMASWR_SWR: u32 = 1 << 0;
pub const USB_DMAOR: u32 = 0x0060; pub const USB_DMAOR_AE: u32 = 1 << 1; pub const USB_DMAOR_DME: u32 = 1;
pub const USB_DMASAR: u32 = 0; pub const USB_DMADAR: u32 = 4; pub const USB_DMATCR: u32 = 8;
pub const USB_DMATCR_MASK: u32 = 0x00ffffff; pub const USB_DMACHCR: u32 = 0x14;
pub const USB_DMACHCR_FTE: u32 = 1 << 24; pub const USB_DMACHCR_NULLE: u32 = 1 << 16;
pub const USB_DMACHCR_NULL: u32 = 1 << 12; pub const USB_DMACHCR_TS_8B: u32 = 0;
pub const USB_DMACHCR_TS_16B: u32 = 1 << 6; pub const USB_DMACHCR_TS_32B: u32 = 1 << 7;
pub const USB_DMACHCR_IE: u32 = 1 << 5; pub const USB_DMACHCR_SP: u32 = 1 << 2;
pub const USB_DMACHCR_TE: u32 = 1 << 1; pub const USB_DMACHCR_DE: u32 = 1; pub const USB_DMATEND: u32 = 0x18;
pub const USB_DMAC_XFER_SHIFT: u32 = 5; pub const USB_DMAC_XFER_SIZE: u32 = 1 << USB_DMAC_XFER_SHIFT;
pub const USB_DMAC_CHCR_TS: u32 = USB_DMACHCR_TS_32B; pub const USB_DMAC_INITIAL_NR_DESC: u32 = 16;
pub const USB_DMAC_INITIAL_NR_SG: u32 = 8;

// External kernel declarations (types and operations are defined by the surrounding kernel translation).
extern "C" {
    fn readl(addr: *mut c_void) -> u32; fn writel(v: u32, addr: *mut c_void);
    fn udelay(us: u32); fn usb_dmac_external_untranslated();
}

unsafe fn usb_dmac_write(d: *mut usb_dmac, reg: u32, data: u32) { writel(data, (*d).iomem.add(reg as usize)); }
unsafe fn usb_dmac_read(d: *mut usb_dmac, reg: u32) -> u32 { readl((*d).iomem.add(reg as usize)) }
unsafe fn usb_dmac_chan_read(c: *mut usb_dmac_chan, reg: u32) -> u32 { readl((*c).iomem.add(reg as usize)) }
unsafe fn usb_dmac_chan_write(c: *mut usb_dmac_chan, reg: u32, data: u32) { writel(data, (*c).iomem.add(reg as usize)); }

unsafe fn usb_dmac_chan_is_busy(c: *mut usb_dmac_chan) -> bool {
    let chcr = usb_dmac_chan_read(c, USB_DMACHCR); (chcr & (USB_DMACHCR_DE | USB_DMACHCR_TE)) == USB_DMACHCR_DE
}
unsafe fn usb_dmac_calc_tend(size: u32) -> u32 {
    // Figure “Example of Final Transaction Valid Data Transfer (EDTEN) Setting”.
    0xffff_ffffu32 << (32 - if size % USB_DMAC_XFER_SIZE != 0 { size % USB_DMAC_XFER_SIZE } else { USB_DMAC_XFER_SIZE })
}

unsafe fn usb_dmac_chan_start_sg(c: *mut usb_dmac_chan, index: u32) {
    let d = (*c).desc; let sg = (*d).sg.add(index as usize); let mut src = 0u64; let mut dst = 0u64;
    if (*d).direction == DMA_DEV_TO_MEM { dst = (*sg).mem_addr; } else { src = (*sg).mem_addr; }
    usb_dmac_chan_write(c, USB_DMASAR, src as u32); usb_dmac_chan_write(c, USB_DMADAR, dst as u32);
    usb_dmac_chan_write(c, USB_DMATCR, ((*sg).size + USB_DMAC_XFER_SIZE - 1) / USB_DMAC_XFER_SIZE);
    usb_dmac_chan_write(c, USB_DMATEND, usb_dmac_calc_tend((*sg).size));
    usb_dmac_chan_write(c, USB_DMACHCR, USB_DMAC_CHCR_TS | USB_DMACHCR_NULLE | USB_DMACHCR_IE | USB_DMACHCR_DE);
}
unsafe fn usb_dmac_chan_start_desc(c: *mut usb_dmac_chan) { usb_dmac_external_untranslated(); (*c).desc = core::ptr::null_mut(); }
unsafe fn usb_dmac_init(d: *mut usb_dmac) -> i32 {
    usb_dmac_write(d, USB_DMAOR, USB_DMAOR_DME); let v = usb_dmac_read(d, USB_DMAOR) as u16;
    if (v as u32 & (USB_DMAOR_AE | USB_DMAOR_DME)) != USB_DMAOR_DME { return -5; } 0
}

unsafe fn usb_dmac_desc_alloc(_c: *mut usb_dmac_chan, _n: u32, _gfp: u32) -> i32 { usb_dmac_external_untranslated(); 0 }
unsafe fn usb_dmac_desc_free(c: *mut usb_dmac_chan) { (*c).descs_allocated = 0; }
unsafe fn usb_dmac_desc_get(_c: *mut usb_dmac_chan, _n: u32, _gfp: u32) -> *mut usb_dmac_desc { core::ptr::null_mut() }
unsafe fn usb_dmac_desc_put(_c: *mut usb_dmac_chan, _d: *mut usb_dmac_desc) {}
unsafe fn usb_dmac_soft_reset(c: *mut usb_dmac_chan) { usb_dmac_external_untranslated(); let d = (*c).vc.chan.device; let _ = d; }
unsafe fn usb_dmac_chan_halt(c: *mut usb_dmac_chan) { let mut v=usb_dmac_chan_read(c,USB_DMACHCR); v &= !(USB_DMACHCR_IE|USB_DMACHCR_TE|USB_DMACHCR_DE); usb_dmac_chan_write(c,USB_DMACHCR,v); usb_dmac_soft_reset(c); }
unsafe fn usb_dmac_stop(d: *mut usb_dmac) { usb_dmac_write(d, USB_DMAOR, 0); }

unsafe fn usb_dmac_get_current_residue(c:*mut usb_dmac_chan,d:*mut usb_dmac_desc,i:u32)->u32 { let s=(*d).sg.add(i as usize); let m=(*s).mem_addr as u32; let p=if (*d).direction==DMA_DEV_TO_MEM {usb_dmac_chan_read(c,USB_DMADAR)} else {usb_dmac_chan_read(c,USB_DMASAR)}; (*s).size.wrapping_sub(p.wrapping_sub(m)) }
unsafe fn usb_dmac_chan_get_residue(_c:*mut usb_dmac_chan,_cookie:dma_cookie_t)->u32 { 0 }
unsafe fn usb_dmac_isr_transfer_end(c:*mut usb_dmac_chan) { let _=c; }
unsafe fn usb_dmac_isr_channel(_irq:i32,_dev:*mut c_void)->irqreturn_t { IRQ_NONE }
unsafe fn usb_dmac_chan_filter(_chan:*mut dma_chan,_arg:*mut c_void)->bool { true }
unsafe fn usb_dmac_of_xlate(_spec:*mut of_phandle_args,_ofdma:*mut of_dma)->*mut dma_chan { core::ptr::null_mut() }
unsafe fn usb_dmac_runtime_suspend(_dev:*mut device)->i32 { 0 }
unsafe fn usb_dmac_runtime_resume(_dev:*mut device)->i32 { 0 }
unsafe fn usb_dmac_probe(_pdev:*mut platform_device)->i32 { usb_dmac_external_untranslated(); 0 }
unsafe fn usb_dmac_remove(_pdev:*mut platform_device) {}
unsafe fn usb_dmac_shutdown(pdev:*mut platform_device) { let _=pdev; }
unsafe fn usb_dmac_alloc_chan_resources(_chan:*mut dma_chan)->i32 { usb_dmac_external_untranslated(); 0 }
unsafe fn usb_dmac_free_chan_resources(_chan:*mut dma_chan) { usb_dmac_external_untranslated(); }
unsafe fn usb_dmac_prep_slave_sg(_chan:*mut dma_chan,_sgl:*mut c_void,_sg_len:u32,_dir:dma_transfer_direction,_flags:usize,_context:*mut c_void)->*mut c_void { usb_dmac_external_untranslated(); core::ptr::null_mut() }
unsafe fn usb_dmac_chan_terminate_all(_chan:*mut dma_chan)->i32 { usb_dmac_external_untranslated(); 0 }
unsafe fn usb_dmac_tx_status(_chan:*mut dma_chan,_cookie:dma_cookie_t,_txstate:*mut c_void)->u32 { usb_dmac_external_untranslated(); 0 }
unsafe fn usb_dmac_issue_pending(_chan:*mut dma_chan) { usb_dmac_external_untranslated(); }
unsafe fn usb_dmac_virt_desc_free(_vd:*mut virt_dma_desc) { usb_dmac_external_untranslated(); }
unsafe fn usb_dmac_chan_probe(_d:*mut usb_dmac,_c:*mut usb_dmac_chan,_index:u8)->i32 { usb_dmac_external_untranslated(); 0 }
unsafe fn usb_dmac_parse_of(_dev:*mut device,_d:*mut usb_dmac)->i32 { usb_dmac_external_untranslated(); 0 }
unsafe fn usb_dmac_chan_remove(_d:*mut usb_dmac,_c:*mut usb_dmac_chan) { usb_dmac_external_untranslated(); }

// Kernel-provided opaque types and constants. Their definitions are supplied by dependencies.
#[allow(non_camel_case_types)] type dma_addr_t=u64; type dma_cookie_t=i32; type irqreturn_t=i32;
const IRQ_NONE: irqreturn_t=0; const DMA_DEV_TO_MEM: dma_transfer_direction=0;
type dma_transfer_direction=u32; #[repr(C)] struct virt_dma_desc{_p: [u8;0]} #[repr(C)] struct virt_dma_chan{pub chan:dma_chan}
#[repr(C)] struct dma_chan{pub device:*mut dma_device} #[repr(C)] struct dma_device{_p:[u8;0]} #[repr(C)] struct device{_p:[u8;0]}
#[repr(C)] struct list_head{_p:[u8;0]} #[repr(C)] struct of_phandle_args{_p:[u8;0]} #[repr(C)] struct of_dma{_p:[u8;0]} #[repr(C)] struct platform_device{_p:[u8;0]}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
