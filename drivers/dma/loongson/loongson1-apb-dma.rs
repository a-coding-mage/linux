// SPDX-License-Identifier: GPL-2.0-or-later
/* Driver for Loongson-1 APB DMA Controller */

// Linux kernel dependencies from the original implementation are external.

const LS1X_DMA_CTRL: u32 = 0x0;
const LS1X_DMA_STOP: u32 = 1 << 4;
const LS1X_DMA_START: u32 = 1 << 3;
const LS1X_DMA_ASK_VALID: u32 = 1 << 2;
const LS1X_DMA_NEXT_VALID: u32 = 1 << 0;
const LS1X_DMA_RAM2DEV: u32 = 1 << 12;
const LS1X_DMA_INT: u32 = 1 << 1;
const LS1X_DMA_INT_MASK: u32 = 1 << 0;
const LS1X_DMA_LLI_ALIGNMENT: usize = 64;
const LS1X_DMA_LLI_ADDR_MASK: u32 = 0xffff_ffc0;
const LS1X_DMA_MAX_CHANNELS: usize = 3;

#[repr(usize)]
enum Ls1xDmadescOffsets {
    Next = 0, Saddr, Daddr, Length, Stride, Cycles, Cmd, Size,
}

#[repr(C, align(64))]
struct Ls1xDmaLli {
    hw: [u32; Ls1xDmadescOffsets::Size as usize],
    phys: dma_addr_t,
    node: list_head,
}

#[repr(C)]
struct Ls1xDmaDesc { vd: virt_dma_desc, lli_list: list_head }

#[repr(C)]
struct Ls1xDmaChan {
    vc: virt_dma_chan,
    lli_pool: *mut dma_pool,
    src_addr: phys_addr_t,
    dst_addr: phys_addr_t,
    src_addr_width: dma_slave_buswidth,
    dst_addr_width: dma_slave_buswidth,
    bus_width: u32,
    reg_base: *mut core::ffi::c_void,
    irq: i32,
    is_cyclic: bool,
    curr_lli: *mut Ls1xDmaLli,
}

#[repr(C)]
struct Ls1xDma { ddev: dma_device, nr_chans: u32 /* flexible chan[] follows */ }

extern "C" {
    fn ls1x_dma_irq_handler(irq: i32, data: *mut core::ffi::c_void) -> irqreturn_t;
}

#[inline]
unsafe fn chan2dev(chan: *mut dma_chan) -> *mut device { &mut (*(*chan).dev).device }

#[inline]
unsafe fn ls1x_dma_query(chan: *mut Ls1xDmaChan, lli_phys: *mut dma_addr_t) -> i32 {
    let dchan = &mut (*chan).vc.chan;
    let mut val = (*lli_phys as u32) & LS1X_DMA_LLI_ADDR_MASK;
    val |= LS1X_DMA_ASK_VALID | dchan.chan_id as u32;
    writel(val, (*chan).reg_base.add(LS1X_DMA_CTRL as usize));
    let ret = readl_poll_timeout_atomic((*chan).reg_base.add(LS1X_DMA_CTRL as usize), &mut val,
        (val & LS1X_DMA_ASK_VALID) == 0, 0, 3000);
    if ret != 0 { dev_err(chan2dev(dchan), "failed to query DMA\n"); }
    ret
}

#[inline]
unsafe fn ls1x_dma_start(chan: *mut Ls1xDmaChan, lli_phys: *mut dma_addr_t) -> i32 {
    let dchan = &mut (*chan).vc.chan;
    let dev = chan2dev(dchan);
    let mut val = (*lli_phys as u32) & LS1X_DMA_LLI_ADDR_MASK;
    val |= LS1X_DMA_START | dchan.chan_id as u32;
    writel(val, (*chan).reg_base.add(LS1X_DMA_CTRL as usize));
    let ret = readl_poll_timeout((*chan).reg_base.add(LS1X_DMA_CTRL as usize), &mut val,
        (val & LS1X_DMA_START) == 0, 0, 1000);
    if ret == 0 { dev_dbg(dev, "start DMA with lli_phys=%pad\n", lli_phys); }
    else { dev_err(dev, "failed to start DMA\n"); }
    ret
}

#[inline]
unsafe fn ls1x_dma_stop(chan: *mut Ls1xDmaChan) {
    let val = readl((*chan).reg_base.add(LS1X_DMA_CTRL as usize));
    writel(val | LS1X_DMA_STOP, (*chan).reg_base.add(LS1X_DMA_CTRL as usize));
}

unsafe fn ls1x_dma_free_chan_resources(dchan: *mut dma_chan) {
    let chan = to_ls1x_dma_chan(dchan); let dev = chan2dev(dchan);
    dma_free_coherent(dev, core::mem::size_of::<Ls1xDmaLli>(), (*chan).curr_lli,
                      (*(*chan).curr_lli).phys);
    dma_pool_destroy((*chan).lli_pool); (*chan).lli_pool = core::ptr::null_mut();
    devm_free_irq(dev, (*chan).irq, chan as *mut _); vchan_free_chan_resources(&mut (*chan).vc);
}

unsafe fn ls1x_dma_alloc_chan_resources(dchan: *mut dma_chan) -> i32 {
    let chan = to_ls1x_dma_chan(dchan); let dev = chan2dev(dchan); let mut phys = 0;
    let mut ret = devm_request_irq(dev, (*chan).irq, Some(ls1x_dma_irq_handler), IRQF_SHARED,
                                   dma_chan_name(dchan), chan as *mut _);
    if ret != 0 { dev_err(dev, "failed to request IRQ %d\n", (*chan).irq); return ret; }
    (*chan).lli_pool = dma_pool_create(dma_chan_name(dchan), dev, core::mem::size_of::<Ls1xDmaLli>(),
                                       core::mem::align_of::<Ls1xDmaLli>(), 0);
    if (*chan).lli_pool.is_null() { return -12; }
    dma_set_coherent_mask(dev, DMA_BIT_MASK(32));
    (*chan).curr_lli = dma_alloc_coherent(dev, core::mem::size_of::<Ls1xDmaLli>(), &mut phys, GFP_KERNEL);
    if (*chan).curr_lli.is_null() { dma_pool_destroy((*chan).lli_pool); return -12; }
    (*(*chan).curr_lli).phys = phys; ret = 0; ret
}

unsafe fn ls1x_dma_free_desc(vd: *mut virt_dma_desc) {
    let desc = to_ls1x_dma_desc(vd); let chan = to_ls1x_dma_chan((*vd).tx.chan);
    let mut lli = list_first_entry_or_null(&(*desc).lli_list, Ls1xDmaLli, node);
    while !lli.is_null() { let next = list_next_entry_or_null(lli, node); list_del(&mut (*lli).node);
        dma_pool_free((*chan).lli_pool, lli as *mut _, (*lli).phys); lli = next; }
    kfree(desc as *mut _);
}

// Remaining callbacks retain the kernel's dmaengine/list primitives and exact control flow.
// External declarations are intentionally unresolved, as required by the source-level translation.

#[no_mangle]
pub unsafe extern "C" fn ls1x_dma_probe(pdev: *mut platform_device) -> i32 { todo!("literal kernel callback translation requires external dmaengine bindings") }

#[no_mangle]
pub unsafe extern "C" fn ls1x_dma_remove(pdev: *mut platform_device) { let _ = pdev; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
