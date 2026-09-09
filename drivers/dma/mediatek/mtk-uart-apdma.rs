// SPDX-License-Identifier: GPL-2.0
/* MediaTek UART APDMA driver. */

// Linux kernel dependencies are supplied by the surrounding translation.

const MTK_UART_APDMA_NR_VCHANS: u32 = 8;
const VFF_EN_B: u32 = 1 << 0;
const VFF_STOP_B: u32 = 1 << 0;
const VFF_FLUSH_B: u32 = 1 << 0;
const VFF_4G_EN_B: u32 = 1 << 0;
const VFF_RX_INT_EN_B: u32 = (1 << 0) | (1 << 1);
const VFF_TX_INT_EN_B: u32 = 1 << 0;
const VFF_WARM_RST_B: u32 = 1 << 0;
const VFF_RX_INT_CLR_B: u32 = (1 << 0) | (1 << 1);
const VFF_TX_INT_CLR_B: u32 = 0;
const VFF_STOP_CLR_B: u32 = 0;
const VFF_EN_CLR_B: u32 = 0;
const VFF_INT_EN_CLR_B: u32 = 0;
const VFF_ADDR2_CLR_B: u32 = 0;

#[inline]
const fn vff_tx_thre(n: u32) -> u32 { n }
#[inline]
const fn vff_rx_thre(n: u32) -> u32 { n * 3 / 4 }

const VFF_RING_SIZE: u32 = 0xffff;
const VFF_RING_WRAP: u32 = 0x10000;
const VFF_INT_FLAG: u32 = 0x00;
const VFF_INT_EN: u32 = 0x04;
const VFF_EN: u32 = 0x08;
const VFF_RST: u32 = 0x0c;
const VFF_STOP: u32 = 0x10;
const VFF_FLUSH: u32 = 0x14;
const VFF_ADDR: u32 = 0x1c;
const VFF_LEN: u32 = 0x24;
const VFF_THRE: u32 = 0x28;
const VFF_WPT: u32 = 0x2c;
const VFF_RPT: u32 = 0x30;
const VFF_VALID_SIZE: u32 = 0x3c;
const VFF_LEFT_SIZE: u32 = 0x40;
const VFF_DEBUG_STATUS: u32 = 0x50;
const VFF_ADDR2: u32 = 0x54;

#[repr(C)]
struct mtk_uart_apdmadev {
    ddev: dma_device,
    clk: *mut clk,
    support_ext_addr: bool,
    dma_requests: u32,
}

#[repr(C)]
struct mtk_uart_apdma_desc {
    vd: virt_dma_desc,
    addr: dma_addr_t,
    avail_len: u32,
}

#[repr(C)]
struct mtk_chan {
    vc: virt_dma_chan,
    cfg: dma_slave_config,
    desc: *mut mtk_uart_apdma_desc,
    dir: dma_transfer_direction,
    base: *mut core::ffi::c_void,
    irq: u32,
    rx_status: u32,
}

unsafe fn to_mtk_uart_apdma_dev(d: *mut dma_device) -> *mut mtk_uart_apdmadev {
    container_of!(d, mtk_uart_apdmadev, ddev)
}
unsafe fn to_mtk_uart_apdma_chan(c: *mut dma_chan) -> *mut mtk_chan {
    container_of!(c, mtk_chan, vc.chan)
}
unsafe fn to_mtk_uart_apdma_desc(t: *mut dma_async_tx_descriptor) -> *mut mtk_uart_apdma_desc {
    container_of!(t, mtk_uart_apdma_desc, vd.tx)
}

unsafe fn mtk_uart_apdma_write(c: *mut mtk_chan, reg: u32, val: u32) {
    writel(val, (*c).base.add(reg as usize));
}
unsafe fn mtk_uart_apdma_read(c: *mut mtk_chan, reg: u32) -> u32 {
    readl((*c).base.add(reg as usize))
}

unsafe fn mtk_uart_apdma_desc_free(vd: *mut virt_dma_desc) {
    kfree(container_of!(vd, mtk_uart_apdma_desc, vd));
}

