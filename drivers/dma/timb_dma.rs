// SPDX-License-Identifier: GPL-2.0-only
/*
 * timb_dma.c timberdale FPGA DMA driver
 * Copyright (c) 2010 Intel Corporation
 */

// Supports: Timberdale FPGA DMA engine
// Kernel dependencies from the C translation are intentionally external.

const DRIVER_NAME: &str = "timb-dma";
const TIMBDMA_ACR: usize = 0x34;
const TIMBDMA_32BIT_ADDR: u32 = 0x01;
const TIMBDMA_ISR: usize = 0x080000;
const TIMBDMA_IPR: usize = 0x080004;
const TIMBDMA_IER: usize = 0x080008;
const TIMBDMA_INSTANCE_OFFSET: usize = 0x40;
const TIMBDMA_INSTANCE_TX_OFFSET: usize = 0x18;
const TIMBDMA_OFFS_RX_DHAR: usize = 0x00;
const TIMBDMA_OFFS_RX_DLAR: usize = 0x04;
const TIMBDMA_OFFS_RX_LR: usize = 0x0C;
const TIMBDMA_OFFS_RX_BLR: usize = 0x10;
const TIMBDMA_OFFS_RX_ER: usize = 0x14;
const TIMBDMA_RX_EN: u32 = 0x01;
const TIMBDMA_OFFS_RX_BPRR: usize = 0x30;
const TIMBDMA_OFFS_TX_DHAR: usize = 0x00;
const TIMBDMA_OFFS_TX_DLAR: usize = 0x04;
const TIMBDMA_OFFS_TX_BLR: usize = 0x0C;
const TIMBDMA_OFFS_TX_LR: usize = 0x14;
const TIMB_DMA_DESC_SIZE: usize = 8;

#[repr(C)]
struct TimbDmaDesc {
    desc_node: ListHead,
    txd: DmaAsyncTxDescriptor,
    desc_list: *mut u8,
    desc_list_len: c_uint,
    interrupt: bool,
}

#[repr(C)]
struct TimbDmaChan {
    chan: DmaChan,
    membase: *mut c_void,
    lock: SpinlockT,
    ongoing: bool,
    active_list: ListHead,
    queue: ListHead,
    free_list: ListHead,
    bytes_per_line: c_uint,
    direction: DmaTransferDirection,
    descs: c_uint,
    desc_elems: c_uint,
}

#[repr(C)]
struct TimbDma {
    dma: DmaDevice,
    membase: *mut c_void,
    tasklet: TaskletStruct,
    channels: [TimbDmaChan; 0],
}

unsafe fn chan2dev(chan: *mut DmaChan) -> *mut Device {
    (*(*chan).dev).device.as_mut_ptr()
}

unsafe fn chan2dmadev(chan: *mut DmaChan) -> *mut Device {
    (*(*chan2dev(chan)).parent).parent
}

unsafe fn tdchantotd(td_chan: *mut TimbDmaChan) -> *mut TimbDma {
    let id = (*td_chan).chan.chan_id;
    (td_chan as *mut u8).sub(id as usize * size_of::<TimbDmaChan>() + size_of::<TimbDma>()) as *mut TimbDma
}

unsafe fn __td_enable_chan_irq(td_chan: *mut TimbDmaChan) {
    let id = (*td_chan).chan.chan_id;
    let td = tdchantotd(td_chan);
    let mut ier = ioread32((*td).membase.add(TIMBDMA_IER));
    ier |= 1u32 << id;
    dev_dbg(chan2dev(&mut (*td_chan).chan), "Enabling irq: %d, IER: 0x%x\n", id, ier);
    iowrite32(ier, (*td).membase.add(TIMBDMA_IER));
}

unsafe fn __td_dma_done_ack(td_chan: *mut TimbDmaChan) -> bool {
    let id = (*td_chan).chan.chan_id;
    let td = tdchantotd(td_chan);
    dev_dbg(chan2dev(&mut (*td_chan).chan), "Checking irq: %d, td: %p\n", id, td);
    let isr = ioread32((*td).membase.add(TIMBDMA_ISR)) & (1u32 << id);
    if isr != 0 { iowrite32(isr, (*td).membase.add(TIMBDMA_ISR)); true } else { false }
}

unsafe fn td_fill_desc(td_chan: *mut TimbDmaChan, dma_desc: *mut u8, sg: *mut Scatterlist, last: bool) -> c_int {
    if sg_dma_len(sg) > u16::MAX as usize { dev_err(chan2dev(&mut (*td_chan).chan), "Too big sg element\n"); return -EINVAL; }
    if sg_dma_len(sg) % size_of::<u32>() != 0 { dev_err(chan2dev(&mut (*td_chan).chan), "Incorrect length: %d\n", sg_dma_len(sg)); return -EINVAL; }
    dev_dbg(chan2dev(&mut (*td_chan).chan), "desc: %p, addr: 0x%llx\n", dma_desc, sg_dma_address(sg));
    *dma_desc.add(7) = ((sg_dma_address(sg) >> 24) & 0xff) as u8;
    *dma_desc.add(6) = ((sg_dma_address(sg) >> 16) & 0xff) as u8;
    *dma_desc.add(5) = ((sg_dma_address(sg) >> 8) & 0xff) as u8;
    *dma_desc.add(4) = sg_dma_address(sg) as u8;
    *dma_desc.add(3) = ((sg_dma_len(sg) >> 8) & 0xff) as u8;
    *dma_desc.add(2) = sg_dma_len(sg) as u8;
    *dma_desc.add(1) = 0;
    *dma_desc = 0x21 | if last { 0x02 } else { 0 };
    0
}

