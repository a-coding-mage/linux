// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2019 Linaro Ltd.
// Copyright (C) 2019 Socionext Inc.

// Linux kernel dependencies are supplied by the surrounding translation.

const MLB_HDMAC_DMACR: usize = 0x0;
const MLB_HDMAC_DE: u32 = 1 << 31;
const MLB_HDMAC_DS: u32 = 1 << 30;
const MLB_HDMAC_PR: u32 = 1 << 28;
const MLB_HDMAC_DH: u32 = 0xf << 24;
const MLB_HDMAC_CH_STRIDE: usize = 0x10;
const MLB_HDMAC_DMACA: usize = 0x0;
const MLB_HDMAC_EB: u32 = 1 << 31;
const MLB_HDMAC_PB: u32 = 1 << 30;
const MLB_HDMAC_ST: u32 = 1 << 29;
const MLB_HDMAC_IS: u32 = 0x1f << 24;
const MLB_HDMAC_BT: u32 = 0xf << 20;
const MLB_HDMAC_BC: u32 = 0xf << 16;
const MLB_HDMAC_TC: u32 = 0xffff;
const MLB_HDMAC_DMACB: usize = 0x4;
const MLB_HDMAC_TT: u32 = 0x3 << 30;
const MLB_HDMAC_MS: u32 = 0x3 << 28;
const MLB_HDMAC_TW: u32 = 0x3 << 26;
const MLB_HDMAC_FS: u32 = 1 << 25;
const MLB_HDMAC_FD: u32 = 1 << 24;
const MLB_HDMAC_RC: u32 = 1 << 23;
const MLB_HDMAC_RS: u32 = 1 << 22;
const MLB_HDMAC_RD: u32 = 1 << 21;
const MLB_HDMAC_EI: u32 = 1 << 20;
const MLB_HDMAC_CI: u32 = 1 << 19;
const HDMAC_PAUSE: u32 = 0x7;
const MLB_HDMAC_SS: u32 = 0x7 << 16;
const MLB_HDMAC_SP: u32 = 0xf << 12;
const MLB_HDMAC_DP: u32 = 0xf << 8;
const MLB_HDMAC_DMACSA: usize = 0x8;
const MLB_HDMAC_DMACDA: usize = 0xc;
const MLB_HDMAC_BUSWIDTHS: u32 = (1 << DMA_SLAVE_BUSWIDTH_1_BYTE) |
    (1 << DMA_SLAVE_BUSWIDTH_2_BYTES) | (1 << DMA_SLAVE_BUSWIDTH_4_BYTES);

#[repr(C)]
struct milbeaut_hdmac_desc {
    vd: virt_dma_desc,
    sgl: *mut scatterlist,
    sg_len: u32,
    sg_cur: u32,
    dir: dma_transfer_direction,
}
#[repr(C)]
struct milbeaut_hdmac_chan {
    vc: virt_dma_chan,
    mdev: *mut milbeaut_hdmac_device,
    md: *mut milbeaut_hdmac_desc,
    reg_ch_base: *mut core::ffi::c_void,
    slave_id: u32,
    cfg: dma_slave_config,
}
#[repr(C)]
struct milbeaut_hdmac_device {
    ddev: dma_device,
    clk: *mut clk,
    reg_base: *mut core::ffi::c_void,
    channels: [milbeaut_hdmac_chan; 0],
}

unsafe fn to_milbeaut_hdmac_chan(vc: *mut virt_dma_chan) -> *mut milbeaut_hdmac_chan {
    container_of!(vc, milbeaut_hdmac_chan, vc)
}
unsafe fn to_milbeaut_hdmac_desc(vd: *mut virt_dma_desc) -> *mut milbeaut_hdmac_desc {
    container_of!(vd, milbeaut_hdmac_desc, vd)
}

unsafe fn milbeaut_hdmac_next_desc(mc: *mut milbeaut_hdmac_chan) -> *mut milbeaut_hdmac_desc {
    let vd = vchan_next_desc(&mut (*mc).vc);
    if vd.is_null() { (*mc).md = core::ptr::null_mut(); return core::ptr::null_mut(); }
    list_del(&mut (*vd).node);
    (*mc).md = to_milbeaut_hdmac_desc(vd);
    (*mc).md
}