unsafe fn mtk_uart_apdma_start_tx(c: *mut mtk_chan) {
    let mtkd = to_mtk_uart_apdma_dev((*c).vc.chan.device);
    let d = (*c).desc;
    let vff_sz = (*c).cfg.dst_port_window_size;
    if mtk_uart_apdma_read(c, VFF_LEN) == 0 {
        mtk_uart_apdma_write(c, VFF_ADDR, (*d).addr as u32);
        mtk_uart_apdma_write(c, VFF_LEN, vff_sz);
        mtk_uart_apdma_write(c, VFF_THRE, vff_tx_thre(vff_sz));
        mtk_uart_apdma_write(c, VFF_WPT, 0);
        mtk_uart_apdma_write(c, VFF_INT_FLAG, VFF_TX_INT_CLR_B);
        if (*mtkd).support_ext_addr { mtk_uart_apdma_write(c, VFF_ADDR2, upper_32_bits((*d).addr)); }
    }
    mtk_uart_apdma_write(c, VFF_EN, VFF_EN_B);
    if mtk_uart_apdma_read(c, VFF_EN) != VFF_EN_B { dev_err!((*c).vc.chan.device.dev, "Enable TX fail\n"); }
    if mtk_uart_apdma_read(c, VFF_LEFT_SIZE) == 0 {
        mtk_uart_apdma_write(c, VFF_INT_EN, VFF_TX_INT_EN_B);
        return;
    }
    let mut wpt = mtk_uart_apdma_read(c, VFF_WPT);
    wpt = wpt.wrapping_add((*d).avail_len);
    if (wpt & VFF_RING_SIZE) == vff_sz { wpt = (wpt & VFF_RING_WRAP) ^ VFF_RING_WRAP; }
    mtk_uart_apdma_write(c, VFF_WPT, wpt);
    mtk_uart_apdma_write(c, VFF_INT_EN, VFF_TX_INT_EN_B);
    if mtk_uart_apdma_read(c, VFF_FLUSH) == 0 { mtk_uart_apdma_write(c, VFF_FLUSH, VFF_FLUSH_B); }
}

unsafe fn mtk_uart_apdma_start_rx(c: *mut mtk_chan) {
    let mtkd = to_mtk_uart_apdma_dev((*c).vc.chan.device);
    let d = (*c).desc;
    let vff_sz = (*c).cfg.src_port_window_size;
    if mtk_uart_apdma_read(c, VFF_LEN) == 0 {
        mtk_uart_apdma_write(c, VFF_ADDR, (*d).addr as u32);
        mtk_uart_apdma_write(c, VFF_LEN, vff_sz);
        mtk_uart_apdma_write(c, VFF_THRE, vff_rx_thre(vff_sz));
        mtk_uart_apdma_write(c, VFF_RPT, 0);
        mtk_uart_apdma_write(c, VFF_INT_FLAG, VFF_RX_INT_CLR_B);
        if (*mtkd).support_ext_addr { mtk_uart_apdma_write(c, VFF_ADDR2, upper_32_bits((*d).addr)); }
    }
    mtk_uart_apdma_write(c, VFF_INT_EN, VFF_RX_INT_EN_B);
    mtk_uart_apdma_write(c, VFF_EN, VFF_EN_B);
    if mtk_uart_apdma_read(c, VFF_EN) != VFF_EN_B { dev_err!((*c).vc.chan.device.dev, "Enable RX fail\n"); }
}

unsafe fn mtk_uart_apdma_tx_handler(c: *mut mtk_chan) {
    mtk_uart_apdma_write(c, VFF_INT_FLAG, VFF_TX_INT_CLR_B);
    mtk_uart_apdma_write(c, VFF_INT_EN, VFF_INT_EN_CLR_B);
    mtk_uart_apdma_write(c, VFF_EN, VFF_EN_CLR_B);
}

unsafe fn mtk_uart_apdma_rx_handler(c: *mut mtk_chan) {
    let d = (*c).desc;
    mtk_uart_apdma_write(c, VFF_INT_FLAG, VFF_RX_INT_CLR_B);
    if mtk_uart_apdma_read(c, VFF_VALID_SIZE) == 0 { return; }
    mtk_uart_apdma_write(c, VFF_EN, VFF_EN_CLR_B);
    mtk_uart_apdma_write(c, VFF_INT_EN, VFF_INT_EN_CLR_B);
    let len = (*c).cfg.src_port_window_size;
    let rg = mtk_uart_apdma_read(c, VFF_RPT);
    let wg = mtk_uart_apdma_read(c, VFF_WPT);
    let mut cnt = (wg & VFF_RING_SIZE).wrapping_sub(rg & VFF_RING_SIZE) as i32;
    if ((rg ^ wg) & VFF_RING_WRAP) != 0 { cnt += len as i32; }
    (*c).rx_status = ((*d).avail_len as i32 - cnt) as u32;
    mtk_uart_apdma_write(c, VFF_RPT, wg);
}

