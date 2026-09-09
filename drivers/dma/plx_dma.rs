// SPDX-License-Identifier: GPL-2.0
/*
 * Microsemi Switchtec(tm) PCIe Management Driver
 * Copyright (c) 2019, Logan Gunthorpe <logang@deltatee.com>
 * Copyright (c) 2019, GigaIO Networks, Inc
 */

// External kernel declarations and macros are supplied by the surrounding
// Rust kernel environment.

const PLX_REG_DESC_RING_ADDR: usize = 0x214;
const PLX_REG_DESC_RING_ADDR_HI: usize = 0x218;
const PLX_REG_DESC_RING_NEXT_ADDR: usize = 0x21C;
const PLX_REG_DESC_RING_COUNT: usize = 0x220;
const PLX_REG_DESC_RING_LAST_ADDR: usize = 0x224;
const PLX_REG_DESC_RING_LAST_SIZE: usize = 0x228;
const PLX_REG_PREF_LIMIT: usize = 0x234;
const PLX_REG_CTRL: usize = 0x238;
const PLX_REG_CTRL2: usize = 0x23A;
const PLX_REG_INTR_CTRL: usize = 0x23C;
const PLX_REG_INTR_STATUS: usize = 0x23E;

const PLX_REG_PREF_LIMIT_PREF_FOUR: u32 = 8;
const PLX_REG_CTRL_GRACEFUL_PAUSE: u32 = 1 << 0;
const PLX_REG_CTRL_ABORT: u32 = 1 << 1;
const PLX_REG_CTRL_WRITE_BACK_EN: u32 = 1 << 2;
const PLX_REG_CTRL_START: u32 = 1 << 3;
const PLX_REG_CTRL_RING_STOP_MODE: u32 = 1 << 4;
const PLX_REG_CTRL_DESC_MODE_BLOCK: u32 = 0 << 5;
const PLX_REG_CTRL_DESC_MODE_ON_CHIP: u32 = 1 << 5;
const PLX_REG_CTRL_DESC_MODE_OFF_CHIP: u32 = 2 << 5;
const PLX_REG_CTRL_DESC_INVALID: u32 = 1 << 8;
const PLX_REG_CTRL_GRACEFUL_PAUSE_DONE: u32 = 1 << 9;
const PLX_REG_CTRL_ABORT_DONE: u32 = 1 << 10;
const PLX_REG_CTRL_IMM_PAUSE_DONE: u32 = 1 << 12;
const PLX_REG_CTRL_IN_PROGRESS: u32 = 1 << 30;
const PLX_REG_CTRL_RESET_VAL: u32 = PLX_REG_CTRL_DESC_INVALID | PLX_REG_CTRL_GRACEFUL_PAUSE_DONE | PLX_REG_CTRL_ABORT_DONE | PLX_REG_CTRL_IMM_PAUSE_DONE;
const PLX_REG_CTRL_START_VAL: u32 = PLX_REG_CTRL_WRITE_BACK_EN | PLX_REG_CTRL_DESC_MODE_OFF_CHIP | PLX_REG_CTRL_START | PLX_REG_CTRL_RESET_VAL;

const PLX_REG_CTRL2_MAX_TXFR_SIZE_64B: u32 = 0;
const PLX_REG_CTRL2_MAX_TXFR_SIZE_128B: u32 = 1;
const PLX_REG_CTRL2_MAX_TXFR_SIZE_256B: u32 = 2;
const PLX_REG_CTRL2_MAX_TXFR_SIZE_512B: u32 = 3;
const PLX_REG_CTRL2_MAX_TXFR_SIZE_1KB: u32 = 4;
const PLX_REG_CTRL2_MAX_TXFR_SIZE_2KB: u32 = 5;
const PLX_REG_CTRL2_MAX_TXFR_SIZE_4B: u32 = 7;

const PLX_REG_INTR_CRTL_ERROR_EN: u32 = 1 << 0;
const PLX_REG_INTR_CRTL_INV_DESC_EN: u32 = 1 << 1;
const PLX_REG_INTR_CRTL_ABORT_DONE_EN: u32 = 1 << 3;
const PLX_REG_INTR_CRTL_PAUSE_DONE_EN: u32 = 1 << 4;
const PLX_REG_INTR_CRTL_IMM_PAUSE_DONE_EN: u32 = 1 << 5;
const PLX_REG_INTR_STATUS_ERROR: u32 = 1 << 0;
const PLX_REG_INTR_STATUS_INV_DESC: u32 = 1 << 1;
const PLX_REG_INTR_STATUS_DESC_DONE: u32 = 1 << 2;
const PLX_REG_INTR_CRTL_ABORT_DONE: u32 = 1 << 3;