unsafe fn milbeaut_chan_start(mc: *mut milbeaut_hdmac_chan, md: *mut milbeaut_hdmac_desc) {
    let sg = (*md).sgl.add((*md).sg_cur as usize);
    let len = sg_dma_len(sg);
    let mut cb = MLB_HDMAC_CI | MLB_HDMAC_EI;
    let (width, burst, src_addr, dest_addr) = if (*md).dir == DMA_MEM_TO_DEV {
        cb |= MLB_HDMAC_FD;
        ((*mc).cfg.dst_addr_width, (*mc).cfg.dst_maxburst, sg_dma_address(sg), (*mc).cfg.dst_addr)
    } else {
        cb |= MLB_HDMAC_FS;
        ((*mc).cfg.src_addr_width, (*mc).cfg.src_maxburst, (*mc).cfg.src_addr, sg_dma_address(sg))
    };
    cb |= field_prep(MLB_HDMAC_TW, width >> 1);
    cb |= field_prep(MLB_HDMAC_MS, 2);
    writel_relaxed(MLB_HDMAC_DE, (*(*mc).mdev).reg_base.add(MLB_HDMAC_DMACR));
    writel_relaxed(src_addr, (*mc).reg_ch_base.add(MLB_HDMAC_DMACSA));
    writel_relaxed(dest_addr, (*mc).reg_ch_base.add(MLB_HDMAC_DMACDA));
    writel_relaxed(cb, (*mc).reg_ch_base.add(MLB_HDMAC_DMACB));
    let mut ca = field_prep(MLB_HDMAC_IS, (*mc).slave_id);
    if burst == 16 { ca |= field_prep(MLB_HDMAC_BT, 0xf); }
    else if burst == 8 { ca |= field_prep(MLB_HDMAC_BT, 0xd); }
    else if burst == 4 { ca |= field_prep(MLB_HDMAC_BT, 0xb); }
    let burst = burst * width;
    ca |= field_prep(MLB_HDMAC_TC, len / burst - 1);
    writel_relaxed(ca, (*mc).reg_ch_base.add(MLB_HDMAC_DMACA));
    writel_relaxed(ca | MLB_HDMAC_EB, (*mc).reg_ch_base.add(MLB_HDMAC_DMACA));
}

unsafe fn milbeaut_hdmac_start(mc: *mut milbeaut_hdmac_chan) {
    let md = milbeaut_hdmac_next_desc(mc);
    if !md.is_null() { milbeaut_chan_start(mc, md); }
}

unsafe extern "C" fn milbeaut_hdmac_interrupt(_irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let mc = dev_id as *mut milbeaut_hdmac_chan;
    spin_lock(&mut (*mc).vc.lock);
    let mut val = readl_relaxed((*mc).reg_ch_base.add(MLB_HDMAC_DMACB));
    val &= !field_prep(MLB_HDMAC_SS, HDMAC_PAUSE);
    writel_relaxed(val, (*mc).reg_ch_base.add(MLB_HDMAC_DMACB));
    val &= !MLB_HDMAC_EI; val &= !MLB_HDMAC_CI;
    writel_relaxed(val, (*mc).reg_ch_base.add(MLB_HDMAC_DMACB));
    let mut md = (*mc).md;
    if md.is_null() { spin_unlock(&mut (*mc).vc.lock); return IRQ_HANDLED; }
    (*md).sg_cur += 1;
    if (*md).sg_cur >= (*md).sg_len {
        vchan_cookie_complete(&mut (*md).vd);
        md = milbeaut_hdmac_next_desc(mc);
        if md.is_null() { spin_unlock(&mut (*mc).vc.lock); return IRQ_HANDLED; }
    }
    milbeaut_chan_start(mc, md);
    spin_unlock(&mut (*mc).vc.lock);
    IRQ_HANDLED
}