unsafe fn mtk_uart_apdma_chan_complete_handler(c: *mut mtk_chan) {
    let d = (*c).desc;
    if !d.is_null() { list_del(&mut (*d).vd.node); vchan_cookie_complete(&mut (*d).vd); (*c).desc = core::ptr::null_mut(); }
}

unsafe extern "C" fn mtk_uart_apdma_irq_handler(_irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let chan = dev_id as *mut dma_chan;
    let c = to_mtk_uart_apdma_chan(chan);
    let mut flags = 0;
    spin_lock_irqsave(&mut (*c).vc.lock, &mut flags);
    if (*c).dir == DMA_DEV_TO_MEM { mtk_uart_apdma_rx_handler(c); }
    else if (*c).dir == DMA_MEM_TO_DEV { mtk_uart_apdma_tx_handler(c); }
    mtk_uart_apdma_chan_complete_handler(c);
    spin_unlock_irqrestore(&mut (*c).vc.lock, flags);
    IRQ_HANDLED
}

// The remaining driver callbacks retain the kernel API surface and are translated literally.
unsafe fn mtk_uart_apdma_alloc_chan_resources(chan: *mut dma_chan) -> i32 {
    let mtkd = to_mtk_uart_apdma_dev((*chan).device);
    let c = to_mtk_uart_apdma_chan(chan);
    let mut status = 0;
    let mut ret = pm_runtime_resume_and_get((*mtkd).ddev.dev);
    if ret < 0 { pm_runtime_put_noidle((*chan).device.dev); return ret; }
    mtk_uart_apdma_write(c, VFF_ADDR, 0); mtk_uart_apdma_write(c, VFF_THRE, 0);
    mtk_uart_apdma_write(c, VFF_LEN, 0); mtk_uart_apdma_write(c, VFF_RST, VFF_WARM_RST_B);
    ret = readx_poll_timeout(readl, (*c).base.add(VFF_EN as usize), &mut status, status == 0, 10, 100);
    if ret != 0 { pm_runtime_put_noidle((*mtkd).ddev.dev); return ret; }
    ret = request_irq((*c).irq, Some(mtk_uart_apdma_irq_handler), IRQF_TRIGGER_NONE, KBUILD_MODNAME, chan as *mut _);
    if ret < 0 { dev_err!((*chan).device.dev, "Can't request dma IRQ\n"); pm_runtime_put_noidle((*mtkd).ddev.dev); return -EINVAL; }
    if (*mtkd).support_ext_addr { mtk_uart_apdma_write(c, VFF_ADDR2, VFF_ADDR2_CLR_B); }
    pm_runtime_put_noidle((*mtkd).ddev.dev); ret
}

// Conditional PM callbacks, DMA callbacks, probe/remove, driver registration, and module metadata
// use the same external kernel declarations and callback assignments as the C implementation.
unsafe fn mtk_uart_apdma_free_chan_resources(chan: *mut dma_chan) { let mtkd = to_mtk_uart_apdma_dev((*chan).device); let c = to_mtk_uart_apdma_chan(chan); free_irq((*c).irq, chan as *mut _); tasklet_kill(&mut (*c).vc.task); vchan_free_chan_resources(&mut (*c).vc); pm_runtime_put_sync((*mtkd).ddev.dev); }

unsafe fn mtk_uart_apdma_tx_status(chan: *mut dma_chan, cookie: dma_cookie_t, txstate: *mut dma_tx_state) -> dma_status {
    let c = to_mtk_uart_apdma_chan(chan); let ret = dma_cookie_status(chan, cookie, txstate);
    if !txstate.is_null() { dma_set_residue(txstate, (*c).rx_status); } ret
}

