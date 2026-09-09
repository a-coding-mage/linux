// SPDX-License-Identifier: GPL-2.0-only
/* Offload engine driver for the Marvell XOR engine. */

// Kernel headers and local headers from the C implementation are external
// dependencies and are intentionally not reimplemented here.

#[repr(C)]
#[derive(Copy, Clone)]
pub enum MvXorType { XorOrion, XorArmada38x, XorArmada37xx }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum MvXorMode { InReg, InDesc }

extern "C" {
    fn mv_xor_issue_pending(chan: *mut dma_chan);
}

/* The following declarations mirror the C driver's external kernel types. */
#[allow(non_camel_case_types)] type dma_addr_t = usize;
#[allow(non_camel_case_types)] type dma_cookie_t = i32;
#[allow(non_camel_case_types)] type u8 = core::primitive::u8;
#[allow(non_camel_case_types)] type u32 = core::primitive::u32;
#[allow(non_camel_case_types)] type dma_cap_mask_t = usize;
#[repr(C)] pub struct dma_chan { _private: [u8; 0] }
#[repr(C)] pub struct dma_async_tx_descriptor { _private: [u8; 0] }
#[repr(C)] pub struct dma_tx_state { _private: [u8; 0] }
#[repr(C)] pub struct dma_device { _private: [u8; 0] }
#[repr(C)] pub struct dmaengine_unmap_data { _private: [u8; 0] }
#[repr(C)] pub struct dma_mbus_dram_target_info { _private: [u8; 0] }
#[repr(C)] pub struct platform_device { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct page { _private: [u8; 0] }
#[repr(C)] pub struct tasklet_struct { _private: [u8; 0] }
#[repr(C)] pub struct pm_message_t { _private: [u8; 0] }

#[repr(C)] pub struct mv_xor_desc { pub status: u32, pub phy_next_desc: u32,
    pub desc_command: u32, pub phy_dest_addr: dma_addr_t, pub byte_count: u32,
    pub phy_src_addr: [dma_addr_t; 16] }
#[repr(C)] pub struct mv_xor_desc_slot { pub hw_desc: *mut mv_xor_desc,
    pub async_tx: dma_async_tx_descriptor, pub type_: u32, pub idx: i32 }
#[repr(C)] pub struct mv_xor_chan { pub dmachan: dma_chan, pub xordev: *mut mv_xor_device,
    pub idx: i32, pub irq: i32, pub op_in_desc: MvXorMode, pub pending: i32,
    pub slots_allocated: i32, pub dma_desc_pool: dma_addr_t,
    pub dma_desc_pool_virt: *mut u8, pub dummy_src_addr: dma_addr_t,
    pub dummy_dst_addr: dma_addr_t, pub mmr_base: *mut u8, pub mmr_high_base: *mut u8,
    pub saved_config_reg: u32, pub saved_int_mask_reg: u32,
    pub dummy_src: [u8; 64], pub dummy_dst: [u8; 64] }
#[repr(C)] pub struct mv_xor_device { pub xor_type: MvXorType,
    pub xor_base: *mut u8, pub xor_high_base: *mut u8,
    pub channels: [*mut mv_xor_chan; 4], pub win_start: [u32; 8],
    pub win_end: [u32; 8], pub clk: *mut u8 }

/* Direct translations of the descriptor helpers. */
unsafe fn mv_desc_init(desc: *mut mv_xor_desc_slot, addr: dma_addr_t,
                       byte_count: u32, flags: u32) {
    let hw = (*desc).hw_desc;
    (*hw).status = XOR_DESC_DMA_OWNED;
    (*hw).phy_next_desc = 0;
    (*hw).desc_command = if flags & DMA_PREP_INTERRUPT != 0 { XOR_DESC_EOD_INT_EN } else { 0 };
    (*hw).phy_dest_addr = addr;
    (*hw).byte_count = byte_count;
}
unsafe fn mv_desc_set_mode(desc: *mut mv_xor_desc_slot) {
    match (*desc).type_ {
        DMA_XOR | DMA_INTERRUPT => (*(*desc).hw_desc).desc_command |= XOR_DESC_OPERATION_XOR,
        DMA_MEMCPY => (*(*desc).hw_desc).desc_command |= XOR_DESC_OPERATION_MEMCPY,
        _ => BUG(),
    }
}
unsafe fn mv_desc_set_next_desc(desc: *mut mv_xor_desc_slot, next: u32) {
    BUG_ON((*(*desc).hw_desc).phy_next_desc != 0);
    (*(*desc).hw_desc).phy_next_desc = next;
}
unsafe fn mv_desc_set_src_addr(desc: *mut mv_xor_desc_slot, index: i32, addr: dma_addr_t) {
    (*(*desc).hw_desc).phy_src_addr[mv_phy_src_idx(index) as usize] = addr;
    if (*desc).type_ == DMA_XOR { (*(*desc).hw_desc).desc_command |= 1u32 << index; }
}

