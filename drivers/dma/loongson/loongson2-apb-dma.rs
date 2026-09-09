// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Driver for the Loongson-2 APB DMA Controller
 *
 * Copyright (C) 2017-2023 Loongson Corporation
 */

// Linux kernel dependencies are supplied by the surrounding translation.

/* Global Configuration Register */
const LDMA_ORDER_ERG: usize = 0x0;

/* Bitfield definitions */
const LDMA_64BIT_EN: u64 = BIT(0); // 1: 64 bit support
const LDMA_UNCOHERENT_EN: u64 = BIT(1); // 0: cache, 1: uncache
const LDMA_ASK_VALID: u64 = BIT(2);
const LDMA_START: u64 = BIT(3); // DMA start operation
const LDMA_STOP: u64 = BIT(4); // DMA stop operation
const LDMA_CONFIG_MASK: u64 = GENMASK_ULL(4, 0); // DMA controller config bits mask

/* Bitfields in ndesc_addr field of HW descriptor */
const LDMA_DESC_EN: u32 = BIT(0); //1: The next descriptor is valid
const LDMA_DESC_ADDR_LOW: u32 = GENMASK(31, 1);

/* Bitfields in cmd field of HW descriptor */
const LDMA_INT: u32 = BIT(1); // Enable DMA interrupts
const LDMA_DATA_DIRECTION: u32 = BIT(12); // 1: write to device, 0: read from device

const LDMA_SLAVE_BUSWIDTHS: u32 =
    BIT(DMA_SLAVE_BUSWIDTH_4_BYTES) | BIT(DMA_SLAVE_BUSWIDTH_8_BYTES);
const LDMA_MAX_TRANS_LEN: u32 = U32_MAX;

/*--  descriptors  -----------------------------------------------------*/

#[repr(C, packed)]
struct ls2x_dma_hw_desc {
    ndesc_addr: u32,
    mem_addr: u32,
    apb_addr: u32,
    len: u32,
    step_len: u32,
    step_times: u32,
    cmd: u32,
    stats: u32,
    high_ndesc_addr: u32,
    high_mem_addr: u32,
    reserved: [u32; 2],
}

#[repr(C)]
struct ls2x_dma_sg {
    hw: *mut ls2x_dma_hw_desc,
    llp: dma_addr_t,
    phys: dma_addr_t,
    len: u32,
}

#[repr(C)]
struct ls2x_dma_desc {
    vdesc: virt_dma_desc,
    cyclic: bool,
    burst_size: usize,
    desc_num: u32,
    direction: dma_transfer_direction,
    status: dma_status,
    sg: [ls2x_dma_sg; 0],
}

/*--  Channels  --------------------------------------------------------*/

#[repr(C)]
struct ls2x_dma_chan {
    vchan: virt_dma_chan,
    desc: *mut ls2x_dma_desc,
    pool: *mut core::ffi::c_void,
    irq: i32,
    sconfig: dma_slave_config,
}

/*--  Controller  ------------------------------------------------------*/

#[repr(C)]
struct ls2x_dma_priv {
    ddev: dma_device,
    dma_clk: *mut clk,
    regs: *mut core::ffi::c_void,
    lchan: ls2x_dma_chan,
}

/*--  Helper functions  ------------------------------------------------*/

unsafe fn to_ldma_desc(vdesc: *mut virt_dma_desc) -> *mut ls2x_dma_desc {
    container_of!(vdesc, ls2x_dma_desc, vdesc)
}

unsafe fn to_ldma_chan(chan: *mut dma_chan) -> *mut ls2x_dma_chan {
    container_of!(chan, ls2x_dma_chan, vchan.chan)
}

unsafe fn to_ldma_priv(ddev: *mut dma_device) -> *mut ls2x_dma_priv {
    container_of!(ddev, ls2x_dma_priv, ddev)
}

unsafe fn chan2dev(chan: *mut dma_chan) -> *mut device {
    &mut (*(*chan).dev).device
}