#[repr(C)]
struct plx_dma_hw_std_desc {
    flags_and_size: u32,
    dst_addr_hi: u16,
    src_addr_hi: u16,
    dst_addr_lo: u32,
    src_addr_lo: u32,
}

const PLX_DESC_SIZE_MASK: u32 = 0x7ffffff;
const PLX_DESC_FLAG_VALID: u32 = 1 << 31;
const PLX_DESC_FLAG_INT_WHEN_DONE: u32 = 1 << 30;
const PLX_DESC_WB_SUCCESS: u32 = 1 << 30;
const PLX_DESC_WB_RD_FAIL: u32 = 1 << 29;
const PLX_DESC_WB_WR_FAIL: u32 = 1 << 28;
const PLX_DMA_RING_COUNT: usize = 2048;

#[repr(C)]
struct plx_dma_desc {
    txd: dma_async_tx_descriptor,
    hw: *mut plx_dma_hw_std_desc,
    orig_size: u32,
}

#[repr(C)]
struct plx_dma_dev {
    dma_dev: dma_device,
    dma_chan: dma_chan,
    pdev: *mut pci_dev,
    bar: *mut core::ffi::c_void,
    desc_task: tasklet_struct,
    ring_lock: spinlock_t,
    ring_active: bool,
    head: i32,
    tail: i32,
    hw_ring: *mut plx_dma_hw_std_desc,
    hw_ring_dma: dma_addr_t,
    desc_ring: *mut *mut plx_dma_desc,
}

unsafe fn chan_to_plx_dma_dev(c: *mut dma_chan) -> *mut plx_dma_dev {
    container_of!(c, plx_dma_dev, dma_chan)
}

unsafe fn to_plx_desc(txd: *mut dma_async_tx_descriptor) -> *mut plx_dma_desc {
    container_of!(txd, plx_dma_desc, txd)
}

unsafe fn plx_dma_get_desc(plxdev: *mut plx_dma_dev, i: i32) -> *mut plx_dma_desc {
    *(*plxdev).desc_ring.add((i as usize) & (PLX_DMA_RING_COUNT - 1))
}

unsafe fn plx_dma_process_desc(plxdev: *mut plx_dma_dev) {
    let mut res: dmaengine_result = core::mem::zeroed();
    spin_lock(&mut (*plxdev).ring_lock);
    while (*plxdev).tail != (*plxdev).head {
        let desc = plx_dma_get_desc(plxdev, (*plxdev).tail);
        let flags = core::ptr::read_volatile(&(*(*desc).hw).flags_and_size).to_le();
        if flags & PLX_DESC_FLAG_VALID != 0 { break; }
        res.residue = (*desc).orig_size - (flags & PLX_DESC_SIZE_MASK);
        res.result = if flags & PLX_DESC_WB_SUCCESS != 0 { DMA_TRANS_NOERROR } else if flags & PLX_DESC_WB_WR_FAIL != 0 { DMA_TRANS_WRITE_FAILED } else { DMA_TRANS_READ_FAILED };
        dma_cookie_complete(&mut (*desc).txd);
        dma_descriptor_unmap(&mut (*desc).txd);
        dmaengine_desc_get_callback_invoke(&mut (*desc).txd, &mut res);
        (*desc).txd.callback = None;
        (*desc).txd.callback_result = None;
        (*plxdev).tail += 1;
    }
    spin_unlock(&mut (*plxdev).ring_lock);
}

unsafe fn plx_dma_abort_desc(plxdev: *mut plx_dma_dev) {
    plx_dma_process_desc(plxdev);
    spin_lock_bh(&mut (*plxdev).ring_lock);
    while (*plxdev).tail != (*plxdev).head {
        let desc = plx_dma_get_desc(plxdev, (*plxdev).tail);
        let mut res = dmaengine_result { residue: (*desc).orig_size, result: DMA_TRANS_ABORTED };
        dma_cookie_complete(&mut (*desc).txd);
        dma_descriptor_unmap(&mut (*desc).txd);
        dmaengine_desc_get_callback_invoke(&mut (*desc).txd, &mut res);
        (*desc).txd.callback = None;
        (*desc).txd.callback_result = None;
        (*plxdev).tail += 1;
    }
    spin_unlock_bh(&mut (*plxdev).ring_lock);
}

