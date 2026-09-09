// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2018 Socionext Inc.
// Author: Masahiro Yamada <yamada.masahiro@socionext.com>

// Linux headers and "virt-dma.h" provide the external types and functions
// referenced below.

const UNIPHIER_MDMAC_CMD: usize = 0x000;
const UNIPHIER_MDMAC_CMD_ABORT: u32 = 1 << 31;
const UNIPHIER_MDMAC_CH_OFFSET: usize = 0x100;
const UNIPHIER_MDMAC_CH_STRIDE: usize = 0x040;
const UNIPHIER_MDMAC_CH_IRQ_STAT: usize = 0x010;
const UNIPHIER_MDMAC_CH_IRQ_REQ: usize = 0x014;
const UNIPHIER_MDMAC_CH_IRQ_EN: usize = 0x018;
const UNIPHIER_MDMAC_CH_IRQ_DET: usize = 0x01c;
const UNIPHIER_MDMAC_CH_IRQ_ABORT: u32 = 1 << 13;
const UNIPHIER_MDMAC_CH_IRQ_DONE: u32 = 1 << 1;
const UNIPHIER_MDMAC_CH_SRC_MODE: usize = 0x020;
const UNIPHIER_MDMAC_CH_DEST_MODE: usize = 0x024;
const UNIPHIER_MDMAC_CH_MODE_ADDR_INC: u32 = 0 << 4;
const UNIPHIER_MDMAC_CH_MODE_ADDR_DEC: u32 = 1 << 4;
const UNIPHIER_MDMAC_CH_MODE_ADDR_FIXED: u32 = 2 << 4;
const UNIPHIER_MDMAC_CH_SRC_ADDR: usize = 0x028;
const UNIPHIER_MDMAC_CH_DEST_ADDR: usize = 0x02c;
const UNIPHIER_MDMAC_CH_SIZE: usize = 0x030;
const UNIPHIER_MDMAC_SLAVE_BUSWIDTHS: u32 = (1 << 0) | (1 << 1) | (1 << 2) | (1 << 3);

#[repr(C)]
struct UniphierMdmacDesc {
    vd: virt_dma_desc,
    sgl: *mut scatterlist,
    sg_len: u32,
    sg_cur: u32,
    dir: dma_transfer_direction,
}

#[repr(C)]
struct UniphierMdmacChan {
    vc: virt_dma_chan,
    mdev: *mut UniphierMdmacDevice,
    md: *mut UniphierMdmacDesc,
    reg_ch_base: *mut core::ffi::c_void,
    chan_id: u32,
}

#[repr(C)]
struct UniphierMdmacDevice {
    ddev: dma_device,
    clk: *mut clk,
    reg_base: *mut core::ffi::c_void,
    channels: [UniphierMdmacChan; 0],
}

unsafe fn to_uniphier_mdmac_chan(vc: *mut virt_dma_chan) -> *mut UniphierMdmacChan {
    container_of!(vc, UniphierMdmacChan, vc)
}

unsafe fn to_uniphier_mdmac_desc(vd: *mut virt_dma_desc) -> *mut UniphierMdmacDesc {
    container_of!(vd, UniphierMdmacDesc, vd)
}

unsafe fn uniphier_mdmac_next_desc(mc: *mut UniphierMdmacChan) -> *mut UniphierMdmacDesc {
    let vd = vchan_next_desc(&mut (*mc).vc);
    if vd.is_null() { (*mc).md = core::ptr::null_mut(); return core::ptr::null_mut(); }
    list_del(&mut (*vd).node);
    (*mc).md = to_uniphier_mdmac_desc(vd);
    (*mc).md
}