unsafe extern "C" fn ls2x_dma_desc_free(vdesc: *mut virt_dma_desc) {
    let lchan = to_ldma_chan((*vdesc).tx.chan);
    let desc = to_ldma_desc(vdesc);
    let mut i = 0;

    while i < (*desc).desc_num {
        let sg = &mut (*desc).sg.add(i as usize);
        if !(*sg).hw.is_null() {
            dma_pool_free((*lchan).pool, (*sg).hw, (*sg).llp);
        }
        i += 1;
    }
    kfree(desc as *mut core::ffi::c_void);
}

unsafe fn ls2x_dma_write_cmd(lchan: *mut ls2x_dma_chan, cmd: bool) {
    let priv_ = to_ldma_priv((*lchan).vchan.chan.device);
    let mut val = lo_hi_readq((*priv_).regs.add(LDMA_ORDER_ERG)) & !LDMA_CONFIG_MASK;
    val |= LDMA_64BIT_EN | cmd as u64;
    lo_hi_writeq(val, (*priv_).regs.add(LDMA_ORDER_ERG));
}

unsafe fn ls2x_dma_start_transfer(lchan: *mut ls2x_dma_chan) {
    let priv_ = to_ldma_priv((*lchan).vchan.chan.device);
    let vdesc = vchan_next_desc(&mut (*lchan).vchan);
    if vdesc.is_null() {
        (*lchan).desc = core::ptr::null_mut();
        return;
    }

    list_del(&mut (*vdesc).node);
    (*lchan).desc = to_ldma_desc(vdesc);
    let ldma_sg = (*lchan).desc.cast::<u8>()
        .add(core::mem::offset_of!(ls2x_dma_desc, sg))
        .cast::<ls2x_dma_sg>();

    lo_hi_writeq(0, (*priv_).regs.add(LDMA_ORDER_ERG));
    let val = ((*ldma_sg).llp & !LDMA_CONFIG_MASK) | LDMA_64BIT_EN | LDMA_START;
    lo_hi_writeq(val, (*priv_).regs.add(LDMA_ORDER_ERG));
}

unsafe fn ls2x_dmac_detect_burst(lchan: *mut ls2x_dma_chan) -> usize {
    if ((*lchan).sconfig.src_addr_width & LDMA_SLAVE_BUSWIDTHS) != 0
        && ((*lchan).sconfig.dst_addr_width & LDMA_SLAVE_BUSWIDTHS) != 0
    {
        return 0;
    }
    let (maxburst, buswidth) = if (*lchan).sconfig.direction == DMA_MEM_TO_DEV {
        ((*lchan).sconfig.dst_maxburst, (*lchan).sconfig.dst_addr_width)
    } else {
        ((*lchan).sconfig.src_maxburst, (*lchan).sconfig.src_addr_width)
    };
    if maxburst != 0 { ((maxburst * buswidth) >> 2) as usize } else { LDMA_MAX_TRANS_LEN as usize }
}

unsafe fn ls2x_dma_fill_desc(lchan: *mut ls2x_dma_chan, sg_index: u32, desc: *mut ls2x_dma_desc) {
    let ldma_sg = &mut *(*desc).sg.add(sg_index as usize);
    if (*desc).direction == DMA_MEM_TO_DEV {
        (*ldma_sg).hw.as_mut().unwrap().cmd = LDMA_INT | LDMA_DATA_DIRECTION;
        (*ldma_sg).hw.as_mut().unwrap().apb_addr = (*lchan).sconfig.dst_addr;
    } else {
        (*ldma_sg).hw.as_mut().unwrap().cmd = LDMA_INT;
        (*ldma_sg).hw.as_mut().unwrap().apb_addr = (*lchan).sconfig.src_addr;
    }
    (*ldma_sg).hw.as_mut().unwrap().mem_addr = lower_32_bits((*ldma_sg).phys);
    (*ldma_sg).hw.as_mut().unwrap().high_mem_addr = upper_32_bits((*ldma_sg).phys);
    let num_segments = DIV_ROUND_UP(((*ldma_sg).len + 3) >> 2, (*desc).burst_size as u32);
    let segment_size = DIV_ROUND_UP(((*ldma_sg).len + 3) >> 2, num_segments);
    (*ldma_sg).hw.as_mut().unwrap().len = segment_size;
    (*ldma_sg).hw.as_mut().unwrap().step_times = num_segments;
    (*ldma_sg).hw.as_mut().unwrap().step_len = 0;
    if sg_index != 0 {
        let prev = &mut *(*desc).sg.add((sg_index - 1) as usize);
        prev.hw.as_mut().unwrap().ndesc_addr = (*ldma_sg).llp as u32 | LDMA_DESC_EN;
        prev.hw.as_mut().unwrap().high_ndesc_addr = upper_32_bits((*ldma_sg).llp);
    }
}