/* Channel/MMIO helpers retain the volatile ordering of readl_relaxed/writel_relaxed. */
unsafe fn mv_chan_get_current_desc(chan: *mut mv_xor_chan) -> u32 { readl_relaxed(XOR_CURR_DESC(chan)) }
unsafe fn mv_chan_set_next_descriptor(chan: *mut mv_xor_chan, addr: u32) { writel_relaxed(addr, XOR_NEXT_DESC(chan)); }
unsafe fn mv_chan_unmask_interrupts(chan: *mut mv_xor_chan) { let mut v=readl_relaxed(XOR_INTR_MASK(chan)); v |= XOR_INTR_MASK_VALUE << ((*chan).idx*16); writel_relaxed(v,XOR_INTR_MASK(chan)); }
unsafe fn mv_chan_get_intr_cause(chan: *mut mv_xor_chan) -> u32 { (readl_relaxed(XOR_INTR_CAUSE(chan)) >> ((*chan).idx*16)) & 0xffff }
unsafe fn mv_chan_clear_eoc_cause(chan: *mut mv_xor_chan) { let v=!( (XOR_INT_END_OF_DESC|XOR_INT_END_OF_CHAIN|XOR_INT_STOPPED) << ((*chan).idx*16)); writel_relaxed(v,XOR_INTR_CAUSE(chan)); }
unsafe fn mv_chan_clear_err_status(chan: *mut mv_xor_chan) { writel_relaxed(0xffff0000 >> ((*chan).idx*16),XOR_INTR_CAUSE(chan)); }
unsafe fn mv_chan_set_mode(chan: *mut mv_xor_chan, op: u32) { let mut c=readl_relaxed(XOR_CONFIG(chan)); c &= !7; c |= op; #[cfg(target_endian="big")] { c |= XOR_DESCRIPTOR_SWAP; } #[cfg(not(target_endian="big"))] { c &= !XOR_DESCRIPTOR_SWAP; } writel_relaxed(c,XOR_CONFIG(chan)); }
unsafe fn mv_chan_activate(chan: *mut mv_xor_chan) { writel(BIT(0),XOR_ACTIVATION(chan)); }
unsafe fn mv_chan_is_busy(chan: *mut mv_xor_chan) -> u8 { if ((readl_relaxed(XOR_ACTIVATION(chan))>>4)&3)==1 {1} else {0} }

/* DMA API entry points and platform-driver routines are kept as declarations
 * where their implementations require the Linux DMA-engine object layout. */
extern "C" {
    fn mv_xor_tx_submit(tx: *mut dma_async_tx_descriptor) -> dma_cookie_t;
    fn mv_xor_alloc_chan_resources(chan: *mut dma_chan) -> i32;
    fn mv_xor_free_chan_resources(chan: *mut dma_chan);
    fn mv_xor_status(chan: *mut dma_chan, cookie: dma_cookie_t, state: *mut dma_tx_state) -> i32;
    fn mv_xor_prep_dma_xor(chan: *mut dma_chan, dest: dma_addr_t, src: *mut dma_addr_t, src_cnt: u32, len: usize, flags: usize) -> *mut dma_async_tx_descriptor;
    fn mv_xor_prep_dma_memcpy(chan: *mut dma_chan, dest: dma_addr_t, src: dma_addr_t, len: usize, flags: usize) -> *mut dma_async_tx_descriptor;
    fn mv_xor_prep_dma_interrupt(chan: *mut dma_chan, flags: usize) -> *mut dma_async_tx_descriptor;
    fn mv_xor_channel_remove(chan: *mut mv_xor_chan) -> i32;
    fn mv_xor_channel_add(dev: *mut mv_xor_device, pdev: *mut platform_device, idx: i32, mask: dma_cap_mask_t, irq: i32) -> *mut mv_xor_chan;
    fn mv_xor_suspend(pdev: *mut platform_device, state: pm_message_t) -> i32;
    fn mv_xor_resume(dev: *mut platform_device) -> i32;
    fn mv_xor_probe(pdev: *mut platform_device) -> i32;
}

// Constants/macros and kernel-provided operations referenced above are supplied
// by dmaengine.h, mv_xor.h, and the Linux kernel build environment.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