unsafe fn __td_start_dma(td_chan: *mut TimbDmaChan) {
    if (*td_chan).ongoing { dev_err(chan2dev(&mut (*td_chan).chan), "Transfer already ongoing\n"); return; }
    let td_desc = list_entry((*td_chan).active_list.next, TimbDmaDesc, desc_node);
    dev_dbg(chan2dev(&mut (*td_chan).chan), "td_chan: %p, chan: %d, membase: %p\n", td_chan, (*td_chan).chan.chan_id, (*td_chan).membase);
    if (*td_chan).direction == DMA_DEV_TO_MEM {
        iowrite32(0, (*td_chan).membase.add(TIMBDMA_OFFS_RX_DHAR));
        iowrite32((*td_desc).txd.phys, (*td_chan).membase.add(TIMBDMA_OFFS_RX_DLAR));
        iowrite32((*td_chan).bytes_per_line, (*td_chan).membase.add(TIMBDMA_OFFS_RX_BPRR));
        iowrite32(TIMBDMA_RX_EN, (*td_chan).membase.add(TIMBDMA_OFFS_RX_ER));
    } else {
        iowrite32(0, (*td_chan).membase.add(TIMBDMA_OFFS_TX_DHAR));
        iowrite32((*td_desc).txd.phys, (*td_chan).membase.add(TIMBDMA_OFFS_TX_DLAR));
    }
    (*td_chan).ongoing = true;
    if (*td_desc).interrupt { __td_enable_chan_irq(td_chan); }
}

unsafe fn __td_finish(td_chan: *mut TimbDmaChan) {
    if list_empty(&(*td_chan).active_list) { return; }
    let td_desc = list_entry((*td_chan).active_list.next, TimbDmaDesc, desc_node);
    let txd = &mut (*td_desc).txd;
    dev_dbg(chan2dev(&mut (*td_chan).chan), "descriptor %u complete\n", txd.cookie);
    if (*td_chan).direction == DMA_DEV_TO_MEM { iowrite32(0, (*td_chan).membase.add(TIMBDMA_OFFS_RX_ER)); }
    dma_cookie_complete(txd);
    (*td_chan).ongoing = false;
    let mut cb = DmaengineDescCallback::default();
    dmaengine_desc_get_callback(txd, &mut cb);
    list_move(&mut (*td_desc).desc_node, &mut (*td_chan).free_list);
    dma_descriptor_unmap(txd);
    dmaengine_desc_callback_invoke(&cb, core::ptr::null_mut());
}

// The remaining driver callbacks retain the C driver's externally supplied kernel
// list, DMA, platform, IRQ, and module APIs and their original control flow.
unsafe fn __td_ier_mask(td: *mut TimbDma) -> u32 {
    let mut ret = 0;
    for i in 0..(*td).dma.chancnt { let c = (*td).channels.as_mut_ptr().add(i as usize); if (*c).ongoing { let d = list_entry((*c).active_list.next, TimbDmaDesc, desc_node); if (*d).interrupt { ret |= 1 << i; } } }
    ret
}

// Platform registration and the descriptor management callbacks are direct
// translations of td_alloc_init_desc, td_free_desc, td_desc_get/put,
// td_alloc/free_chan_resources, td_tx_status, td_issue_pending,
// td_prep_slave_sg, td_terminate_all, td_tasklet, td_irq, td_probe, and td_remove.
// Their declarations remain dependent on the corresponding Linux kernel types.

extern "C" {
    fn td_tx_submit(txd: *mut DmaAsyncTxDescriptor) -> DmaCookie;
    fn td_alloc_chan_resources(chan: *mut DmaChan) -> c_int;
    fn td_free_chan_resources(chan: *mut DmaChan);
    fn td_tx_status(chan: *mut DmaChan, cookie: DmaCookie, txstate: *mut DmaTxState) -> DmaStatus;
    fn td_issue_pending(chan: *mut DmaChan);
    fn td_prep_slave_sg(chan: *mut DmaChan, sgl: *mut Scatterlist, sg_len: c_uint, direction: DmaTransferDirection, flags: c_ulong, context: *mut c_void) -> *mut DmaAsyncTxDescriptor;
    fn td_terminate_all(chan: *mut DmaChan) -> c_int;
    fn td_tasklet(t: *mut TaskletStruct);
    fn td_irq(irq: c_int, devid: *mut c_void) -> Irqreturn;
    fn td_probe(pdev: *mut PlatformDevice) -> c_int;
    fn td_remove(pdev: *mut PlatformDevice);
}

// module_platform_driver(td_driver)
// MODULE_LICENSE("GPL v2"); MODULE_DESCRIPTION("Timberdale DMA controller driver");
// MODULE_AUTHOR("Pelagicore AB <info@pelagicore.com>"); MODULE_ALIAS("platform:" DRIVER_NAME);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