// DMA Engine API callbacks and driver registration are preserved below as
// direct kernel-facing declarations/definitions; their external kernel types
// and helpers are supplied by the surrounding translation.

unsafe extern "C" fn ls2x_dma_alloc_chan_resources(chan: *mut dma_chan) -> i32 {
    let lchan = to_ldma_chan(chan);
    (*lchan).pool = dma_pool_create(dev_name(chan2dev(chan)), (*chan).device.dev,
        PAGE_SIZE, core::mem::align_of::<ls2x_dma_hw_desc>(), 0);
    if (*lchan).pool.is_null() { dev_err(chan2dev(chan), "No memory for descriptors\n"); return -ENOMEM; }
    1
}

unsafe extern "C" fn ls2x_dma_free_chan_resources(chan: *mut dma_chan) {
    let lchan = to_ldma_chan(chan);
    vchan_free_chan_resources(to_virt_chan(chan));
    dma_pool_destroy((*lchan).pool);
    (*lchan).pool = core::ptr::null_mut();
}

unsafe extern "C" fn ls2x_dma_prep_slave_sg(chan: *mut dma_chan, sgl: *mut scatterlist,
    sg_len: u32, direction: dma_transfer_direction, flags: c_ulong, _context: *mut c_void)
    -> *mut dma_async_tx_descriptor {
    let lchan = to_ldma_chan(chan);
    if sg_len == 0 || !is_slave_direction(direction) { return core::ptr::null_mut(); }
    let burst_size = ls2x_dmac_detect_burst(lchan); if burst_size == 0 { return core::ptr::null_mut(); }
    let desc = kzalloc_flex::<ls2x_dma_desc>(sg_len); if desc.is_null() { return core::ptr::null_mut(); }
    (*desc).desc_num = sg_len; (*desc).direction = direction; (*desc).burst_size = burst_size;
    for i in 0..sg_len {
        let ldma_sg = &mut *(*desc).sg.add(i as usize);
        ldma_sg.hw = dma_pool_alloc((*lchan).pool, GFP_NOWAIT, &mut ldma_sg.llp);
        if ldma_sg.hw.is_null() { (*desc).desc_num = i; ls2x_dma_desc_free(&mut (*desc).vdesc); return core::ptr::null_mut(); }
        let sg = for_each_sg_entry(sgl, i); ldma_sg.phys = sg_dma_address(sg); ldma_sg.len = sg_dma_len(sg);
        ls2x_dma_fill_desc(lchan, i, desc);
    }
    (*(*desc).sg.add((sg_len - 1) as usize)).hw.as_mut().unwrap().ndesc_addr &= !LDMA_DESC_EN;
    (*desc).status = DMA_IN_PROGRESS;
    vchan_tx_prep(&mut (*lchan).vchan, &mut (*desc).vdesc, flags)
}