// The remaining callbacks retain the kernel DMA-engine interfaces and helper calls verbatim.
// Their declarations are supplied by the surrounding Linux Rust bindings.
unsafe fn milbeaut_hdmac_free_chan_resources(chan: *mut dma_chan) { vchan_free_chan_resources(to_virt_chan(chan)); }
unsafe fn milbeaut_hdmac_chan_config(chan: *mut dma_chan, cfg: *mut dma_slave_config) -> i32 { let vc=to_virt_chan(chan); let mc=to_milbeaut_hdmac_chan(vc); spin_lock(&mut (*mc).vc.lock); (*mc).cfg=*cfg; spin_unlock(&mut (*mc).vc.lock); 0 }
unsafe fn milbeaut_hdmac_chan_pause(chan: *mut dma_chan) -> i32 { let mc=to_milbeaut_hdmac_chan(to_virt_chan(chan)); spin_lock(&mut (*mc).vc.lock); let v=readl_relaxed((*mc).reg_ch_base.add(MLB_HDMAC_DMACA))|MLB_HDMAC_PB; writel_relaxed(v,(*mc).reg_ch_base.add(MLB_HDMAC_DMACA)); spin_unlock(&mut (*mc).vc.lock); 0 }
unsafe fn milbeaut_hdmac_chan_resume(chan: *mut dma_chan) -> i32 { let mc=to_milbeaut_hdmac_chan(to_virt_chan(chan)); spin_lock(&mut (*mc).vc.lock); let v=readl_relaxed((*mc).reg_ch_base.add(MLB_HDMAC_DMACA))&!MLB_HDMAC_PB; writel_relaxed(v,(*mc).reg_ch_base.add(MLB_HDMAC_DMACA)); spin_unlock(&mut (*mc).vc.lock); 0 }

unsafe fn milbeaut_hdmac_prep_slave_sg(chan:*mut dma_chan, sgl:*mut scatterlist, sg_len:u32, direction:dma_transfer_direction, flags:usize, _context:*mut core::ffi::c_void)->*mut dma_async_tx_descriptor {
    if !is_slave_direction(direction) { return core::ptr::null_mut(); }
    let vc=to_virt_chan(chan); let md=kzalloc::<milbeaut_hdmac_desc>(GFP_NOWAIT); if md.is_null(){return core::ptr::null_mut();}
    (*md).sgl=kzalloc_array::<scatterlist>(sg_len as usize,GFP_NOWAIT); if (*md).sgl.is_null(){kfree(md as *mut _);return core::ptr::null_mut();}
    for i in 0..sg_len as usize { *(*md).sgl.add(i)=*sgl.add(i); }
    (*md).sg_len=sg_len; (*md).dir=direction; vchan_tx_prep(vc,&mut (*md).vd,flags)
}
unsafe fn milbeaut_hdmac_terminate_all(chan:*mut dma_chan)->i32 { let vc=to_virt_chan(chan); let mc=to_milbeaut_hdmac_chan(vc); let mut head=list_head(); spin_lock_irqsave(&mut (*vc).lock); let mut v=readl_relaxed((*mc).reg_ch_base.add(MLB_HDMAC_DMACA))&!MLB_HDMAC_EB; writel_relaxed(v,(*mc).reg_ch_base.add(MLB_HDMAC_DMACA)); if !(*mc).md.is_null(){vchan_terminate_vdesc(&mut (*(*mc).md).vd);(*mc).md=core::ptr::null_mut();} vchan_get_all_descriptors(vc,&mut head); spin_unlock_irqrestore(&mut (*vc).lock); vchan_dma_desc_free_list(vc,&mut head); 0 }
unsafe fn milbeaut_hdmac_synchronize(chan:*mut dma_chan){vchan_synchronize(to_virt_chan(chan));}
unsafe fn milbeaut_hdmac_tx_status(chan:*mut dma_chan,cookie:dma_cookie_t,txstate:*mut dma_tx_state)->dma_status { let stat=dma_cookie_status(chan,cookie,txstate); if stat==DMA_COMPLETE||txstate.is_null(){return stat;} let vc=to_virt_chan(chan); let mc=to_milbeaut_hdmac_chan(vc); spin_lock_irqsave(&mut (*vc).lock); if !(*mc).md.is_null()&&(*(*mc).md).vd.tx.cookie==cookie {let md=(*mc).md;let sg=(*md).sgl.add((*md).sg_cur as usize);let done=if (*md).dir==DMA_DEV_TO_MEM{readl_relaxed((*mc).reg_ch_base.add(MLB_HDMAC_DMACDA))}else{readl_relaxed((*mc).reg_ch_base.add(MLB_HDMAC_DMACSA))};(*txstate).residue=sg_dma_len(sg).wrapping_sub(done.wrapping_sub(sg_dma_address(sg)));} spin_unlock_irqrestore(&mut (*vc).lock); stat }
unsafe fn milbeaut_hdmac_issue_pending(chan:*mut dma_chan){let vc=to_virt_chan(chan);let mc=to_milbeaut_hdmac_chan(vc);spin_lock_irqsave(&mut (*vc).lock);if vchan_issue_pending(vc)&&(*mc).md.is_null(){milbeaut_hdmac_start(mc);}spin_unlock_irqrestore(&mut (*vc).lock);}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