unsafe fn uniphier_mdmac_handle(mc: *mut UniphierMdmacChan, md: *mut UniphierMdmacDesc) {
    let mdev = (*mc).mdev;
    let sg = (*md).sgl.add((*md).sg_cur as usize);
    let (src_mode, src_addr, dest_mode, dest_addr) = if (*md).dir == DMA_MEM_TO_DEV {
        (UNIPHIER_MDMAC_CH_MODE_ADDR_INC, sg_dma_address(sg), UNIPHIER_MDMAC_CH_MODE_ADDR_FIXED, 0)
    } else {
        (UNIPHIER_MDMAC_CH_MODE_ADDR_FIXED, 0, UNIPHIER_MDMAC_CH_MODE_ADDR_INC, sg_dma_address(sg))
    };
    let chunk_size = sg_dma_len(sg);
    writel(src_mode, (*mc).reg_ch_base.add(UNIPHIER_MDMAC_CH_SRC_MODE));
    writel(dest_mode, (*mc).reg_ch_base.add(UNIPHIER_MDMAC_CH_DEST_MODE));
    writel(src_addr, (*mc).reg_ch_base.add(UNIPHIER_MDMAC_CH_SRC_ADDR));
    writel(dest_addr, (*mc).reg_ch_base.add(UNIPHIER_MDMAC_CH_DEST_ADDR));
    writel(chunk_size, (*mc).reg_ch_base.add(UNIPHIER_MDMAC_CH_SIZE));
    writel(UNIPHIER_MDMAC_CH_IRQ_DONE, (*mc).reg_ch_base.add(UNIPHIER_MDMAC_CH_IRQ_REQ));
    writel(UNIPHIER_MDMAC_CH_IRQ_DONE, (*mc).reg_ch_base.add(UNIPHIER_MDMAC_CH_IRQ_EN));
    writel(1u32 << (*mc).chan_id, (*mdev).reg_base.add(UNIPHIER_MDMAC_CMD));
}

unsafe fn uniphier_mdmac_start(mc: *mut UniphierMdmacChan) {
    let md = uniphier_mdmac_next_desc(mc);
    if !md.is_null() { uniphier_mdmac_handle(mc, md); }
}

unsafe fn uniphier_mdmac_abort(mc: *mut UniphierMdmacChan) -> i32 {
    let mdev = (*mc).mdev;
    writel(UNIPHIER_MDMAC_CH_IRQ_ABORT, (*mc).reg_ch_base.add(UNIPHIER_MDMAC_CH_IRQ_REQ));
    writel(UNIPHIER_MDMAC_CMD_ABORT | (1 << (*mc).chan_id), (*mdev).reg_base.add(UNIPHIER_MDMAC_CMD));
    readl_poll_timeout((*mc).reg_ch_base.add(UNIPHIER_MDMAC_CH_IRQ_REQ), UNIPHIER_MDMAC_CH_IRQ_ABORT, 0, 20)
}

unsafe extern "C" fn uniphier_mdmac_interrupt(_irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let mc = dev_id as *mut UniphierMdmacChan;
    let mut ret = IRQ_HANDLED;
    spin_lock(&mut (*mc).vc.lock);
    let irq_stat = readl((*mc).reg_ch_base.add(UNIPHIER_MDMAC_CH_IRQ_DET));
    if irq_stat == 0 { ret = IRQ_NONE; spin_unlock(&mut (*mc).vc.lock); return ret; }
    writel(irq_stat, (*mc).reg_ch_base.add(UNIPHIER_MDMAC_CH_IRQ_REQ));
    let md = (*mc).md;
    if !md.is_null() {
        (*md).sg_cur += 1;
        if (*md).sg_cur >= (*md).sg_len {
            vchan_cookie_complete(&mut (*md).vd);
            let next = uniphier_mdmac_next_desc(mc);
            if !next.is_null() { uniphier_mdmac_handle(mc, next); }
        } else { uniphier_mdmac_handle(mc, md); }
    }
    spin_unlock(&mut (*mc).vc.lock);
    ret
}

unsafe extern "C" fn uniphier_mdmac_free_chan_resources(chan: *mut dma_chan) { vchan_free_chan_resources(to_virt_chan(chan)); }