unsafe extern "C" fn ls2x_dma_prep_dma_cyclic(chan: *mut dma_chan, buf_addr: dma_addr_t,
    buf_len: usize, period_len: usize, direction: dma_transfer_direction, flags: c_ulong)
    -> *mut dma_async_tx_descriptor {
    let lchan = to_ldma_chan(chan);
    if buf_len == 0 || period_len == 0 || !is_slave_direction(direction) { return core::ptr::null_mut(); }
    let burst_size = ls2x_dmac_detect_burst(lchan); if burst_size == 0 { return core::ptr::null_mut(); }
    let num_periods = (buf_len / period_len) as u32;
    let desc = kzalloc_flex::<ls2x_dma_desc>(num_periods); if desc.is_null() { return core::ptr::null_mut(); }
    (*desc).desc_num = num_periods; (*desc).direction = direction; (*desc).burst_size = burst_size;
    for i in 0..num_periods {
        let ldma_sg = &mut *(*desc).sg.add(i as usize);
        ldma_sg.hw = dma_pool_alloc((*lchan).pool, GFP_NOWAIT, &mut ldma_sg.llp);
        if ldma_sg.hw.is_null() { (*desc).desc_num = i; ls2x_dma_desc_free(&mut (*desc).vdesc); return core::ptr::null_mut(); }
        ldma_sg.phys = buf_addr + period_len as u64 * i as u64; ldma_sg.len = period_len as u32;
        ls2x_dma_fill_desc(lchan, i, desc);
    }
    let last = &mut *(*desc).sg.add((num_periods - 1) as usize); let first = &*(*desc).sg;
    last.hw.as_mut().unwrap().ndesc_addr = first.llp as u32 | LDMA_DESC_EN;
    last.hw.as_mut().unwrap().high_ndesc_addr = upper_32_bits(first.llp);
    (*desc).cyclic = true; (*desc).status = DMA_IN_PROGRESS;
    vchan_tx_prep(&mut (*lchan).vchan, &mut (*desc).vdesc, flags)
}

unsafe extern "C" fn ls2x_dma_slave_config(chan: *mut dma_chan, config: *mut dma_slave_config) -> i32 {
    let lchan = to_ldma_chan(chan); core::ptr::copy_nonoverlapping(config, &mut (*lchan).sconfig, 1); 0
}
unsafe extern "C" fn ls2x_dma_issue_pending(chan: *mut dma_chan) { let lchan = to_ldma_chan(chan); guard_spinlock_irqsave!(&mut (*lchan).vchan.lock); if vchan_issue_pending(&mut (*lchan).vchan) && (*lchan).desc.is_null() { ls2x_dma_start_transfer(lchan); } }
unsafe extern "C" fn ls2x_dma_terminate_all(chan: *mut dma_chan) -> i32 { let lchan = to_ldma_chan(chan); ls2x_dma_write_cmd(lchan, LDMA_STOP != 0); if !(*lchan).desc.is_null() { vchan_terminate_vdesc(&mut (*(*lchan).desc).vdesc); (*lchan).desc = core::ptr::null_mut(); } vchan_free_all_descriptors(&mut (*lchan).vchan); 0 }
unsafe extern "C" fn ls2x_dma_synchronize(chan: *mut dma_chan) { vchan_synchronize(&mut (*to_ldma_chan(chan)).vchan); }
unsafe extern "C" fn ls2x_dma_pause(chan: *mut dma_chan) -> i32 { let lchan = to_ldma_chan(chan); if !(*lchan).desc.is_null() && (*(*lchan).desc).status == DMA_IN_PROGRESS { ls2x_dma_write_cmd(lchan, true); (*(*lchan).desc).status = DMA_PAUSED; } 0 }
unsafe extern "C" fn ls2x_dma_resume(chan: *mut dma_chan) -> i32 { let lchan = to_ldma_chan(chan); if !(*lchan).desc.is_null() && (*(*lchan).desc).status == DMA_PAUSED { (*(*lchan).desc).status = DMA_IN_PROGRESS; ls2x_dma_write_cmd(lchan, true); } 0 }

unsafe extern "C" fn ls2x_dma_isr(_irq: i32, dev_id: *mut c_void) -> irqreturn_t {
    let lchan = dev_id as *mut ls2x_dma_chan; let desc = (*lchan).desc;
    if !desc.is_null() { if (*desc).cyclic { vchan_cyclic_callback(&mut (*desc).vdesc); } else { (*desc).status = DMA_COMPLETE; vchan_cookie_complete(&mut (*desc).vdesc); ls2x_dma_start_transfer(lchan); } if (*lchan).desc.is_null() { ls2x_dma_write_cmd(lchan, true); } }
    IRQ_HANDLED
}

// Probe, remove, OF matching, platform-driver registration, and module metadata
// retain their kernel-facing interfaces and are supplied by the surrounding API.