unsafe fn __plx_dma_stop(plxdev: *mut plx_dma_dev) {
    let timeout = jiffies() + msecs_to_jiffies(1000);
    let mut val = readl((*plxdev).bar.add(PLX_REG_CTRL));
    if val & !PLX_REG_CTRL_GRACEFUL_PAUSE == 0 { return; }
    writel(PLX_REG_CTRL_RESET_VAL | PLX_REG_CTRL_GRACEFUL_PAUSE, (*plxdev).bar.add(PLX_REG_CTRL));
    while !time_after(jiffies(), timeout) { val = readl((*plxdev).bar.add(PLX_REG_CTRL)); if val & PLX_REG_CTRL_GRACEFUL_PAUSE_DONE != 0 { break; } cpu_relax(); }
    if val & PLX_REG_CTRL_GRACEFUL_PAUSE_DONE == 0 { dev_err((*plxdev).dma_dev.dev, "Timeout waiting for graceful pause!\n"); }
    writel(PLX_REG_CTRL_RESET_VAL | PLX_REG_CTRL_GRACEFUL_PAUSE, (*plxdev).bar.add(PLX_REG_CTRL));
    writel(0, (*plxdev).bar.add(PLX_REG_DESC_RING_COUNT)); writel(0, (*plxdev).bar.add(PLX_REG_DESC_RING_ADDR)); writel(0, (*plxdev).bar.add(PLX_REG_DESC_RING_ADDR_HI)); writel(0, (*plxdev).bar.add(PLX_REG_DESC_RING_NEXT_ADDR));
}

unsafe fn plx_dma_stop(plxdev: *mut plx_dma_dev) { rcu_read_lock(); if (*plxdev).pdev.is_null() { rcu_read_unlock(); return; } __plx_dma_stop(plxdev); rcu_read_unlock(); }

unsafe extern "C" fn plx_dma_desc_task(t: *mut tasklet_struct) { let plxdev = from_tasklet!(t, plx_dma_dev, desc_task); plx_dma_process_desc(plxdev); }

unsafe fn plx_dma_prep_memcpy(c: *mut dma_chan, dma_dst: dma_addr_t, dma_src: dma_addr_t, mut len: usize, flags: ulong) -> *mut dma_async_tx_descriptor {
    let dev = chan_to_plx_dma_dev(c); spin_lock_bh(&mut (*dev).ring_lock);
    if !(*dev).ring_active || circ_space((*dev).head, (*dev).tail, PLX_DMA_RING_COUNT as i32) == 0 || len > PLX_DESC_SIZE_MASK as usize { spin_unlock_bh(&mut (*dev).ring_lock); return core::ptr::null_mut(); }
    let desc = plx_dma_get_desc(dev, (*dev).head); (*dev).head += 1;
    (*(*desc).hw).dst_addr_lo = (dma_dst as u32).to_le(); (*(*desc).hw).dst_addr_hi = ((dma_dst >> 32) as u16).to_le();
    (*(*desc).hw).src_addr_lo = (dma_src as u32).to_le(); (*(*desc).hw).src_addr_hi = ((dma_src >> 32) as u16).to_le(); (*desc).orig_size = len as u32;
    if flags & DMA_PREP_INTERRUPT != 0 { len |= PLX_DESC_FLAG_INT_WHEN_DONE as usize; } (*(*desc).hw).flags_and_size = (len as u32).to_le(); (*desc).txd.flags = flags; desc as *mut dma_async_tx_descriptor
}

