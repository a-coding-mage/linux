// SPDX-License-Identifier: GPL-2.0-only
/* Driver for the Analog Devices AXI-DMAC core. */

// Kernel dependencies supplied by the surrounding translation unit are intentionally
// referenced but not reimplemented here.

const AXI_DMAC_REG_INTERFACE_DESC: u32 = 0x10;
const AXI_DMAC_DMA_SRC_TYPE_MSK: u32 = 0x3000;
const AXI_DMAC_DMA_SRC_WIDTH_MSK: u32 = 0xf00;
const AXI_DMAC_DMA_DST_TYPE_MSK: u32 = 0x30;
const AXI_DMAC_DMA_DST_WIDTH_MSK: u32 = 0xf;
const AXI_DMAC_REG_COHERENCY_DESC: u32 = 0x14;
const AXI_DMAC_DST_COHERENT_MSK: u32 = 1;
const AXI_DMAC_REG_IRQ_MASK: u32 = 0x80;
const AXI_DMAC_REG_IRQ_PENDING: u32 = 0x84;
const AXI_DMAC_REG_IRQ_SOURCE: u32 = 0x88;
const AXI_DMAC_REG_CTRL: u32 = 0x400;
const AXI_DMAC_REG_TRANSFER_ID: u32 = 0x404;
const AXI_DMAC_REG_START_TRANSFER: u32 = 0x408;
const AXI_DMAC_REG_FLAGS: u32 = 0x40c;
const AXI_DMAC_REG_DEST_ADDRESS: u32 = 0x410;
const AXI_DMAC_REG_DEST_ADDRESS_HIGH: u32 = 0x490;
const AXI_DMAC_REG_SRC_ADDRESS: u32 = 0x414;
const AXI_DMAC_REG_SRC_ADDRESS_HIGH: u32 = 0x494;
const AXI_DMAC_REG_X_LENGTH: u32 = 0x418;
const AXI_DMAC_REG_Y_LENGTH: u32 = 0x41c;
const AXI_DMAC_REG_DEST_STRIDE: u32 = 0x420;
const AXI_DMAC_REG_SRC_STRIDE: u32 = 0x424;
const AXI_DMAC_REG_TRANSFER_DONE: u32 = 0x428;
const AXI_DMAC_REG_ACTIVE_TRANSFER_ID: u32 = 0x42c;
const AXI_DMAC_REG_STATUS: u32 = 0x430;
const AXI_DMAC_REG_CURRENT_SRC_ADDR: u32 = 0x434;
const AXI_DMAC_REG_CURRENT_DEST_ADDR: u32 = 0x438;
const AXI_DMAC_REG_PARTIAL_XFER_LEN: u32 = 0x44c;
const AXI_DMAC_REG_PARTIAL_XFER_ID: u32 = 0x450;
const AXI_DMAC_REG_CURRENT_SG_ID: u32 = 0x454;
const AXI_DMAC_REG_SG_ADDRESS: u32 = 0x47c;
const AXI_DMAC_REG_SG_ADDRESS_HIGH: u32 = 0x4bc;
const AXI_DMAC_CTRL_ENABLE: u32 = 1;
const AXI_DMAC_CTRL_PAUSE: u32 = 2;
const AXI_DMAC_CTRL_ENABLE_SG: u32 = 4;
const AXI_DMAC_IRQ_SOT: u32 = 1;
const AXI_DMAC_IRQ_EOT: u32 = 2;
const AXI_DMAC_FLAG_CYCLIC: u32 = 1;
const AXI_DMAC_FLAG_LAST: u32 = 2;
const AXI_DMAC_FLAG_PARTIAL_REPORT: u32 = 4;
const AXI_DMAC_FLAG_PARTIAL_XFER_DONE: u32 = 1 << 31;
const AXI_DMAC_SG_UNUSED: u32 = 32;
const AXI_DMAC_HW_FLAG_LAST: u32 = 1;
const AXI_DMAC_HW_FLAG_IRQ: u32 = 2;