unsafe fn ls2x_dma_chan_init(pdev: *mut platform_device, priv_: *mut ls2x_dma_priv) -> i32 {
    let lchan = &mut (*priv_).lchan;
    lchan.irq = platform_get_irq(pdev, 0); if lchan.irq < 0 { return lchan.irq; }
    let ret = devm_request_irq(&mut (*pdev).dev, lchan.irq, ls2x_dma_isr, IRQF_TRIGGER_RISING, dev_name(&mut (*pdev).dev), lchan as *mut _ as *mut c_void);
    if ret != 0 { return ret; }
    init_list_head(&mut (*priv_).ddev.channels); lchan.vchan.desc_free = Some(ls2x_dma_desc_free); vchan_init(&mut lchan.vchan, &mut (*priv_).ddev); 0
}

unsafe extern "C" fn ls2x_dma_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev; let priv_ = devm_kzalloc(dev, core::mem::size_of::<ls2x_dma_priv>(), GFP_KERNEL) as *mut ls2x_dma_priv; if priv_.is_null() { return -ENOMEM; }
    (*priv_).regs = devm_platform_ioremap_resource(pdev, 0); if is_err((*priv_).regs) { return dev_err_probe(dev, ptr_err((*priv_).regs), "devm_platform_ioremap_resource failed.\n"); }
    (*priv_).dma_clk = devm_clk_get_enabled(dev, core::ptr::null()); if is_err((*priv_).dma_clk) { return dev_err_probe(dev, ptr_err((*priv_).dma_clk), "Couldn't start the clock.\n"); }
    let ret = ls2x_dma_chan_init(pdev, priv_); if ret != 0 { return ret; }
    let ddev = &mut (*priv_).ddev; ddev.dev = dev; dma_cap_zero(&mut ddev.cap_mask); dma_cap_set(DMA_SLAVE, &mut ddev.cap_mask); dma_cap_set(DMA_CYCLIC, &mut ddev.cap_mask);
    ddev.device_alloc_chan_resources = Some(ls2x_dma_alloc_chan_resources); ddev.device_free_chan_resources = Some(ls2x_dma_free_chan_resources); ddev.device_tx_status = Some(dma_cookie_status); ddev.device_issue_pending = Some(ls2x_dma_issue_pending); ddev.device_prep_slave_sg = Some(ls2x_dma_prep_slave_sg); ddev.device_prep_dma_cyclic = Some(ls2x_dma_prep_dma_cyclic); ddev.device_config = Some(ls2x_dma_slave_config); ddev.device_terminate_all = Some(ls2x_dma_terminate_all); ddev.device_synchronize = Some(ls2x_dma_synchronize); ddev.device_pause = Some(ls2x_dma_pause); ddev.device_resume = Some(ls2x_dma_resume);
    ddev.src_addr_widths = LDMA_SLAVE_BUSWIDTHS; ddev.dst_addr_widths = LDMA_SLAVE_BUSWIDTHS; ddev.directions = BIT(DMA_DEV_TO_MEM) | BIT(DMA_MEM_TO_DEV);
    let ret = dmaenginem_async_device_register(ddev); if ret < 0 { return dev_err_probe(dev, ret, "Failed to register DMA engine device.\n"); }
    let ret = of_dma_controller_register(dev.of_node, of_dma_xlate_by_chan_id, priv_ as *mut c_void); if ret < 0 { return dev_err_probe(dev, ret, "Failed to register dma controller.\n"); }
    platform_set_drvdata(pdev, priv_ as *mut c_void); dev_info(dev, "Loongson LS2X APB DMA driver registered successfully.\n"); 0
}

unsafe extern "C" fn ls2x_dma_remove(pdev: *mut platform_device) { of_dma_controller_free((*pdev).dev.of_node); }

static mut ls2x_dma_of_match_table: [of_device_id; 2] = [
    of_device_id { compatible: "loongson,ls2k1000-apbdma", ..OF_DEVICE_ID_EMPTY },
    OF_DEVICE_ID_EMPTY,
];

static mut ls2x_dmac_driver: platform_driver = platform_driver {
    probe: Some(ls2x_dma_probe), remove: Some(ls2x_dma_remove), driver: driver {
        name: "ls2x-apbdma", of_match_table: ls2x_dma_of_match_table.as_ptr(), ..DRIVER_DEFAULT
    },
};

module_platform_driver!(ls2x_dmac_driver);
module_description!("Loongson-2 APB DMA Controller driver");
module_author!("Loongson Technology Corporation Limited");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