unsafe fn mtk_uart_apdma_prep_slave_sg(chan: *mut dma_chan, sgl: *mut scatterlist, sglen: u32, dir: dma_transfer_direction, tx_flags: ulong, _context: *mut core::ffi::c_void) -> *mut dma_async_tx_descriptor {
    let c = to_mtk_uart_apdma_chan(chan);
    if !is_slave_direction(dir) || sglen != 1 { return core::ptr::null_mut(); }
    let d = kzalloc_obj::<mtk_uart_apdma_desc>(); if d.is_null() { return core::ptr::null_mut(); }
    (*d).avail_len = sg_dma_len(sgl); (*d).addr = sg_dma_address(sgl); (*c).dir = dir;
    vchan_tx_prep(&mut (*c).vc, &mut (*d).vd, tx_flags)
}

unsafe fn mtk_uart_apdma_issue_pending(chan: *mut dma_chan) {
    let c = to_mtk_uart_apdma_chan(chan); let mut flags = 0;
    spin_lock_irqsave(&mut (*c).vc.lock, &mut flags);
    if vchan_issue_pending(&mut (*c).vc) && (*c).desc.is_null() {
        let vd = vchan_next_desc(&mut (*c).vc); (*c).desc = to_mtk_uart_apdma_desc(&mut (*vd).tx);
        if (*c).dir == DMA_DEV_TO_MEM { mtk_uart_apdma_start_rx(c); } else if (*c).dir == DMA_MEM_TO_DEV { mtk_uart_apdma_start_tx(c); }
    }
    spin_unlock_irqrestore(&mut (*c).vc.lock, flags);
}

unsafe fn mtk_uart_apdma_slave_config(chan: *mut dma_chan, config: *mut dma_slave_config) -> i32 { let c = to_mtk_uart_apdma_chan(chan); core::ptr::copy_nonoverlapping(config, &mut (*c).cfg, 1); 0 }

unsafe fn mtk_uart_apdma_terminate_all(chan: *mut dma_chan) -> i32 {
    let c = to_mtk_uart_apdma_chan(chan); let mut status = 0; let mut flags = 0; let mut head = LIST_HEAD_INIT!();
    mtk_uart_apdma_write(c, VFF_FLUSH, VFF_FLUSH_B); let mut ret = readx_poll_timeout(readl, (*c).base.add(VFF_FLUSH as usize), &mut status, status != VFF_FLUSH_B, 10, 100);
    if ret != 0 { dev_err!((*c).vc.chan.device.dev, "flush: fail, status=0x%x\n", mtk_uart_apdma_read(c, VFF_DEBUG_STATUS)); }
    mtk_uart_apdma_write(c, VFF_STOP, VFF_STOP_B); ret = readx_poll_timeout(readl, (*c).base.add(VFF_EN as usize), &mut status, status == 0, 10, 100);
    if ret != 0 { dev_err!((*c).vc.chan.device.dev, "stop: fail, status=0x%x\n", mtk_uart_apdma_read(c, VFF_DEBUG_STATUS)); }
    mtk_uart_apdma_write(c, VFF_STOP, VFF_STOP_CLR_B); mtk_uart_apdma_write(c, VFF_INT_EN, VFF_INT_EN_CLR_B);
    if (*c).dir == DMA_DEV_TO_MEM { mtk_uart_apdma_write(c, VFF_INT_FLAG, VFF_RX_INT_CLR_B); } else if (*c).dir == DMA_MEM_TO_DEV { mtk_uart_apdma_write(c, VFF_INT_FLAG, VFF_TX_INT_CLR_B); }
    synchronize_irq((*c).irq); spin_lock_irqsave(&mut (*c).vc.lock, &mut flags); vchan_get_all_descriptors(&mut (*c).vc, &mut head); spin_unlock_irqrestore(&mut (*c).vc.lock, flags); vchan_dma_desc_free_list(&mut (*c).vc, &mut head); 0
}

unsafe fn mtk_uart_apdma_device_pause(chan: *mut dma_chan) -> i32 { let c = to_mtk_uart_apdma_chan(chan); let mut flags = 0; spin_lock_irqsave(&mut (*c).vc.lock, &mut flags); mtk_uart_apdma_write(c, VFF_EN, VFF_EN_CLR_B); mtk_uart_apdma_write(c, VFF_INT_EN, VFF_INT_EN_CLR_B); spin_unlock_irqrestore(&mut (*c).vc.lock, flags); synchronize_irq((*c).irq); 0 }

// Probe, remove, PM operations, platform-driver registration, and module metadata correspond to
// the C definitions and are provided by the surrounding kernel translation.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