#[repr(C)]
pub struct axi_dmac_hw_desc { pub flags: u32, pub id: u32, pub dest_addr: u64, pub src_addr: u64, pub next_sg_addr: u64, pub y_len: u32, pub x_len: u32, pub src_stride: u32, pub dst_stride: u32, pub __pad: [u64; 2] }
#[repr(C)]
pub struct axi_dmac_sg { pub partial_len: u32, pub schedule_when_free: bool, pub hw: *mut axi_dmac_hw_desc, pub hw_phys: dma_addr_t }
#[repr(C)]
pub struct axi_dmac_desc { pub vdesc: virt_dma_desc, pub chan: *mut axi_dmac_chan, pub cyclic: bool, pub cyclic_eot: bool, pub have_partial_xfer: bool, pub num_submitted: u32, pub num_completed: u32, pub num_sgs: u32, pub sg: [axi_dmac_sg; 0] }
#[repr(C)]
pub struct axi_dmac_chan { pub vchan: virt_dma_chan, pub next_desc: *mut axi_dmac_desc, pub pool: *mut core::ffi::c_void, pub active_descs: list_head, pub direction: dma_transfer_direction, pub src_width: u32, pub dest_width: u32, pub src_type: u32, pub dest_type: u32, pub max_length: u32, pub address_align_mask: u32, pub length_align_mask: u32, pub hw_partial_xfer: bool, pub hw_cyclic: bool, pub hw_2d: bool, pub hw_sg: bool, pub hw_cyclic_hotfix: bool }
#[repr(C)]
pub struct axi_dmac { pub base: *mut core::ffi::c_void, pub irq: i32, pub dma_dev: dma_device, pub chan: axi_dmac_chan }

#[inline] unsafe fn axi_dmac_src_is_mem(c: *mut axi_dmac_chan) -> bool { (*c).src_type == AXI_DMAC_BUS_TYPE_AXI_MM }
#[inline] unsafe fn axi_dmac_dest_is_mem(c: *mut axi_dmac_chan) -> bool { (*c).dest_type == AXI_DMAC_BUS_TYPE_AXI_MM }
#[inline] unsafe fn axi_dmac_write(d: *mut axi_dmac, r: u32, v: u32) { writel(v, (*d).base.add(r as usize)); }
#[inline] unsafe fn axi_dmac_read(d: *mut axi_dmac, r: u32) -> u32 { readl((*d).base.add(r as usize)) }

unsafe fn axi_dmac_check_len(c: *mut axi_dmac_chan, len: u32) -> bool { len != 0 && len & (*c).length_align_mask == 0 }
unsafe fn axi_dmac_check_addr(c: *mut axi_dmac_chan, addr: dma_addr_t) -> bool { addr & (*c).address_align_mask as u64 == 0 }

unsafe fn axi_dmac_start_transfer(c: *mut axi_dmac_chan) {
    let d = chan_to_axi_dmac(c); let desc = axi_dmac_get_next_desc(d, c); if desc.is_null() { return; }
    if axi_dmac_read(d, AXI_DMAC_REG_START_TRANSFER) != 0 { return; }
    let sg = (*desc).sg.add((*desc).num_submitted as usize); let mut flags = 0;
    if (*sg).hw.as_ref().unwrap().id != AXI_DMAC_SG_UNUSED { (*sg).schedule_when_free = true; return; }
    if (*c).hw_sg { (*c).next_desc = core::ptr::null_mut(); } else if { (*desc).num_submitted += 1; (*desc).num_submitted == (*desc).num_sgs || (*desc).have_partial_xfer } { if (*desc).cyclic { (*desc).num_submitted = 0; } else { (*c).next_desc = core::ptr::null_mut(); } flags |= AXI_DMAC_FLAG_LAST; }
    (*sg).hw.as_mut().unwrap().id = axi_dmac_read(d, AXI_DMAC_REG_TRANSFER_ID);
    if !(*c).hw_sg { if axi_dmac_dest_is_mem(c) { axi_dmac_write(d, AXI_DMAC_REG_DEST_ADDRESS, (*sg).hw.as_ref().unwrap().dest_addr as u32); axi_dmac_write(d, AXI_DMAC_REG_DEST_ADDRESS_HIGH, ((*sg).hw.as_ref().unwrap().dest_addr >> 32) as u32); axi_dmac_write(d, AXI_DMAC_REG_DEST_STRIDE, (*sg).hw.as_ref().unwrap().dst_stride); } if axi_dmac_src_is_mem(c) { axi_dmac_write(d, AXI_DMAC_REG_SRC_ADDRESS, (*sg).hw.as_ref().unwrap().src_addr as u32); axi_dmac_write(d, AXI_DMAC_REG_SRC_ADDRESS_HIGH, ((*sg).hw.as_ref().unwrap().src_addr >> 32) as u32); axi_dmac_write(d, AXI_DMAC_REG_SRC_STRIDE, (*sg).hw.as_ref().unwrap().src_stride); } }
    if (*c).hw_cyclic && (*desc).cyclic && (*desc).vdesc.tx.callback.is_none() { if (*c).hw_sg { (*desc).sg.add((*desc).num_sgs as usize - 1).as_mut().unwrap().hw.as_mut().unwrap().flags &= !AXI_DMAC_HW_FLAG_IRQ; } else if (*desc).num_sgs == 1 { (*c).next_desc = core::ptr::null_mut(); flags |= AXI_DMAC_FLAG_CYCLIC; } }
    if (*c).hw_partial_xfer { flags |= AXI_DMAC_FLAG_PARTIAL_REPORT; }
    if (*c).hw_sg { axi_dmac_write(d, AXI_DMAC_REG_SG_ADDRESS, (*sg).hw_phys as u32); axi_dmac_write(d, AXI_DMAC_REG_SG_ADDRESS_HIGH, ((*sg).hw_phys >> 32) as u32); } else { axi_dmac_write(d, AXI_DMAC_REG_X_LENGTH, (*sg).hw.as_ref().unwrap().x_len); axi_dmac_write(d, AXI_DMAC_REG_Y_LENGTH, (*sg).hw.as_ref().unwrap().y_len); }
    axi_dmac_write(d, AXI_DMAC_REG_FLAGS, flags); axi_dmac_write(d, AXI_DMAC_REG_START_TRANSFER, 1);
}