unsafe fn plx_dma_tx_submit(desc: *mut dma_async_tx_descriptor) -> dma_cookie_t { let dev = chan_to_plx_dma_dev((*desc).chan); let d = to_plx_desc(desc); let cookie = dma_cookie_assign(desc); wmb(); (*(*d).hw).flags_and_size |= (PLX_DESC_FLAG_VALID).to_le(); spin_unlock_bh(&mut (*dev).ring_lock); cookie }
unsafe fn plx_dma_tx_status(chan: *mut dma_chan, cookie: dma_cookie_t, txstate: *mut dma_tx_state) -> dma_status { let dev = chan_to_plx_dma_dev(chan); let ret = dma_cookie_status(chan, cookie, txstate); if ret == DMA_COMPLETE { return ret; } plx_dma_process_desc(dev); dma_cookie_status(chan, cookie, txstate) }
unsafe fn plx_dma_issue_pending(chan: *mut dma_chan) { let dev = chan_to_plx_dma_dev(chan); rcu_read_lock(); if (*dev).pdev.is_null() { rcu_read_unlock(); return; } wmb(); writew(PLX_REG_CTRL_START_VAL as u16, (*dev).bar.add(PLX_REG_CTRL)); rcu_read_unlock(); }
unsafe extern "C" fn plx_dma_isr(_irq: i32, devid: *mut core::ffi::c_void) -> irqreturn_t { let dev = devid as *mut plx_dma_dev; let status = readw((*dev).bar.add(PLX_REG_INTR_STATUS)); if status == 0 { return IRQ_NONE; } if status & PLX_REG_INTR_STATUS_DESC_DONE as u16 != 0 && (*dev).ring_active { tasklet_schedule(&mut (*dev).desc_task); } writew(status, (*dev).bar.add(PLX_REG_INTR_STATUS)); IRQ_HANDLED }

unsafe fn plx_dma_alloc_desc(dev: *mut plx_dma_dev) -> i32 { (*dev).desc_ring = kzalloc_array(PLX_DMA_RING_COUNT); if (*dev).desc_ring.is_null() { return -ENOMEM; } for i in 0..PLX_DMA_RING_COUNT { let d: *mut plx_dma_desc = kzalloc(); if d.is_null() { return -ENOMEM; } dma_async_tx_descriptor_init(&mut (*d).txd, &mut (*dev).dma_chan); (*d).txd.tx_submit = Some(plx_dma_tx_submit); (*d).hw = (*dev).hw_ring.add(i); *(*dev).desc_ring.add(i) = d; } 0 }
unsafe fn plx_dma_alloc_chan_resources(chan: *mut dma_chan) -> i32 { let dev=chan_to_plx_dma_dev(chan); (*dev).head=0; (*dev).tail=0; let sz=PLX_DMA_RING_COUNT*core::mem::size_of::<plx_dma_hw_std_desc>(); (*dev).hw_ring=dma_alloc_coherent((*dev).dma_dev.dev,sz,&mut (*dev).hw_ring_dma,GFP_KERNEL); if (*dev).hw_ring.is_null(){return -ENOMEM;} let rc=plx_dma_alloc_desc(dev); if rc!=0{return rc;} if (*dev).pdev.is_null(){return -ENODEV;} writel(PLX_REG_CTRL_RESET_VAL,(*dev).bar.add(PLX_REG_CTRL)); writel((*dev).hw_ring_dma as u32,(*dev).bar.add(PLX_REG_DESC_RING_ADDR)); writel(((*dev).hw_ring_dma>>32) as u32,(*dev).bar.add(PLX_REG_DESC_RING_ADDR_HI)); writel((*dev).hw_ring_dma as u32,(*dev).bar.add(PLX_REG_DESC_RING_NEXT_ADDR)); writel(PLX_DMA_RING_COUNT as u32,(*dev).bar.add(PLX_REG_DESC_RING_COUNT)); writel(PLX_REG_PREF_LIMIT_PREF_FOUR,(*dev).bar.add(PLX_REG_PREF_LIMIT)); (*dev).ring_active=true; PLX_DMA_RING_COUNT as i32 }
unsafe fn plx_dma_free_chan_resources(chan:*mut dma_chan){let d=chan_to_plx_dma_dev(chan);(*d).ring_active=false;plx_dma_stop(d);plx_dma_abort_desc(d);}
unsafe fn plx_dma_remove(pdev:*mut pci_dev){let d=pci_get_drvdata(pdev) as *mut plx_dma_dev; rcu_assign_pointer(&mut (*d).pdev,core::ptr::null_mut()); synchronize_rcu(); (*d).ring_active=false; __plx_dma_stop(d); plx_dma_abort_desc(d); (*d).bar=core::ptr::null_mut(); dma_async_device_unregister(&mut (*d).dma_dev);}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