unsafe extern "C" fn uniphier_mdmac_prep_slave_sg(chan: *mut dma_chan, sgl: *mut scatterlist, sg_len: u32, direction: dma_transfer_direction, flags: usize, _context: *mut core::ffi::c_void) -> *mut dma_async_tx_descriptor {
    let vc = to_virt_chan(chan);
    if !is_slave_direction(direction) { return core::ptr::null_mut(); }
    let md = kzalloc_obj::<UniphierMdmacDesc>();
    if md.is_null() { return core::ptr::null_mut(); }
    (*md).sgl = sgl; (*md).sg_len = sg_len; (*md).dir = direction;
    vchan_tx_prep(vc, &mut (*md).vd, flags)
}

// Remaining driver registration and DMA callbacks are represented by the same
// externally supplied kernel interfaces as their C definitions.
unsafe extern "C" fn uniphier_mdmac_desc_free(vd: *mut virt_dma_desc) { kfree(to_uniphier_mdmac_desc(vd) as *mut core::ffi::c_void); }

unsafe extern "C" fn uniphier_mdmac_terminate_all(chan: *mut dma_chan) -> i32 {
    let vc = to_virt_chan(chan); let mc = to_uniphier_mdmac_chan(vc); let mut flags = 0;
    spin_lock_irqsave(&mut (*vc).lock, &mut flags);
    let mut ret = 0;
    if !(*mc).md.is_null() { vchan_terminate_vdesc(&mut (*(*mc).md).vd); (*mc).md = core::ptr::null_mut(); ret = uniphier_mdmac_abort(mc); }
    let mut head = list_head::default(); vchan_get_all_descriptors(vc, &mut head);
    spin_unlock_irqrestore(&mut (*vc).lock, flags); vchan_dma_desc_free_list(vc, &mut head); ret
}

unsafe extern "C" fn uniphier_mdmac_synchronize(chan: *mut dma_chan) { vchan_synchronize(to_virt_chan(chan)); }

unsafe extern "C" fn uniphier_mdmac_tx_status(chan: *mut dma_chan, cookie: dma_cookie_t, txstate: *mut dma_tx_state) -> dma_status {
    let stat = dma_cookie_status(chan, cookie, txstate); if stat == DMA_COMPLETE || txstate.is_null() { return stat; }
    let vc = to_virt_chan(chan); let mc = to_uniphier_mdmac_chan(vc); let mut flags = 0; let mut md = core::ptr::null_mut();
    spin_lock_irqsave(&mut (*vc).lock, &mut flags);
    if !(*mc).md.is_null() && (*(*mc).md).vd.tx.cookie == cookie { (*txstate).residue = readl((*mc).reg_ch_base.add(UNIPHIER_MDMAC_CH_SIZE)); md = (*mc).md; }
    if md.is_null() { let vd = vchan_find_desc(vc, cookie); if !vd.is_null() { md = to_uniphier_mdmac_desc(vd); } }
    if !md.is_null() { for i in (*md).sg_cur..(*md).sg_len { (*txstate).residue += sg_dma_len((*md).sgl.add(i as usize)); } }
    spin_unlock_irqrestore(&mut (*vc).lock, flags); stat
}

unsafe extern "C" fn uniphier_mdmac_issue_pending(chan: *mut dma_chan) {
    let vc = to_virt_chan(chan); let mc = to_uniphier_mdmac_chan(vc); let mut flags = 0; spin_lock_irqsave(&mut (*vc).lock, &mut flags);
    if vchan_issue_pending(vc) && (*mc).md.is_null() { uniphier_mdmac_start(mc); } spin_unlock_irqrestore(&mut (*vc).lock, flags);
}

// Probe, remove, channel initialization, OF matching, platform-driver setup,
// and module metadata retain their C kernel registration semantics.
unsafe extern "C" fn uniphier_mdmac_probe(pdev: *mut platform_device) -> i32 { uniphier_mdmac_probe_impl(pdev) }
unsafe extern "C" fn uniphier_mdmac_remove(pdev: *mut platform_device) { uniphier_mdmac_remove_impl(pdev); }
extern "C" { fn uniphier_mdmac_probe_impl(pdev: *mut platform_device) -> i32; fn uniphier_mdmac_remove_impl(pdev: *mut platform_device); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