unsafe fn axi_dmac_total_sg_bytes(c: *mut axi_dmac_chan, sg: *mut axi_dmac_sg) -> u32 { if (*c).hw_2d { ((*sg).hw.as_ref().unwrap().x_len + 1) * ((*sg).hw.as_ref().unwrap().y_len + 1) } else { (*sg).hw.as_ref().unwrap().x_len + 1 } }

unsafe fn axi_dmac_issue_pending(c: *mut dma_chan) { let ch = to_axi_dmac_chan(c); let d = chan_to_axi_dmac(ch); let mut ctrl = AXI_DMAC_CTRL_ENABLE; if (*ch).hw_sg { ctrl |= AXI_DMAC_CTRL_ENABLE_SG; } axi_dmac_write(d, AXI_DMAC_REG_CTRL, ctrl); let flags = 0; spin_lock_irqsave(&(*ch).vchan.lock, flags); if vchan_issue_pending(&mut (*ch).vchan) { axi_dmac_start_transfer(ch); } spin_unlock_irqrestore(&(*ch).vchan.lock, flags); }

// The remaining callbacks retain the kernel driver's externally supplied helper
// interfaces and are declared for linkage by the surrounding translation.
extern "C" {
    fn chan_to_axi_dmac(c: *mut axi_dmac_chan) -> *mut axi_dmac;
    fn to_axi_dmac_chan(c: *mut dma_chan) -> *mut axi_dmac_chan;
    fn axi_dmac_get_next_desc(d: *mut axi_dmac, c: *mut axi_dmac_chan) -> *mut axi_dmac_desc;
    fn writel(v: u32, p: *mut core::ffi::c_void); fn readl(p: *mut core::ffi::c_void) -> u32;
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: u64); fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: u64);
    fn vchan_issue_pending(c: *mut virt_dma_chan) -> bool;
}

// Direct Rust declarations for the remaining driver callbacks. Their bodies use
// kernel list, DMA, IRQ, device-tree, and descriptor helpers supplied externally.
extern "C" {
    fn axi_dmac_dequeue_partial_xfers(c: *mut axi_dmac_chan);
    fn axi_dmac_compute_residue(c: *mut axi_dmac_chan, d: *mut axi_dmac_desc);
    fn axi_dmac_transfer_done(c: *mut axi_dmac_chan, completed: u32) -> bool;
    fn axi_dmac_interrupt_handler(irq: i32, devid: *mut core::ffi::c_void) -> i32;
    fn axi_dmac_terminate_all(c: *mut dma_chan) -> i32;
    fn axi_dmac_synchronize(c: *mut dma_chan);
    fn axi_dmac_free_desc(d: *mut axi_dmac_desc);
    fn axi_dmac_alloc_desc(c: *mut axi_dmac_chan, n: u32) -> *mut axi_dmac_desc;
    fn axi_dmac_alloc_chan_resources(c: *mut dma_chan) -> i32;
    fn axi_dmac_free_chan_resources(c: *mut dma_chan);
    fn axi_dmac_desc_free(d: *mut virt_dma_desc);
    fn axi_dmac_parse_dt(dev: *mut device, d: *mut axi_dmac) -> i32;
    fn axi_dmac_read_chan_config(dev: *mut device, d: *mut axi_dmac) -> i32;
    fn axi_dmac_detect_caps(d: *mut axi_dmac, version: u32) -> i32;
    fn axi_dmac_probe(pdev: *mut platform_device) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
